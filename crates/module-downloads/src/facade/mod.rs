use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId,
};
use launcher_kernel_jobs::AcceptedJob;

use crate::contracts::{
    CancelDownloadRequestDto, DownloadJobListDto, DownloadJobSnapshotDto, DownloadPolicyDto,
    GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto,
    PauseDownloadRequestDto, ResumeDownloadRequestDto, StartDownloadRequestDto,
    UpdateDownloadPolicyRequestDto,
};

#[derive(Debug, Clone)]
pub struct DownloadModuleDeps<J, C, M, S, R> {
    pub job_repo: J,
    pub checkpoint_repo: C,
    pub manifest_provider: M,
    pub staging_store: S,
    pub job_runtime: R,
}

pub struct DownloadFacade<J, C, M, S, R> {
    deps: DownloadModuleDeps<J, C, M, S, R>,
}

impl<J, C, M, S, R> DownloadFacade<J, C, M, S, R> {
    pub fn new(deps: DownloadModuleDeps<J, C, M, S, R>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &DownloadModuleDeps<J, C, M, S, R> {
        &self.deps
    }

    pub fn start_download(&self, request: StartDownloadRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("start_download"))
    }

    pub fn pause_download(&self, request: PauseDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("pause_download"))
    }

    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("resume_download"))
    }

    pub fn cancel_download(&self, request: CancelDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("cancel_download"))
    }

    pub fn list_jobs(&self, query: ListDownloadJobsQueryDto) -> AppResult<DownloadJobListDto> {
        let _ = query;
        Err(not_wired("list_jobs"))
    }

    pub fn get_job_snapshot(
        &self,
        query: GetDownloadJobQueryDto,
    ) -> AppResult<Option<DownloadJobSnapshotDto>> {
        let _ = query;
        Err(not_wired("get_job_snapshot"))
    }

    pub fn get_policy(
        &self,
        query: GetDownloadPolicyQueryDto,
    ) -> AppResult<DownloadPolicyDto> {
        let _ = query;
        Err(not_wired("get_policy"))
    }

    pub fn update_policy(&self, request: UpdateDownloadPolicyRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("update_policy"))
    }
}

fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "DOWNLOADS_NOT_WIRED",
        format!("downloads facade operation `{operation}` is not wired in C2"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}