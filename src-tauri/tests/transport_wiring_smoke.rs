use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use launcher_kernel_foundation::PageRequest;
use launcher_module_fab::contracts::FabInventoryListQueryDto;
use my_epic_launcher_desktop::{build_desktop_host_bootstrap, commands};

#[test]
fn transport_wiring_smoke() {
    let bootstrap = build_desktop_host_bootstrap()
        .expect("host bootstrap should wire desktop services in E2");

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
            panic!("transport command path should stay callable, got {}", error.code);
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