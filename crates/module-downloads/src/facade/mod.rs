//! downloads 模块的公开 facade 边界。
//!
//! 这里集中暴露下载任务的命令入口、最小持久化记录契约和依赖束，供
//! composition root 与宿主传输层装配。当前 `start_download` 已返回后端拥有的
//! accepted job，`pause_download` / `cancel_download` 已转接 shared job runtime；
//! 恢复、查询和策略入口仍保留 `DOWNLOADS_NOT_WIRED` stub 语义。

use launcher_kernel_foundation::{AppError, AppErrorSeverity, AppResult, CorrelationId, JobId};
use launcher_kernel_jobs::{AcceptedJob, EnqueueJobRequest, JobPriority, JobRuntime};

use crate::contracts::{
    CancelDownloadRequestDto, DownloadJobListDto, DownloadJobSnapshotDto, DownloadPolicyDto,
    GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto,
    PauseDownloadRequestDto, ResumeDownloadRequestDto, StartDownloadRequestDto,
    UpdateDownloadPolicyRequestDto,
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
    /// 同步 downloads 模块拥有的粗粒度状态迁移结果。
    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()>;
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

/// 组装 downloads facade 所需的依赖束。
#[derive(Debug, Clone)]
pub struct DownloadModuleDeps<J, C, M, S, R> {
    /// 保存 intake 元数据与粗粒度下载状态的模块仓储。
    pub job_repo: J,
    /// 为暂停、恢复和重启场景保留的 checkpoint 能力。
    pub checkpoint_repo: C,
    /// 负责解析 provider manifest 的适配器能力，留给后续切片接线。
    pub manifest_provider: M,
    /// 管理本地 staging 产物落盘位置的能力，留给后续切片接线。
    pub staging_store: S,
    /// 真正拥有 accepted job 生命周期的共享 job runtime。
    pub job_runtime: R,
}

/// downloads 模块对外暴露的 use-case facade。
pub struct DownloadFacade<J, C, M, S, R> {
    deps: DownloadModuleDeps<J, C, M, S, R>,
}

impl<J, C, M, S, R> DownloadFacade<J, C, M, S, R> {
    /// 用已经装配好的依赖束创建 downloads facade。
    pub fn new(deps: DownloadModuleDeps<J, C, M, S, R>) -> Self {
        Self { deps }
    }

    /// 暴露依赖束，主要服务于 composition-root smoke test 与局部诊断。
    pub fn deps(&self) -> &DownloadModuleDeps<J, C, M, S, R> {
        &self.deps
    }
}

impl<J: DownloadJobRepository, C, M, S, R: JobRuntime<Extension = ()>>
    DownloadFacade<J, C, M, S, R>
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
        let _ = query;
        Err(not_wired("list_jobs"))
    }

    /// 预留单任务快照查询；当前仍由模块边界返回未接线错误。
    pub fn get_job_snapshot(
        &self,
        query: GetDownloadJobQueryDto,
    ) -> AppResult<Option<DownloadJobSnapshotDto>> {
        let _ = query;
        Err(not_wired("get_job_snapshot"))
    }

    /// 预留下载策略读取；当前仍由模块边界返回未接线错误。
    pub fn get_policy(&self, query: GetDownloadPolicyQueryDto) -> AppResult<DownloadPolicyDto> {
        let _ = query;
        Err(not_wired("get_policy"))
    }

    /// 预留下载策略更新；当前仍由模块边界返回未接线错误。
    pub fn update_policy(&self, request: UpdateDownloadPolicyRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("update_policy"))
    }
}

impl<
        J: DownloadJobRepository,
        C: DownloadCheckpointRepository,
        M: DownloadManifestProviderPort,
        S: DownloadStagingObjectStore,
        R: JobRuntime<Extension = ()>,
    > DownloadFacade<J, C, M, S, R>
{
    /// Loads module state, checkpoint, staging, and manifest before later resume decisions.
    /// 先读取模块状态、checkpoint、staging 和 manifest，然后才进入后续恢复决策。
    ///
    /// Missing module state is a stable downloads-domain failure; full manifest
    /// and runtime resume orchestration stays in a later slice.
    /// Missing module state is a stable downloads-domain failure; runtime resume
    /// enqueue still stays in a later slice.
    /// 缺失模块状态是稳定的 downloads 域失败；runtime 恢复入队仍留给后续切片。
    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob> {
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

        if has_runtime_enqueue_candidate && !has_reject_mismatch {
            // Keep this first resume slice job-level; segment payloads stay in downloads.
            // 首个恢复切片只做 job-level 入队；segment payload 仍留在 downloads 内部。
            return self.deps.job_runtime.enqueue(EnqueueJobRequest {
                job_id: request.job_id,
                module: "downloads".to_string(),
                kind: "download".to_string(),
                priority: job.priority,
                recoverable: true,
                extension: None,
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
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AppErrorSeverity, AppResult, IsoDateTime, JobId};
    use launcher_kernel_jobs::{JobPriority, JobProgress, JobSnapshot, JobState, JobUiState};

    use crate::driver::{DownloadCheckpointRecord, DownloadCheckpointRepository};

    use super::{
        build_resume_segment_decisions, DownloadFacade, DownloadJobRecord, DownloadJobRecordState,
        DownloadJobRepository, DownloadManifestPlan, DownloadManifestProviderPort,
        DownloadManifestSegment, DownloadModuleDeps, DownloadResumeSegmentAction,
        DownloadStagingObjectStore, DownloadStagingRoot,
    };
    use crate::contracts::{
        CancelDownloadRequestDto, PauseDownloadRequestDto, ResumeDownloadRequestDto,
        StartDownloadRequestDto,
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

    #[derive(Default)]
    struct RecordingJobRuntime {
        enqueued_requests: Mutex<Vec<launcher_kernel_jobs::EnqueueJobRequest<()>>>,
        paused_job_ids: Mutex<Vec<JobId>>,
        canceled_job_ids: Mutex<Vec<JobId>>,
    }

    impl RecordingJobRuntime {
        fn enqueued_requests(&self) -> Vec<launcher_kernel_jobs::EnqueueJobRequest<()>> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
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

    #[test]
    fn start_download_persists_request_metadata_and_enqueue_priority() {
        let job_repo = RecordingDownloadJobRepository::default();
        let runtime = RecordingJobRuntime::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo,
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            job_runtime: runtime,
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
    fn pause_download_delegates_to_runtime_control() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
            job_runtime: RecordingJobRuntime::default(),
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
    fn cancel_download_delegates_to_runtime_control() {
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo: RecordingDownloadJobRepository::default(),
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            job_runtime: RecordingJobRuntime::default(),
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
