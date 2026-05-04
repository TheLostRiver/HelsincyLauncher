//! Restore drivers for the currently registered Fab background job kinds.
//!
//! These drivers keep stage-2 restore deterministic while the current Fab runtime
//! semantics stay intentionally simple: both registered job kinds can resume by
//! re-enqueueing without consulting an additional business checkpoint.

use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// Restore driver for the `fab/inventory_startup_prewarm` job kind.
///
/// Startup prewarm is intentionally idempotent in the current baseline, so a
/// restored queued job can re-enter the runtime as `Resumed` on every restart.
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

/// Restore driver for the `fab/inventory_sync` job kind.
///
/// Incremental sync is restart-safe in the current design because cursor state
/// lives below this driver boundary, so restore can resume without a separate
/// driver-owned checkpoint.
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
