use launcher_kernel_foundation::{IsoDateTime, JobId};
use serde::{Deserialize, Serialize};

fn default_recoverable() -> bool { true }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    ClaimingLease,
    Restoring,
    Running,
    PauseRequested,
    Paused,
    ResumeRequested,
    Completing,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobUiState {
    Queued,
    Running,
    Paused,
    AwaitingUser,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobProgress {
    pub completed_steps: u32,
    pub total_steps: Option<u32>,
    pub detail: Option<String>,
}

impl JobProgress {
    pub fn pending() -> Self {
        Self {
            completed_steps: 0,
            total_steps: None,
            detail: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptedJob {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub queued_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnqueueJobRequest<E> {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub priority: JobPriority,
    #[serde(default = "default_recoverable")]
    pub recoverable: bool,
    pub extension: Option<E>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreDisposition {
    Resumed,
    FailedAsUnrecoverable { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobSnapshot<E> {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub state: JobState,
    pub ui_state: JobUiState,
    pub progress: JobProgress,
    #[serde(default = "default_recoverable")]
    pub recoverable: bool,
    pub updated_at: IsoDateTime,
    pub extension: Option<E>,
}