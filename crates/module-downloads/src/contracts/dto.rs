use launcher_kernel_foundation::{JobId, PageSlice};
use launcher_kernel_jobs::{JobSnapshot, JobUiState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobExtensionDto {
    pub target_id: String,
    pub install_intent: Option<String>,
    pub completed_bytes: u64,
    pub total_bytes: Option<u64>,
    pub retryable: bool,
}

pub type DownloadJobSnapshotDto = JobSnapshot<DownloadJobExtensionDto>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobListItemDto {
    pub job_id: JobId,
    pub title: String,
    pub ui_state: JobUiState,
    pub progress_label: Option<String>,
    pub throughput_bytes_per_sec: Option<u64>,
}

pub type DownloadJobListDto = PageSlice<DownloadJobListItemDto>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadPolicyDto {
    pub concurrency_slots: u32,
    pub bandwidth_limit_bytes_per_sec: Option<u64>,
    pub auto_resume: bool,
}