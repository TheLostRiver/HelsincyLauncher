//! downloads 模块的宿主 transport handler。
//!
//! 该边界把 IPC 侧下载命令和查询转接到 `DesktopServices` 暴露的后端 downloads facade。
//! 在完整 read path 尚未全部接线前，查询 handler 仍负责把明确的
//! `DOWNLOADS_NOT_WIRED` 错误投影为 shell 可消费的临时 fallback。

use launcher_kernel_foundation::PageSlice;
use launcher_module_downloads::contracts::{
    CancelDownloadRequestDto, DownloadJobListDto, DownloadJobSnapshotDto, DownloadPolicyDto,
    GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto,
    PauseDownloadRequestDto, ResumeDownloadRequestDto, StartDownloadRequestDto,
    UpdateDownloadPolicyRequestDto,
};

use super::{
    map_accepted_job_result, map_command_result, map_download_resume_outcome_result,
    map_query_result_or_stub, CommandResultDto, DesktopServices, QueryResultDto,
};

/// 提交后端拥有的下载作业，并投影 accepted-job envelope。
pub async fn downloads_start(
    services: &DesktopServices,
    request: StartDownloadRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.downloads.start_download(request))
}

/// 请求暂停已有的后端下载作业。
pub async fn downloads_pause(
    services: &DesktopServices,
    request: PauseDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.pause_download(request))
}

/// 请求恢复已暂停的后端下载作业，并投影 downloads resume outcome。
pub async fn downloads_resume(
    services: &DesktopServices,
    request: ResumeDownloadRequestDto,
) -> CommandResultDto<super::DownloadResumeOutcomeDto> {
    map_download_resume_outcome_result(services.downloads.resume_download_outcome(request))
}

/// 请求取消已有的后端下载作业。
pub async fn downloads_cancel(
    services: &DesktopServices,
    request: CancelDownloadRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.cancel_download(request))
}

/// 读取下载作业投影列表，并在当前 host stub 路径上回退为空分页。
pub async fn downloads_list_jobs(
    services: &DesktopServices,
    query: ListDownloadJobsQueryDto,
) -> QueryResultDto<DownloadJobListDto> {
    map_query_result_or_stub(
        services.downloads.list_jobs(query),
        "DOWNLOADS_NOT_WIRED",
        || PageSlice::new(Vec::new(), None),
    )
}

/// 读取单个下载作业快照投影，并在当前 host stub 路径上回退为空结果。
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

/// 读取下载策略投影，并在当前 host stub 路径上回退为宿主占位策略。
pub async fn downloads_get_policy(
    services: &DesktopServices,
    query: GetDownloadPolicyQueryDto,
) -> QueryResultDto<DownloadPolicyDto> {
    map_query_result_or_stub(
        services.downloads.get_policy(query),
        "DOWNLOADS_NOT_WIRED",
        || DownloadPolicyDto {
            concurrency_slots: 3,
            bandwidth_limit_bytes_per_sec: None,
            auto_resume: false,
        },
    )
}

/// 更新后端拥有的下载策略。
pub async fn downloads_update_policy(
    services: &DesktopServices,
    request: UpdateDownloadPolicyRequestDto,
) -> CommandResultDto<()> {
    map_command_result(services.downloads.update_policy(request))
}
