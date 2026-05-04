use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// Stub restore driver for the `downloads/download` job kind.
///
/// Always returns `Resumed` because a download job can be recovered
/// from checkpoint on any restart. Real staging-file verification
/// will be added in a later slice.
pub struct DownloadJobDriver;

impl JobDriver<()> for DownloadJobDriver {
    fn module(&self) -> &'static str {
        "downloads"
    }

    fn kind(&self) -> &'static str {
        "download"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        Ok(RestoreDisposition::Resumed)
    }
}
