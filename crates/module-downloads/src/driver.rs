use std::sync::Arc;

use launcher_kernel_foundation::{AppResult, JobId};
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

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

/// `downloads/download` 任务种类的恢复驱动。
///
/// 只有在持久化下载 checkpoint 仍然存在时，stage 2 restore 才允许把
/// 一个下载任务继续视为可恢复。staging 文件校验仍留给后续切片处理。
#[derive(Clone)]
pub struct DownloadJobDriver {
    checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
    pending_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};

    use launcher_kernel_foundation::{IsoDateTime, JobId};
    use launcher_kernel_jobs::{JobDriver, JobProgress, JobState, JobUiState};

    use crate::facade::{
        DownloadPendingResumeWork, DownloadResumeWorkItem, DownloadResumeWorkMode,
        DownloadResumeWorkPlan, DownloadResumeWorkScheduler, InMemoryDownloadResumeWorkScheduler,
    };

    use super::{
        DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadDriverExecutionTurn,
        DownloadJobDriver, DownloadSegmentExecutionRequest,
    };

    #[derive(Default)]
    struct InMemoryCheckpointRepository {
        job_ids: Mutex<HashSet<JobId>>,
    }

    impl DownloadCheckpointRepository for InMemoryCheckpointRepository {
        fn load(
            &self,
            job_id: &JobId,
        ) -> launcher_kernel_foundation::AppResult<Option<DownloadCheckpointRecord>> {
            let has_checkpoint = self
                .job_ids
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .contains(job_id);

            Ok(has_checkpoint.then(|| DownloadCheckpointRecord::empty(job_id.clone())))
        }

        fn save(
            &self,
            checkpoint: &DownloadCheckpointRecord,
        ) -> launcher_kernel_foundation::AppResult<()> {
            self.job_ids
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .insert(checkpoint.job_id.clone());
            Ok(())
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
}
