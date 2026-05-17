//! downloads 模块的公开 facade 边界。
//!
//! 这里集中暴露下载任务的命令入口、最小持久化记录契约和依赖束，供
//! composition root 与宿主传输层装配。当前 `start_download` 已返回后端拥有的
//! accepted job，`pause_download` / `cancel_download` 已转接 shared job runtime；
//! 恢复、查询和策略入口仍保留 `DOWNLOADS_NOT_WIRED` stub 语义。

use std::sync::{Arc, Mutex};

use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId, JobId, PageSlice,
};
use launcher_kernel_jobs::{
    AcceptedJob, EnqueueJobRequest, JobPriority, JobRuntime, JobSnapshot, JobUiState,
};

use crate::contracts::{
    CancelDownloadRequestDto, DownloadJobExtensionDto, DownloadJobListDto, DownloadJobListItemDto,
    DownloadJobSnapshotDto, DownloadPolicyDto, GetDownloadJobQueryDto, GetDownloadPolicyQueryDto,
    ListDownloadJobsQueryDto, PauseDownloadRequestDto, ResumeDownloadRequestDto,
    StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
};
use crate::driver::{
    DownloadCheckpointRepository, DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus,
};

/// 下载任务在模块仓储里的粗粒度持久化状态。
///
/// 这层状态只服务于 downloads 自身的 intake 与恢复边界，不试图完整复制
/// 共享 job runtime 的更细粒度快照状态机。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadJobRecordState {
    /// 已创建并持久化，但尚未真正进入运行时执行。
    Queued,
    /// 已由运行时接管并处于执行中。
    Running,
    /// 已被显式暂停，等待后续恢复。
    Paused,
    /// 已完成并产出预期下载结果。
    Completed,
    /// 已失败，后续是否重试由更高层策略决定。
    Failed,
    /// 已被取消，不再继续推进。
    Canceled,
}

/// 下载任务在模块仓储中的最小持久化记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadJobRecord {
    /// 后端拥有的稳定 job 标识，用来关联 runtime 快照与模块记录。
    pub job_id: JobId,
    /// 用户请求要下载的业务目标标识，例如资产或引擎条目。
    pub target_id: String,
    /// 下载内容类型快照，供后续投影和安装链路区分用途。
    pub kind: String,
    /// 启动下载时携带的安装意图，避免 intake 元数据在入队前丢失。
    pub install_intent: Option<String>,
    /// 调度优先级快照，由 facade intake 固化到仓储记录中。
    pub priority: JobPriority,
    /// downloads 模块自己维护的粗粒度持久化状态。
    pub state: DownloadJobRecordState,
}

/// 持久化 downloads intake 记录的最小仓储边界。
pub trait DownloadJobRepository: Send + Sync {
    /// 在任务真正入队前写入最小记录，保证 intake 元数据先落盘。
    fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()>;
    /// 按 job 标识读取模块自己的最小下载记录。
    fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>>;
    /// Lists module-owned download job records for conservative query projection.
    /// 列出 downloads 模块拥有的任务记录，供保守的查询投影使用。
    fn list_jobs(
        &self,
        query: &ListDownloadJobsQueryDto,
    ) -> AppResult<PageSlice<DownloadJobRecord>>;
    /// 同步 downloads 模块拥有的粗粒度状态迁移结果。
    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()>;
}

const MIN_DOWNLOAD_POLICY_CONCURRENCY_SLOTS: u32 = 1;
const MAX_DOWNLOAD_POLICY_CONCURRENCY_SLOTS: u32 = 128;

// Builds the default user-facing downloads policy without touching runtime queue internals.
// 构建面向用户的默认 downloads 策略，但不触碰 shared runtime 的队列内部状态。
fn default_download_policy(concurrency_slots: u32) -> DownloadPolicyDto {
    normalize_download_policy(DownloadPolicyDto {
        concurrency_slots,
        bandwidth_limit_bytes_per_sec: None,
        auto_resume: true,
    })
}

// Normalizes policy facts at the module boundary before they become the stored snapshot.
// 在模块边界先规整策略事实，然后才写入当前策略快照。
fn normalize_download_policy(policy: DownloadPolicyDto) -> DownloadPolicyDto {
    DownloadPolicyDto {
        concurrency_slots: policy.concurrency_slots.clamp(
            MIN_DOWNLOAD_POLICY_CONCURRENCY_SLOTS,
            MAX_DOWNLOAD_POLICY_CONCURRENCY_SLOTS,
        ),
        bandwidth_limit_bytes_per_sec: policy.bandwidth_limit_bytes_per_sec,
        auto_resume: policy.auto_resume,
    }
}

// Projects an update command into the stored policy snapshot shape.
// 将更新命令投影为模块内部保存的策略快照形状。
fn policy_from_update_request(request: UpdateDownloadPolicyRequestDto) -> DownloadPolicyDto {
    normalize_download_policy(DownloadPolicyDto {
        concurrency_slots: request.concurrency_slots,
        bandwidth_limit_bytes_per_sec: request.bandwidth_limit_bytes_per_sec,
        auto_resume: request.auto_resume,
    })
}

/// Downloads-owned policy snapshot store used before SQLite persistence is introduced.
/// SQLite 持久化落地前，由 downloads 模块拥有的策略快照存储端口。
pub trait DownloadPolicyStore: Send + Sync {
    /// Reads the current user-facing downloads policy snapshot.
    /// 读取当前面向用户的 downloads 策略快照。
    fn load_policy(&self) -> AppResult<DownloadPolicyDto>;

    /// Stores a normalized downloads policy snapshot for later queries.
    /// 保存已规整的 downloads 策略快照，供后续查询读取。
    fn save_policy(&self, policy: &DownloadPolicyDto) -> AppResult<()>;
}

/// Applies a normalized downloads policy to a runtime-facing policy surface.
/// 将已规整的 downloads 策略应用到面向 runtime 的策略表面。
pub trait DownloadRuntimePolicyApplier: Send + Sync {
    /// Applies the already-persisted downloads policy snapshot.
    /// 应用已经持久化的 downloads 策略快照。
    fn apply_download_policy(&self, policy: &DownloadPolicyDto) -> AppResult<()>;
}

/// No-op runtime policy applier used until composition-root wires a concrete runtime.
/// 在 composition-root 接入具体 runtime 之前使用的空操作策略应用器。
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopDownloadRuntimePolicyApplier;

impl DownloadRuntimePolicyApplier for NoopDownloadRuntimePolicyApplier {
    fn apply_download_policy(&self, _policy: &DownloadPolicyDto) -> AppResult<()> {
        Ok(())
    }
}

/// In-memory policy store that keeps AT-208 inside the module boundary.
/// 内存态策略存储，让 AT-208 仍然停留在模块边界内。
#[derive(Debug, Clone)]
pub struct InMemoryDownloadPolicyStore {
    /// Shared snapshot protected by a mutex because facade clones may share the store.
    /// 使用 mutex 保护共享快照，因为 facade 克隆后可能共享同一个 store。
    current_policy: Arc<Mutex<DownloadPolicyDto>>,
}

impl InMemoryDownloadPolicyStore {
    /// Creates a store initialized from the current downloads concurrency-slot baseline.
    /// 使用当前 downloads 并发槽位基线创建策略 store。
    pub fn new(concurrency_slots: u32) -> Self {
        Self::with_policy(default_download_policy(concurrency_slots))
    }

    /// Creates a store from an explicit policy snapshot after boundary normalization.
    /// 从显式策略快照创建 store，并先执行模块边界规整。
    pub fn with_policy(policy: DownloadPolicyDto) -> Self {
        Self {
            current_policy: Arc::new(Mutex::new(normalize_download_policy(policy))),
        }
    }
}

impl Default for InMemoryDownloadPolicyStore {
    /// Uses the current desktop bootstrap default until persisted settings land.
    /// 在持久化设置落地前，使用当前桌面 bootstrap 的默认槽位。
    fn default() -> Self {
        Self::new(3)
    }
}

impl DownloadPolicyStore for InMemoryDownloadPolicyStore {
    /// Returns the current snapshot without deriving runtime queue budgets.
    /// 返回当前策略快照，不在这里派生 runtime 队列预算。
    fn load_policy(&self) -> AppResult<DownloadPolicyDto> {
        Ok(self
            .current_policy
            .lock()
            .expect("download policy mutex should not be poisoned")
            .clone())
    }

    /// Replaces the snapshot after normalization; runtime policy application is later work.
    /// 规整后替换快照；应用到 runtime 策略属于后续任务。
    fn save_policy(&self, policy: &DownloadPolicyDto) -> AppResult<()> {
        *self
            .current_policy
            .lock()
            .expect("download policy mutex should not be poisoned") =
            normalize_download_policy(policy.clone());
        Ok(())
    }
}

/// Minimal staging root handle returned before later manifest/runtime resume work.
/// 后续 manifest/runtime 恢复编排前返回的最小 staging 根句柄。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadStagingRoot {
    /// Stable job identifier whose staging area has been validated or prepared.
    /// 已被校验或准备 staging 区的稳定任务标识。
    pub job_id: JobId,
}

/// Port for validating or preparing staging artifacts needed by resume.
/// 校验或准备恢复所需 staging 产物的端口。
pub trait DownloadStagingObjectStore: Send + Sync {
    /// Ensures the staging root for a resumable download is available.
    /// 确认可恢复下载的 staging 根仍然可用。
    fn ensure_staging_root(&self, job_id: &JobId) -> AppResult<DownloadStagingRoot>;
}

impl DownloadStagingObjectStore for () {
    /// Keeps the current composition placeholder compatible until a real adapter lands.
    /// 在真实 adapter 落地前，保持当前 composition 占位依赖可编译。
    fn ensure_staging_root(&self, job_id: &JobId) -> AppResult<DownloadStagingRoot> {
        Ok(DownloadStagingRoot {
            job_id: job_id.clone(),
        })
    }
}

/// Manifest segment shape used by resume planning before runtime enqueue.
/// runtime 入队前用于恢复规划的 manifest segment 形状。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadManifestSegment {
    /// Stable segment identifier inside the manifest plan.
    /// manifest plan 内稳定的 segment 标识。
    pub segment_id: String,
    /// Stable logical file identifier for multi-file downloads.
    /// 多文件下载中的稳定逻辑文件标识。
    pub file_id: String,
    /// Byte offset inside the logical file.
    /// 逻辑文件内的字节偏移。
    pub offset: u64,
    /// Expected segment length in bytes.
    /// segment 预期字节长度。
    pub length: u64,
    /// Provider-specific source reference kept behind this module boundary.
    /// 保留在模块边界内的 provider 专用来源引用。
    pub source_locator: String,
    /// Optional segment integrity expectation when available.
    /// 可用时的 segment 完整性期望值。
    pub expected_hash: Option<String>,
    /// Staging-relative write target path or object key.
    /// staging 相对写入目标路径或对象键。
    pub write_target: String,
}

/// Minimal manifest handle needed before later runtime resume enqueue.
/// 后续 runtime 恢复入队前所需的最小 manifest 句柄。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadManifestPlan {
    /// Provider target identifier used to rebuild the resumable download plan.
    /// 用于重建可恢复下载计划的 provider 目标标识。
    pub target_id: String,
    /// Logical segments that may become sealed or enqueued during resume.
    /// 恢复时可能被封存或入队的逻辑 segment。
    pub segments: Vec<DownloadManifestSegment>,
}

