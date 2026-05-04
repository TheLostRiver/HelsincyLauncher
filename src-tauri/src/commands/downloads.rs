//! Host transport handlers for the downloads module.
//!
//! This boundary adapts IPC-facing downloads commands and queries onto the
//! backend-owned downloads facade exposed through `DesktopServices`. Until the
//! full downloads read path is wired everywhere, the query handlers also own the
//! temporary `DOWNLOADS_NOT_WIRED` fallback projections returned to the shell.

use launcher_kernel_foundation::PageSlice;
use launcher_module_downloads::contracts::{
    CancelDownloadRequestDto, DownloadJobListDto, DownloadJobSnapshotDto, DownloadPolicyDto,
    GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto,
    PauseDownloadRequestDto, ResumeDownloadRequestDto, StartDownloadRequestDto,
    UpdateDownloadPolicyRequestDto,
};

use super::{
    map_accepted_job_result, map_command_result, map_query_result_or_stub, CommandResultDto,
    DesktopServices, QueryResultDto,
};

/// Starts a backend-owned download job and projects the accepted job envelope.
pub async fn downloads_start(
    services: &DesktopServices,
    request: StartDownloadRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.downloads.start_download(request))
}

/// Pauses an existing backend-owned download job.
pub async fn downloads_pause(
    services: &DesktopServices,
    request: PauseDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.pause_download(request))
}

/// Resumes a paused backend-owned download job and projects the accepted job envelope.
pub async fn downloads_resume(
    services: &DesktopServices,
    request: ResumeDownloadRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.downloads.resume_download(request))
}

/// Cancels an existing backend-owned download job.
pub async fn downloads_cancel(
    services: &DesktopServices,
    request: CancelDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.cancel_download(request))
}

/// Lists projected download jobs and falls back to an empty page on the current host stub path.
pub async fn downloads_list_jobs(
    services: &DesktopServices,
    query: ListDownloadJobsQueryDto,
) -> QueryResultDto<DownloadJobListDto> {
    map_query_result_or_stub(services.downloads.list_jobs(query), "DOWNLOADS_NOT_WIRED", || {
        PageSlice::new(Vec::new(), None)
    })
}

/// Gets one projected download job snapshot and falls back to `None` on the current host stub path.
pub async fn downloads_get_job_snapshot(
    services: &DesktopServices,
    query: GetDownloadJobQueryDto,
) -> QueryResultDto<Option<DownloadJobSnapshotDto>> {
    map_query_result_or_stub(
        services.downloads.get_job_snapshot(query),
        "DOWNLOADS_NOT_WIRED",
        || None,
    )
}

/// Reads the downloads policy projection and falls back to the current host-owned placeholder policy.
pub async fn downloads_get_policy(
    services: &DesktopServices,
    query: GetDownloadPolicyQueryDto,
) -> QueryResultDto<DownloadPolicyDto> {
    map_query_result_or_stub(services.downloads.get_policy(query), "DOWNLOADS_NOT_WIRED", || {
        DownloadPolicyDto {
            concurrency_slots: 3,
            bandwidth_limit_bytes_per_sec: None,
            auto_resume: false,
        }
    })
}

/// Updates the backend-owned downloads policy.
pub async fn downloads_update_policy(
    services: &DesktopServices,
    request: UpdateDownloadPolicyRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.update_policy(request))
}