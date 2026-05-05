//! downloads 模块的公开 contracts 入口。
//!
//! 调用方从这个边界导入下载命令/查询输入、读模型和事件载荷，
//! 而不是依赖模块内部的文件布局。

/// 表达 downloads 侧用户意图的命令输入 contracts。
pub mod commands;

/// 从后端拥有的下载状态投影出的读模型与快照 DTO。
pub mod dto;

/// 下载事实变化时广播的事件载荷。
pub mod events;

/// 稳定下载读取路径使用的查询输入 contracts。
pub mod queries;

pub use commands::{
    CancelDownloadRequestDto, PauseDownloadRequestDto, ResumeDownloadRequestDto,
    StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
};
pub use dto::{
    DownloadJobExtensionDto, DownloadJobListDto, DownloadJobListItemDto,
    DownloadJobSnapshotDto, DownloadPolicyDto,
};
pub use events::DownloadEventDto;
pub use queries::{GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto};