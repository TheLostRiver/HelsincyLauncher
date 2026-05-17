use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use launcher_kernel_foundation::PageRequest;
use launcher_kernel_jobs::JobPriority;
use launcher_module_downloads::contracts::{
    CancelDownloadRequestDto, GetDownloadPolicyQueryDto, PauseDownloadRequestDto,
    StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
};
use launcher_module_engines::contracts::RunEngineVerificationRequestDto;
use launcher_module_fab::contracts::FabInventoryListQueryDto;
use my_epic_launcher_desktop::{build_desktop_host_bootstrap, commands};

#[test]
fn transport_wiring_smoke() {
    let bootstrap =
        build_desktop_host_bootstrap().expect("host bootstrap should wire desktop services in E2");

    assert!(bootstrap.services.is_wired_to_composition_root());
    assert_eq!(bootstrap.registered_commands, commands::REGISTERED_COMMANDS);

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