/// Port for reloading or reconstructing provider manifest data needed by resume.
/// 重新加载或重建恢复所需 provider manifest 数据的端口。
pub trait DownloadManifestProviderPort: Send + Sync {
    /// Fetches the manifest plan for a module-owned download target.
    /// 为 downloads 模块拥有的下载目标读取 manifest plan。
    fn fetch_manifest(&self, target_id: &str) -> AppResult<DownloadManifestPlan>;
}

impl DownloadManifestProviderPort for () {
    /// Keeps current composition wiring compatible until a real provider adapter lands.
    /// 在真实 provider adapter 落地前，保持当前 composition 占位依赖可编译。
    fn fetch_manifest(&self, target_id: &str) -> AppResult<DownloadManifestPlan> {
        Ok(DownloadManifestPlan {
            target_id: target_id.to_string(),
            segments: Vec::new(),
        })
    }
}

/// Resume action derived from manifest segments plus persisted checkpoint facts.
/// 根据 manifest segment 与持久化 checkpoint 事实推导出的恢复动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadResumeSegmentAction {
    /// Completed checkpoint is valid and must not be enqueued again.
    /// 已完成 checkpoint 有效，不能再次入队。
    SealCompleted,
    /// Partial checkpoint can later resume from an offset.
    /// 部分 checkpoint 后续可从偏移继续恢复。
    ResumePartial,
    /// Segment should be queued from the beginning.
    /// segment 应从头入队。
    QueueRemaining,
    /// Manifest/checkpoint mismatch prevents automatic resume.
    /// manifest/checkpoint 不匹配，阻止自动恢复。
    RejectMismatch,
}

/// In-memory resume decision for a single manifest segment.
/// 单个 manifest segment 的内存态恢复决策。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadResumeSegmentDecision {
    /// Segment identifier the decision applies to.
    /// 此决策适用的 segment 标识。
    pub segment_id: String,
    /// Derived action for this segment.
    /// 此 segment 的推导动作。
    pub action: DownloadResumeSegmentAction,
}

impl DownloadResumeSegmentDecision {
    /// Returns whether this segment should be handed to runtime enqueue.
    /// 返回此 segment 是否应该交给 runtime 入队。
    pub fn is_runtime_enqueue_candidate(&self) -> bool {
        matches!(
            self.action,
            DownloadResumeSegmentAction::ResumePartial
                | DownloadResumeSegmentAction::QueueRemaining
        )
    }
}

/// Module-local resume work mode for the future downloads scheduler/driver.
/// 面向后续 downloads scheduler/driver 的模块本地恢复工作模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadResumeWorkMode {
    /// Continue a segment from a validated partial checkpoint.
    /// 从已校验的部分 checkpoint 继续下载 segment。
    Partial,
    /// Start or restart the segment from its manifest offset.
    /// 从 manifest offset 开始或重新开始下载 segment。
    FromStart,
}

/// Module-owned resume work item derived after manifest/checkpoint decisions.
/// 根据 manifest/checkpoint 决策推导出的 downloads 模块自有恢复工作项。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadResumeWorkItem {
    /// Stable segment identifier used by the future scheduler route.
    /// 后续 scheduler 路由使用的稳定 segment 标识。
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
    /// Optional verifier expectation copied from the manifest segment.
    /// 从 manifest segment 复制的可选校验期望。
    pub expected_hash: Option<String>,
    /// Segment-relative resume offset or manifest start offset, depending on mode.
    /// 根据模式保存 segment 相对续传偏移或 manifest 起始偏移。
    pub start_offset: u64,
    /// Total segment length expected by scheduler and verifier.
    /// scheduler 与 verifier 期望的 segment 总长度。
    pub length: u64,
    /// Whether the future driver should resume partially or start from scratch.
    /// 后续 driver 应部分续传还是从头开始。
    pub resume_mode: DownloadResumeWorkMode,
    /// Module-local checkpoint reference when this item came from a checkpoint.
    /// 当工作项来自 checkpoint 时保存模块本地 checkpoint 引用。
    pub checkpoint_ref: Option<String>,
}

/// Module-local resume work plan that must not be stored in `kernel-jobs`.
/// 不能存入 `kernel-jobs` 的 downloads 模块本地恢复工作计划。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadResumeWorkPlan {
    /// Work items that the future downloads scheduler/driver may consume.
    /// 后续 downloads scheduler/driver 可消费的工作项。
    pub items: Vec<DownloadResumeWorkItem>,
}

/// Transient pending resume work registered for a downloads job.
/// 为 downloads job 登记的瞬时 pending resume work。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadPendingResumeWork {
    /// Existing downloads job id that owns the pending work.
    /// 拥有这批 pending work 的既有 downloads job id。
    pub job_id: JobId,
    /// Derived resume plan kept inside the downloads module boundary.
    /// 保留在 downloads 模块边界内的派生恢复计划。
    pub plan: DownloadResumeWorkPlan,
}

/// In-memory scheduler shell that registers pending resume work for later driver use.
/// 为后续 driver 消费登记 pending resume work 的内存态 scheduler 壳。
#[derive(Debug, Clone, Default)]
pub struct InMemoryDownloadResumeWorkScheduler {
    pending_work: Arc<Mutex<Vec<DownloadPendingResumeWork>>>,
}

impl InMemoryDownloadResumeWorkScheduler {
    /// Creates an empty pending-work scheduler shell.
    /// 创建一个空的 pending-work scheduler 壳。
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a snapshot of currently registered pending resume work.
    /// 返回当前已登记 pending resume work 的快照。
    pub fn pending_work(&self) -> Vec<DownloadPendingResumeWork> {
        self.pending_work
            .lock()
            .expect("pending resume work mutex should not be poisoned")
            .clone()
    }

    /// Drains registered pending resume work for a future module-owned driver loop.
    /// 为后续模块自有 driver loop 取走已登记的 pending resume work。
    pub fn drain_pending_work(&self) -> Vec<DownloadPendingResumeWork> {
        self.pending_work
            .lock()
            .expect("pending resume work mutex should not be poisoned")
            .drain(..)
            .collect()
    }
}

/// Downloads-owned source boundary for future driver-side pending resume work consumption.
/// 后续 driver 侧消费 pending resume work 的 downloads 自有 source 边界。
pub trait DownloadPendingResumeWorkSource: Send + Sync {
    /// Drains pending resume work for one job while preserving unrelated jobs.
    /// 取走单个 job 的 pending resume work，同时保留其他 job 的待处理工作。
    fn drain_pending_resume_work(
        &self,
        job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>>;
}

impl DownloadPendingResumeWorkSource for () {
    /// Keeps driver construction compatible until a real pending-work source is wired.
    /// 在真实 pending-work source 接入前保持 driver 构造兼容。
    fn drain_pending_resume_work(
        &self,
        _job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>> {
        Ok(Vec::new())
    }
}

impl DownloadPendingResumeWorkSource for InMemoryDownloadResumeWorkScheduler {
    /// Drains only the requested job's pending work without running download IO.
    /// 只取走指定 job 的 pending work，不执行下载 IO。
    fn drain_pending_resume_work(
        &self,
        job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>> {
        let mut pending_work = self
            .pending_work
            .lock()
            .expect("pending resume work mutex should not be poisoned");
        let mut drained = Vec::new();
        let mut remaining = Vec::with_capacity(pending_work.len());

        for work in pending_work.drain(..) {
            if &work.job_id == job_id {
                drained.push(work);
            } else {
                remaining.push(work);
            }
        }

        *pending_work = remaining;
        Ok(drained)
    }
}

/// Downloads-owned scheduler boundary that prepares resume work before runtime enqueue.
/// 在 runtime 入队前准备恢复工作的 downloads 自有 scheduler 边界。
pub trait DownloadResumeWorkScheduler: Send + Sync {
    /// Schedules or stages the derived resume work plan for a module-owned job.
    /// 为模块拥有的任务调度或暂存已派生的恢复工作计划。
    fn schedule_resume_work(&self, job_id: &JobId, plan: &DownloadResumeWorkPlan) -> AppResult<()>;
}

impl DownloadResumeWorkScheduler for () {
    /// Keeps current composition wiring valid until a real scheduler/driver lands.
    /// 在真实 scheduler/driver 落地前保持当前 composition 接线可用。
    fn schedule_resume_work(
        &self,
        _job_id: &JobId,
        _plan: &DownloadResumeWorkPlan,
    ) -> AppResult<()> {
        Ok(())
    }
}

impl DownloadResumeWorkScheduler for InMemoryDownloadResumeWorkScheduler {
    /// Registers the derived work plan without running fetch/write/verify work.
    /// 只登记派生工作计划，不执行 fetch/write/verify 工作。
    fn schedule_resume_work(&self, job_id: &JobId, plan: &DownloadResumeWorkPlan) -> AppResult<()> {
        self.pending_work
            .lock()
            .expect("pending resume work mutex should not be poisoned")
            .push(DownloadPendingResumeWork {
                job_id: job_id.clone(),
                plan: plan.clone(),
            });
        Ok(())
    }
}

/// Module-owned outcome for resume planning before host transport projection.
/// host transport 投影前，downloads 模块自有的恢复规划结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadResumeOutcome {
    /// Resume work was accepted by the shared runtime queue.
    /// 恢复工作已被 shared runtime 队列接收。
    RuntimeAccepted(AcceptedJob),
    /// Resume planning found all segments already sealed and no runtime work remains.
    /// 恢复规划发现所有 segment 已封存，且没有剩余 runtime 工作。
    AlreadyComplete {
        /// Completed job identifier owned by downloads.
        /// downloads 拥有的已完成作业标识。
        job_id: JobId,
        /// Module name kept local until transport projection is explicitly designed.
        /// 模块名在 transport 投影明确设计前保持为模块本地结果字段。
        module: String,
        /// Downloads job kind kept local until public DTO adaptation is scoped.
        /// 下载作业类型在公开 DTO 适配被纳入范围前保持为本地字段。
        kind: String,
    },
}

/// Builds in-memory resume decisions from manifest segments and checkpoint facts.
/// 根据 manifest segment 与 checkpoint 事实构建内存态恢复决策。
pub fn build_resume_segment_decisions(
    manifest: &DownloadManifestPlan,
    checkpoints: &[DownloadSegmentCheckpointRecord],
) -> AppResult<Vec<DownloadResumeSegmentDecision>> {
    Ok(manifest
        .segments
        .iter()
        .map(|segment| {
            let matching_checkpoint = checkpoints
                .iter()
                .find(|checkpoint| checkpoint.segment_id == segment.segment_id);

            let action = match matching_checkpoint {
                Some(checkpoint)
                    if checkpoint.file_id != segment.file_id
                        || checkpoint.offset != segment.offset
                        || checkpoint.length != segment.length =>
                {
                    DownloadResumeSegmentAction::RejectMismatch
                }
                Some(checkpoint)
                    if checkpoint.status == DownloadSegmentCheckpointStatus::Completed
                        && checkpoint.downloaded_bytes == segment.length =>
                {
                    DownloadResumeSegmentAction::SealCompleted
                }
                Some(checkpoint)
                    if checkpoint.status == DownloadSegmentCheckpointStatus::InProgress
                        && checkpoint.downloaded_bytes > 0
                        && checkpoint.downloaded_bytes < segment.length
                        && checkpoint.partial_path.is_some()
                        && checkpoint.etag.is_some() =>
                {
                    DownloadResumeSegmentAction::ResumePartial
                }
                _ => DownloadResumeSegmentAction::QueueRemaining,
            };

            DownloadResumeSegmentDecision {
                segment_id: segment.segment_id.clone(),
                action,
            }
        })
        .collect())
}

/// Builds the downloads-owned scheduler work plan from resume decisions.
/// 从恢复决策构建 downloads 自有的 scheduler 工作计划。
pub fn build_resume_work_plan(
    manifest: &DownloadManifestPlan,
    checkpoints: &[DownloadSegmentCheckpointRecord],
    decisions: &[DownloadResumeSegmentDecision],
) -> AppResult<DownloadResumeWorkPlan> {
    let mut items = Vec::new();

    for decision in decisions {
        let segment = manifest
            .segments
            .iter()
            .find(|segment| segment.segment_id == decision.segment_id)
            .ok_or_else(|| resume_work_plan_inconsistent(&decision.segment_id, "manifest"))?;

        match decision.action {
            DownloadResumeSegmentAction::ResumePartial => {
                let checkpoint = checkpoints
                    .iter()
                    .find(|checkpoint| checkpoint.segment_id == decision.segment_id)
                    .ok_or_else(|| {
                        resume_work_plan_inconsistent(&decision.segment_id, "checkpoint")
                    })?;

                items.push(DownloadResumeWorkItem {
                    segment_id: segment.segment_id.clone(),
                    file_id: segment.file_id.clone(),
                    source_locator: segment.source_locator.clone(),
                    write_target: segment.write_target.clone(),
                    expected_hash: segment.expected_hash.clone(),
                    start_offset: checkpoint.downloaded_bytes,
                    length: segment.length,
                    resume_mode: DownloadResumeWorkMode::Partial,
                    checkpoint_ref: Some(checkpoint.segment_id.clone()),
                });
            }
            DownloadResumeSegmentAction::QueueRemaining => {
                items.push(DownloadResumeWorkItem {
                    segment_id: segment.segment_id.clone(),
                    file_id: segment.file_id.clone(),
                    source_locator: segment.source_locator.clone(),
                    write_target: segment.write_target.clone(),
                    expected_hash: segment.expected_hash.clone(),
                    start_offset: segment.offset,
                    length: segment.length,
                    resume_mode: DownloadResumeWorkMode::FromStart,
                    checkpoint_ref: None,
                });
            }
            DownloadResumeSegmentAction::SealCompleted
            | DownloadResumeSegmentAction::RejectMismatch => {}
        }
    }

    Ok(DownloadResumeWorkPlan { items })
}

/// 组装 downloads facade 所需的依赖束。
#[derive(Debug, Clone)]
pub struct DownloadModuleDeps<J, C, M, S, W, R, P> {
    /// 保存 intake 元数据与粗粒度下载状态的模块仓储。
    pub job_repo: J,
    /// 为暂停、恢复和重启场景保留的 checkpoint 能力。
    pub checkpoint_repo: C,
    /// 负责解析 provider manifest 的适配器能力，留给后续切片接线。
    pub manifest_provider: M,
    /// 管理本地 staging 产物落盘位置的能力，留给后续切片接线。
    pub staging_store: S,
    /// 准备 downloads 自有恢复工作计划的 scheduler/driver 边界。
    pub resume_scheduler: W,
    /// 真正拥有 accepted job 生命周期的共享 job runtime。
    pub job_runtime: R,
    /// Stores downloads-facing policy snapshots without mutating runtime queue policy.
    /// 保存面向 downloads 的策略快照，但不直接修改 runtime 队列策略。
    pub policy_store: P,
}

/// downloads 模块对外暴露的 use-case facade。
pub struct DownloadFacade<J, C, M, S, W, R, P> {
    deps: DownloadModuleDeps<J, C, M, S, W, R, P>,
    /// Applies persisted downloads policy snapshots to runtime-facing policy control.
    /// 将已持久化的 downloads 策略快照应用到面向 runtime 的策略控制面。
    runtime_policy_applier: Arc<dyn DownloadRuntimePolicyApplier>,
}

impl<J, C, M, S, W, R, P> DownloadFacade<J, C, M, S, W, R, P> {
    /// 用已经装配好的依赖束创建 downloads facade。
    pub fn new(deps: DownloadModuleDeps<J, C, M, S, W, R, P>) -> Self {
        Self::with_runtime_policy_applier(deps, NoopDownloadRuntimePolicyApplier)
    }

