use std::sync::Arc;

use launcher_kernel_foundation::{AppResult, JobId};
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

use crate::facade::{DownloadPendingResumeWork, DownloadPendingResumeWorkSource};

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

    use super::{DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadJobDriver};

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
}
