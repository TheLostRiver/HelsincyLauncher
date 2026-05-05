//! downloads 模块的命令输入 contracts。
//!
//! 这些 DTO 描述会修改后端拥有的下载状态或调度策略的用户意图。

use launcher_kernel_foundation::JobId;
use launcher_kernel_jobs::JobPriority;
use serde::{Deserialize, Serialize};

/// 请求为目标资产或引擎载荷创建新的下载任务。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartDownloadRequestDto {
    /// 后端已知的内容标识，表示用户要下载的目标。
    pub target_id: String,

    /// 下游编排与修复流程使用的逻辑内容类型。
    pub kind: String,

    /// intake 时捕获的可选后续安装意图。
    pub install_intent: Option<String>,

    /// 后端入队下载任务时应用的队列优先级快照。
    pub priority: JobPriority,
}

/// 请求暂停一个已有的下载任务。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseDownloadRequestDto {
    pub job_id: JobId,
}

/// 请求恢复一个已经暂停的下载任务。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDownloadRequestDto {
    pub job_id: JobId,
}

/// 请求取消一个已有的下载任务。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelDownloadRequestDto {
    pub job_id: JobId,
}

/// 请求更新面向用户暴露的 downloads 调度策略。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateDownloadPolicyRequestDto {
    /// 面向用户的并发下载槽位预算，而不是原始 OS 线程数。
    pub concurrency_slots: u32,

    /// 后端调度器应用的可选聚合带宽上限。
    pub bandwidth_limit_bytes_per_sec: Option<u64>,

    /// 支持的重启之后，排队下载是否应自动恢复。
    pub auto_resume: bool,
}