    /// Creates a downloads facade with an explicit runtime policy applier.
    /// 使用显式 runtime 策略应用器创建 downloads facade。
    pub fn with_runtime_policy_applier<A>(
        deps: DownloadModuleDeps<J, C, M, S, W, R, P>,
        runtime_policy_applier: A,
    ) -> Self
    where
        A: DownloadRuntimePolicyApplier + 'static,
    {
        Self {
            deps,
            runtime_policy_applier: Arc::new(runtime_policy_applier),
        }
    }

    /// 暴露依赖束，主要服务于 composition-root smoke test 与局部诊断。
    pub fn deps(&self) -> &DownloadModuleDeps<J, C, M, S, W, R, P> {
        &self.deps
    }
}

impl<
        J: DownloadJobRepository,
        C,
        M,
        S,
        W,
        R: JobRuntime<Extension = ()>,
        P: DownloadPolicyStore,
    > DownloadFacade<J, C, M, S, W, R, P>
{
    /// 记录用户下载意图并向共享 job runtime 提交后端拥有的下载任务。
    pub fn start_download(&self, request: StartDownloadRequestDto) -> AppResult<AcceptedJob> {
        let job_id = JobId::generate();
        let StartDownloadRequestDto {
            target_id,
            kind,
            install_intent,
            priority,
        } = request;

        self.deps.job_repo.create_job(&DownloadJobRecord {
            job_id: job_id.clone(),
            target_id,
            kind,
            install_intent,
            priority,
            state: DownloadJobRecordState::Queued,
        })?;

        let enqueue_request = EnqueueJobRequest {
            job_id,
            module: "downloads".to_string(),
            kind: "download".to_string(),
            priority,
            recoverable: true,
            extension: None,
        };
        self.deps.job_runtime.enqueue(enqueue_request)
    }

    /// 请求共享 runtime 暂停已有下载作业。
    pub fn pause_download(&self, request: PauseDownloadRequestDto) -> AppResult<()> {
        self.deps.job_runtime.pause(&request.job_id)
    }

    /// 请求共享 runtime 取消已有下载作业。
    pub fn cancel_download(&self, request: CancelDownloadRequestDto) -> AppResult<()> {
        self.deps.job_runtime.cancel(&request.job_id)
    }

    /// 预留任务列表查询；当前仍由模块边界返回未接线错误。
    pub fn list_jobs(&self, query: ListDownloadJobsQueryDto) -> AppResult<DownloadJobListDto> {
        let page = self.deps.job_repo.list_jobs(&query)?;
        Ok(PageSlice::new(
            page.items
                .iter()
                .map(project_download_job_list_item)
                .collect(),
            page.next_cursor,
        ))
    }

    /// 预留单任务快照查询；当前仍由模块边界返回未接线错误。
    pub fn get_job_snapshot(
        &self,
        query: GetDownloadJobQueryDto,
    ) -> AppResult<Option<DownloadJobSnapshotDto>> {
        let job = match self.deps.job_repo.get_job(&query.job_id)? {
            Some(job) => job,
            None => return Err(missing_job_record(&query.job_id)),
        };

        let snapshot = match self.deps.job_runtime.snapshot(&query.job_id)? {
            Some(snapshot) => snapshot,
            None => return Err(missing_job_snapshot(&query.job_id)),
        };

        Ok(Some(project_download_job_snapshot(&job, snapshot)))
    }

    /// Reads the current downloads-facing policy snapshot.
    /// 读取当前面向 downloads 的策略快照。
    pub fn get_policy(&self, query: GetDownloadPolicyQueryDto) -> AppResult<DownloadPolicyDto> {
        let _ = query;
        self.deps.policy_store.load_policy()
    }

    /// Stores the downloads-facing policy snapshot without mutating runtime queue policy.
    /// 保存面向 downloads 的策略快照，但不修改 runtime 队列策略。
    pub fn update_policy(&self, request: UpdateDownloadPolicyRequestDto) -> AppResult<()> {
        let policy = policy_from_update_request(request);
        self.deps.policy_store.save_policy(&policy)?;
        self.runtime_policy_applier.apply_download_policy(&policy)
    }
}

impl<
        J: DownloadJobRepository,
        C: DownloadCheckpointRepository,
        M: DownloadManifestProviderPort,
        S: DownloadStagingObjectStore,
        W: DownloadResumeWorkScheduler,
        R: JobRuntime<Extension = ()>,
        P,
    > DownloadFacade<J, C, M, S, W, R, P>
{
    /// Loads module state, checkpoint, staging, and manifest before resume decisions.
    /// 先读取模块状态、checkpoint、staging 和 manifest，然后才进入恢复决策。
    ///
    /// Missing or unsafe module-owned resume facts are stable downloads-domain failures.
    /// 缺失或不安全的模块恢复事实会收敛为稳定的 downloads 域失败。
    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob> {
        match self.resume_download_outcome(request)? {
            DownloadResumeOutcome::RuntimeAccepted(accepted) => Ok(accepted),
            DownloadResumeOutcome::AlreadyComplete { .. } => Err(not_wired("resume_download")),
        }
    }

    /// Returns the module-owned resume outcome before public transport projection.
    /// 在公开 transport 投影前，返回 downloads 模块自有的恢复结果。
    pub fn resume_download_outcome(
        &self,
        request: ResumeDownloadRequestDto,
    ) -> AppResult<DownloadResumeOutcome> {
        let job = match self.deps.job_repo.get_job(&request.job_id)? {
            Some(job) => job,
            None => return Err(missing_job_record(&request.job_id)),
        };

        let checkpoint = match self.deps.checkpoint_repo.load(&request.job_id)? {
            Some(checkpoint) => checkpoint,
            None => return Err(missing_checkpoint(&request.job_id)),
        };

        let _staging_root = self
            .deps
            .staging_store
            .ensure_staging_root(&request.job_id)?;
        let manifest = self.deps.manifest_provider.fetch_manifest(&job.target_id)?;
        let resume_decisions = build_resume_segment_decisions(&manifest, &checkpoint.segments)?;
        let has_runtime_enqueue_candidate = resume_decisions
            .iter()
            .any(DownloadResumeSegmentDecision::is_runtime_enqueue_candidate);
        let has_reject_mismatch = resume_decisions
            .iter()
            .any(|decision| decision.action == DownloadResumeSegmentAction::RejectMismatch);
        let is_all_sealed = !resume_decisions.is_empty()
            && resume_decisions
                .iter()
                .all(|decision| decision.action == DownloadResumeSegmentAction::SealCompleted);

        if has_reject_mismatch {
            return Err(resume_segment_mismatch(&request.job_id));
        }

        if has_runtime_enqueue_candidate {
            let work_plan =
                build_resume_work_plan(&manifest, &checkpoint.segments, &resume_decisions)?;
            self.deps
                .resume_scheduler
                .schedule_resume_work(&request.job_id, &work_plan)?;

            // Keep this first resume slice job-level; segment payloads stay in downloads.
            // 首个恢复切片只做 job-level 入队；segment payload 仍留在 downloads 内部。
            let accepted = self.deps.job_runtime.enqueue(EnqueueJobRequest {
                job_id: request.job_id,
                module: "downloads".to_string(),
                kind: "download".to_string(),
                priority: job.priority,
                recoverable: true,
                extension: None,
            })?;
            return Ok(DownloadResumeOutcome::RuntimeAccepted(accepted));
        }

        if is_all_sealed {
            return Ok(DownloadResumeOutcome::AlreadyComplete {
                job_id: request.job_id,
                module: "downloads".to_string(),
                kind: "download".to_string(),
            });
        }

        Err(not_wired("resume_download"))
    }
}

// Verifies the command targets a module-owned download record before resume work continues.
// 在继续恢复流程前，先确认命令指向 downloads 模块拥有的下载记录。
fn missing_job_record(job_id: &JobId) -> AppError {
    AppError::new(
        "DL_JOB_NOT_FOUND",
        format!("download job record missing for job {job_id}; resume cannot continue"),
        false,
        AppErrorSeverity::Error,
        CorrelationId::generate(),
    )
}

// Reports a missing shared runtime snapshot for a module-owned downloads job.
// 当 downloads 模块记录存在但共享 runtime 快照缺失时，返回稳定查询错误。
fn missing_job_snapshot(job_id: &JobId) -> AppError {
    AppError::new(
        "DL_JOB_SNAPSHOT_MISSING",
        format!("download runtime snapshot missing for job {job_id}; query cannot continue"),
        false,
        AppErrorSeverity::Error,
        CorrelationId::generate(),
    )
}

// Projects shared runtime facts plus downloads intake metadata into the public snapshot read model.
// 将共享 runtime 事实和 downloads intake 元数据组合成公开快照读模型。
fn project_download_job_snapshot(
    job: &DownloadJobRecord,
    snapshot: JobSnapshot<()>,
) -> DownloadJobSnapshotDto {
    DownloadJobSnapshotDto {
        job_id: snapshot.job_id,
        module: snapshot.module,
        kind: snapshot.kind,
        state: snapshot.state,
        ui_state: snapshot.ui_state,
        progress: snapshot.progress,
        recoverable: snapshot.recoverable,
        updated_at: snapshot.updated_at,
        extension: Some(DownloadJobExtensionDto {
            target_id: job.target_id.clone(),
            install_intent: job.install_intent.clone(),
            completed_bytes: 0,
            total_bytes: None,
            retryable: true,
        }),
    }
}

// Projects a module job record into the conservative public list-row read model.
// 将模块任务记录投影成保守的公开列表行读模型。
fn project_download_job_list_item(job: &DownloadJobRecord) -> DownloadJobListItemDto {
    DownloadJobListItemDto {
        job_id: job.job_id.clone(),
        title: job.target_id.clone(),
        ui_state: download_job_record_ui_state(job.state),
        progress_label: None,
        throughput_bytes_per_sec: None,
    }
}

// Maps module-owned coarse state to the current public UI state filter/projection.
// 将 downloads 模块自有的粗粒度状态映射为当前公开 UI 状态筛选与投影值。
fn download_job_record_ui_state(state: DownloadJobRecordState) -> JobUiState {
    match state {
        DownloadJobRecordState::Queued => JobUiState::Queued,
        DownloadJobRecordState::Running => JobUiState::Running,
        DownloadJobRecordState::Paused => JobUiState::Paused,
        DownloadJobRecordState::Completed => JobUiState::Completed,
        DownloadJobRecordState::Failed => JobUiState::Failed,
        DownloadJobRecordState::Canceled => JobUiState::Canceled,
    }
}

// Projects unsafe segment checkpoint facts as a downloads-domain resume failure.
// 将不安全的 segment checkpoint 事实投影为 downloads 域恢复失败。
fn resume_segment_mismatch(job_id: &JobId) -> AppError {
    AppError::new(
        "DL_RESUME_SEGMENT_MISMATCH",
        format!(
            "download segment checkpoint mismatch for job {job_id}; resume cannot continue safely"
        ),
        false,
        AppErrorSeverity::Error,
        CorrelationId::generate(),
    )
}

// Reports an impossible resume work-plan input mismatch inside the downloads module.
// 上报 downloads 模块内部恢复工作计划输入不一致的异常状态。
fn resume_work_plan_inconsistent(segment_id: &str, missing_source: &str) -> AppError {
    AppError::new(
        "DL_RESUME_WORK_PLAN_INCONSISTENT",
        format!("resume work plan cannot find {missing_source} facts for segment {segment_id}"),
        false,
        AppErrorSeverity::Error,
        CorrelationId::generate(),
    )
}

// Keeps the missing-checkpoint branch owned by downloads instead of leaking into runtime.
// 将缺失 checkpoint 的分支保留在 downloads 域内，避免泄露到 runtime 里猜测。
fn missing_checkpoint(job_id: &JobId) -> AppError {
    AppError::new(
        "DL_CHECKPOINT_MISSING",
        format!("download checkpoint missing for job {job_id}; resume cannot continue"),
        false,
        AppErrorSeverity::Error,
        CorrelationId::generate(),
    )
}

// 在 facade 边界统一保留 C2 阶段的未接线错误码，避免宿主层自己发明 fallback。
fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "DOWNLOADS_NOT_WIRED",
        format!("downloads facade operation `{operation}` is not wired in C2"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use launcher_kernel_foundation::{
        AppError, AppErrorSeverity, AppResult, CorrelationId, IsoDateTime, JobId, PageRequest,
        PageSlice,
    };
    use launcher_kernel_jobs::{JobPriority, JobProgress, JobSnapshot, JobState, JobUiState};

    use crate::driver::{DownloadCheckpointRecord, DownloadCheckpointRepository};

    use super::{
        build_resume_segment_decisions, build_resume_work_plan, DownloadFacade, DownloadJobRecord,
        DownloadJobRecordState, DownloadJobRepository, DownloadManifestPlan,
        DownloadManifestProviderPort, DownloadManifestSegment, DownloadModuleDeps,
        DownloadPendingResumeWork, DownloadPendingResumeWorkSource, DownloadResumeOutcome,
        DownloadResumeSegmentAction, DownloadResumeWorkItem, DownloadResumeWorkMode,
        DownloadResumeWorkPlan, DownloadResumeWorkScheduler, DownloadRuntimePolicyApplier,
        DownloadStagingObjectStore, DownloadStagingRoot, InMemoryDownloadPolicyStore,
        InMemoryDownloadResumeWorkScheduler,
    };
    use crate::contracts::{
        CancelDownloadRequestDto, DownloadPolicyDto, GetDownloadJobQueryDto,
        GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto, PauseDownloadRequestDto,
        ResumeDownloadRequestDto, StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
    };
    use crate::driver::{DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus};

    #[derive(Default)]
    struct RecordingDownloadJobRepository {
        created_jobs: Mutex<Vec<DownloadJobRecord>>,
        looked_up_job_ids: Mutex<Vec<JobId>>,
    }

    impl RecordingDownloadJobRepository {
        fn with_job(job: DownloadJobRecord) -> Self {
            Self {
                created_jobs: Mutex::new(vec![job]),
                looked_up_job_ids: Mutex::new(Vec::new()),
            }
        }

        fn with_jobs(jobs: Vec<DownloadJobRecord>) -> Self {
            Self {
                created_jobs: Mutex::new(jobs),
                looked_up_job_ids: Mutex::new(Vec::new()),
            }
        }

        fn created_jobs(&self) -> Vec<DownloadJobRecord> {
            self.created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .clone()
        }

        fn looked_up_job_ids(&self) -> Vec<JobId> {
            self.looked_up_job_ids
                .lock()
                .expect("looked-up job ids mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadJobRepository for RecordingDownloadJobRepository {
        fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()> {
            self.created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .push(job.clone());
            Ok(())
        }

        fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
            self.looked_up_job_ids
                .lock()
                .expect("looked-up job ids mutex should not be poisoned")
                .push(job_id.clone());

            Ok(self
                .created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .iter()
                .find(|job| &job.job_id == job_id)
                .cloned())
        }

        fn list_jobs(
            &self,
            query: &ListDownloadJobsQueryDto,
        ) -> AppResult<PageSlice<DownloadJobRecord>> {
            let limit = query.page.limit as usize;
            let jobs = self
                .created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned");
            let items = jobs
                .iter()
                .filter(|job| match query.ui_state {
                    Some(ui_state) => super::download_job_record_ui_state(job.state) == ui_state,
                    None => true,
                })
                .take(limit)
                .cloned()
                .collect();

            Ok(PageSlice::new(items, None))
        }

        fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()> {
            if let Some(job) = self
                .created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .iter_mut()
                .find(|job| &job.job_id == job_id)
            {
                job.state = state;
            }

            Ok(())
        }
    }

    fn make_download_job(job_id: JobId) -> DownloadJobRecord {
        DownloadJobRecord {
            job_id,
            target_id: "asset-123".into(),
            kind: "engine".into(),
            install_intent: Some("install".into()),
            priority: JobPriority::Normal,
            state: DownloadJobRecordState::Paused,
        }
    }

    #[derive(Default)]
    struct RecordingCheckpointRepository {
        loaded_job_ids: Mutex<Vec<JobId>>,
        segments: Vec<DownloadSegmentCheckpointRecord>,
        returns_missing: bool,
    }

    impl RecordingCheckpointRepository {
        fn with_segments(segments: Vec<DownloadSegmentCheckpointRecord>) -> Self {
            Self {
                loaded_job_ids: Mutex::new(Vec::new()),
                segments,
                returns_missing: false,
            }
        }

        fn missing() -> Self {
            Self {
                loaded_job_ids: Mutex::new(Vec::new()),
                segments: Vec::new(),
                returns_missing: true,
            }
        }

        fn loaded_job_ids(&self) -> Vec<JobId> {
            self.loaded_job_ids
                .lock()
                .expect("loaded job ids mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadCheckpointRepository for RecordingCheckpointRepository {
        fn load(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>> {
            self.loaded_job_ids
                .lock()
                .expect("loaded job ids mutex should not be poisoned")
                .push(job_id.clone());

            if self.returns_missing {
                return Ok(None);
            }

            Ok(Some(DownloadCheckpointRecord {
                job_id: job_id.clone(),
                segments: self.segments.clone(),
            }))
        }

        fn save(&self, _checkpoint: &DownloadCheckpointRecord) -> AppResult<()> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingStagingObjectStore {
        ensured_job_ids: Mutex<Vec<JobId>>,
    }

    impl RecordingStagingObjectStore {
        fn ensured_job_ids(&self) -> Vec<JobId> {
            self.ensured_job_ids
                .lock()
                .expect("ensured job ids mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadStagingObjectStore for RecordingStagingObjectStore {
        fn ensure_staging_root(&self, job_id: &JobId) -> AppResult<DownloadStagingRoot> {
            self.ensured_job_ids
                .lock()
                .expect("ensured job ids mutex should not be poisoned")
                .push(job_id.clone());

            Ok(DownloadStagingRoot {
                job_id: job_id.clone(),
            })
        }
    }

    #[derive(Default)]
    struct RecordingManifestProvider {
        fetched_target_ids: Mutex<Vec<String>>,
        segments: Vec<DownloadManifestSegment>,
    }

    impl RecordingManifestProvider {
        fn with_segments(segments: Vec<DownloadManifestSegment>) -> Self {
            Self {
                fetched_target_ids: Mutex::new(Vec::new()),
                segments,
            }
        }

        fn fetched_target_ids(&self) -> Vec<String> {
            self.fetched_target_ids
                .lock()
                .expect("fetched target ids mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadManifestProviderPort for RecordingManifestProvider {
        fn fetch_manifest(&self, target_id: &str) -> AppResult<DownloadManifestPlan> {
            self.fetched_target_ids
                .lock()
                .expect("fetched target ids mutex should not be poisoned")
                .push(target_id.to_string());

            Ok(DownloadManifestPlan {
                target_id: target_id.to_string(),
                segments: self.segments.clone(),
            })
        }
    }

    #[derive(Clone, Default)]
    struct ResumeRuntimeEvents {
        entries: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ResumeRuntimeEvents {
        fn record(&self, event: &'static str) {
            self.entries
                .lock()
                .expect("resume runtime events mutex should not be poisoned")
                .push(event);
        }

        fn entries(&self) -> Vec<&'static str> {
            self.entries
                .lock()
                .expect("resume runtime events mutex should not be poisoned")
                .clone()
        }
    }

    #[derive(Default)]
    struct RecordingResumeWorkScheduler {
        scheduled_plans: Mutex<Vec<(JobId, DownloadResumeWorkPlan)>>,
        events: ResumeRuntimeEvents,
        fail_with: Option<AppError>,
    }

    impl RecordingResumeWorkScheduler {
        fn with_events(events: ResumeRuntimeEvents) -> Self {
            Self {
                scheduled_plans: Mutex::new(Vec::new()),
                events,
                fail_with: None,
            }
        }

        fn failing_with(events: ResumeRuntimeEvents, error: AppError) -> Self {
            Self {
                scheduled_plans: Mutex::new(Vec::new()),
                events,
                fail_with: Some(error),
            }
        }

        fn scheduled_plans(&self) -> Vec<(JobId, DownloadResumeWorkPlan)> {
            self.scheduled_plans
                .lock()
                .expect("scheduled plans mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadResumeWorkScheduler for RecordingResumeWorkScheduler {
        fn schedule_resume_work(
            &self,
            job_id: &JobId,
            plan: &DownloadResumeWorkPlan,
        ) -> AppResult<()> {
            self.events.record("schedule_work");
            self.scheduled_plans
                .lock()
                .expect("scheduled plans mutex should not be poisoned")
                .push((job_id.clone(), plan.clone()));
            if let Some(error) = &self.fail_with {
                return Err(error.clone());
            }
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingJobRuntime {
        enqueued_requests: Mutex<Vec<launcher_kernel_jobs::EnqueueJobRequest<()>>>,
        snapshotted_job_ids: Mutex<Vec<JobId>>,
        paused_job_ids: Mutex<Vec<JobId>>,
        canceled_job_ids: Mutex<Vec<JobId>>,
        events: ResumeRuntimeEvents,
        snapshots_missing: bool,
    }

    impl RecordingJobRuntime {
        fn with_events(events: ResumeRuntimeEvents) -> Self {
            Self {
                events,
                ..Self::default()
            }
        }

        fn missing_snapshot() -> Self {
            Self {
                snapshots_missing: true,
                ..Self::default()
            }
        }

        fn enqueued_requests(&self) -> Vec<launcher_kernel_jobs::EnqueueJobRequest<()>> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .clone()
        }

        fn snapshotted_job_ids(&self) -> Vec<JobId> {
            self.snapshotted_job_ids
                .lock()
                .expect("snapshotted job ids mutex should not be poisoned")
                .clone()
        }

        fn paused_job_ids(&self) -> Vec<JobId> {
            self.paused_job_ids
                .lock()
                .expect("paused job ids mutex should not be poisoned")
                .clone()
        }

        fn canceled_job_ids(&self) -> Vec<JobId> {
            self.canceled_job_ids
                .lock()
                .expect("canceled job ids mutex should not be poisoned")
                .clone()
        }
    }

    impl launcher_kernel_jobs::JobRuntime for RecordingJobRuntime {
        type Extension = ();

        fn enqueue(
            &self,
            request: launcher_kernel_jobs::EnqueueJobRequest<Self::Extension>,
        ) -> AppResult<launcher_kernel_jobs::AcceptedJob> {
            self.events.record("runtime_enqueue");
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .push(request.clone());

            Ok(launcher_kernel_jobs::AcceptedJob {
                job_id: request.job_id,
                module: request.module,
                kind: request.kind,
                queued_at: IsoDateTime::now(),
            })
        }

        fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
            self.snapshotted_job_ids
                .lock()
                .expect("snapshotted job ids mutex should not be poisoned")
                .push(job_id.clone());
            if self.snapshots_missing {
                return Ok(None);
            }

            Ok(Some(JobSnapshot {
                job_id: job_id.clone(),
                module: "downloads".into(),
                kind: "download".into(),
                state: JobState::Queued,
                ui_state: JobUiState::Queued,
                progress: JobProgress::pending(),
                recoverable: true,
                updated_at: IsoDateTime::now(),
                extension: None,
            }))
        }

        fn pause(&self, job_id: &JobId) -> AppResult<()> {
            self.paused_job_ids
                .lock()
                .expect("paused job ids mutex should not be poisoned")
                .push(job_id.clone());
            Ok(())
        }

        fn resume(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }

        fn cancel(&self, job_id: &JobId) -> AppResult<()> {
            self.canceled_job_ids
                .lock()
                .expect("canceled job ids mutex should not be poisoned")
                .push(job_id.clone());
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct RecordingDownloadRuntimePolicyApplier {
        applied_policies: Arc<Mutex<Vec<DownloadPolicyDto>>>,
    }

    impl RecordingDownloadRuntimePolicyApplier {
        fn applied_policies(&self) -> Vec<DownloadPolicyDto> {
            self.applied_policies
                .lock()
                .expect("applied policy mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadRuntimePolicyApplier for RecordingDownloadRuntimePolicyApplier {
        fn apply_download_policy(&self, policy: &DownloadPolicyDto) -> AppResult<()> {
            self.applied_policies
                .lock()
                .expect("applied policy mutex should not be poisoned")
                .push(policy.clone());
            Ok(())
        }
    }

    #[derive(Clone)]
    struct PendingAwareJobRuntime {
        resume_scheduler: InMemoryDownloadResumeWorkScheduler,
        enqueued_requests: Arc<Mutex<Vec<launcher_kernel_jobs::EnqueueJobRequest<()>>>>,
        pending_seen_at_enqueue: Arc<Mutex<Vec<bool>>>,
    }

    impl PendingAwareJobRuntime {
        fn new(resume_scheduler: InMemoryDownloadResumeWorkScheduler) -> Self {
            Self {
                resume_scheduler,
                enqueued_requests: Arc::new(Mutex::new(Vec::new())),
                pending_seen_at_enqueue: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn enqueued_requests(&self) -> Vec<launcher_kernel_jobs::EnqueueJobRequest<()>> {
            self.enqueued_requests
                .lock()
                .expect("pending-aware enqueued requests mutex should not be poisoned")
                .clone()
        }

        fn pending_seen_at_enqueue(&self) -> Vec<bool> {
            self.pending_seen_at_enqueue
                .lock()
                .expect("pending seen at enqueue mutex should not be poisoned")
                .clone()
        }
    }

    impl launcher_kernel_jobs::JobRuntime for PendingAwareJobRuntime {
        type Extension = ();

        fn enqueue(
            &self,
            request: launcher_kernel_jobs::EnqueueJobRequest<Self::Extension>,
        ) -> AppResult<launcher_kernel_jobs::AcceptedJob> {
            let pending_work = self.resume_scheduler.pending_work();
            let pending_seen = pending_work
                .iter()
                .any(|work| work.job_id == request.job_id);

            self.pending_seen_at_enqueue
                .lock()
                .expect("pending seen at enqueue mutex should not be poisoned")
                .push(pending_seen);
            self.enqueued_requests
                .lock()
                .expect("pending-aware enqueued requests mutex should not be poisoned")
                .push(request.clone());

            Ok(launcher_kernel_jobs::AcceptedJob {
                job_id: request.job_id,
                module: request.module,
                kind: request.kind,
                queued_at: IsoDateTime::now(),
            })
        }

        fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
            Ok(Some(JobSnapshot {
                job_id: job_id.clone(),
                module: "downloads".into(),
                kind: "download".into(),
                state: JobState::Queued,
                ui_state: JobUiState::Queued,
                progress: JobProgress::pending(),
                recoverable: true,
                updated_at: IsoDateTime::now(),
                extension: None,
            }))
        }

        fn pause(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }

        fn resume(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }

        fn cancel(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
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

    #[test]
    fn start_download_persists_request_metadata_and_enqueue_priority() {
        let job_repo = RecordingDownloadJobRepository::default();
        let runtime = RecordingJobRuntime::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo,
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: runtime,
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let request = StartDownloadRequestDto {
            target_id: "asset-123".into(),
            kind: "engine".into(),
            install_intent: Some("install".into()),
            priority: JobPriority::High,
        };

        let accepted = facade
            .start_download(request.clone())
            .expect("start_download should accept the job");

        let created_jobs = facade.deps().job_repo.created_jobs();
        let enqueued_requests = facade.deps().job_runtime.enqueued_requests();

        assert_eq!(
            created_jobs.len(),
            1,
            "start_download should persist a download job record"
        );
        assert_eq!(
            enqueued_requests.len(),
            1,
            "start_download should enqueue one runtime job"
        );

        let created_job = &created_jobs[0];
        assert_eq!(created_job.job_id, accepted.job_id);
        assert_eq!(created_job.target_id, request.target_id);
        assert_eq!(created_job.kind, request.kind);
        assert_eq!(created_job.install_intent, request.install_intent);
        assert_eq!(created_job.priority, request.priority);
        assert_eq!(created_job.state, DownloadJobRecordState::Queued);

        let enqueued_request = &enqueued_requests[0];
        assert_eq!(enqueued_request.job_id, accepted.job_id);
        assert_eq!(enqueued_request.priority, request.priority);
        assert_eq!(enqueued_request.kind, "download");
    }

    #[test]
    fn list_jobs_projects_download_records_from_repository_page() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let first = StartDownloadRequestDto {
            target_id: "asset-a".into(),
            kind: "engine".into(),
            install_intent: Some("install".into()),
            priority: JobPriority::High,
        };
        let second = StartDownloadRequestDto {
            target_id: "asset-b".into(),
            kind: "asset".into(),
            install_intent: None,
            priority: JobPriority::Normal,
        };
        let first_job = facade
            .start_download(first.clone())
            .expect("first download should be accepted");
        let second_job = facade
            .start_download(second.clone())
            .expect("second download should be accepted");

        let page = facade
            .list_jobs(ListDownloadJobsQueryDto {
                page: PageRequest::new(10, None),
                ui_state: None,
            })
            .expect("list_jobs should return module-owned download records");

        assert_eq!(page.items.len(), 2);
        assert_eq!(page.next_cursor, None);
        assert_eq!(page.items[0].job_id, first_job.job_id);
        assert_eq!(page.items[0].title, first.target_id);
        assert_eq!(page.items[0].ui_state, JobUiState::Queued);
        assert_eq!(page.items[0].progress_label, None);
        assert_eq!(page.items[0].throughput_bytes_per_sec, None);
        assert_eq!(page.items[1].job_id, second_job.job_id);
        assert_eq!(page.items[1].title, second.target_id);
        assert_eq!(page.items[1].ui_state, JobUiState::Queued);
    }

    #[test]
    fn list_jobs_filters_by_requested_ui_state() {
        let mut queued_job = make_download_job(JobId::generate());
        queued_job.state = DownloadJobRecordState::Queued;
        let mut paused_job = make_download_job(JobId::generate());
        paused_job.target_id = "paused-asset".into();
        paused_job.state = DownloadJobRecordState::Paused;
        let mut completed_job = make_download_job(JobId::generate());
        completed_job.target_id = "completed-asset".into();
        completed_job.state = DownloadJobRecordState::Completed;
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_jobs(vec![
                queued_job,
                paused_job.clone(),
                completed_job,
            ]),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let page = facade
            .list_jobs(ListDownloadJobsQueryDto {
                page: PageRequest::new(10, None),
                ui_state: Some(JobUiState::Paused),
            })
            .expect("list_jobs should filter module records by requested UI state");

        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].job_id, paused_job.job_id);
        assert_eq!(page.items[0].title, "paused-asset");
        assert_eq!(page.items[0].ui_state, JobUiState::Paused);
    }

    #[test]
    fn get_policy_returns_current_downloads_policy_snapshot_from_store() {
        let expected = DownloadPolicyDto {
            concurrency_slots: 7,
            bandwidth_limit_bytes_per_sec: Some(64 * 1024),
            auto_resume: false,
        };
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::with_policy(expected.clone()),
        });

        let policy = facade
            .get_policy(GetDownloadPolicyQueryDto::default())
            .expect("get_policy should read the downloads-owned policy store");

        assert_eq!(policy, expected);
    }

    #[test]
    fn update_policy_clamps_and_stores_snapshot_for_later_reads() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        facade
            .update_policy(UpdateDownloadPolicyRequestDto {
                concurrency_slots: 0,
                bandwidth_limit_bytes_per_sec: Some(128 * 1024),
                auto_resume: false,
            })
            .expect("update_policy should store a low-clamped policy");
        let low_clamped = facade
            .get_policy(GetDownloadPolicyQueryDto::default())
            .expect("get_policy should read the low-clamped policy");
        assert_eq!(low_clamped.concurrency_slots, 1);
        assert_eq!(low_clamped.bandwidth_limit_bytes_per_sec, Some(128 * 1024));
        assert!(!low_clamped.auto_resume);

        facade
            .update_policy(UpdateDownloadPolicyRequestDto {
                concurrency_slots: 512,
                bandwidth_limit_bytes_per_sec: None,
                auto_resume: true,
            })
            .expect("update_policy should store a high-clamped policy");
        let high_clamped = facade
            .get_policy(GetDownloadPolicyQueryDto::default())
            .expect("get_policy should read the high-clamped policy");
        assert_eq!(high_clamped.concurrency_slots, 128);
        assert_eq!(high_clamped.bandwidth_limit_bytes_per_sec, None);
        assert!(high_clamped.auto_resume);
    }

    #[test]
    fn update_policy_applies_normalized_snapshot_to_runtime_applier() {
        let policy_applier = RecordingDownloadRuntimePolicyApplier::default();
        let facade = DownloadFacade::with_runtime_policy_applier(
            DownloadModuleDeps {
                job_repo: RecordingDownloadJobRepository::default(),
                checkpoint_repo: (),
                manifest_provider: (),
                staging_store: (),
                resume_scheduler: (),
                job_runtime: RecordingJobRuntime::default(),
                policy_store: InMemoryDownloadPolicyStore::new(3),
            },
            policy_applier.clone(),
        );

        facade
            .update_policy(UpdateDownloadPolicyRequestDto {
                concurrency_slots: 512,
                bandwidth_limit_bytes_per_sec: Some(256 * 1024),
                auto_resume: false,
            })
            .expect("update_policy should store and apply the normalized policy");

        let expected = DownloadPolicyDto {
            concurrency_slots: 128,
            bandwidth_limit_bytes_per_sec: Some(256 * 1024),
            auto_resume: false,
        };
        assert_eq!(policy_applier.applied_policies(), vec![expected.clone()]);
        assert_eq!(
            facade
                .get_policy(GetDownloadPolicyQueryDto::default())
                .expect("policy store should retain the same normalized policy"),
            expected
        );
    }

    #[test]
    fn update_policy_default_applier_keeps_policy_store_behavior() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        facade
            .update_policy(UpdateDownloadPolicyRequestDto {
                concurrency_slots: 8,
                bandwidth_limit_bytes_per_sec: None,
                auto_resume: true,
            })
            .expect("default runtime policy applier should be a no-op");

        let stored = facade
            .get_policy(GetDownloadPolicyQueryDto::default())
            .expect("default no-op applier should keep policy-store behavior intact");
        assert_eq!(stored.concurrency_slots, 8);
        assert_eq!(stored.bandwidth_limit_bytes_per_sec, None);
        assert!(stored.auto_resume);
    }

    #[test]
    fn get_job_snapshot_projects_download_record_with_runtime_snapshot() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let request = StartDownloadRequestDto {
            target_id: "asset-123".into(),
            kind: "engine".into(),
            install_intent: Some("install".into()),
            priority: JobPriority::High,
        };
        let accepted = facade
            .start_download(request.clone())
            .expect("start_download should create a queryable downloads job");

        let snapshot = facade
            .get_job_snapshot(GetDownloadJobQueryDto {
                job_id: accepted.job_id.clone(),
            })
            .expect("get_job_snapshot should return the shared runtime snapshot")
            .expect("existing downloads job should have a snapshot projection");

        assert_eq!(
            facade.deps().job_repo.looked_up_job_ids(),
            vec![accepted.job_id.clone()],
            "get_job_snapshot should verify the module job record first"
        );
        assert_eq!(
            facade.deps().job_runtime.snapshotted_job_ids(),
            vec![accepted.job_id.clone()],
            "get_job_snapshot should load the shared runtime snapshot after module lookup"
        );
        assert_eq!(snapshot.job_id, accepted.job_id);
        assert_eq!(snapshot.module, "downloads");
        assert_eq!(snapshot.kind, "download");
        assert_eq!(snapshot.state, JobState::Queued);
        assert_eq!(snapshot.ui_state, JobUiState::Queued);
        assert_eq!(snapshot.progress, JobProgress::pending());
        assert!(snapshot.recoverable);
        let extension = snapshot
            .extension
            .expect("downloads snapshot should include module extension facts");
        assert_eq!(extension.target_id, request.target_id);
        assert_eq!(extension.install_intent, request.install_intent);
        assert_eq!(extension.completed_bytes, 0);
        assert_eq!(extension.total_bytes, None);
        assert!(extension.retryable);
    }

    #[test]
    fn get_job_snapshot_returns_stable_error_when_job_record_is_missing() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let job_id = JobId::generate();

        let error = facade
            .get_job_snapshot(GetDownloadJobQueryDto {
                job_id: job_id.clone(),
            })
            .expect_err("missing module record should stop the query before runtime snapshot");

        assert_eq!(facade.deps().job_repo.looked_up_job_ids(), vec![job_id]);
        assert!(
            facade.deps().job_runtime.snapshotted_job_ids().is_empty(),
            "missing module record must stop before runtime snapshot lookup"
        );
        assert_eq!(error.code, "DL_JOB_NOT_FOUND");
        assert!(!error.retryable);
        assert_eq!(error.severity, AppErrorSeverity::Error);
    }

    #[test]
    fn get_job_snapshot_returns_stable_error_when_runtime_snapshot_is_missing() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::missing_snapshot(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let error = facade
            .get_job_snapshot(GetDownloadJobQueryDto {
                job_id: job_id.clone(),
            })
            .expect_err("missing runtime snapshot should be a stable downloads query error");

        assert_eq!(
            facade.deps().job_repo.looked_up_job_ids(),
            vec![job_id.clone()]
        );
        assert_eq!(
            facade.deps().job_runtime.snapshotted_job_ids(),
            vec![job_id]
        );
        assert_eq!(error.code, "DL_JOB_SNAPSHOT_MISSING");
        assert!(!error.retryable);
        assert_eq!(error.severity, AppErrorSeverity::Error);
    }

    #[test]
    fn pause_download_delegates_to_runtime_control() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let job_id = JobId::generate();

        facade
            .pause_download(PauseDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("pause_download should delegate to the runtime control port");

        assert_eq!(facade.deps().job_runtime.paused_job_ids(), vec![job_id]);
    }

    #[test]
    fn resume_download_reads_checkpoint_before_resume_decision() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::default(),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let result = facade.resume_download(ResumeDownloadRequestDto {
            job_id: job_id.clone(),
        });

        assert_eq!(
            facade.deps().checkpoint_repo.loaded_job_ids(),
            vec![job_id],
            "resume_download must read the module checkpoint before deciding how to resume"
        );
        let error = result.expect_err("full resume orchestration should remain out of scope");
        assert_eq!(error.code, "DOWNLOADS_NOT_WIRED");
    }

    #[test]
    fn resume_download_returns_stable_error_when_checkpoint_is_missing() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::missing(),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let error = facade
            .resume_download(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect_err("missing checkpoint should stop resume before later orchestration");

        assert_eq!(facade.deps().checkpoint_repo.loaded_job_ids(), vec![job_id]);
        assert_eq!(error.code, "DL_CHECKPOINT_MISSING");
        assert!(!error.retryable);
        assert_eq!(error.severity, AppErrorSeverity::Error);
    }

    #[test]
    fn resume_download_returns_stable_error_when_job_record_is_missing() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: RecordingCheckpointRepository::default(),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let job_id = JobId::generate();

        let error = facade
            .resume_download(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect_err("missing job record should stop resume before checkpoint lookup");

        assert_eq!(facade.deps().job_repo.looked_up_job_ids(), vec![job_id]);
        assert!(
            facade.deps().checkpoint_repo.loaded_job_ids().is_empty(),
            "missing job record must stop before checkpoint lookup"
        );
        assert_eq!(error.code, "DL_JOB_NOT_FOUND");
        assert!(!error.retryable);
        assert_eq!(error.severity, AppErrorSeverity::Error);
    }

    #[test]
    fn resume_download_validates_staging_after_checkpoint_is_present() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::default(),
            manifest_provider: (),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let result = facade.resume_download(ResumeDownloadRequestDto {
            job_id: job_id.clone(),
        });

        assert_eq!(
            facade.deps().staging_store.ensured_job_ids(),
            vec![job_id],
            "resume_download must validate staging before later manifest/runtime resume work"
        );
        let error = result.expect_err("full resume orchestration should remain out of scope");
        assert_eq!(error.code, "DOWNLOADS_NOT_WIRED");
    }

    #[test]
    fn resume_download_reconstructs_manifest_after_staging_is_valid() {
        let job_id = JobId::generate();
        let job = make_download_job(job_id.clone());
        let expected_target_id = job.target_id.clone();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(job),
            checkpoint_repo: RecordingCheckpointRepository::default(),
            manifest_provider: RecordingManifestProvider::default(),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let result = facade.resume_download(ResumeDownloadRequestDto {
            job_id: job_id.clone(),
        });

        assert_eq!(
            facade.deps().staging_store.ensured_job_ids(),
            vec![job_id],
            "manifest reconstruction should only run after staging validation succeeds"
        );
        assert_eq!(
            facade.deps().manifest_provider.fetched_target_ids(),
            vec![expected_target_id],
            "resume_download must reload or reconstruct the manifest plan before runtime enqueue"
        );
        let error = result.expect_err("full resume orchestration should remain out of scope");
        assert_eq!(error.code, "DOWNLOADS_NOT_WIRED");
    }

    #[test]
    fn resume_download_enqueues_existing_job_when_decisions_have_runtime_candidates() {
        let job_id = JobId::generate();
        let mut job = make_download_job(job_id.clone());
        job.priority = JobPriority::High;
        let expected_target_id = job.target_id.clone();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(job),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(Vec::new()),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin".into(),
                    expected_hash: Some("sha256:segment".into()),
                    write_target: "file.bin.part".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let accepted = facade
            .resume_download(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("resume_download should enqueue the existing runtime job");

        assert_eq!(
            facade.deps().checkpoint_repo.loaded_job_ids(),
            vec![job_id.clone()]
        );
        assert_eq!(
            facade.deps().staging_store.ensured_job_ids(),
            vec![job_id.clone()]
        );
        assert_eq!(
            facade.deps().manifest_provider.fetched_target_ids(),
            vec![expected_target_id]
        );
        assert_eq!(accepted.job_id, job_id);
        assert_eq!(accepted.module, "downloads");
        assert_eq!(accepted.kind, "download");

        let enqueued_requests = facade.deps().job_runtime.enqueued_requests();
        assert_eq!(
            enqueued_requests.len(),
            1,
            "resume_download should enqueue one job-level runtime request"
        );

        let enqueued_request = &enqueued_requests[0];
        assert_eq!(enqueued_request.job_id, accepted.job_id);
        assert_eq!(enqueued_request.module, "downloads");
        assert_eq!(enqueued_request.kind, "download");
        assert_eq!(enqueued_request.priority, JobPriority::High);
        assert!(enqueued_request.recoverable);
        assert_eq!(enqueued_request.extension, None);
    }

    #[test]
    fn resume_download_schedules_work_plan_before_runtime_enqueue() {
        let job_id = JobId::generate();
        let events = ResumeRuntimeEvents::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-partial".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    downloaded_bytes: 512,
                    status: DownloadSegmentCheckpointStatus::InProgress,
                    partial_path: Some("file.bin.part-0".into()),
                    etag: Some("etag-partial".into()),
                    hash_state_ref: Some("hash-partial".into()),
                },
            ]),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-partial".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin#partial".into(),
                    expected_hash: Some("sha256:partial".into()),
                    write_target: "file.bin.part-0".into(),
                },
                DownloadManifestSegment {
                    segment_id: "segment-remaining".into(),
                    file_id: "file-1".into(),
                    offset: 1024,
                    length: 2048,
                    source_locator: "https://example.invalid/file.bin#remaining".into(),
                    expected_hash: None,
                    write_target: "file.bin.part-1".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: RecordingResumeWorkScheduler::with_events(events.clone()),
            job_runtime: RecordingJobRuntime::with_events(events.clone()),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let outcome = facade
            .resume_download_outcome(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("resume_download_outcome should schedule work before runtime enqueue");

        assert!(
            matches!(outcome, DownloadResumeOutcome::RuntimeAccepted(_)),
            "runtime candidates should still return accepted runtime work"
        );
        assert_eq!(
            events.entries(),
            vec!["schedule_work", "runtime_enqueue"],
            "downloads scheduler work must be prepared before shared runtime enqueue"
        );

        let scheduled_plans = facade.deps().resume_scheduler.scheduled_plans();
        assert_eq!(scheduled_plans.len(), 1);
        assert_eq!(scheduled_plans[0].0, job_id);
        assert_eq!(scheduled_plans[0].1.items.len(), 2);
        assert_eq!(scheduled_plans[0].1.items[0].segment_id, "segment-partial");
        assert_eq!(scheduled_plans[0].1.items[0].start_offset, 512);
        assert_eq!(
            scheduled_plans[0].1.items[1].segment_id,
            "segment-remaining"
        );
        assert_eq!(scheduled_plans[0].1.items[1].start_offset, 1024);
    }

    #[test]
    fn resume_download_registers_pending_work_before_runtime_enqueue() {
        let job_id = JobId::generate();
        let resume_scheduler = InMemoryDownloadResumeWorkScheduler::default();
        let runtime = PendingAwareJobRuntime::new(resume_scheduler.clone());
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-partial".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    downloaded_bytes: 512,
                    status: DownloadSegmentCheckpointStatus::InProgress,
                    partial_path: Some("file.bin.part-0".into()),
                    etag: Some("etag-partial".into()),
                    hash_state_ref: Some("hash-partial".into()),
                },
            ]),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-partial".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin#partial".into(),
                    expected_hash: Some("sha256:partial".into()),
                    write_target: "file.bin.part-0".into(),
                },
                DownloadManifestSegment {
                    segment_id: "segment-remaining".into(),
                    file_id: "file-1".into(),
                    offset: 1024,
                    length: 2048,
                    source_locator: "https://example.invalid/file.bin#remaining".into(),
                    expected_hash: None,
                    write_target: "file.bin.part-1".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler,
            job_runtime: runtime.clone(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let outcome = facade
            .resume_download_outcome(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("resume_download_outcome should register pending work before enqueue");

        assert!(
            matches!(outcome, DownloadResumeOutcome::RuntimeAccepted(_)),
            "runtime candidates should still enqueue the existing job"
        );
        assert_eq!(
            runtime.pending_seen_at_enqueue(),
            vec![true],
            "runtime enqueue should only run after pending resume work is registered"
        );

        let pending_work: Vec<DownloadPendingResumeWork> =
            facade.deps().resume_scheduler.pending_work();
        assert_eq!(pending_work.len(), 1);
        assert_eq!(pending_work[0].job_id, job_id);
        assert_eq!(pending_work[0].plan.items.len(), 2);
        assert_eq!(pending_work[0].plan.items[0].segment_id, "segment-partial");
        assert_eq!(pending_work[0].plan.items[0].start_offset, 512);
        assert_eq!(
            pending_work[0].plan.items[1].segment_id,
            "segment-remaining"
        );
        assert_eq!(pending_work[0].plan.items[1].start_offset, 1024);
        assert_eq!(runtime.enqueued_requests().len(), 1);
    }

    #[test]
    fn pending_resume_work_source_drains_matching_job_and_preserves_unrelated_work() {
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let job_id = JobId::generate();
        let unrelated_job_id = JobId::generate();
        let job_plan = make_resume_work_plan("segment-match");
        let unrelated_plan = make_resume_work_plan("segment-unrelated");

        scheduler
            .schedule_resume_work(&job_id, &job_plan)
            .expect("scheduling matching pending work should succeed");
        scheduler
            .schedule_resume_work(&unrelated_job_id, &unrelated_plan)
            .expect("scheduling unrelated pending work should succeed");

        let drained = scheduler
            .drain_pending_resume_work(&job_id)
            .expect("draining pending work for one job should succeed");

        assert_eq!(
            drained,
            vec![DownloadPendingResumeWork {
                job_id: job_id.clone(),
                plan: job_plan,
            }]
        );
        assert_eq!(
            scheduler.pending_work(),
            vec![DownloadPendingResumeWork {
                job_id: unrelated_job_id,
                plan: unrelated_plan,
            }],
            "draining one job must preserve pending work for other jobs"
        );
    }

    #[test]
    fn pending_resume_work_source_returns_empty_for_job_without_pending_work() {
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let existing_job_id = JobId::generate();
        let missing_job_id = JobId::generate();
        let existing_plan = make_resume_work_plan("segment-existing");

        scheduler
            .schedule_resume_work(&existing_job_id, &existing_plan)
            .expect("scheduling existing pending work should succeed");

        let drained = scheduler
            .drain_pending_resume_work(&missing_job_id)
            .expect("draining a job with no pending work should succeed");

        assert!(
            drained.is_empty(),
            "missing job drain should return an empty work list"
        );
        assert_eq!(
            scheduler.pending_work(),
            vec![DownloadPendingResumeWork {
                job_id: existing_job_id,
                plan: existing_plan,
            }],
            "empty drain must not discard unrelated pending work"
        );
    }

    #[test]
    fn resume_download_skips_runtime_enqueue_when_scheduler_fails() {
        let job_id = JobId::generate();
        let events = ResumeRuntimeEvents::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(Vec::new()),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-remaining".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin#remaining".into(),
                    expected_hash: None,
                    write_target: "file.bin.part".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: RecordingResumeWorkScheduler::failing_with(
                events.clone(),
                AppError::new(
                    "DL_RESUME_SCHEDULER_UNAVAILABLE",
                    "resume scheduler unavailable for test",
                    false,
                    AppErrorSeverity::Error,
                    CorrelationId::generate(),
                ),
            ),
            job_runtime: RecordingJobRuntime::with_events(events.clone()),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let error = facade
            .resume_download_outcome(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect_err("scheduler failure should stop before shared runtime enqueue");

        assert_eq!(error.code, "DL_RESUME_SCHEDULER_UNAVAILABLE");
        assert_eq!(events.entries(), vec!["schedule_work"]);
        assert_eq!(facade.deps().resume_scheduler.scheduled_plans().len(), 1);
        assert_eq!(
            facade.deps().job_runtime.enqueued_requests().len(),
            0,
            "scheduler failure must not enqueue shared runtime work"
        );
    }

    #[test]
    fn resume_download_outcome_returns_already_complete_when_all_segments_are_sealed() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    downloaded_bytes: 1024,
                    status: DownloadSegmentCheckpointStatus::Completed,
                    partial_path: Some("file.bin.part".into()),
                    etag: Some("etag-1".into()),
                    hash_state_ref: None,
                },
            ]),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin".into(),
                    expected_hash: Some("sha256:segment".into()),
                    write_target: "file.bin.part".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let outcome = facade
            .resume_download_outcome(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("all-sealed resume should return a module-owned completion outcome");

        assert_eq!(
            outcome,
            DownloadResumeOutcome::AlreadyComplete {
                job_id: job_id.clone(),
                module: "downloads".into(),
                kind: "download".into(),
            }
        );
        assert_eq!(
            facade.deps().job_runtime.enqueued_requests().len(),
            0,
            "all-sealed resume must not enqueue runtime work"
        );
    }

    #[test]
    fn resume_download_all_sealed_does_not_touch_scheduler() {
        let job_id = JobId::generate();
        let events = ResumeRuntimeEvents::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    downloaded_bytes: 1024,
                    status: DownloadSegmentCheckpointStatus::Completed,
                    partial_path: Some("file.bin.part".into()),
                    etag: Some("etag-1".into()),
                    hash_state_ref: None,
                },
            ]),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin".into(),
                    expected_hash: Some("sha256:segment".into()),
                    write_target: "file.bin.part".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: RecordingResumeWorkScheduler::failing_with(
                events.clone(),
                AppError::new(
                    "DL_RESUME_SCHEDULER_SHOULD_NOT_RUN",
                    "all-sealed resume should not touch scheduler",
                    false,
                    AppErrorSeverity::Error,
                    CorrelationId::generate(),
                ),
            ),
            job_runtime: RecordingJobRuntime::with_events(events.clone()),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let outcome = facade
            .resume_download_outcome(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("all-sealed resume should not call the failing scheduler");

        assert_eq!(
            outcome,
            DownloadResumeOutcome::AlreadyComplete {
                job_id,
                module: "downloads".into(),
                kind: "download".into(),
            }
        );
        assert!(
            events.entries().is_empty(),
            "all-sealed resume must not schedule work or enqueue runtime jobs"
        );
        assert_eq!(facade.deps().resume_scheduler.scheduled_plans().len(), 0);
        assert_eq!(facade.deps().job_runtime.enqueued_requests().len(), 0);
    }

    #[test]
    fn resume_download_returns_stable_error_when_segment_checkpoint_mismatches_manifest() {
        let job_id = JobId::generate();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::with_job(make_download_job(job_id.clone())),
            checkpoint_repo: RecordingCheckpointRepository::with_segments(vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-1".into(),
                    file_id: "stale-file".into(),
                    offset: 0,
                    length: 1024,
                    downloaded_bytes: 512,
                    status: DownloadSegmentCheckpointStatus::InProgress,
                    partial_path: Some("file.bin.part".into()),
                    etag: Some("etag-1".into()),
                    hash_state_ref: None,
                },
            ]),
            manifest_provider: RecordingManifestProvider::with_segments(vec![
                DownloadManifestSegment {
                    segment_id: "segment-1".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin".into(),
                    expected_hash: Some("sha256:segment".into()),
                    write_target: "file.bin.part".into(),
                },
            ]),
            staging_store: RecordingStagingObjectStore::default(),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });

