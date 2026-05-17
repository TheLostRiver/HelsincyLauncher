use std::path::{Component, Path};
use std::sync::Arc;

use launcher_kernel_foundation::{AppResult, JobId};
use launcher_kernel_jobs::{
    JobDriver, JobExecutionContext, JobRunDisposition, JobSnapshot, RestoreDisposition,
};

use crate::facade::{
    DownloadPendingResumeWork, DownloadPendingResumeWorkSource, DownloadResumeWorkMode,
};

/// 持久化下载 checkpoint 的最小记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadCheckpointRecord {
    /// 与该 checkpoint 关联的稳定下载任务标识。
    pub job_id: JobId,
    /// Segment-level persisted facts owned by downloads resume logic.
    /// downloads 恢复逻辑拥有的 segment 级持久化事实。
    pub segments: Vec<DownloadSegmentCheckpointRecord>,
}

impl DownloadCheckpointRecord {
    /// Builds an empty checkpoint that preserves current adapter compatibility.
    /// 创建空 segment checkpoint，用于保持当前 adapter 兼容性。
    pub fn empty(job_id: JobId) -> Self {
        Self {
            job_id,
            segments: Vec::new(),
        }
    }
}

/// Persisted status for a single download segment checkpoint.
/// 单个下载 segment checkpoint 的持久化状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadSegmentCheckpointStatus {
    /// Segment has not started yet.
    /// segment 尚未开始。
    Pending,
    /// Segment has partial bytes and may later resume.
    /// segment 已有部分字节，后续可能继续恢复。
    InProgress,
    /// Segment completed and can be sealed when it still matches the manifest.
    /// segment 已完成，且与 manifest 仍匹配时可封存。
    Completed,
    /// Segment failed and needs a later retry or attention decision.
    /// segment 已失败，需要后续重试或注意状态决策。
    Failed,
}

/// Segment-level persisted checkpoint fact owned by downloads.
/// downloads 拥有的 segment 级持久化 checkpoint 事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentCheckpointRecord {
    /// Stable downloads job identifier.
    /// 稳定的 downloads 作业标识。
    pub job_id: JobId,
    /// Stable segment identifier inside the manifest plan.
    /// manifest plan 内稳定的 segment 标识。
    pub segment_id: String,
    /// Logical file identifier used to detect stale manifest boundaries.
    /// 用于识别过期 manifest 边界的逻辑文件标识。
    pub file_id: String,
    /// Byte offset inside the logical file.
    /// 逻辑文件内的字节偏移。
    pub offset: u64,
    /// Expected segment length in bytes.
    /// segment 预期字节长度。
    pub length: u64,
    /// Persisted downloaded byte count inside this segment.
    /// 此 segment 内已持久化的已下载字节数。
    pub downloaded_bytes: u64,
    /// Module-owned persisted segment status.
    /// 模块拥有的 segment 持久化状态。
    pub status: DownloadSegmentCheckpointStatus,
    /// Staging-relative partial file path when available.
    /// 可用时记录 staging 相对的临时分片路径。
    pub partial_path: Option<String>,
    /// Provider validator used for safe range resume when available.
    /// 可用时用于安全 range resume 的 provider 校验值。
    pub etag: Option<String>,
    /// Reference to incremental hash state when recomputing is expensive.
    /// 当重新计算成本较高时，指向增量 hash 状态的引用。
    pub hash_state_ref: Option<String>,
}

/// 提供下载 checkpoint 读取与保存能力的最小仓储边界。
pub trait DownloadCheckpointRepository: Send + Sync {
    /// 按任务标识读取已存在的 checkpoint 记录。
    fn load(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>>;
    /// 持久化一个新的或更新后的 checkpoint 记录。
    fn save(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()>;
}

/// Module-local classification for a future downloads driver execution turn.
/// 后续 downloads driver 执行 turn 使用的模块本地分类结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadDriverExecutionTurn {
    /// Durable checkpoint is missing, so pending in-memory work must stay untouched.
    /// durable checkpoint 缺失，因此必须保留内存态 pending work 不被取走。
    CheckpointMissing {
        /// Downloads job id whose durable checkpoint could not be reloaded.
        /// 无法重新读取 durable checkpoint 的 downloads 作业标识。
        job_id: JobId,
    },
    /// Durable checkpoint exists, but no prepared pending work is available.
    /// durable checkpoint 存在，但当前没有可消费的 pending work。
    NoPendingWork {
        /// Reloaded checkpoint that keeps this classification tied to durable state.
        /// 重新读取的 checkpoint，用于把该分类绑定到 durable state。
        checkpoint: DownloadCheckpointRecord,
    },
    /// Durable checkpoint exists and prepared pending work was accepted locally.
    /// durable checkpoint 存在，且本地已接收准备好的 pending work。
    PendingWorkAccepted {
        /// Reloaded checkpoint used to guard the accepted in-memory work.
        /// 用于保护已接收内存态 work 的重新读取 checkpoint。
        checkpoint: DownloadCheckpointRecord,
        /// Drained pending work for the requested job only.
        /// 只为请求的 job 取走的 pending work。
        pending_work: Vec<DownloadPendingResumeWork>,
    },
}

/// Module-local request handed to future segment execution ports.
/// 交给后续 segment 执行端口的模块本地请求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentExecutionRequest {
    /// Downloads job id that owns this segment execution request.
    /// 拥有此 segment 执行请求的 downloads 作业标识。
    pub job_id: JobId,
    /// Stable segment identifier used by the future executor route.
    /// 后续 executor 路由使用的稳定 segment 标识。
    pub segment_id: String,
    /// Stable logical file identifier guarding against stale checkpoints.
    /// 用于防止误用陈旧 checkpoint 的稳定逻辑文件标识。
    pub file_id: String,
    /// Provider fetch reference kept behind the downloads boundary.
    /// 保留在 downloads 边界内的 provider 拉取引用。
    pub source_locator: String,
    /// Staging-relative output target for the future writer.
    /// 后续 writer 使用的 staging 相对输出目标。
    pub write_target: String,
    /// Optional verifier expectation copied from the resume work item.
    /// 从 resume work item 复制的可选校验期望。
    pub expected_hash: Option<String>,
    /// Segment-relative resume offset or manifest start offset.
    /// segment 相对续传偏移或 manifest 起始偏移。
    pub start_offset: u64,
    /// Total segment length expected by scheduler and verifier.
    /// scheduler 与 verifier 期望的 segment 总长度。
    pub length: u64,
    /// Whether execution should resume partially or start from scratch.
    /// 执行时应部分续传还是从头开始。
    pub resume_mode: DownloadResumeWorkMode,
    /// Module-local checkpoint reference carried into later checkpoint mutation.
    /// 带入后续 checkpoint mutation 的模块本地 checkpoint 引用。
    pub checkpoint_ref: Option<String>,
}

/// Module-local result shell for future segment execution ports.
/// 后续 segment 执行端口使用的模块本地结果壳。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadSegmentExecutionResult {
    /// The request was accepted by a fake or future executor boundary.
    /// 请求已被 fake 或后续 executor 边界接收。
    Accepted {
        /// Segment request that crossed the local execution boundary.
        /// 穿过本地执行边界的 segment 请求。
        request: DownloadSegmentExecutionRequest,
    },
    /// A fake or future executor reports that the segment completed locally.
    /// fake 或后续 executor 报告该 segment 已在本地完成。
    Completed {
        /// Segment request that produced the completion result.
        /// 产生完成结果的 segment 请求。
        request: DownloadSegmentExecutionRequest,
        /// Bytes completed by this result; later checkpoint logic owns validation.
        /// 本次结果完成的字节数；后续 checkpoint 逻辑负责校验。
        downloaded_bytes: u64,
        /// Optional staging-relative partial path produced by fake or future writer code.
        /// fake 或后续 writer 代码产生的可选 staging 相对 partial 路径。
        partial_path: Option<String>,
        /// Optional provider validator token retained for later resume safety checks.
        /// 为后续安全恢复检查保留的可选 provider 校验 token。
        etag: Option<String>,
        /// Optional hash-state reference retained for later verifier/checkpoint slices.
        /// 为后续 verifier/checkpoint 切片保留的可选 hash 状态引用。
        hash_state_ref: Option<String>,
    },
    /// A fake or future executor reports a handled local segment failure.
    /// fake 或后续 executor 报告一次已处理的本地 segment 失败。
    Failed {
        /// Segment request that produced the failure result.
        /// 产生失败结果的 segment 请求。
        request: DownloadSegmentExecutionRequest,
        /// Bytes completed before the failure was observed.
        /// 观察到失败前已完成的字节数。
        downloaded_bytes: u64,
        /// Module-local diagnostic reason, not a public `DL_*` error code.
        /// 模块本地诊断原因，不是公开的 `DL_*` 错误码。
        reason: String,
        /// Retry hint for later policy slices; this does not trigger retry here.
        /// 供后续策略切片使用的重试提示；这里不会触发重试。
        retryable: bool,
    },
}

