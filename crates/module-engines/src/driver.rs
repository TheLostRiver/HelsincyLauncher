use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// Stub restore driver for the `engines/verification` job kind.
///
/// Always returns `Resumed` for engine verification jobs.
/// Real restoration logic will check cache and verification state in a later slice.
pub struct EngineJobDriver;

impl JobDriver<()> for EngineJobDriver {
    fn module(&self) -> &'static str {
        "engines"
    }

    fn kind(&self) -> &'static str {
        "verification"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        // TODO: AT-054 — Check engine cache and verification state for recovery
        // For now, stub returns Resumed.
        Ok(RestoreDisposition::Resumed)
    }
}
