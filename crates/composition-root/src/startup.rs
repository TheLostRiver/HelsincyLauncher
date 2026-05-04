use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{AcceptedJob, JobSnapshotStore};
use launcher_module_fab::{
    contracts::FabInventoryPrewarmRequestDto,
    facade::FabStartupPrewarmJobAcceptance,
    FabFacade,
};

pub trait FabStartupPrewarmPort: Send + Sync {
    fn run_startup_prewarm(&self, request: FabInventoryPrewarmRequestDto) -> AppResult<AcceptedJob>;
}

impl<P, C, M, J, K> FabStartupPrewarmPort for FabFacade<P, C, M, J, K>
where
    P: Send + Sync,
    C: Send + Sync,
    M: Send + Sync,
    J: FabStartupPrewarmJobAcceptance + Send + Sync,
    K: Send + Sync,
{
    fn run_startup_prewarm(&self, request: FabInventoryPrewarmRequestDto) -> AppResult<AcceptedJob> {
        FabFacade::run_startup_prewarm(self, request)
    }
}

#[derive(Clone)]
pub struct StartupPipelineFacade {
    enable_startup_prewarm: bool,
    fab_prewarm: Option<Arc<dyn FabStartupPrewarmPort>>,
    snapshot_store: Option<Arc<dyn JobSnapshotStore<()>>>,
}

impl Debug for StartupPipelineFacade {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StartupPipelineFacade")
            .field("enable_startup_prewarm", &self.enable_startup_prewarm)
            .field("has_fab_prewarm", &self.fab_prewarm.is_some())
            .field("has_snapshot_store", &self.snapshot_store.is_some())
            .finish()
    }
}

impl Default for StartupPipelineFacade {
    fn default() -> Self {
        Self::new(false, None, None)
    }
}

impl StartupPipelineFacade {
    pub fn new(
        enable_startup_prewarm: bool,
        fab_prewarm: Option<Arc<dyn FabStartupPrewarmPort>>,
        snapshot_store: Option<Arc<dyn JobSnapshotStore<()>>>,
    ) -> Self {
        Self {
            enable_startup_prewarm,
            fab_prewarm,
            snapshot_store,
        }
    }

    pub async fn run_stage1_shell_ready(&self) -> AppResult<()> {
        Ok(())
    }

    pub async fn run_stage2_restore_runtime_state(&self) -> AppResult<()> {
        if let Some(store) = self.snapshot_store.as_ref() {
            let resumable = store.list_resumable()?;
            tracing::info!(
                stage = "stage2_restore",
                resumable_count = resumable.len(),
                "stage-2 runtime state restore: found {} resumable job(s)",
                resumable.len()
            );
        }
        Ok(())
    }

    pub async fn run_stage3_background_prewarm(&self) -> AppResult<()> {
        if !self.enable_startup_prewarm {
            return Ok(());
        }

        if let Some(fab_prewarm) = self.fab_prewarm.as_ref() {
            let _ = fab_prewarm.run_startup_prewarm(FabInventoryPrewarmRequestDto {
                reason: "startup-stage-3".into(),
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};
    use launcher_kernel_jobs::AcceptedJob;
    use launcher_module_fab::contracts::FabInventoryPrewarmRequestDto;

    use super::{FabStartupPrewarmPort, StartupPipelineFacade};

    #[derive(Debug, Default)]
    struct RecordingFabPrewarmPort {
        captured_requests: Mutex<Vec<FabInventoryPrewarmRequestDto>>,
    }

    impl FabStartupPrewarmPort for RecordingFabPrewarmPort {
        fn run_startup_prewarm(
            &self,
            request: FabInventoryPrewarmRequestDto,
        ) -> AppResult<AcceptedJob> {
            self.captured_requests
                .lock()
                .expect("prewarm request mutex should not be poisoned")
                .push(request);

            Ok(AcceptedJob {
                job_id: JobId::generate(),
                module: "fab".into(),
                kind: "inventory_startup_prewarm".into(),
                queued_at: IsoDateTime::now(),
            })
        }
    }

    #[test]
    fn run_stage3_background_prewarm_triggers_fab_prewarm_when_enabled() {
        let fab_prewarm = Arc::new(RecordingFabPrewarmPort::default());
        let facade = StartupPipelineFacade::new(true, Some(fab_prewarm.clone()), None);

        block_on_ready(facade.run_stage3_background_prewarm())
            .expect("stage-3 background prewarm should trigger the Fab prewarm path when enabled");

        assert_eq!(
            *fab_prewarm
                .captured_requests
                .lock()
                .expect("prewarm request mutex should not be poisoned"),
            vec![FabInventoryPrewarmRequestDto {
                reason: "startup-stage-3".into(),
            }]
        );
    }

    #[test]
    fn run_stage3_background_prewarm_skips_fab_prewarm_when_disabled() {
        let fab_prewarm = Arc::new(RecordingFabPrewarmPort::default());
        let facade = StartupPipelineFacade::new(false, Some(fab_prewarm.clone()), None);

        block_on_ready(facade.run_stage3_background_prewarm())
            .expect("stage-3 background prewarm should stay a no-op when the capability gate is disabled");

        assert!(fab_prewarm
            .captured_requests
            .lock()
            .expect("prewarm request mutex should not be poisoned")
            .is_empty());
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
            Poll::Pending => panic!("startup prewarm future should be ready without a runtime"),
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
}