/// Bytes fetched for a segment by a module-owned fetch sub-port.
/// 由模块自有 fetch 子端口取得的 segment 字节结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentFetchResult {
    /// In-memory bytes for the first adapter-shell boundary; real streaming is a later slice.
    /// 首个 adapter shell 边界使用的内存字节；真实流式传输属于后续切片。
    pub bytes: Vec<u8>,
    /// Optional provider validator token observed while fetching.
    /// 拉取阶段观测到的可选 provider 校验 token。
    pub etag: Option<String>,
}

/// Staging write facts produced by a module-owned write sub-port.
/// 由模块自有 write 子端口产生的 staging 写入事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentWriteResult {
    /// Bytes written for this segment.
    /// 本次 segment 写入的字节数。
    pub downloaded_bytes: u64,
    /// Optional staging-relative partial path produced by the writer.
    /// writer 产生的可选 staging 相对 partial 路径。
    pub partial_path: Option<String>,
    /// Optional hash-state reference retained for later checkpoint/verifier slices.
    /// 为后续 checkpoint/verifier 切片保留的可选 hash 状态引用。
    pub hash_state_ref: Option<String>,
}

/// Module-local handled failure produced by a fetch/write/verify sub-port.
/// fetch/write/verify 子端口产生的模块本地已处理失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentHandledFailure {
    /// Best-known bytes completed before the handled failure was reported.
    /// 报告已处理失败前已知的已完成字节数。
    pub downloaded_bytes: u64,
    /// Module-local diagnostic reason, not a public `DL_*` error code.
    /// 模块本地诊断原因，不是公开的 `DL_*` 错误码。
    pub reason: String,
    /// Retry hint for later policy slices; this does not trigger retry here.
    /// 供后续策略切片使用的重试提示；这里不会触发重试。
    pub retryable: bool,
}

impl DownloadSegmentHandledFailure {
    fn into_execution_result(
        self,
        request: &DownloadSegmentExecutionRequest,
    ) -> DownloadSegmentExecutionResult {
        DownloadSegmentExecutionResult::Failed {
            request: request.clone(),
            downloaded_bytes: self.downloaded_bytes,
            reason: self.reason,
            retryable: self.retryable,
        }
    }
}

/// Validated staging-relative write target for future segment writer sub-ports.
/// 供后续 segment writer 子端口使用的已验证 staging 相对写入目标。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadSegmentStagingTarget {
    normalized_relative_path: String,
}

impl DownloadSegmentStagingTarget {
    /// Parses a staging-relative write target without touching the host file system.
    /// 在不访问宿主文件系统的前提下解析 staging 相对写入目标。
    pub fn parse(write_target: &str) -> Result<Self, DownloadSegmentHandledFailure> {
        if write_target.trim().is_empty() {
            return Err(Self::unsafe_target_failure(write_target));
        }

        let mut components = Vec::new();
        for component in Path::new(write_target).components() {
            match component {
                Component::Normal(value) => {
                    components.push(value.to_string_lossy().into_owned());
                }
                Component::Prefix(_)
                | Component::RootDir
                | Component::CurDir
                | Component::ParentDir => {
                    return Err(Self::unsafe_target_failure(write_target));
                }
            }
        }

        if components.is_empty() {
            return Err(Self::unsafe_target_failure(write_target));
        }

        Ok(Self {
            normalized_relative_path: components.join("/"),
        })
    }

    /// Returns the normalized staging-relative target string.
    /// 返回规范化后的 staging 相对目标字符串。
    pub fn as_str(&self) -> &str {
        &self.normalized_relative_path
    }

    fn unsafe_target_failure(write_target: &str) -> DownloadSegmentHandledFailure {
        DownloadSegmentHandledFailure {
            downloaded_bytes: 0,
            reason: format!("unsafe staging write target: {write_target}"),
            retryable: false,
        }
    }
}

/// Fetch sub-port outcome: either fetched bytes or a handled segment failure.
/// fetch 子端口结果：已拉取字节，或一个已处理的 segment 失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadSegmentFetchOutcome {
    /// Fetch succeeded and produced in-memory bytes for this boundary slice.
    /// fetch 成功，并为当前边界切片产生内存字节。
    Fetched(DownloadSegmentFetchResult),
    /// Fetch made a segment decision and reported a handled local failure.
    /// fetch 已经形成 segment 决策，并报告一个已处理的本地失败。
    Failed(DownloadSegmentHandledFailure),
}

/// Write sub-port outcome: either staging write facts or a handled segment failure.
/// write 子端口结果：staging 写入事实，或一个已处理的 segment 失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadSegmentWriteOutcome {
    /// Write succeeded and produced staging facts for checkpoint mutation.
    /// write 成功，并产生供 checkpoint mutation 使用的 staging 事实。
    Written(DownloadSegmentWriteResult),
    /// Write made a segment decision and reported a handled local failure.
    /// write 已经形成 segment 决策，并报告一个已处理的本地失败。
    Failed(DownloadSegmentHandledFailure),
}

/// Verify sub-port outcome: either verification succeeded or a handled segment failure.
/// verify 子端口结果：校验成功，或一个已处理的 segment 失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadSegmentVerifyOutcome {
    /// Verification succeeded for the fetched and written facts.
    /// 对 fetch/write 事实的校验成功。
    Verified,
    /// Verify made a segment decision and reported a handled local failure.
    /// verify 已经形成 segment 决策，并报告一个已处理的本地失败。
    Failed(DownloadSegmentHandledFailure),
}

/// Module-owned fetch boundary used behind `DownloadSegmentExecutionPort`.
/// 位于 `DownloadSegmentExecutionPort` 背后的模块自有 fetch 边界。
pub trait DownloadSegmentFetchPort: Send + Sync {
    /// Fetches bytes for the prepared segment request without exposing provider details upward.
    /// 为准备好的 segment 请求拉取字节，不向上层暴露 provider 细节。
    fn fetch_segment(
        &self,
        request: &DownloadSegmentExecutionRequest,
    ) -> AppResult<DownloadSegmentFetchOutcome>;
}

/// Module-owned staging writer boundary used behind `DownloadSegmentExecutionPort`.
/// 位于 `DownloadSegmentExecutionPort` 背后的模块自有 staging writer 边界。
pub trait DownloadSegmentWritePort: Send + Sync {
    /// Writes fetched segment bytes to the staging-relative target described by the request.
    /// 将已拉取的 segment 字节写入请求描述的 staging 相对目标。
    fn write_segment(
        &self,
        request: &DownloadSegmentExecutionRequest,
        fetched: &DownloadSegmentFetchResult,
    ) -> AppResult<DownloadSegmentWriteOutcome>;
}

/// Module-owned verifier boundary used behind `DownloadSegmentExecutionPort`.
/// 位于 `DownloadSegmentExecutionPort` 背后的模块自有 verifier 边界。
pub trait DownloadSegmentVerifyPort: Send + Sync {
    /// Verifies fetched/written facts for the request; concrete hash logic is a later slice.
    /// 校验该请求的 fetch/write 事实；具体 hash 逻辑属于后续切片。
    fn verify_segment(
        &self,
        request: &DownloadSegmentExecutionRequest,
        fetched: &DownloadSegmentFetchResult,
        written: &DownloadSegmentWriteResult,
    ) -> AppResult<DownloadSegmentVerifyOutcome>;
}

/// Port shell for handing segment requests to later fetch/write/verify code.
/// 将 segment 请求交给后续 fetch/write/verify 代码的端口壳。
pub trait DownloadSegmentExecutionPort: Send + Sync {
    /// Accepts a prepared segment request without defining concrete IO here.
    /// 接收已准备好的 segment 请求，但不在此处定义具体 IO。
    fn accept_segment_execution(
        &self,
        request: &DownloadSegmentExecutionRequest,
    ) -> AppResult<DownloadSegmentExecutionResult>;
}

/// Thin adapter that composes fetch/write/verify sub-ports behind the driver-facing port.
/// 薄 adapter：在 driver 面向的端口背后组合 fetch/write/verify 子端口。
#[derive(Clone)]
pub struct DownloadSegmentExecutor {
    fetcher: Arc<dyn DownloadSegmentFetchPort>,
    writer: Arc<dyn DownloadSegmentWritePort>,
    verifier: Arc<dyn DownloadSegmentVerifyPort>,
}

