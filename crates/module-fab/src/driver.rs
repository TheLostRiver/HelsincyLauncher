use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// Stub restore driver for the `fab/inventory_startup_prewarm` job kind.
/// Always returns `Resumed` because a Fab prewarm job can be re-enqueued on
/// any restart without consulting a business checkpoint.
pub struct FabPrewarmJobDriver;

impl JobDriver<()> for FabPrewarmJobDriver {
    fn module(&self) -> &'static str {
        "fab"
    }

    fn kind(&self) -> &'static str {
        "inventory_startup_prewarm"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        Ok(RestoreDisposition::Resumed)
    }
}

/// Stub restore driver for the `fab/inventory_sync` job kind.
/// Always returns `Resumed` — incremental Fab sync can safely restart.
pub struct FabSyncJobDriver;

impl JobDriver<()> for FabSyncJobDriver {
    fn module(&self) -> &'static str {
        "fab"
    }

    fn kind(&self) -> &'static str {
        "inventory_sync"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        Ok(RestoreDisposition::Resumed)
    }
}
