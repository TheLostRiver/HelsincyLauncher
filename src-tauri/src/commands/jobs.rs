use launcher_kernel_foundation::IsoDateTime;
use launcher_kernel_jobs::JobSnapshotDto;

use super::{DesktopServices, QueryResultDto};

/// `jobs.list_active` — returns all non-terminal job snapshots across all modules.
/// Reads directly from the shared `JobSnapshotStore` so every module's jobs appear
/// in a single cross-cutting query without going through individual module facades.
pub async fn jobs_list_active(
    services: &DesktopServices,
) -> QueryResultDto<Vec<JobSnapshotDto>> {
    match services.snapshot_store.list_resumable() {
        Ok(snaps) => QueryResultDto::Success {
            data: snaps.iter().map(JobSnapshotDto::from).collect(),
            as_of: Some(IsoDateTime::now()),
        },
        Err(error) => QueryResultDto::Failure {
            error: error.into(),
        },
    }
}