impl DownloadSegmentExecutor {
    /// Creates a segment executor from explicit module-owned sub-ports.
    /// 使用显式的模块自有子端口创建 segment executor。
    pub fn new(
        fetcher: Arc<dyn DownloadSegmentFetchPort>,
        writer: Arc<dyn DownloadSegmentWritePort>,
        verifier: Arc<dyn DownloadSegmentVerifyPort>,
    ) -> Self {
        Self {
            fetcher,
            writer,
            verifier,
        }
    }
}

impl DownloadSegmentExecutionPort for DownloadSegmentExecutor {
    fn accept_segment_execution(
        &self,
        request: &DownloadSegmentExecutionRequest,
    ) -> AppResult<DownloadSegmentExecutionResult> {
        let fetched = match self.fetcher.fetch_segment(request)? {
            DownloadSegmentFetchOutcome::Fetched(fetched) => fetched,
            DownloadSegmentFetchOutcome::Failed(failure) => {
                return Ok(failure.into_execution_result(request));
            }
        };
        let written = match self.writer.write_segment(request, &fetched)? {
            DownloadSegmentWriteOutcome::Written(written) => written,
            DownloadSegmentWriteOutcome::Failed(failure) => {
                return Ok(failure.into_execution_result(request));
            }
        };
        match self.verifier.verify_segment(request, &fetched, &written)? {
            DownloadSegmentVerifyOutcome::Verified => {}
            DownloadSegmentVerifyOutcome::Failed(failure) => {
                return Ok(failure.into_execution_result(request));
            }
        }

        Ok(DownloadSegmentExecutionResult::Completed {
            request: request.clone(),
            downloaded_bytes: written.downloaded_bytes,
            partial_path: written.partial_path,
            etag: fetched.etag,
            hash_state_ref: written.hash_state_ref,
        })
    }
}

/// `downloads/download` 任务种类的恢复驱动。
///
/// 只有在持久化下载 checkpoint 仍然存在时，stage 2 restore 才允许把
/// 一个下载任务继续视为可恢复。staging 文件校验仍留给后续切片处理。
#[derive(Clone)]
pub struct DownloadJobDriver {
    checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
    pending_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
    segment_execution_port: Option<Arc<dyn DownloadSegmentExecutionPort>>,
}

impl DownloadJobDriver {
    /// 用共享的 checkpoint 仓储能力创建下载恢复驱动。
    pub fn new(checkpoint_repo: Arc<dyn DownloadCheckpointRepository>) -> Self {
        Self::with_pending_resume_work_source(checkpoint_repo, Arc::new(()))
    }

    /// Creates a download driver with an injected pending-work source for future execution turns.
    /// 使用注入的 pending-work source 创建下载 driver，供后续执行 turn 消费。
    pub fn with_pending_resume_work_source(
        checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
        pending_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
    ) -> Self {
        Self {
            checkpoint_repo,
            pending_resume_work_source,
            segment_execution_port: None,
        }
    }

    /// Creates a download driver with pending-work and fake/future segment execution ports.
    /// 使用 pending-work source 与 fake/后续 segment 执行端口创建下载 driver。
    pub fn with_pending_resume_work_source_and_execution_port(
        checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
        pending_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
        segment_execution_port: Arc<dyn DownloadSegmentExecutionPort>,
    ) -> Self {
        Self {
            checkpoint_repo,
            pending_resume_work_source,
            segment_execution_port: Some(segment_execution_port),
        }
    }

    /// Drains pending resume work for a job without running fetch/write/verify logic.
    /// 只取走指定 job 的 pending resume work，不执行 fetch/write/verify 逻辑。
    pub fn drain_pending_resume_work(
        &self,
        job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>> {
        self.pending_resume_work_source
            .drain_pending_resume_work(job_id)
    }

    /// Prepares a module-local resume execution turn without running download IO.
    /// 准备模块本地 resume execution turn，但不执行下载 IO。
    pub fn prepare_resume_execution_turn(
        &self,
        job_id: &JobId,
    ) -> AppResult<DownloadDriverExecutionTurn> {
        let Some(checkpoint) = self.checkpoint_repo.load(job_id)? else {
            return Ok(DownloadDriverExecutionTurn::CheckpointMissing {
                job_id: job_id.clone(),
            });
        };

        // Durable facts are reloaded before draining transient work so stale in-memory
        // plans never outrun the module-owned checkpoint boundary.
        // 必须先重新读取 durable facts，再取走瞬时 work，避免陈旧内存计划越过模块自有 checkpoint 边界。
        let pending_work = self.drain_pending_resume_work(job_id)?;
        if pending_work.is_empty() {
            return Ok(DownloadDriverExecutionTurn::NoPendingWork { checkpoint });
        }

        Ok(DownloadDriverExecutionTurn::PendingWorkAccepted {
            checkpoint,
            pending_work,
        })
    }

    /// Builds stable segment execution requests from an accepted local execution turn.
    /// 从已接收的本地 execution turn 构建稳定的 segment execution 请求。
    pub fn prepare_segment_execution_requests(
        &self,
        turn: &DownloadDriverExecutionTurn,
    ) -> AppResult<Vec<DownloadSegmentExecutionRequest>> {
        let DownloadDriverExecutionTurn::PendingWorkAccepted { pending_work, .. } = turn else {
            return Ok(Vec::new());
        };

        Ok(pending_work
            .iter()
            .flat_map(|work| {
                work.plan
                    .items
                    .iter()
                    .map(|item| DownloadSegmentExecutionRequest {
                        job_id: work.job_id.clone(),
                        segment_id: item.segment_id.clone(),
                        file_id: item.file_id.clone(),
                        source_locator: item.source_locator.clone(),
                        write_target: item.write_target.clone(),
                        expected_hash: item.expected_hash.clone(),
                        start_offset: item.start_offset,
                        length: item.length,
                        resume_mode: item.resume_mode,
                        checkpoint_ref: item.checkpoint_ref.clone(),
                    })
            })
            .collect())
    }

    /// Accepts prepared segment requests through a module-local execution port.
    /// 通过模块本地执行端口接收已准备好的 segment 请求。
    pub fn accept_segment_execution_requests(
        &self,
        execution_port: &dyn DownloadSegmentExecutionPort,
        requests: &[DownloadSegmentExecutionRequest],
    ) -> AppResult<Vec<DownloadSegmentExecutionResult>> {
        requests
            .iter()
            .map(|request| execution_port.accept_segment_execution(request))
            .collect()
    }

    /// Records fake completed segment results into the downloads checkpoint.
    /// 将 fake completed segment 结果记录进 downloads checkpoint。
    pub fn record_completed_segment_checkpoints(
        &self,
        job_id: &JobId,
        results: &[DownloadSegmentExecutionResult],
    ) -> AppResult<Option<DownloadCheckpointRecord>> {
        let Some(mut checkpoint) = self.checkpoint_repo.load(job_id)? else {
            return Ok(None);
        };
        let mut has_completed_result = false;

        for result in results {
            let DownloadSegmentExecutionResult::Completed {
                request,
                downloaded_bytes,
                partial_path,
                etag,
                hash_state_ref,
            } = result
            else {
                continue;
            };
            if &request.job_id != job_id {
                continue;
            }

            has_completed_result = true;
            let existing_index = checkpoint
                .segments
                .iter()
                .position(|segment| segment.segment_id == request.segment_id);
            let offset = existing_index
                .and_then(|index| checkpoint.segments.get(index).map(|segment| segment.offset))
                .unwrap_or(request.start_offset);
            let completed_segment = DownloadSegmentCheckpointRecord {
                job_id: checkpoint.job_id.clone(),
                segment_id: request.segment_id.clone(),
                file_id: request.file_id.clone(),
                offset,
                length: request.length,
                downloaded_bytes: *downloaded_bytes,
                status: DownloadSegmentCheckpointStatus::Completed,
                partial_path: partial_path.clone(),
                etag: etag.clone(),
                hash_state_ref: hash_state_ref.clone(),
            };

            if let Some(index) = existing_index {
                checkpoint.segments[index] = completed_segment;
            } else {
                checkpoint.segments.push(completed_segment);
            }
        }

        if !has_completed_result {
            // Accepted-only or empty result turns must not masquerade as checkpoint mutations.
            // Accepted-only 或空结果 turn 不能伪装成 checkpoint 变更。
            return Ok(None);
        }

        self.checkpoint_repo.save(&checkpoint)?;
        Ok(Some(checkpoint))
    }

    /// Records fake failed segment results into the downloads checkpoint.
    /// 将 fake failed segment 结果记录进 downloads checkpoint。
    pub fn record_failed_segment_checkpoints(
        &self,
        job_id: &JobId,
        results: &[DownloadSegmentExecutionResult],
    ) -> AppResult<Option<DownloadCheckpointRecord>> {
        let Some(mut checkpoint) = self.checkpoint_repo.load(job_id)? else {
            return Ok(None);
        };
        let mut has_failed_result = false;

        for result in results {
            let DownloadSegmentExecutionResult::Failed {
                request,
                downloaded_bytes,
                ..
            } = result
            else {
                continue;
            };
            if &request.job_id != job_id {
                continue;
            }

            has_failed_result = true;
            let existing_index = checkpoint
                .segments
                .iter()
                .position(|segment| segment.segment_id == request.segment_id);
            let (offset, partial_path, etag, hash_state_ref) = existing_index
                .and_then(|index| checkpoint.segments.get(index))
                .map(|segment| {
                    (
                        segment.offset,
                        segment.partial_path.clone(),
                        segment.etag.clone(),
                        segment.hash_state_ref.clone(),
                    )
                })
                .unwrap_or((request.start_offset, None, None, None));
            let failed_segment = DownloadSegmentCheckpointRecord {
                job_id: checkpoint.job_id.clone(),
                segment_id: request.segment_id.clone(),
                file_id: request.file_id.clone(),
                offset,
                length: request.length,
                downloaded_bytes: *downloaded_bytes,
                status: DownloadSegmentCheckpointStatus::Failed,
                partial_path,
                etag,
                hash_state_ref,
            };

            if let Some(index) = existing_index {
                checkpoint.segments[index] = failed_segment;
            } else {
                checkpoint.segments.push(failed_segment);
            }
        }

        if !has_failed_result {
            // Accepted-only or empty result turns must not masquerade as checkpoint mutations.
            // Accepted-only 或空结果 turn 不能伪装成 checkpoint 变更。
            return Ok(None);
        }

        self.checkpoint_repo.save(&checkpoint)?;
        Ok(Some(checkpoint))
    }

    /// Executes one fake/local resume turn by chaining downloads-owned helpers.
    /// 通过串联 downloads 自有 helper 执行一次 fake/local resume turn。
    pub fn execute_local_resume_turn(
        &self,
        job_id: &JobId,
        execution_port: &dyn DownloadSegmentExecutionPort,
    ) -> AppResult<Option<DownloadCheckpointRecord>> {
        let turn = self.prepare_resume_execution_turn(job_id)?;
        let requests = self.prepare_segment_execution_requests(&turn)?;
        let results = self.accept_segment_execution_requests(execution_port, &requests)?;
        let completed_checkpoint = self.record_completed_segment_checkpoints(job_id, &results)?;
        let failed_checkpoint = self.record_failed_segment_checkpoints(job_id, &results)?;
        Ok(failed_checkpoint.or(completed_checkpoint))
    }
}

impl JobDriver<()> for DownloadJobDriver {
    fn module(&self) -> &'static str {
        "downloads"
    }