        let error = facade
            .resume_download(ResumeDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect_err("mismatched checkpoint facts should stop automatic resume");

        assert_eq!(
            facade.deps().job_runtime.enqueued_requests().len(),
            0,
            "mismatched segment facts must not be handed to runtime enqueue"
        );
        assert_eq!(error.code, "DL_RESUME_SEGMENT_MISMATCH");
        assert!(!error.retryable);
        assert_eq!(error.severity, AppErrorSeverity::Error);
    }

    #[test]
    fn resume_segment_decisions_seal_completed_checkpoint_segments() {
        let job_id = JobId::generate();
        let manifest = DownloadManifestPlan {
            target_id: "asset-123".into(),
            segments: vec![DownloadManifestSegment {
                segment_id: "segment-1".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 1024,
                source_locator: "https://example.invalid/file.bin".into(),
                expected_hash: Some("sha256:segment".into()),
                write_target: "file.bin.part".into(),
            }],
        };
        let checkpoints = vec![DownloadSegmentCheckpointRecord {
            job_id,
            segment_id: "segment-1".into(),
            file_id: "file-1".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 1024,
            status: DownloadSegmentCheckpointStatus::Completed,
            partial_path: Some("file.bin.part".into()),
            etag: Some("etag-1".into()),
            hash_state_ref: None,
        }];

        let decisions = build_resume_segment_decisions(&manifest, &checkpoints)
            .expect("matching completed checkpoint should produce resume decisions");

        assert_eq!(decisions.len(), 1);
        assert_eq!(decisions[0].segment_id, "segment-1");
        assert_eq!(
            decisions[0].action,
            DownloadResumeSegmentAction::SealCompleted
        );
        assert!(
            !decisions[0].is_runtime_enqueue_candidate(),
            "sealed completed segments must not be runtime enqueue candidates"
        );
    }

