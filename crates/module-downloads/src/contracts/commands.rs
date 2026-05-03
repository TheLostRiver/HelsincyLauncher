use launcher_kernel_foundation::JobId;
use launcher_kernel_jobs::JobPriority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartDownloadRequestDto {
    pub target_id: String,
    pub kind: String,
    pub install_intent: Option<String>,
    pub priority: JobPriority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseDownloadRequestDto {
    pub job_id: JobId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDownloadRequestDto {
    pub job_id: JobId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelDownloadRequestDto {
    pub job_id: JobId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateDownloadPolicyRequestDto {
    pub concurrency_slots: u32,
    pub bandwidth_limit_bytes_per_sec: Option<u64>,
    pub auto_resume: bool,
}