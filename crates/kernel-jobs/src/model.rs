use launcher_kernel_foundation::{IsoDateTime, JobId};
use serde::{Deserialize, Serialize};

fn default_recoverable() -> bool { true }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// 表示共享作业运行时内部状态机的稳定状态值。
pub enum JobState {
    /// 作业已入队，尚未开始抢占执行权。
    Queued,
    /// 作业正在尝试获取运行租约。
    ClaimingLease,
    /// 作业正在执行恢复流程。
    Restoring,
    /// 作业正在主动执行主流程。
    Running,
    /// 已收到暂停请求，等待运行时完成切换。
    PauseRequested,
    /// 作业已暂停，等待恢复或取消。
    Paused,
    /// 已收到恢复请求，等待运行时重新进入执行态。
    ResumeRequested,
    /// 作业正在完成收尾流程。
    Completing,
    /// 作业已经成功完成。
    Completed,
    /// 作业已经失败。
    Failed,
    /// 作业已经取消。
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// 表示投影给界面层的共享作业 UI 状态值。
pub enum JobUiState {
    /// 界面显示为排队中。
    Queued,
    /// 界面显示为运行中。
    Running,
    /// 界面显示为已暂停。
    Paused,
    /// 界面显示为等待用户处理。
    AwaitingUser,
    /// 界面显示为已完成。
    Completed,
    /// 界面显示为已失败。
    Failed,
    /// 界面显示为已取消。
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

/// Frontend-facing read model for an active job.
/// Extension-generic details are stripped — only kernel fields cross the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobSnapshotDto {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub state: JobState,
    pub ui_state: JobUiState,
    pub progress: JobProgress,
    pub updated_at: IsoDateTime,
}

impl From<&JobSnapshot<()>> for JobSnapshotDto {
    fn from(s: &JobSnapshot<()>) -> Self {
        Self {
            job_id: s.job_id.clone(),
            module: s.module.clone(),
            kind: s.kind.clone(),
            state: s.state,
            ui_state: s.ui_state,
            progress: s.progress.clone(),
            updated_at: s.updated_at.clone(),
        }
    }
}