use launcher_kernel_foundation::{JobId, PageRequest};
use launcher_kernel_jobs::JobUiState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDownloadJobsQueryDto {
    pub page: PageRequest,
    pub ui_state: Option<JobUiState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetDownloadJobQueryDto {
    pub job_id: JobId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GetDownloadPolicyQueryDto;