    fn kind(&self) -> &'static str {
        "download"
    }

    fn restore(&self, snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        if self.checkpoint_repo.load(&snapshot.job_id)?.is_some() {
            return Ok(RestoreDisposition::Resumed);
        }

        Ok(RestoreDisposition::FailedAsUnrecoverable {
            reason: format!(
                "download checkpoint missing for queued job {}",
                snapshot.job_id
            ),
        })
    }

    fn run(&self, context: JobExecutionContext<'_, ()>) -> AppResult<JobRunDisposition> {
        let Some(segment_execution_port) = self.segment_execution_port.as_deref() else {
            return Ok(JobRunDisposition::Deferred {
                reason: format!(
                    "downloads execution port not wired for job {}",
                    context.job_id()
                ),
            });
        };

        match self.execute_local_resume_turn(context.job_id(), segment_execution_port)? {
            Some(_) => Ok(JobRunDisposition::Accepted),
            None => Ok(JobRunDisposition::Deferred {
                reason: format!(
                    "downloads execution produced no checkpoint mutation for job {}",
                    context.job_id()
                ),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use launcher_kernel_foundation::{
        AppError, AppErrorSeverity, CorrelationId, IsoDateTime, JobId,
    };
    use launcher_kernel_jobs::{
        JobDriver, JobExecutionContext, JobProgress, JobRunDisposition, JobState, JobUiState,
    };

    use crate::facade::{
        DownloadPendingResumeWork, DownloadPendingResumeWorkSource, DownloadResumeWorkItem,
        DownloadResumeWorkMode, DownloadResumeWorkPlan, DownloadResumeWorkScheduler,
        InMemoryDownloadResumeWorkScheduler,
    };

    use super::{
        DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadDriverExecutionTurn,
        DownloadJobDriver, DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus,
        DownloadSegmentExecutionPort, DownloadSegmentExecutionRequest,
        DownloadSegmentExecutionResult, DownloadSegmentExecutor, DownloadSegmentFetchOutcome,
        DownloadSegmentFetchPort, DownloadSegmentFetchResult, DownloadSegmentHandledFailure,
        DownloadSegmentStagingTarget, DownloadSegmentVerifyOutcome, DownloadSegmentVerifyPort,
        DownloadSegmentWriteOutcome, DownloadSegmentWritePort, DownloadSegmentWriteResult,
    };

    #[derive(Default)]
    struct InMemoryCheckpointRepository {
        records: Mutex<HashMap<JobId, DownloadCheckpointRecord>>,
    }

    impl DownloadCheckpointRepository for InMemoryCheckpointRepository {
        fn load(
            &self,
            job_id: &JobId,
        ) -> launcher_kernel_foundation::AppResult<Option<DownloadCheckpointRecord>> {
            Ok(self
                .records
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .get(job_id)
                .cloned())
        }

        fn save(
            &self,
            checkpoint: &DownloadCheckpointRecord,
        ) -> launcher_kernel_foundation::AppResult<()> {
            self.records
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .insert(checkpoint.job_id.clone(), checkpoint.clone());
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingSegmentExecutionPort {
        accepted_requests: Mutex<Vec<DownloadSegmentExecutionRequest>>,
    }

    impl RecordingSegmentExecutionPort {
        fn accepted_requests(&self) -> Vec<DownloadSegmentExecutionRequest> {
            self.accepted_requests
                .lock()
                .expect("recording execution port mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadSegmentExecutionPort for RecordingSegmentExecutionPort {
        fn accept_segment_execution(
            &self,
            request: &DownloadSegmentExecutionRequest,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentExecutionResult> {
            self.accepted_requests
                .lock()
                .expect("recording execution port mutex should not be poisoned")
                .push(request.clone());

            Ok(DownloadSegmentExecutionResult::Accepted {
                request: request.clone(),
            })
        }
    }

    struct CompletedSegmentExecutionPort;

    impl DownloadSegmentExecutionPort for CompletedSegmentExecutionPort {
        fn accept_segment_execution(
            &self,
            request: &DownloadSegmentExecutionRequest,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentExecutionResult> {
            Ok(DownloadSegmentExecutionResult::Completed {
                request: request.clone(),
                downloaded_bytes: request.length,
                partial_path: Some(request.write_target.clone()),
                etag: Some(format!("etag-{}", request.segment_id)),
                hash_state_ref: Some(format!("hash-{}", request.segment_id)),
            })
        }
    }

    struct FailedSegmentExecutionPort;

    impl DownloadSegmentExecutionPort for FailedSegmentExecutionPort {
        fn accept_segment_execution(
            &self,
            request: &DownloadSegmentExecutionRequest,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentExecutionResult> {
            Ok(DownloadSegmentExecutionResult::Failed {
                request: request.clone(),
                downloaded_bytes: 128,
                reason: "network timeout while reading segment".into(),
                retryable: true,
            })
        }
    }

    #[derive(Default)]
    struct RecordingCompletedSegmentExecutionPort {
        accepted_requests: Mutex<Vec<DownloadSegmentExecutionRequest>>,
    }

    impl RecordingCompletedSegmentExecutionPort {
        fn accepted_requests(&self) -> Vec<DownloadSegmentExecutionRequest> {
            self.accepted_requests
                .lock()
                .expect("recording completed execution port mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadSegmentExecutionPort for RecordingCompletedSegmentExecutionPort {
        fn accept_segment_execution(
            &self,
            request: &DownloadSegmentExecutionRequest,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentExecutionResult> {
            self.accepted_requests
                .lock()
                .expect("recording completed execution port mutex should not be poisoned")
                .push(request.clone());

            Ok(DownloadSegmentExecutionResult::Completed {
                request: request.clone(),
                downloaded_bytes: request.length,
                partial_path: Some(request.write_target.clone()),
                etag: Some(format!("etag-{}", request.segment_id)),
                hash_state_ref: Some(format!("hash-{}", request.segment_id)),
            })
        }
    }

    #[derive(Default)]
    struct RecordingFetchPort {
        requests: Mutex<Vec<DownloadSegmentExecutionRequest>>,
    }

    impl RecordingFetchPort {
        fn requests(&self) -> Vec<DownloadSegmentExecutionRequest> {
            self.requests
                .lock()
                .expect("recording fetch port mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadSegmentFetchPort for RecordingFetchPort {
        fn fetch_segment(
            &self,
            request: &DownloadSegmentExecutionRequest,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentFetchOutcome> {
            self.requests
                .lock()
                .expect("recording fetch port mutex should not be poisoned")
                .push(request.clone());

            Ok(DownloadSegmentFetchOutcome::Fetched(
                DownloadSegmentFetchResult {
                    bytes: vec![1, 2, 3, 4],
                    etag: Some(format!("etag-{}", request.segment_id)),
                },
            ))
        }
    }

    #[derive(Default)]
    struct RecordingWritePort {
        writes: Mutex<Vec<(DownloadSegmentExecutionRequest, Vec<u8>)>>,
    }

    impl RecordingWritePort {
        fn writes(&self) -> Vec<(DownloadSegmentExecutionRequest, Vec<u8>)> {
            self.writes
                .lock()
                .expect("recording write port mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadSegmentWritePort for RecordingWritePort {
        fn write_segment(
            &self,
            request: &DownloadSegmentExecutionRequest,
            fetched: &DownloadSegmentFetchResult,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentWriteOutcome> {
            self.writes
                .lock()
                .expect("recording write port mutex should not be poisoned")
                .push((request.clone(), fetched.bytes.clone()));

            Ok(DownloadSegmentWriteOutcome::Written(
                DownloadSegmentWriteResult {
                    downloaded_bytes: fetched.bytes.len() as u64,
                    partial_path: Some(request.write_target.clone()),
                    hash_state_ref: Some(format!("hash-{}", request.segment_id)),
                },
            ))
        }
    }

    struct HandledFailureWritePort;

    impl DownloadSegmentWritePort for HandledFailureWritePort {
        fn write_segment(
            &self,
            _request: &DownloadSegmentExecutionRequest,
            _fetched: &DownloadSegmentFetchResult,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentWriteOutcome> {
            Ok(DownloadSegmentWriteOutcome::Failed(
                DownloadSegmentHandledFailure {
                    downloaded_bytes: 2,
                    reason: "staging writer reported a handled short write".into(),
                    retryable: true,
                },
            ))
        }
    }

    #[derive(Default)]
    struct RecordingVerifyPort {
        verifications: Mutex<Vec<(DownloadSegmentExecutionRequest, Option<String>, u64)>>,
    }

    impl RecordingVerifyPort {
        fn verifications(&self) -> Vec<(DownloadSegmentExecutionRequest, Option<String>, u64)> {
            self.verifications
                .lock()
                .expect("recording verify port mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadSegmentVerifyPort for RecordingVerifyPort {
        fn verify_segment(
            &self,
            request: &DownloadSegmentExecutionRequest,
            _fetched: &DownloadSegmentFetchResult,
            written: &DownloadSegmentWriteResult,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentVerifyOutcome> {
            self.verifications
                .lock()
                .expect("recording verify port mutex should not be poisoned")
                .push((
                    request.clone(),
                    request.expected_hash.clone(),
                    written.downloaded_bytes,
                ));

            Ok(DownloadSegmentVerifyOutcome::Verified)
        }
    }

    struct InfrastructureFailureVerifyPort;

    impl DownloadSegmentVerifyPort for InfrastructureFailureVerifyPort {
        fn verify_segment(
            &self,
            _request: &DownloadSegmentExecutionRequest,
            _fetched: &DownloadSegmentFetchResult,
            _written: &DownloadSegmentWriteResult,
        ) -> launcher_kernel_foundation::AppResult<DownloadSegmentVerifyOutcome> {
            Err(AppError::new(
                "TEST_EXECUTOR_INFRASTRUCTURE",
                "executor infrastructure is unavailable",
                false,
                AppErrorSeverity::Fatal,
                CorrelationId::new("executor-infra"),
            ))
        }
    }

    #[test]
    fn download_segment_staging_target_accepts_normal_relative_components() {
        let target = DownloadSegmentStagingTarget::parse("file-a/segment-0001.part")
            .expect("normal relative staging target should be accepted");

        assert_eq!(
            target.as_str(),
            "file-a/segment-0001.part",
            "accepted target should preserve normalized relative components"
        );
    }

    #[test]
    fn download_segment_staging_target_rejects_unsafe_targets_as_handled_failure() {
        let unsafe_targets = [
            "",
            ".",
            "../escape.part",
            "file-a/../escape.part",
            r"C:\escape.part",
            r"\\server\share\escape.part",
            r"\absolute\escape.part",
            "/absolute/escape.part",
        ];

        for unsafe_target in unsafe_targets {
            let failure = DownloadSegmentStagingTarget::parse(unsafe_target)
                .expect_err("unsafe staging target should become a handled failure");

            assert_eq!(
                failure.downloaded_bytes, 0,
                "unsafe target rejection should not claim written bytes"
            );
            assert!(
                !failure.retryable,
                "unsafe target rejection should not be retried automatically"
            );
            assert!(
                failure.reason.contains("unsafe staging write target"),
                "unsafe target rejection should stay module-local and diagnostic"
            );
        }
    }

    fn make_snapshot(job_id: JobId) -> launcher_kernel_jobs::JobSnapshot<()> {
        launcher_kernel_jobs::JobSnapshot {
            job_id,
            module: "downloads".into(),
            kind: "download".into(),
            state: JobState::Queued,
            ui_state: JobUiState::Queued,
            progress: JobProgress::pending(),
            recoverable: true,
            updated_at: IsoDateTime::now(),
            extension: None,
        }
    }

    fn make_resume_work_plan(segment_id: &str) -> DownloadResumeWorkPlan {
        DownloadResumeWorkPlan {
            items: vec![DownloadResumeWorkItem {
                segment_id: segment_id.into(),
                file_id: "file-1".into(),
                source_locator: format!("https://example.invalid/{segment_id}"),
                write_target: format!("{segment_id}.part"),
                expected_hash: None,
                start_offset: 0,
                length: 1024,
                resume_mode: DownloadResumeWorkMode::FromStart,
                checkpoint_ref: None,
            }],
        }
    }

    fn make_resume_work_plan_with_items(segment_ids: &[&str]) -> DownloadResumeWorkPlan {
        DownloadResumeWorkPlan {
            items: segment_ids
                .iter()
                .map(|segment_id| DownloadResumeWorkItem {
                    segment_id: (*segment_id).into(),
                    file_id: "file-1".into(),
                    source_locator: format!("https://example.invalid/{segment_id}"),
                    write_target: format!("{segment_id}.part"),
                    expected_hash: None,
                    start_offset: 0,
                    length: 1024,
                    resume_mode: DownloadResumeWorkMode::FromStart,
                    checkpoint_ref: None,
                })
                .collect(),
        }
    }

    fn make_segment_execution_request(
        job_id: &JobId,
        segment_id: &str,
    ) -> DownloadSegmentExecutionRequest {
        DownloadSegmentExecutionRequest {
            job_id: job_id.clone(),
            segment_id: segment_id.into(),
            file_id: "file-1".into(),
            source_locator: format!("https://example.invalid/{segment_id}"),
            write_target: format!("{segment_id}.part"),
            expected_hash: None,
            start_offset: 0,
            length: 1024,
            resume_mode: DownloadResumeWorkMode::FromStart,
            checkpoint_ref: None,
        }
    }

    fn make_segment_checkpoint_record(
        job_id: &JobId,
        segment_id: &str,
        status: DownloadSegmentCheckpointStatus,
    ) -> DownloadSegmentCheckpointRecord {
        DownloadSegmentCheckpointRecord {
            job_id: job_id.clone(),
            segment_id: segment_id.into(),
            file_id: "file-1".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 0,
            status,
            partial_path: None,
            etag: None,
            hash_state_ref: None,
        }
    }

    #[test]
    fn restore_returns_failed_when_checkpoint_is_missing() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let snapshot = make_snapshot(JobId::generate());

        let disposition = driver
            .restore(&snapshot)
            .expect("restore should not error for missing checkpoint");

        assert!(
            matches!(
                disposition,
                launcher_kernel_jobs::RestoreDisposition::FailedAsUnrecoverable { .. }
            ),
            "missing checkpoint should make the queued download unrecoverable"
        );
    }

    #[test]
    fn restore_returns_resumed_when_checkpoint_exists() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo.clone());
        let snapshot = make_snapshot(JobId::generate());

        repo.save(&DownloadCheckpointRecord::empty(snapshot.job_id.clone()))
            .expect("saving a synthetic checkpoint should succeed");

        let disposition = driver
            .restore(&snapshot)
            .expect("restore should not error for persisted checkpoint");

        assert_eq!(
            disposition,
            launcher_kernel_jobs::RestoreDisposition::Resumed,
            "persisted checkpoint should keep the queued download resumable"
        );
    }

    #[test]
    fn driver_run_defers_without_execution_port_and_keeps_pending_work() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let plan = make_resume_work_plan("segment-run-deferred");
        let driver =
            DownloadJobDriver::with_pending_resume_work_source(repo, Arc::new(scheduler.clone()));

        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");
        let snapshot = make_snapshot(job_id.clone());

        let disposition = driver
            .run(JobExecutionContext::new(&snapshot))
            .expect("missing execution port should be a non-terminal run result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("execution port not wired"));
            }
            other => panic!("missing execution port should defer run, got {other:?}"),
        }
        assert_eq!(
            scheduler
                .drain_pending_resume_work(&job_id)
                .expect("pending work should still be available after deferred run"),
            vec![DownloadPendingResumeWork { job_id, plan }],
            "default run must not drain pending work without an execution port"
        );
    }

    #[test]
    fn driver_run_with_execution_port_records_completed_checkpoint_and_accepts() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let plan = make_resume_work_plan("segment-run-completed");
        let driver = DownloadJobDriver::with_pending_resume_work_source_and_execution_port(
            repo.clone(),
            Arc::new(scheduler.clone()),
            Arc::new(CompletedSegmentExecutionPort),
        );

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");
        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");
        let snapshot = make_snapshot(job_id.clone());

        let disposition = driver
            .run(JobExecutionContext::new(&snapshot))
            .expect("fake completed execution should produce a run disposition");

        assert_eq!(disposition, JobRunDisposition::Accepted);
        assert!(
            scheduler.pending_work().is_empty(),
            "accepted run should consume the pending work it executed"
        );
        assert_eq!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments,
            vec![DownloadSegmentCheckpointRecord {
                job_id,
                segment_id: "segment-run-completed".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 1024,
                downloaded_bytes: 1024,
                status: DownloadSegmentCheckpointStatus::Completed,
                partial_path: Some("segment-run-completed.part".into()),
                etag: Some("etag-segment-run-completed".into()),
                hash_state_ref: Some("hash-segment-run-completed".into()),
            }],
            "fake completed run should persist checkpoint mutation"
        );
    }

    #[test]
    fn driver_run_with_execution_port_defers_and_keeps_pending_work_when_checkpoint_missing() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let plan = make_resume_work_plan("segment-run-missing-checkpoint");
        let driver = DownloadJobDriver::with_pending_resume_work_source_and_execution_port(
            repo,
            Arc::new(scheduler.clone()),
            Arc::new(CompletedSegmentExecutionPort),
        );

        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");
        let snapshot = make_snapshot(job_id.clone());

        let disposition = driver
            .run(JobExecutionContext::new(&snapshot))
            .expect("missing checkpoint should be a non-terminal run result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("no checkpoint mutation"));
            }
            other => panic!("missing checkpoint should defer run, got {other:?}"),
        }
        assert_eq!(
            scheduler
                .drain_pending_resume_work(&job_id)
                .expect("pending work should remain when checkpoint is missing"),
            vec![DownloadPendingResumeWork { job_id, plan }],
            "missing checkpoint must not drain pending work"
        );
    }

    #[test]
    fn driver_run_with_execution_port_defers_when_no_pending_work_exists() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let driver = DownloadJobDriver::with_pending_resume_work_source_and_execution_port(
            repo.clone(),
            Arc::new(scheduler.clone()),
            Arc::new(CompletedSegmentExecutionPort),
        );

        repo.save(&DownloadCheckpointRecord::empty(job_id.clone()))
            .expect("saving a synthetic checkpoint should succeed");
        let snapshot = make_snapshot(job_id.clone());

        let disposition = driver
            .run(JobExecutionContext::new(&snapshot))
            .expect("empty pending work should be a non-terminal run result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("no checkpoint mutation"));
            }
            other => panic!("no pending work should defer run, got {other:?}"),
        }
        assert!(
            scheduler.pending_work().is_empty(),
            "no pending work branch should not create scheduler work"
        );
        assert!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments
                .is_empty(),
            "no pending work must not mutate checkpoint segments"
        );
    }

    #[test]
    fn driver_run_with_accepted_only_port_defers_when_checkpoint_is_not_mutated() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let plan = make_resume_work_plan("segment-run-accepted-only");
        let execution_port = Arc::new(RecordingSegmentExecutionPort::default());
        let driver = DownloadJobDriver::with_pending_resume_work_source_and_execution_port(
            repo.clone(),
            Arc::new(scheduler.clone()),
            execution_port.clone(),
        );

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");
        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");
        let snapshot = make_snapshot(job_id.clone());

        let disposition = driver
            .run(JobExecutionContext::new(&snapshot))
            .expect("accepted-only execution should be a non-terminal run result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("no checkpoint mutation"));
            }
            other => panic!("accepted-only execution should defer run, got {other:?}"),
        }
        assert_eq!(
            execution_port.accepted_requests().len(),
            1,
            "accepted-only port should still receive the prepared request"
        );
        assert!(
            scheduler.pending_work().is_empty(),
            "accepted-only execution consumes the work it accepted"
        );
        assert!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments
                .is_empty(),
            "accepted-only execution must not invent checkpoint mutation"
        );
    }

    #[test]
    fn download_job_driver_pending_resume_work_drains_from_injected_source() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let plan = make_resume_work_plan("segment-driver");
        let driver =
            DownloadJobDriver::with_pending_resume_work_source(repo, Arc::new(scheduler.clone()));

        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");

        let drained = driver
            .drain_pending_resume_work(&job_id)
            .expect("driver local pending-work consumer should delegate to the source");

        assert_eq!(drained, vec![DownloadPendingResumeWork { job_id, plan }]);
        assert!(
            scheduler.pending_work().is_empty(),
            "driver drain should consume the source work for the job"
        );
    }

    #[test]
    fn download_job_driver_pending_resume_work_default_source_returns_empty() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let job_id = JobId::generate();

        let drained = driver
            .drain_pending_resume_work(&job_id)
            .expect("default pending-work source should not fail");

        assert!(
            drained.is_empty(),
            "default driver constructor should expose an empty pending-work source"
        );
    }

    #[test]
    fn download_job_driver_execution_turn_keeps_pending_work_when_checkpoint_missing() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let plan = make_resume_work_plan("segment-missing-checkpoint");
        let driver =
            DownloadJobDriver::with_pending_resume_work_source(repo, Arc::new(scheduler.clone()));

        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");

        let turn = driver
            .prepare_resume_execution_turn(&job_id)
            .expect("missing checkpoint should classify without draining pending work");

        assert_eq!(
            turn,
            DownloadDriverExecutionTurn::CheckpointMissing {
                job_id: job_id.clone()
            }
        );
        assert_eq!(
            scheduler.pending_work(),
            vec![DownloadPendingResumeWork { job_id, plan }],
            "checkpoint-missing turn must preserve pending work for a later durable recovery path"
        );
    }

    #[test]
    fn download_job_driver_execution_turn_accepts_pending_work_after_checkpoint_reload() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let plan = make_resume_work_plan("segment-execution-turn");
        let driver = DownloadJobDriver::with_pending_resume_work_source(
            repo.clone(),
            Arc::new(scheduler.clone()),
        );

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");
        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");

        let turn = driver
            .prepare_resume_execution_turn(&job_id)
            .expect("checkpoint-present turn should drain pending work");

        assert_eq!(
            turn,
            DownloadDriverExecutionTurn::PendingWorkAccepted {
                checkpoint,
                pending_work: vec![DownloadPendingResumeWork { job_id, plan }]
            }
        );
        assert!(
            scheduler.pending_work().is_empty(),
            "checkpoint-present turn should consume pending work for the job"
        );
    }

    #[test]
    fn download_job_driver_execution_turn_classifies_empty_pending_work() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo.clone());
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");

        let turn = driver
            .prepare_resume_execution_turn(&job_id)
            .expect("checkpoint-present empty pending work should classify explicitly");

        assert_eq!(
            turn,
            DownloadDriverExecutionTurn::NoPendingWork { checkpoint },
            "empty pending work must not be confused with runtime completion"
        );
    }

    #[test]
    fn download_job_driver_segment_execution_requests_preserve_pending_work_order() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let first_plan = make_resume_work_plan_with_items(&["segment-a", "segment-b"]);
        let second_plan = make_resume_work_plan("segment-c");
        let turn = DownloadDriverExecutionTurn::PendingWorkAccepted {
            checkpoint,
            pending_work: vec![
                DownloadPendingResumeWork {
                    job_id: job_id.clone(),
                    plan: first_plan,
                },
                DownloadPendingResumeWork {
                    job_id: job_id.clone(),
                    plan: second_plan,
                },
            ],
        };

        let requests = driver
            .prepare_segment_execution_requests(&turn)
            .expect("pending work should convert into segment execution requests");

        assert_eq!(
            requests,
            vec![
                make_segment_execution_request(&job_id, "segment-a"),
                make_segment_execution_request(&job_id, "segment-b"),
                make_segment_execution_request(&job_id, "segment-c"),
            ],
            "segment execution request conversion must preserve pending-work and item order"
        );
    }

    #[test]
    fn download_job_driver_segment_execution_requests_ignore_non_pending_turns() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());

        let missing_requests = driver
            .prepare_segment_execution_requests(&DownloadDriverExecutionTurn::CheckpointMissing {
                job_id,
            })
            .expect("checkpoint-missing turn should not produce execution requests");
        assert!(
            missing_requests.is_empty(),
            "checkpoint-missing turns must not start segment execution"
        );

        let empty_requests = driver
            .prepare_segment_execution_requests(&DownloadDriverExecutionTurn::NoPendingWork {
                checkpoint,
            })
            .expect("no-pending turn should not produce execution requests");
        assert!(
            empty_requests.is_empty(),
            "no-pending turns must not be confused with segment execution"
        );
    }

    #[test]
    fn download_job_driver_segment_execution_acceptance_preserves_request_order() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let execution_port = RecordingSegmentExecutionPort::default();
        let job_id = JobId::generate();
        let requests = vec![
            make_segment_execution_request(&job_id, "segment-a"),
            make_segment_execution_request(&job_id, "segment-b"),
            make_segment_execution_request(&job_id, "segment-c"),
        ];

        let results = driver
            .accept_segment_execution_requests(&execution_port, &requests)
            .expect("fake segment execution acceptance should collect local results");

        assert_eq!(
            execution_port.accepted_requests(),
            requests,
            "fake execution port must be called once per request in stable order"
        );
        assert_eq!(
            results,
            vec![
                DownloadSegmentExecutionResult::Accepted {
                    request: make_segment_execution_request(&job_id, "segment-a"),
                },
                DownloadSegmentExecutionResult::Accepted {
                    request: make_segment_execution_request(&job_id, "segment-b"),
                },
                DownloadSegmentExecutionResult::Accepted {
                    request: make_segment_execution_request(&job_id, "segment-c"),
                },
            ],
            "driver acceptance helper must preserve result order"
        );
    }

    #[test]
    fn download_job_driver_segment_completion_result_preserves_fake_completion_payload() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let execution_port = CompletedSegmentExecutionPort;
        let job_id = JobId::generate();
        let request = make_segment_execution_request(&job_id, "segment-completed");

        let results = driver
            .accept_segment_execution_requests(&execution_port, std::slice::from_ref(&request))
            .expect("fake segment completion result should stay in the local result channel");

        assert_eq!(
            results,
            vec![DownloadSegmentExecutionResult::Completed {
                request,
                downloaded_bytes: 1024,
                partial_path: Some("segment-completed.part".into()),
                etag: Some("etag-segment-completed".into()),
                hash_state_ref: Some("hash-segment-completed".into()),
            }],
            "fake completion must preserve payload facts for a later checkpoint mutation slice"
        );
    }

    #[test]
    fn download_job_driver_segment_failure_result_preserves_fake_failure_payload() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let execution_port = FailedSegmentExecutionPort;
        let job_id = JobId::generate();
        let request = make_segment_execution_request(&job_id, "segment-failed");

        let results = driver
            .accept_segment_execution_requests(&execution_port, std::slice::from_ref(&request))
            .expect("fake segment failure result should stay in the local result channel");

        assert_eq!(
            results,
            vec![DownloadSegmentExecutionResult::Failed {
                request,
                downloaded_bytes: 128,
                reason: "network timeout while reading segment".into(),
                retryable: true,
            }],
            "fake failure must preserve local metadata for a later checkpoint or retry slice"
        );
    }

    #[test]
    fn download_segment_executor_adapter_passes_request_through_sub_ports_and_completes() {
        let job_id = JobId::generate();
        let mut request = make_segment_execution_request(&job_id, "segment-executor");
        request.expected_hash = Some("hash-segment-executor".into());
        request.start_offset = 128;
        request.length = 4;
        request.resume_mode = DownloadResumeWorkMode::Partial;
        request.checkpoint_ref = Some("segment-executor".into());

        let fetcher = Arc::new(RecordingFetchPort::default());
        let writer = Arc::new(RecordingWritePort::default());
        let verifier = Arc::new(RecordingVerifyPort::default());
        let executor =
            DownloadSegmentExecutor::new(fetcher.clone(), writer.clone(), verifier.clone());

        let result = executor
            .accept_segment_execution(&request)
            .expect("executor adapter should complete through fake sub-ports");

        assert_eq!(
            result,
            DownloadSegmentExecutionResult::Completed {
                request: request.clone(),
                downloaded_bytes: 4,
                partial_path: Some("segment-executor.part".into()),
                etag: Some("etag-segment-executor".into()),
                hash_state_ref: Some("hash-segment-executor".into()),
            },
            "executor adapter should project fake sub-port output into the existing completed result"
        );
        assert_eq!(
            fetcher.requests(),
            vec![request.clone()],
            "fetch port should receive the original request facts"
        );
        assert_eq!(
            writer.writes(),
            vec![(request.clone(), vec![1, 2, 3, 4])],
            "write port should receive the same request and fetched bytes"
        );
        assert_eq!(
            verifier.verifications(),
            vec![(request, Some("hash-segment-executor".into()), 4)],
            "verify port should receive the request hash expectation and written byte count"
        );
    }

    #[test]
    fn download_segment_executor_adapter_maps_handled_write_failure_to_failed_result() {
        let job_id = JobId::generate();
        let request = make_segment_execution_request(&job_id, "segment-write-failed");
        let executor = DownloadSegmentExecutor::new(
            Arc::new(RecordingFetchPort::default()),
            Arc::new(HandledFailureWritePort),
            Arc::new(RecordingVerifyPort::default()),
        );

        let result = executor
            .accept_segment_execution(&request)
            .expect("handled write failure should stay in the local result channel");

        assert_eq!(
            result,
            DownloadSegmentExecutionResult::Failed {
                request,
                downloaded_bytes: 2,
                reason: "staging writer reported a handled short write".into(),
                retryable: true,
            },
            "handled sub-port failures should become module-local failed execution results"
        );
    }

    #[test]
    fn download_segment_executor_adapter_propagates_infrastructure_errors() {
        let job_id = JobId::generate();
        let request = make_segment_execution_request(&job_id, "segment-infrastructure-error");
        let executor = DownloadSegmentExecutor::new(
            Arc::new(RecordingFetchPort::default()),
            Arc::new(RecordingWritePort::default()),
            Arc::new(InfrastructureFailureVerifyPort),
        );

        let error = executor
            .accept_segment_execution(&request)
            .expect_err("infrastructure failures should propagate as AppError");

        assert_eq!(
            error.code, "TEST_EXECUTOR_INFRASTRUCTURE",
            "true infrastructure errors must not be converted into local segment results"
        );
    }

    #[test]
    fn download_job_driver_completed_result_checkpoint_mutation_replaces_and_saves_segment() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo.clone());
        let job_id = JobId::generate();
        let unrelated_segment = make_segment_checkpoint_record(
            &job_id,
            "segment-unrelated",
            DownloadSegmentCheckpointStatus::Pending,
        );
        let mut existing_segment = make_segment_checkpoint_record(
            &job_id,
            "segment-completed",
            DownloadSegmentCheckpointStatus::InProgress,
        );
        existing_segment.offset = 256;
        existing_segment.downloaded_bytes = 128;
        existing_segment.partial_path = Some("old-segment-completed.part".into());
        existing_segment.etag = Some("old-etag".into());
        existing_segment.hash_state_ref = Some("old-hash".into());
        repo.save(&DownloadCheckpointRecord {
            job_id: job_id.clone(),
            segments: vec![unrelated_segment.clone(), existing_segment],
        })
        .expect("saving a checkpoint before mutation should succeed");

        let accepted_request = make_segment_execution_request(&job_id, "segment-accepted");
        let mut completed_request = make_segment_execution_request(&job_id, "segment-completed");
        completed_request.start_offset = 512;
        completed_request.resume_mode = DownloadResumeWorkMode::Partial;
        completed_request.checkpoint_ref = Some("segment-completed".into());
        let completed_result = DownloadSegmentExecutionResult::Completed {
            request: completed_request,
            downloaded_bytes: 1024,
            partial_path: Some("segment-completed.part".into()),
            etag: Some("etag-segment-completed".into()),
            hash_state_ref: Some("hash-segment-completed".into()),
        };

        let saved_checkpoint = driver
            .record_completed_segment_checkpoints(
                &job_id,
                &[
                    DownloadSegmentExecutionResult::Accepted {
                        request: accepted_request,
                    },
                    completed_result,
                ],
            )
            .expect("completed results should be saved through the checkpoint repository")
            .expect("existing checkpoint should be reloaded before mutation");

        let expected_completed_segment = DownloadSegmentCheckpointRecord {
            job_id: job_id.clone(),
            segment_id: "segment-completed".into(),
            file_id: "file-1".into(),
            offset: 256,
            length: 1024,
            downloaded_bytes: 1024,
            status: DownloadSegmentCheckpointStatus::Completed,
            partial_path: Some("segment-completed.part".into()),
            etag: Some("etag-segment-completed".into()),
            hash_state_ref: Some("hash-segment-completed".into()),
        };
        let expected_segments = vec![unrelated_segment, expected_completed_segment];

        assert_eq!(
            saved_checkpoint.segments, expected_segments,
            "completed result mutation should replace matching segment facts without reordering unrelated facts"
        );
        assert_eq!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments,
            expected_segments,
            "driver helper must persist the mutated checkpoint through the repository port"
        );
    }

    #[test]
    fn download_job_driver_failed_result_checkpoint_mutation_replaces_and_saves_segment() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo.clone());
        let job_id = JobId::generate();
        let unrelated_segment = make_segment_checkpoint_record(
            &job_id,
            "segment-unrelated",
            DownloadSegmentCheckpointStatus::Pending,
        );
        let mut existing_segment = make_segment_checkpoint_record(
            &job_id,
            "segment-failed",
            DownloadSegmentCheckpointStatus::InProgress,
        );
        existing_segment.offset = 384;
        existing_segment.downloaded_bytes = 64;
        existing_segment.partial_path = Some("old-segment-failed.part".into());
        existing_segment.etag = Some("old-failed-etag".into());
        existing_segment.hash_state_ref = Some("old-failed-hash".into());
        repo.save(&DownloadCheckpointRecord {
            job_id: job_id.clone(),
            segments: vec![unrelated_segment.clone(), existing_segment],
        })
        .expect("saving a checkpoint before failed mutation should succeed");

        let accepted_request = make_segment_execution_request(&job_id, "segment-accepted");
        let completed_request = make_segment_execution_request(&job_id, "segment-completed");
        let mut failed_request = make_segment_execution_request(&job_id, "segment-failed");
        failed_request.start_offset = 768;
        failed_request.resume_mode = DownloadResumeWorkMode::Partial;
        failed_request.checkpoint_ref = Some("segment-failed".into());
        let failed_result = DownloadSegmentExecutionResult::Failed {
            request: failed_request,
            downloaded_bytes: 128,
            reason: "network timeout while reading segment".into(),
            retryable: true,
        };

        let saved_checkpoint = driver
            .record_failed_segment_checkpoints(
                &job_id,
                &[
                    DownloadSegmentExecutionResult::Accepted {
                        request: accepted_request,
                    },
                    DownloadSegmentExecutionResult::Completed {
                        request: completed_request,
                        downloaded_bytes: 1024,
                        partial_path: Some("segment-completed.part".into()),
                        etag: Some("etag-segment-completed".into()),
                        hash_state_ref: Some("hash-segment-completed".into()),
                    },
                    failed_result,
                ],
            )
            .expect("failed results should be saved through the checkpoint repository")
            .expect("existing checkpoint should be reloaded before failed mutation");

        let expected_failed_segment = DownloadSegmentCheckpointRecord {
            job_id: job_id.clone(),
            segment_id: "segment-failed".into(),
            file_id: "file-1".into(),
            offset: 384,
            length: 1024,
            downloaded_bytes: 128,
            status: DownloadSegmentCheckpointStatus::Failed,
            partial_path: Some("old-segment-failed.part".into()),
            etag: Some("old-failed-etag".into()),
            hash_state_ref: Some("old-failed-hash".into()),
        };
        let expected_segments = vec![unrelated_segment, expected_failed_segment];

        assert_eq!(
            saved_checkpoint.segments, expected_segments,
            "failed result mutation should replace matching segment facts without reordering unrelated facts"
        );
        assert_eq!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments,
            expected_segments,
            "driver helper must persist failed checkpoint facts through the repository port"
        );
    }

    #[test]
    fn download_job_driver_fake_local_resume_execution_records_completed_checkpoint() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let plan = make_resume_work_plan("segment-orchestrated");
        let execution_port = RecordingCompletedSegmentExecutionPort::default();
        let driver = DownloadJobDriver::with_pending_resume_work_source(
            repo.clone(),
            Arc::new(scheduler.clone()),
        );

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");
        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");

        let saved_checkpoint = driver
            .execute_local_resume_turn(&job_id, &execution_port)
            .expect("fake local resume turn should chain existing driver helpers")
            .expect("completed fake execution should save checkpoint facts");

        let expected_request = make_segment_execution_request(&job_id, "segment-orchestrated");
        assert_eq!(
            execution_port.accepted_requests(),
            vec![expected_request],
            "fake local orchestration should hand the prepared request to the execution port"
        );
        assert!(
            scheduler.pending_work().is_empty(),
            "fake local orchestration should drain accepted pending work for the job"
        );

        let expected_completed_segment = DownloadSegmentCheckpointRecord {
            job_id: job_id.clone(),
            segment_id: "segment-orchestrated".into(),
            file_id: "file-1".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 1024,
            status: DownloadSegmentCheckpointStatus::Completed,
            partial_path: Some("segment-orchestrated.part".into()),
            etag: Some("etag-segment-orchestrated".into()),
            hash_state_ref: Some("hash-segment-orchestrated".into()),
        };

        assert_eq!(
            saved_checkpoint.segments,
            vec![expected_completed_segment.clone()],
            "fake local orchestration should return the saved completed segment facts"
        );
        assert_eq!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments,
            vec![expected_completed_segment],
            "fake local orchestration must persist completed segment facts through the repository port"
        );
    }

    #[test]
    fn download_job_driver_fake_local_resume_execution_records_failed_checkpoint() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord::empty(job_id.clone());
        let plan = make_resume_work_plan("segment-failed-orchestrated");
        let execution_port = FailedSegmentExecutionPort;
        let driver = DownloadJobDriver::with_pending_resume_work_source(
            repo.clone(),
            Arc::new(scheduler.clone()),
        );

        repo.save(&checkpoint)
            .expect("saving a synthetic checkpoint should succeed");
        scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("scheduling pending work for the driver should succeed");

        let saved_checkpoint = driver
            .execute_local_resume_turn(&job_id, &execution_port)
            .expect("fake local resume turn should chain existing driver helpers")
            .expect("failed fake execution should save checkpoint facts");

        assert!(
            scheduler.pending_work().is_empty(),
            "fake local orchestration should drain accepted pending work for the job"
        );

        let expected_failed_segment = DownloadSegmentCheckpointRecord {
            job_id: job_id.clone(),
            segment_id: "segment-failed-orchestrated".into(),
            file_id: "file-1".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 128,
            status: DownloadSegmentCheckpointStatus::Failed,
            partial_path: None,
            etag: None,
            hash_state_ref: None,
        };

        assert_eq!(
            saved_checkpoint.segments,
            vec![expected_failed_segment.clone()],
            "fake local orchestration should return the saved failed segment facts"
        );
        assert_eq!(
            repo.load(&job_id)
                .expect("loading saved checkpoint should succeed")
                .expect("saved checkpoint should exist")
                .segments,
            vec![expected_failed_segment],
            "fake local orchestration must persist failed segment facts through the repository port"
        );
    }
}
