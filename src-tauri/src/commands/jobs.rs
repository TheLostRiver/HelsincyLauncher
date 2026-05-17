use launcher_kernel_foundation::IsoDateTime;
use launcher_kernel_jobs::JobSnapshotDto;

use super::{
    map_runtime_execution_turn_result, CommandResultDto, DesktopServices, QueryResultDto,
    RuntimeExecutionTurnDto,
};

/// `jobs.list_active` 查询跨模块可恢复作业快照。
///
/// 该 handler 直接读取共享 `JobSnapshotStore`，让宿主能通过一个横切查询看到各模块的
/// resumable runtime 状态，而不需要逐个穿过模块 facade。
pub async fn jobs_list_active(services: &DesktopServices) -> QueryResultDto<Vec<JobSnapshotDto>> {
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

/// `jobs.run_next_execution_turn` asks the backend runtime to advance one queued job.
///
/// This is a command because an accepted turn can mutate backend-owned runtime state.
/// 这是 command，因为被接受的执行轮次可能会推进后端拥有的 runtime 状态。
pub async fn jobs_run_next_execution_turn(
    services: &DesktopServices,
) -> CommandResultDto<RuntimeExecutionTurnDto> {
    map_runtime_execution_turn_result(services.startup.run_one_runtime_execution_turn())
}
