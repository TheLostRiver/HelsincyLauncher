//! Host transport handlers for the engines module.
//!
//! This boundary adapts IPC-facing engine verification intent onto the
//! backend-owned engines facade exposed through `DesktopServices` and projects
//! the resulting accepted job into the shared host transport envelope.

use launcher_module_engines::contracts::RunEngineVerificationRequestDto;

use super::{map_accepted_job_result, CommandResultDto, DesktopServices};

/// Starts backend-owned engine verification and projects the accepted job envelope.
pub async fn engines_run_verification(
    services: &DesktopServices,
    request: RunEngineVerificationRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.engines.run_verification(request))
}