//! Command input contracts for the downloads module.
//!
//! These DTOs describe user intents that can mutate backend-owned download state
//! or scheduling policy.

use launcher_kernel_foundation::JobId;
use launcher_kernel_jobs::JobPriority;
use serde::{Deserialize, Serialize};

/// Requests creation of a new download job for a target asset or engine payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartDownloadRequestDto {
    /// Backend-known identifier of the content the user wants to download.
    pub target_id: String,

    /// Logical content kind used by downstream orchestration and repair flows.
    pub kind: String,

    /// Optional follow-on install intent captured at intake time.
    pub install_intent: Option<String>,

    /// Queue priority snapshot applied when the backend enqueues the download job.
    pub priority: JobPriority,
}

/// Requests pausing an existing download job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseDownloadRequestDto {
    pub job_id: JobId,
}

/// Requests resuming an existing paused download job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDownloadRequestDto {
    pub job_id: JobId,
}

/// Requests canceling an existing download job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelDownloadRequestDto {
    pub job_id: JobId,
}

/// Requests updating the downloads scheduling policy exposed to users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateDownloadPolicyRequestDto {
    /// User-facing concurrent download slot budget, not a raw OS thread count.
    pub concurrency_slots: u32,

    /// Optional aggregate bandwidth cap applied by the backend scheduler.
    pub bandwidth_limit_bytes_per_sec: Option<u64>,

    /// Whether queued downloads should resume automatically after supported restarts.
    pub auto_resume: bool,
}