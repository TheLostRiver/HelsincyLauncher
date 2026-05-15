//! 下载事实发生变化时发出的事件载荷。
//!
//! 这些事件是从后端状态迁移中导出的、可安全穿过传输边界的通知，
//! 而不是内部运行时消息本身。

use launcher_kernel_foundation::JobId;
use serde::{Deserialize, Serialize};

/// 下载生命周期通知使用的稳定事件联合体。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DownloadEventDto {
    /// 下载任务完成并产出具体制品引用时发出。
    JobCompleted { job_id: JobId, artifact_ref: String },

    /// 下载任务失败并报告是否仍允许重试时发出。
    JobFailed { job_id: JobId, retryable: bool },

    /// 下载任务在完成前被取消时发出。
    JobCanceled { job_id: JobId },
}
