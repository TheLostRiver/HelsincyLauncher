use std::path::Path;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};
use launcher_kernel_jobs::{EnqueueJobRequest, JobPriority, JobRuntime, JobState, JobUiState};
use launcher_module_downloads::DownloadCheckpointRecord;
use launcher_module_downloads::contracts::StartDownloadRequestDto;
use launcher_module_engines::contracts::RunEngineVerificationRequestDto;
use launcher_module_fab::contracts::FabInventoryPrewarmRequestDto;

#[test]
fn bootstrap_wiring_smoke() {
    let services = build_desktop_services(DesktopBootstrapConfig::default())
        .expect("composition root should assemble desktop app services in D2");

    assert_eq!(
        services.fab.deps().projection_repo.config().database_path(),
        Path::new("launcher.sqlite3")
    );
    assert_eq!(
        services.downloads.deps().job_repo.config().database_path(),
        Path::new("launcher.sqlite3")
    );
    assert_eq!(
        services
            .fab
            .deps()
            .catalog_provider
            .config()
            .client_name(),
        "my-epic-launcher-desktop"
    );

    let accepted_job = services
        .fab
        .run_startup_prewarm(FabInventoryPrewarmRequestDto {
            reason: "bootstrap-smoke".into(),
        })
        .expect("bootstrap wiring should expose a Fab prewarm path backed by the shared runtime host");

    let snapshot = services
        .fab
        .deps()
        .job_runtime
        .snapshot(&accepted_job.job_id)
        .expect("shared runtime host should allow reading the queued snapshot")
        .expect("shared runtime host should store the queued prewarm snapshot");

    assert_eq!(snapshot.state, JobState::Queued);
    assert_eq!(snapshot.ui_state, JobUiState::Queued);

    block_on_ready(services.startup.run_stage3_background_prewarm())
        .expect("startup stage-3 prewarm should stay callable in the composition-root baseline");
}

fn block_on_ready<F>(future: F) -> F::Output
where
    F: Future,
{
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("composition-root startup future should be ready without a runtime"),
    }
}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

fn noop_raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker {
        noop_raw_waker()
    }

    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}

    RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
    )
}

