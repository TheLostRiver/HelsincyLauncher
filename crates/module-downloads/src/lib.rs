//! Downloads 模块的 crate 入口与公开导出边界。
//!
//! 该文件把 contracts、driver、facade 三个子边界收口到同一个稳定入口，
//! 让上层装配和宿主 transport 只依赖 downloads 模块公开表面，
//! 而不必分散导入内部子模块路径。

pub mod contracts;
pub mod driver;
pub mod facade;

pub use driver::{
    DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadDriverExecutionTurn,
    DownloadJobDriver, DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus,
    DownloadSegmentExecutionPort, DownloadSegmentExecutionRequest, DownloadSegmentExecutionResult,
    DownloadSegmentExecutor, DownloadSegmentFetchOutcome, DownloadSegmentFetchPort,
    DownloadSegmentFetchResult, DownloadSegmentFilesystemWritePort,
    DownloadSegmentGuardedWritePort, DownloadSegmentHandledFailure,
    DownloadSegmentLengthVerifyPort, DownloadSegmentStagingTarget, DownloadSegmentStaticFetchPort,
    DownloadSegmentStaticFetchSource, DownloadSegmentVerifyOutcome, DownloadSegmentVerifyPort,
    DownloadSegmentWriteOutcome, DownloadSegmentWritePort, DownloadSegmentWriteResult,
};
pub use facade::{
    build_resume_segment_decisions, build_resume_work_plan, DownloadFacade, DownloadJobRecord,
    DownloadJobRecordState, DownloadJobRepository, DownloadManifestPlan,
    DownloadManifestProviderPort, DownloadManifestSegment, DownloadModuleDeps,
    DownloadPendingResumeWork, DownloadPendingResumeWorkSource, DownloadPolicyStore,
    DownloadResumeSegmentAction, DownloadResumeSegmentDecision, DownloadResumeWorkItem,
    DownloadResumeWorkMode, DownloadResumeWorkPlan, DownloadResumeWorkScheduler,
    DownloadRuntimePolicyApplier, InMemoryDownloadPolicyStore, InMemoryDownloadResumeWorkScheduler,
    NoopDownloadRuntimePolicyApplier,
};
