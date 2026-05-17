use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};
use launcher_kernel_foundation::PageRequest;
use launcher_kernel_jobs::{JobPriority, JobState, JobUiState};
use launcher_module_downloads::contracts::{
    CancelDownloadRequestDto, GetDownloadPolicyQueryDto, PauseDownloadRequestDto,
    StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
};
use launcher_module_engines::contracts::RunEngineVerificationRequestDto;
use launcher_module_fab::contracts::FabInventoryListQueryDto;
use my_epic_launcher_desktop::{build_desktop_host_bootstrap, commands, DesktopAppServicesHandle};

#[test]
fn transport_wiring_smoke() {
    let bootstrap =
        build_desktop_host_bootstrap().expect("host bootstrap should wire desktop services in E2");

    assert!(bootstrap.services.is_wired_to_composition_root());
    assert_eq!(bootstrap.registered_commands, commands::REGISTERED_COMMANDS);
    assert!(bootstrap
        .registered_commands
        .contains(&"jobs_run_next_execution_turn"));

    // Use an isolated host graph so the no-queued expectation is not polluted by the default smoke database.
    // 使用隔离的 host 服务图，避免默认 smoke 数据库污染 no-queued 断言。
    let (runtime_services, runtime_sqlite_path) =
        isolated_desktop_services("at236_runtime_execution_command.sqlite3");
    let runtime_turn = block_on_ready(commands::jobs::jobs_run_next_execution_turn(
        runtime_services.services(),
    ));

    match runtime_turn {
        commands::CommandResultDto::Success { data } => {
            assert_eq!(
                data.disposition,
                commands::RuntimeExecutionTurnDispositionDto::Deferred
            );
            assert!(data
                .reason
                .as_deref()
                .expect("deferred execution turn should include a reason")
                .contains("no queued job"));
        }
        commands::CommandResultDto::Failure { error } => {
            panic!(
                "runtime execution command should return a disposition DTO, got {}",
                error.code
            );
        }
    }
    drop(runtime_services);
    cleanup_sqlite_path(&runtime_sqlite_path);

    // Production downloads execution is intentionally deferred until a real segment execution port exists.
    // 在真实 segment execution port 接入前，生产 downloads 执行应明确保持 deferred。
    let (download_runtime_services, download_runtime_sqlite_path) =
        isolated_desktop_services("at237_downloads_runtime_deferred.sqlite3");
    let queued_download = block_on_ready(commands::downloads::downloads_start(
        download_runtime_services.services(),
        StartDownloadRequestDto {
            target_id: "ue-5.5".into(),
            kind: "engine".into(),
            install_intent: None,
            priority: JobPriority::Normal,
        },
    ));
    let queued_download_job_id = match queued_download {
        commands::CommandResultDto::Success { data } => data.job_id,
        commands::CommandResultDto::Failure { error } => {
            panic!(
                "isolated downloads start should queue runtime work, got {}",
                error.code
            );
        }
    };
    let download_turn = block_on_ready(commands::jobs::jobs_run_next_execution_turn(
        download_runtime_services.services(),
    ));

    match download_turn {
        commands::CommandResultDto::Success { data } => {
            assert_eq!(
                data.disposition,
                commands::RuntimeExecutionTurnDispositionDto::Deferred
            );
            assert!(data
                .reason
                .as_deref()
                .expect("downloads deferred turn should include a reason")
                .contains("execution port not wired"));
        }
        commands::CommandResultDto::Failure { error } => {
            panic!(
                "downloads deferred execution should remain a successful command DTO, got {}",
                error.code
            );
        }
    }
    let queued_snapshot = download_runtime_services
        .services()
        .snapshot_store
        .get(&queued_download_job_id)
        .expect("queued download snapshot should remain readable")
        .expect("queued download snapshot should still exist");
    assert_eq!(queued_snapshot.state, JobState::Queued);
    assert_eq!(queued_snapshot.ui_state, JobUiState::Queued);
    drop(download_runtime_services);
    cleanup_sqlite_path(&download_runtime_sqlite_path);

    let result = block_on_ready(commands::fab::fab_list_inventory(
        bootstrap.services.services(),
        FabInventoryListQueryDto {
            page: PageRequest::new(20, None),
            search: None,
            category_key: None,
            include_incompatible: false,
        },
    ));

    match result {
        commands::QueryResultDto::Success { data, .. } => {
            assert!(data.items.is_empty());
        }
        commands::QueryResultDto::Failure { error } => {
            panic!(
                "transport command path should stay callable, got {}",
                error.code
            );
        }
    }

    let download_start = block_on_ready(commands::downloads::downloads_start(
        bootstrap.services.services(),
        StartDownloadRequestDto {
            target_id: "ue-5.4".into(),
            kind: "engine".into(),
            install_intent: None,
            priority: JobPriority::Normal,
        },
    ));

    let download_job_id = match download_start {
        commands::CommandResultDto::Success { data } => {
            assert!(data.accepted);
            assert_eq!(data.module, "downloads");
            assert_eq!(data.kind, "download");
            data.job_id
        }
        commands::CommandResultDto::Failure { error } => {
            panic!(
                "downloads start command path should stay callable, got {}",
                error.code
            );
        }
    };

    let pause = block_on_ready(commands::downloads::downloads_pause(
        bootstrap.services.services(),
        PauseDownloadRequestDto {
            job_id: download_job_id.clone(),
        },
    ));

    if let commands::CommandResultDto::Failure { error } = pause {
        panic!(
            "downloads pause command path should stay callable, got {}",
            error.code
        );
    }

    let cancel = block_on_ready(commands::downloads::downloads_cancel(
        bootstrap.services.services(),
        CancelDownloadRequestDto {
            job_id: download_job_id,
        },
    ));

    if let commands::CommandResultDto::Failure { error } = cancel {
        panic!(
            "downloads cancel command path should stay callable, got {}",
            error.code
        );
    }

    // Keep this smoke at the host boundary: command envelope plus runtime policy readback.
    // 将这个 smoke 保持在宿主边界：同时验证命令 envelope 和 runtime 策略回读。
    let policy_update = block_on_ready(commands::downloads::downloads_update_policy(
        bootstrap.services.services(),
        UpdateDownloadPolicyRequestDto {
            concurrency_slots: 11,
            bandwidth_limit_bytes_per_sec: Some(4096),
            auto_resume: true,
        },
    ));

    if let commands::CommandResultDto::Failure { error } = policy_update {
        panic!(
            "downloads policy update command path should stay callable, got {}",
            error.code
        );
    }

    let policy_read = block_on_ready(commands::downloads::downloads_get_policy(
        bootstrap.services.services(),
        GetDownloadPolicyQueryDto::default(),
    ));

    match policy_read {
        commands::QueryResultDto::Success { data, .. } => {
            assert_eq!(data.concurrency_slots, 11);
            assert_eq!(data.bandwidth_limit_bytes_per_sec, Some(4096));
            assert!(data.auto_resume);
        }
        commands::QueryResultDto::Failure { error } => {
            panic!(
                "downloads policy get command path should stay callable, got {}",
                error.code
            );
        }
    }

    assert_eq!(
        bootstrap
            .services
            .services()
            .downloads
            .deps()
            .job_runtime
            .policy()
            .max_concurrent_jobs,
        11
    );

    let verification = block_on_ready(commands::engines::engines_run_verification(
        bootstrap.services.services(),
        RunEngineVerificationRequestDto {
            engine_id: "ue-5.4".into(),
        },
    ));

    match verification {
        commands::CommandResultDto::Success { data } => {
            assert!(data.accepted);
            assert_eq!(data.module, "engines");
            assert_eq!(data.kind, "verification");
        }
        commands::CommandResultDto::Failure { error } => {
            panic!(
                "engines verification command path should stay callable, got {}",
                error.code
            );
        }
    }
}

fn isolated_desktop_services(name: &str) -> (DesktopAppServicesHandle, PathBuf) {
    let sqlite_path = project_local_sqlite_path(name);
    let services = build_desktop_services(DesktopBootstrapConfig::new(
        "app-data",
        "cache",
        "logs",
        sqlite_path.clone(),
    ))
    .expect("isolated host services should wire with a project-local sqlite path");

    (
        DesktopAppServicesHandle::from_services(services),
        sqlite_path,
    )
}

fn project_local_sqlite_path(name: &str) -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .expect("src-tauri manifest should live under the workspace root");
    let tmp_dir = workspace_root.join(".artifacts").join("tmp");
    std::fs::create_dir_all(&tmp_dir).expect("project-local temp directory should be creatable");
    let sqlite_path = tmp_dir.join(name);
    cleanup_sqlite_path(&sqlite_path);
    sqlite_path
}

fn cleanup_sqlite_path(path: &Path) {
    let _ = std::fs::remove_file(path);
    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        let _ = std::fs::remove_file(path.with_file_name(format!("{file_name}-wal")));
        let _ = std::fs::remove_file(path.with_file_name(format!("{file_name}-shm")));
    }
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
        Poll::Pending => panic!("transport smoke future should be ready without a runtime"),
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