    #[test]
    fn resume_segment_decisions_resume_partial_checkpoint_segments() {
        let job_id = JobId::generate();
        let manifest = DownloadManifestPlan {
            target_id: "asset-123".into(),
            segments: vec![DownloadManifestSegment {
                segment_id: "segment-1".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 1024,
                source_locator: "https://example.invalid/file.bin".into(),
                expected_hash: Some("sha256:segment".into()),
                write_target: "file.bin.part".into(),
            }],
        };
        let checkpoints = vec![DownloadSegmentCheckpointRecord {
            job_id,
            segment_id: "segment-1".into(),
            file_id: "file-1".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 512,
            status: DownloadSegmentCheckpointStatus::InProgress,
            partial_path: Some("file.bin.part".into()),
            etag: Some("etag-1".into()),
            hash_state_ref: Some("hash-state-1".into()),
        }];

        let decisions = build_resume_segment_decisions(&manifest, &checkpoints)
            .expect("matching partial checkpoint should produce resume decisions");

        assert_eq!(decisions.len(), 1);
        assert_eq!(decisions[0].segment_id, "segment-1");
        assert_eq!(
            decisions[0].action,
            DownloadResumeSegmentAction::ResumePartial
        );
        assert!(
            decisions[0].is_runtime_enqueue_candidate(),
            "partial segments should remain runtime enqueue candidates"
        );
    }

