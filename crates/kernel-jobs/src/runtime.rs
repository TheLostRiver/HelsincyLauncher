use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};

use crate::model::{
    AcceptedJob, EnqueueJobRequest, JobProgress, JobSnapshot, JobState, JobUiState,
    RestoreDisposition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 表示共享作业运行时选择可执行作业时使用的队列策略。
pub struct RuntimeQueuePolicy {
    /// 允许共享运行时同时推进的最大作业数。
    pub max_concurrent_jobs: usize,
}

impl RuntimeQueuePolicy {
    /// 按调用方提供的全局并发预算构造队列策略。
    pub fn new(max_concurrent_jobs: usize) -> Self {
        Self {
            max_concurrent_jobs,
        }
    }
}

/// 表示模块提供给共享作业运行时的作业驱动边界。
///
/// 运行时只用 `module` 与 `kind` 做路由，并在恢复阶段回调驱动；具体业务执行和 checkpoint 仍由模块拥有。
pub trait JobDriver<E>: Send + Sync {
    /// 返回该驱动所属的稳定模块名。
    fn module(&self) -> &'static str;
    /// 返回该驱动能够处理的模块内稳定作业类型名。
    fn kind(&self) -> &'static str;
    /// 基于持久化快照尝试恢复模块作业，并返回运行时应采用的恢复处置。
    fn restore(&self, snapshot: &JobSnapshot<E>) -> AppResult<RestoreDisposition>;
}

/// 表示共享运行时用于按模块和作业类型查找驱动的注册表。
pub struct JobDriverRegistry<E> {
    /// 以 `(module, kind)` 作为稳定路由键保存模块驱动。
    drivers: HashMap<(String, String), Arc<dyn JobDriver<E>>>,
}

impl<E: 'static> Default for JobDriverRegistry<E> {
    fn default() -> Self {
        Self {
            drivers: HashMap::new(),
        }
    }
}

impl<E: 'static> JobDriverRegistry<E> {
    /// 构造一个尚未注册任何模块驱动的空注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个模块驱动；相同 `(module, kind)` 的后注册驱动会覆盖旧驱动。
    pub fn register(&mut self, driver: Arc<dyn JobDriver<E>>) {
        self.drivers.insert(
            (driver.module().to_string(), driver.kind().to_string()),
            driver,
        );
    }

    /// 按模块名和作业类型解析驱动，找不到时返回空结果。
    pub fn resolve(&self, module: &str, kind: &str) -> Option<&dyn JobDriver<E>> {
        self.drivers
            .get(&(module.to_string(), kind.to_string()))
            .map(|d| d.as_ref())
    }
}

