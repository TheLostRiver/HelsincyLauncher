//! downloads 模块稳定读取路径的查询输入 contracts。
//!
//! 这些 DTO 让调用方读取分页任务列表、单个作业快照或当前下载策略，
//! 同时避免把下载运行时内部细节泄露到调用边界。

use launcher_kernel_foundation::{JobId, PageRequest};
use launcher_kernel_jobs::JobUiState;
use serde::{Deserialize, Serialize};

/// 表示按分页窗口读取下载作业列表的查询输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDownloadJobsQueryDto {
    /// 稳定下载作业 read model 使用的分页窗口。
    pub page: PageRequest,

    /// 可选的 UI 投影状态过滤条件。
    pub ui_state: Option<JobUiState>,
}

/// 表示按稳定作业标识读取单个下载作业快照的查询输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetDownloadJobQueryDto {
    /// 要读取的下载作业标识。
    pub job_id: JobId,
}

/// 表示读取当前下载策略快照的查询输入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GetDownloadPolicyQueryDto;
