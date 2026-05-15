//! 从后端拥有的下载状态投影出的读模型 DTO。
//!
//! 这些类型是暴露给 downloads 模块外部的稳定传输契约，用于列表/详情读取
//! 以及策略快照查询。

use launcher_kernel_foundation::{JobId, PageSlice};
use launcher_kernel_jobs::{JobSnapshot, JobUiState};
use serde::{Deserialize, Serialize};

/// 附加在通用任务快照上的下载专属扩展事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobExtensionDto {
    /// 后端识别的下载目标标识，可指向资产或引擎载荷。
    pub target_id: String,

    /// 创建任务时捕获的可选安装后续意图。
    pub install_intent: Option<String>,

    /// 已经成功落地到本地的载荷字节数。
    pub completed_bytes: u64,

    /// 后端能够确定时的载荷总大小。
    pub total_bytes: Option<u64>,

    /// 最近一次失败形态是否仍允许后端再次重试。
    pub retryable: bool,
}

/// 单个下载任务的完整投影快照。
pub type DownloadJobSnapshotDto = JobSnapshot<DownloadJobExtensionDto>;

/// 分页下载任务列表中使用的摘要行。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobListItemDto {
    pub job_id: JobId,

    /// 展示在下载列表中的面向用户标题。
    pub title: String,

    /// 供壳层和前端筛选消费的投影 UI 状态。
    pub ui_state: JobUiState,

    /// 原始字节进度不足以表达状态时使用的可选人类可读进度文案。
    pub progress_label: Option<String>,

    /// 运行时当前可提供时的最近观测传输速率。
    pub throughput_bytes_per_sec: Option<u64>,
}

/// 分页下载任务列表读模型。
pub type DownloadJobListDto = PageSlice<DownloadJobListItemDto>;

/// 后端当前生效的下载策略快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadPolicyDto {
    pub concurrency_slots: u32,
    pub bandwidth_limit_bytes_per_sec: Option<u64>,
    pub auto_resume: bool,
}
