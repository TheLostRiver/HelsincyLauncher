//! 聚合共享作业模型与运行时能力，对外暴露统一的长任务内核导出面。

pub mod model;
pub mod runtime;

pub use model::{
    AcceptedJob, EnqueueJobRequest, JobPriority, JobProgress, JobSnapshot, JobSnapshotDto,
    JobState, JobUiState, RestoreDisposition,
};
pub use runtime::{
    JobDriver, JobDriverRegistry, JobExecutionContext, JobRunDisposition, JobRuntime,
    JobSnapshotStore, RuntimeQueuePolicy, SharedJobRuntimeHost,
};
