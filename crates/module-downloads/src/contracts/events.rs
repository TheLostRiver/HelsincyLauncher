use launcher_kernel_foundation::JobId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DownloadEventDto {
    JobCompleted { job_id: JobId, artifact_ref: String },
    JobFailed { job_id: JobId, retryable: bool },
    JobCanceled { job_id: JobId },
}