/// 表示共享运行时持久化、更新和查询作业快照的存储端口。
///
/// 该端口只处理通用 snapshot，不接管模块自己的业务 checkpoint。
pub trait JobSnapshotStore<E>: Send + Sync {
    /// 创建一条新的作业快照记录。
    fn create(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    /// 用新的快照内容覆盖同一作业的现有记录。
    fn update(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    /// 按作业标识读取当前快照，找不到时返回空结果。
    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<E>>>;
    /// 列出运行时启动后可以尝试恢复的快照。
    fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<E>>>;
}

#[derive(Debug, Clone, Default)]
/// 表示共享运行时默认使用的内存快照存储实现。
struct InMemoryJobSnapshotStore {
    /// 按 `JobId` 保存快照；`Mutex` 保护运行时共享访问，`Arc` 允许 host 克隆同一份存储状态。
    snapshots: Arc<Mutex<HashMap<JobId, JobSnapshot<()>>>>,
}

impl JobSnapshotStore<()> for InMemoryJobSnapshotStore {
    fn create(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(snapshot.job_id.clone(), snapshot.clone());
        Ok(())
    }

    fn update(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(snapshot.job_id.clone(), snapshot.clone());
        Ok(())
    }

    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<()>>> {
        Ok(self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(job_id)
            .cloned())
    }

    fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<()>>> {
        let resumable_states = [
            JobState::Queued,
            JobState::ClaimingLease,
            JobState::Restoring,
            JobState::Running,
        ];
        Ok(self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .values()
            .filter(|s| resumable_states.contains(&s.state))
            .cloned()
            .collect())
    }
}

#[derive(Clone)]
/// 表示共享作业运行时的宿主对象，持有队列策略和快照存储端口。
pub struct SharedJobRuntimeHost {
    /// 当前 host 使用的队列策略。
    policy: RuntimeQueuePolicy,
    /// 当前 host 读写通用作业快照的存储端口。
    snapshot_store: Arc<dyn JobSnapshotStore<()>>,
}

impl SharedJobRuntimeHost {
    /// 使用默认内存快照存储构造共享运行时 host。
    pub fn new(policy: RuntimeQueuePolicy) -> Self {
        Self::with_store(policy, Arc::new(InMemoryJobSnapshotStore::default()))
    }

    /// 使用外部提供的快照存储端口构造共享运行时 host。
    pub fn with_store(policy: RuntimeQueuePolicy, snapshot_store: Arc<dyn JobSnapshotStore<()>>) -> Self {
        Self {
            policy,
            snapshot_store,
        }
    }

    /// 返回当前 host 的队列策略快照。
    pub fn policy(&self) -> RuntimeQueuePolicy {
        self.policy
    }
}

impl Debug for SharedJobRuntimeHost {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SharedJobRuntimeHost")
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

/// 表示模块和宿主用于入队、查询和控制共享作业的运行时端口。
pub trait JobRuntime: Send + Sync {
    /// 模块附加在通用作业快照上的扩展摘要类型。
    type Extension;

    /// 接受一个模块作业入队请求，并返回稳定的入队确认。
    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob>;
    /// 按作业标识查询当前共享快照，找不到时返回空结果。
    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>>;
    /// 请求暂停指定作业。
    fn pause(&self, job_id: &JobId) -> AppResult<()>;
    /// 请求恢复指定作业。
    fn resume(&self, job_id: &JobId) -> AppResult<()>;
    /// 请求取消指定作业。
    fn cancel(&self, job_id: &JobId) -> AppResult<()>;
}

impl JobRuntime for SharedJobRuntimeHost {
    type Extension = ();

    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob> {
        let queued_at = IsoDateTime::now();
        let accepted = AcceptedJob {
            job_id: request.job_id.clone(),
            module: request.module.clone(),
            kind: request.kind.clone(),
            queued_at: queued_at.clone(),
        };
        let snapshot = JobSnapshot {
            job_id: request.job_id.clone(),
            module: request.module,
            kind: request.kind,
            state: JobState::Queued,
            ui_state: JobUiState::Queued,
            progress: JobProgress::pending(),
            recoverable: request.recoverable,
            updated_at: queued_at,
            extension: request.extension,
        };

        self.snapshot_store.create(&snapshot)?;

        Ok(accepted)
    }

    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
        self.snapshot_store.get(job_id)
    }

    fn pause(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Paused;
            snapshot.ui_state = JobUiState::Paused;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
        }

        Ok(())
    }

    fn resume(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Running;
            snapshot.ui_state = JobUiState::Running;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
        }

        Ok(())
    }

    fn cancel(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Canceled;
            snapshot.ui_state = JobUiState::Canceled;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use launcher_kernel_foundation::JobId;

    use super::{JobRuntime, RuntimeQueuePolicy, SharedJobRuntimeHost};
    use crate::{EnqueueJobRequest, JobPriority, JobState, JobUiState};

    #[test]
    fn shared_job_runtime_host_records_enqueued_snapshot() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(3));
        let job_id = JobId::generate();

        let accepted = runtime
            .enqueue(EnqueueJobRequest {
                job_id: job_id.clone(),
                module: "fab".into(),
                kind: "inventory_sync".into(),
                priority: JobPriority::Normal,
                recoverable: true,
                extension: None,
            })
            .expect("shared runtime host should accept enqueued jobs");

        let snapshot = runtime
            .snapshot(&job_id)
            .expect("shared runtime host should expose enqueued snapshots")
            .expect("shared runtime host should store the queued snapshot");

        assert_eq!(accepted.job_id, job_id);
        assert_eq!(snapshot.state, JobState::Queued);
        assert_eq!(snapshot.ui_state, JobUiState::Queued);
        assert_eq!(snapshot.module, "fab");
        assert_eq!(snapshot.kind, "inventory_sync");
    }
}