#[test]
fn runtime_snapshot_persists_across_rebuilds() {
    let tmp_path = std::env::temp_dir().join("at045_runtime_snapshot_test.sqlite3");
    // Clean up any leftover file from a previous run so the test is idempotent.
    // 清理上一次运行可能残留的文件，让该测试保持幂等。
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    // First build: enqueue a Fab prewarm job.
    // 第一次构建：入队一个 Fab prewarm job。
    let services1 = build_desktop_services(config.clone())
        .expect("first build_desktop_services should succeed");

    let accepted = services1
        .fab
        .run_startup_prewarm(FabInventoryPrewarmRequestDto {
            reason: "at045-persist-test".into(),
        })
        .expect("run_startup_prewarm on first build should succeed");

    let job_id = accepted.job_id.clone();

    // Verify the snapshot is already readable in the first instance.
    // 确认第一次实例中已经可以读取该 snapshot。
    services1
        .fab
        .deps()
        .job_runtime
        .snapshot(&job_id)
        .expect("snapshot call on first runtime should not error")
        .expect("snapshot should exist in first runtime host");

    // Drop the first services to release the sqlite connection.
    // 丢弃第一次服务图以释放 sqlite 连接。
    drop(services1);

    // Second build: rebuild with the same sqlite path.
    // 第二次构建：使用同一个 sqlite 路径重新装配。
    let services2 = build_desktop_services(config)
        .expect("second build_desktop_services should succeed");

    let recovered = services2
        .fab
        .deps()
        .job_runtime
        .snapshot(&job_id)
        .expect("snapshot call on second runtime should not error")
        .expect("snapshot should still be readable after rebuild from same sqlite path");

    assert_eq!(
        recovered.job_id, job_id,
        "recovered snapshot job_id must match the original"
    );

    // Clean up temp file.
    // 清理临时文件。
    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn stage2_restore_reads_resumable_snapshots() {
    let tmp_path = std::env::temp_dir().join("at046_stage2_restore_test.sqlite3");
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    // Build services, enqueue a job, then call stage-2 restore — it should not error.
    // 构建服务、入队作业，然后调用 stage-2 restore；该路径不应报错。
    let services = build_desktop_services(config.clone())
        .expect("build_desktop_services should succeed for stage-2 test");

    services
        .fab
        .run_startup_prewarm(FabInventoryPrewarmRequestDto {
            reason: "at046-stage2-test".into(),
        })
        .expect("prewarm should be accepted");

    block_on_ready(services.startup.run_stage2_restore_runtime_state())
        .expect("stage-2 restore should succeed when a resumable job exists in the store");

    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn stage2_restore_marks_download_job_failed_without_checkpoint() {
    let tmp_path = std::env::temp_dir().join("at052_download_restore_missing_checkpoint.sqlite3");
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    let services = build_desktop_services(config)
        .expect("build_desktop_services should succeed for download restore test");

    let job_id = launcher_kernel_foundation::JobId::generate();
    services
        .downloads
        .deps()
        .job_runtime
        .enqueue(EnqueueJobRequest {
            job_id: job_id.clone(),
            module: "downloads".into(),
            kind: "download".into(),
            priority: JobPriority::Normal,
            recoverable: true,
            extension: None,
        })
        .expect("seeding a queued download job should succeed");

    block_on_ready(services.startup.run_stage2_restore_runtime_state())
        .expect("stage-2 restore should succeed for queued download job");

    let recovered = services
        .downloads
        .deps()
        .job_runtime
        .snapshot(&job_id)
        .expect("reading restored download snapshot should not error")
        .expect("queued download snapshot should still exist after restore");

    assert_eq!(
        recovered.state,
        JobState::Failed,
        "queued download without checkpoint should be marked Failed by the restore driver"
    );

    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn stage2_restore_keeps_download_job_queued_with_checkpoint() {
    let tmp_path = std::env::temp_dir().join("at052_download_restore_with_checkpoint.sqlite3");
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    let services = build_desktop_services(config)
        .expect("build_desktop_services should succeed for checkpoint-backed restore test");

    let job_id = launcher_kernel_foundation::JobId::generate();
    services
        .downloads
        .deps()
        .checkpoint_repo
        .save_checkpoint(&DownloadCheckpointRecord {
            job_id: job_id.clone(),
        })
        .expect("seeding a synthetic download checkpoint should succeed");

    services
        .downloads
        .deps()
        .job_runtime
        .enqueue(EnqueueJobRequest {
            job_id: job_id.clone(),
            module: "downloads".into(),
            kind: "download".into(),
            priority: JobPriority::Normal,
            recoverable: true,
            extension: None,
        })
        .expect("seeding a queued download job should succeed");

    block_on_ready(services.startup.run_stage2_restore_runtime_state())
        .expect("stage-2 restore should succeed for checkpoint-backed download job");

    let recovered = services
        .downloads
        .deps()
        .job_runtime
        .snapshot(&job_id)
        .expect("reading restored download snapshot should not error")
        .expect("queued download snapshot should still exist after restore");

    assert_eq!(
        recovered.state,
        JobState::Queued,
        "queued download with checkpoint should remain resumable after restore"
    );

    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn downloads_start_persists_request_metadata() {
    let tmp_path = std::env::temp_dir().join("at053_download_start_persists_metadata.sqlite3");
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    let services = build_desktop_services(config)
        .expect("build_desktop_services should succeed for start_download persistence test");

    let request = StartDownloadRequestDto {
        target_id: "asset-123".into(),
        kind: "engine".into(),
        install_intent: Some("install".into()),
        priority: JobPriority::High,
    };

    let accepted = services
        .downloads
        .start_download(request.clone())
        .expect("downloads facade should accept the start request");

    let persisted = services
        .downloads
        .deps()
        .job_repo
        .get_job(&accepted.job_id)
        .expect("reading the persisted download job should not error")
        .expect("start_download should persist the download job metadata");

    assert_eq!(persisted.job_id, accepted.job_id);
    assert_eq!(persisted.target_id, request.target_id);
    assert_eq!(persisted.kind, request.kind);
    assert_eq!(persisted.install_intent, request.install_intent);
    assert_eq!(persisted.priority, request.priority);

    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn engines_run_verification_enqueues_job() {
    let tmp_path = std::env::temp_dir().join("at054_engine_verification_job.sqlite3");
    let _ = std::fs::remove_file(&tmp_path);

    let config = DesktopBootstrapConfig {
        sqlite_path: tmp_path.clone(),
        ..DesktopBootstrapConfig::default()
    };

    let services = build_desktop_services(config)
        .expect("build_desktop_services should succeed for engine verification test");

    let accepted = services
        .engines
        .run_verification(RunEngineVerificationRequestDto {
            engine_id: "ue-5.4".into(),
        })
        .expect("engine verification should return an accepted job");

    let snapshot = services
        .engines
        .deps()
        .job_runtime
        .snapshot(&accepted.job_id)
        .expect("reading the engine verification snapshot should not error")
        .expect("engine verification should be stored in the shared runtime host");

    assert_eq!(accepted.module, "engines");
    assert_eq!(accepted.kind, "verification");
    assert_eq!(snapshot.state, JobState::Queued);
    assert_eq!(snapshot.ui_state, JobUiState::Queued);
    assert_eq!(snapshot.module, "engines");
    assert_eq!(snapshot.kind, "verification");

    let _ = std::fs::remove_file(&tmp_path);
}
