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
/// 表示共享运行时调度队列使用的通用优先级。
pub enum JobPriority {
    /// 低优先级作业，适合后台补偿或非交互任务。
    Low,
    /// 默认优先级作业，适合普通用户触发的后台任务。
    Normal,
    /// 高优先级作业，适合需要更快进入执行窗口的用户可见任务。
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示共享作业快照中跨模块复用的聚合进度。
pub struct JobProgress {
    /// 已完成的通用步骤数，不承载模块专属 checkpoint 语义。
    pub completed_steps: u32,
    /// 可选总步骤数；未知总量的流式任务可以保持为空。
    pub total_steps: Option<u32>,
    /// 面向调试或界面摘要的短文本详情，不替代模块业务状态。
    pub detail: Option<String>,
}

impl JobProgress {
    /// 构造一个尚未产生可量化进度的初始进度快照。
    pub fn pending() -> Self {
        Self {
            completed_steps: 0,
            total_steps: None,
            detail: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示运行时接受入队请求后返回给调用方的稳定确认结果。
pub struct AcceptedJob {
    /// 被接受作业的全局稳定标识。
    pub job_id: JobId,
    /// 拥有该作业业务语义的模块名。
    pub module: String,
    /// 模块内用于区分作业类型的稳定名称。
    pub kind: String,
    /// 运行时接受该作业进入队列的 UTC 时间。
    pub queued_at: IsoDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示模块 facade 提交给共享运行时的通用入队请求。
pub struct EnqueueJobRequest<E> {
    /// 调用方生成并用于后续查询、控制和恢复的作业标识。
    pub job_id: JobId,
    /// 提交作业的模块名，运行时只把它作为调度和投影维度。
    pub module: String,
    /// 模块内的作业类型名称，运行时不解释其业务细节。
    pub kind: String,
    /// 进入共享队列时使用的通用调度优先级。
    pub priority: JobPriority,
    /// 标记该作业是否允许在崩溃或重启后尝试恢复。
    #[serde(default = "default_recoverable")]
    pub recoverable: bool,
    /// 模块保留的扩展快照数据；共享运行时只持久化和回传，不解释内容。
    pub extension: Option<E>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// 表示运行时尝试恢复持久化作业快照后的结果。
pub enum RestoreDisposition {
    /// 作业已经被恢复流程接管，可以继续进入运行时调度。
    Resumed,
    /// 作业无法安全恢复，应按不可恢复失败状态投影。
    FailedAsUnrecoverable { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 表示共享运行时持久化和投影使用的通用作业快照。
pub struct JobSnapshot<E> {
    /// 快照所属作业的全局稳定标识。
    pub job_id: JobId,
    /// 拥有该作业业务语义的模块名。
    pub module: String,
    /// 模块内用于区分作业类型的稳定名称。
    pub kind: String,
    /// 运行时内部使用的生命周期状态。
    pub state: JobState,
    /// 面向界面和宿主投影的简化状态。
    pub ui_state: JobUiState,
    /// 跨模块共享的聚合进度。
    pub progress: JobProgress,
    /// 标记该快照是否允许后续恢复流程尝试重新接管作业。
    #[serde(default = "default_recoverable")]
    pub recoverable: bool,
    /// 快照最后一次被运行时更新的 UTC 时间。
    pub updated_at: IsoDateTime,
    /// 模块保留的扩展状态；共享运行时只持久化和回传，不解释内容。
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