    #[test]
    fn resume_segment_decisions_reject_mismatched_checkpoint_segments() {
        let job_id = JobId::generate();
        let manifest = DownloadManifestPlan {
            target_id: "asset-123".into(),
            segments: vec![DownloadManifestSegment {
                segment_id: "segment-1".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 1024,
                source_locator: "https://example.invalid/file.bin".into(),
                expected_hash: Some("sha256:segment".into()),
                write_target: "file.bin.part".into(),
            }],
        };
        let checkpoints = vec![DownloadSegmentCheckpointRecord {
            job_id,
            segment_id: "segment-1".into(),
            file_id: "stale-file".into(),
            offset: 0,
            length: 1024,
            downloaded_bytes: 1024,
            status: DownloadSegmentCheckpointStatus::Completed,
            partial_path: Some("file.bin.part".into()),
            etag: Some("etag-1".into()),
            hash_state_ref: None,
        }];

        let decisions = build_resume_segment_decisions(&manifest, &checkpoints)
            .expect("mismatched checkpoint facts should produce resume decisions");

        assert_eq!(decisions.len(), 1);
        assert_eq!(decisions[0].segment_id, "segment-1");
        assert_eq!(
            decisions[0].action,
            DownloadResumeSegmentAction::RejectMismatch
        );
        assert!(
            !decisions[0].is_runtime_enqueue_candidate(),
            "mismatched segments must not be runtime enqueue candidates"
        );
    }

