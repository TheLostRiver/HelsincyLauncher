use launcher_kernel_foundation::IsoDateTime;
use launcher_kernel_jobs::JobSnapshotDto;

use super::{DesktopServices, QueryResultDto};

/// `jobs.list_active` 查询跨模块可恢复作业快照。
///
/// 该 handler 直接读取共享 `JobSnapshotStore`，让宿主能通过一个横切查询看到各模块的
/// resumable runtime 状态，而不需要逐个穿过模块 facade。
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
