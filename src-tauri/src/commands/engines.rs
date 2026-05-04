use launcher_module_engines::contracts::RunEngineVerificationRequestDto;

use super::{map_accepted_job_result, CommandResultDto, DesktopServices};

pub async fn engines_run_verification(
    services: &DesktopServices,
    request: RunEngineVerificationRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.engines.run_verification(request))
}