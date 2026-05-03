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

pub async fn downloads_start(
    services: &DesktopServices,
    request: StartDownloadRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.downloads.start_download(request))
}

pub async fn downloads_pause(
    services: &DesktopServices,
    request: PauseDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.pause_download(request))
}

pub async fn downloads_resume(
    services: &DesktopServices,
    request: ResumeDownloadRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.downloads.resume_download(request))
}

pub async fn downloads_cancel(
    services: &DesktopServices,
    request: CancelDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.cancel_download(request))
}

pub async fn downloads_list_jobs(
    services: &DesktopServices,
    query: ListDownloadJobsQueryDto,
) -> QueryResultDto<DownloadJobListDto> {
    map_query_result_or_stub(services.downloads.list_jobs(query), "DOWNLOADS_NOT_WIRED", || {
        PageSlice::new(Vec::new(), None)
    })
}

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

pub async fn downloads_update_policy(
    services: &DesktopServices,
    request: UpdateDownloadPolicyRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.update_policy(request))
}