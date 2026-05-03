use std::path::Path;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};

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