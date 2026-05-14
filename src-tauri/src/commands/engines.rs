//! engines 模块的宿主 transport handler。
//!
//! 该边界把 IPC 侧的引擎验证意图转接到 `DesktopServices` 暴露的后端 engines facade，
//! 并把返回的 accepted job 投影进共享宿主 transport envelope。

use launcher_module_engines::contracts::RunEngineVerificationRequestDto;

use super::{map_accepted_job_result, CommandResultDto, DesktopServices};

/// 启动后端拥有的引擎验证作业，并投影 accepted-job envelope。
pub async fn engines_run_verification(
    services: &DesktopServices,
    request: RunEngineVerificationRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.engines.run_verification(request))
}