    #[test]
    fn resume_segment_decisions_queue_remaining_without_checkpoint() {
        let manifest = DownloadManifestPlan {
            target_id: "asset-123".into(),
            segments: vec![DownloadManifestSegment {
                segment_id: "segment-1".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 1024,
                source_locator: "https://example.invalid/file.bin".into(),
                expected_hash: Some("sha256:segment".into()),
                write_target: "file.bin.part".into(),
            }],
        };
        let checkpoints = Vec::<DownloadSegmentCheckpointRecord>::new();

        let decisions = build_resume_segment_decisions(&manifest, &checkpoints)
            .expect("missing checkpoint should produce resume decisions");

        assert_eq!(decisions.len(), 1);
        assert_eq!(decisions[0].segment_id, "segment-1");
        assert_eq!(
            decisions[0].action,
            DownloadResumeSegmentAction::QueueRemaining
        );
        assert!(
            decisions[0].is_runtime_enqueue_candidate(),
            "remaining segments should be runtime enqueue candidates"
        );
    }

    #[test]
    fn resume_work_plan_derives_only_partial_and_remaining_items() {
        let job_id = JobId::generate();
        let manifest = DownloadManifestPlan {
            target_id: "asset-123".into(),
            segments: vec![
                DownloadManifestSegment {
                    segment_id: "segment-sealed".into(),
                    file_id: "file-1".into(),
                    offset: 0,
                    length: 256,
                    source_locator: "https://example.invalid/file.bin#sealed".into(),
                    expected_hash: Some("sha256:sealed".into()),
                    write_target: "file.bin.part-0".into(),
                },
                DownloadManifestSegment {
                    segment_id: "segment-partial".into(),
                    file_id: "file-1".into(),
                    offset: 256,
                    length: 512,
                    source_locator: "https://example.invalid/file.bin#partial".into(),
                    expected_hash: Some("sha256:partial".into()),
                    write_target: "file.bin.part-1".into(),
                },
                DownloadManifestSegment {
                    segment_id: "segment-remaining".into(),
                    file_id: "file-1".into(),
                    offset: 768,
                    length: 1024,
                    source_locator: "https://example.invalid/file.bin#remaining".into(),
                    expected_hash: None,
                    write_target: "file.bin.part-2".into(),
                },
                DownloadManifestSegment {
                    segment_id: "segment-mismatch".into(),
                    file_id: "file-1".into(),
                    offset: 1792,
                    length: 128,
                    source_locator: "https://example.invalid/file.bin#mismatch".into(),
                    expected_hash: Some("sha256:mismatch".into()),
                    write_target: "file.bin.part-3".into(),
                },
            ],
        };
        let checkpoints = vec![
            DownloadSegmentCheckpointRecord {
                job_id: job_id.clone(),
                segment_id: "segment-sealed".into(),
                file_id: "file-1".into(),
                offset: 0,
                length: 256,
                downloaded_bytes: 256,
                status: DownloadSegmentCheckpointStatus::Completed,
                partial_path: Some("file.bin.part-0".into()),
                etag: Some("etag-sealed".into()),
                hash_state_ref: None,
            },
            DownloadSegmentCheckpointRecord {
                job_id: job_id.clone(),
                segment_id: "segment-partial".into(),
                file_id: "file-1".into(),
                offset: 256,
                length: 512,
                downloaded_bytes: 128,
                status: DownloadSegmentCheckpointStatus::InProgress,
                partial_path: Some("file.bin.part-1".into()),
                etag: Some("etag-partial".into()),
                hash_state_ref: Some("hash-partial".into()),
            },
            DownloadSegmentCheckpointRecord {
                job_id,
                segment_id: "segment-mismatch".into(),
                file_id: "stale-file".into(),
                offset: 1792,
                length: 128,
                downloaded_bytes: 128,
                status: DownloadSegmentCheckpointStatus::Completed,
                partial_path: Some("file.bin.part-3".into()),
                etag: Some("etag-mismatch".into()),
                hash_state_ref: None,
            },
        ];
        let decisions = build_resume_segment_decisions(&manifest, &checkpoints)
            .expect("mixed checkpoint facts should still produce decisions");

        let work_plan = build_resume_work_plan(&manifest, &checkpoints, &decisions)
            .expect("mixed resume decisions should produce a work plan");

        assert_eq!(
            work_plan.items.len(),
            2,
            "only partial and remaining segments should produce work items"
        );

        let partial = work_plan
            .items
            .iter()
            .find(|item| item.segment_id == "segment-partial")
            .expect("partial segment should become a work item");
        assert_eq!(partial.file_id, "file-1");
        assert_eq!(
            partial.source_locator,
            "https://example.invalid/file.bin#partial"
        );
        assert_eq!(partial.write_target, "file.bin.part-1");
        assert_eq!(partial.expected_hash.as_deref(), Some("sha256:partial"));
        assert_eq!(partial.start_offset, 128);
        assert_eq!(partial.length, 512);
        assert_eq!(partial.resume_mode, DownloadResumeWorkMode::Partial);
        assert_eq!(partial.checkpoint_ref.as_deref(), Some("segment-partial"));

        let remaining = work_plan
            .items
            .iter()
            .find(|item| item.segment_id == "segment-remaining")
            .expect("remaining segment should become a work item");
        assert_eq!(remaining.file_id, "file-1");
        assert_eq!(
            remaining.source_locator,
            "https://example.invalid/file.bin#remaining"
        );
        assert_eq!(remaining.write_target, "file.bin.part-2");
        assert_eq!(remaining.expected_hash, None);
        assert_eq!(remaining.start_offset, 768);
        assert_eq!(remaining.length, 1024);
        assert_eq!(remaining.resume_mode, DownloadResumeWorkMode::FromStart);
        assert_eq!(remaining.checkpoint_ref, None);
    }

    #[test]
    fn cancel_download_delegates_to_runtime_control() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            resume_scheduler: (),
            job_runtime: RecordingJobRuntime::default(),
            policy_store: InMemoryDownloadPolicyStore::new(3),
        });
        let job_id = JobId::generate();

        facade
            .cancel_download(CancelDownloadRequestDto {
                job_id: job_id.clone(),
            })
            .expect("cancel_download should delegate to the runtime control port");

        assert_eq!(facade.deps().job_runtime.canceled_job_ids(), vec![job_id]);
    }
}
