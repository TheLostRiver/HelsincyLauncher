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

/// Disposition returned by one module driver execution turn.
/// 模块 driver 执行一个运行回合后返回的处置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobRunDisposition {
    /// The driver accepted the execution turn without claiming terminal completion.
    /// driver 已接受执行回合，但不声明终态完成。
    Accepted,
    /// The driver has no concrete execution path yet; this is not terminal failure.
    /// driver 尚未接入具体执行路径；这不是终态失败。
    Deferred { reason: String },
    /// The driver rejected or failed this execution turn without projecting terminal state.
    /// driver 拒绝或执行失败该回合，但不直接投影终态状态。
    Failed { reason: String },
}

/// Read-only context passed from the shared runtime into a module execution turn.
/// 共享 runtime 传给模块执行回合的只读上下文。
#[derive(Debug, Clone, Copy)]
pub struct JobExecutionContext<'a, E> {
    /// Runtime-owned snapshot for the job being executed.
    /// runtime 拥有的当前执行作业快照。
    snapshot: &'a JobSnapshot<E>,
}

impl<'a, E> JobExecutionContext<'a, E> {
    /// Creates a context for one execution turn from a runtime snapshot.
    /// 从 runtime 快照创建一次执行回合的上下文。
    pub fn new(snapshot: &'a JobSnapshot<E>) -> Self {
        Self { snapshot }
    }

    /// Returns the runtime snapshot without giving the driver mutation rights.
    /// 返回 runtime 快照，但不授予 driver 修改权限。
    pub fn snapshot(&self) -> &'a JobSnapshot<E> {
        self.snapshot
    }

    /// Returns the stable job id for this execution turn.
    /// 返回该执行回合的稳定作业标识。
    pub fn job_id(&self) -> &'a JobId {
        &self.snapshot.job_id
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
    /// Runs one module execution turn through a runtime-owned read-only context.
    /// 通过 runtime 拥有的只读上下文执行一个模块回合。
    fn run(&self, context: JobExecutionContext<'_, E>) -> AppResult<JobRunDisposition> {
        Ok(JobRunDisposition::Deferred {
            reason: format!(
                "execution not wired for {}/{}",
                context.snapshot().module,
                context.snapshot().kind
            ),
        })
    }
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
    /// Holds the runtime queue policy shared by cloned host handles.
    /// 持有由克隆 host 句柄共享的运行时队列策略。
    policy: Arc<Mutex<RuntimeQueuePolicy>>,
    /// 当前 host 读写通用作业快照的存储端口。
    snapshot_store: Arc<dyn JobSnapshotStore<()>>,
}

impl SharedJobRuntimeHost {
    /// 使用默认内存快照存储构造共享运行时 host。
    pub fn new(policy: RuntimeQueuePolicy) -> Self {
        Self::with_store(policy, Arc::new(InMemoryJobSnapshotStore::default()))
    }

    /// 使用外部提供的快照存储端口构造共享运行时 host。
    pub fn with_store(
        policy: RuntimeQueuePolicy,
        snapshot_store: Arc<dyn JobSnapshotStore<()>>,
    ) -> Self {
        Self {
            policy: Arc::new(Mutex::new(policy)),
            snapshot_store,
        }
    }

    /// Returns the current queue policy as a by-value snapshot.
    /// 以按值快照形式返回当前 host 的队列策略。
    pub fn policy(&self) -> RuntimeQueuePolicy {
        *self
            .policy
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Replaces the runtime queue-policy snapshot for future scheduling reads.
    /// 替换 runtime 队列策略快照，供后续调度读取。
    pub fn update_policy(&self, policy: RuntimeQueuePolicy) {
        *self
            .policy
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = policy;
    }

    /// Dispatches exactly one execution turn for a stored snapshot through the matching driver.
    /// 通过匹配的 driver 为已存储快照调度一次执行回合。
    pub fn run_one_execution_turn(
        &self,
        job_id: &JobId,
        registry: &JobDriverRegistry<()>,
    ) -> AppResult<JobRunDisposition> {
        let Some(snapshot) = self.snapshot_store.get(job_id)? else {
            return Ok(JobRunDisposition::Deferred {
                reason: format!("snapshot missing for job {job_id}"),
            });
        };

        let Some(driver) = registry.resolve(&snapshot.module, &snapshot.kind) else {
            return Ok(JobRunDisposition::Deferred {
                reason: format!(
                    "driver not registered for {}/{}",
                    snapshot.module, snapshot.kind
                ),
            });
        };

        driver.run(JobExecutionContext::new(&snapshot))
    }
}

impl Debug for SharedJobRuntimeHost {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SharedJobRuntimeHost")
            .field("policy", &self.policy())
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
    use std::sync::{Arc, Mutex};

    use launcher_kernel_foundation::{AppResult, JobId};

    use super::{
        JobDriver, JobExecutionContext, JobRunDisposition, JobRuntime, RuntimeQueuePolicy,
        SharedJobRuntimeHost,
    };
    use crate::{
        EnqueueJobRequest, JobPriority, JobProgress, JobSnapshot, JobState, JobUiState,
        RestoreDisposition,
    };

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

    #[test]
    fn shared_job_runtime_host_updates_policy_for_cloned_handles() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(3));
        let cloned_runtime = runtime.clone();

        runtime.update_policy(RuntimeQueuePolicy::new(9));

        assert_eq!(runtime.policy().max_concurrent_jobs, 9);
        assert_eq!(cloned_runtime.policy().max_concurrent_jobs, 9);
    }

    struct RestoreOnlyDriver;

    impl JobDriver<()> for RestoreOnlyDriver {
        fn module(&self) -> &'static str {
            "test"
        }

        fn kind(&self) -> &'static str {
            "restore_only"
        }

        fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
            Ok(RestoreDisposition::Resumed)
        }
    }

    #[test]
    fn job_driver_default_run_is_deferred_until_module_execution_is_wired() {
        let snapshot = test_snapshot("test", "restore_only");

        let disposition = RestoreOnlyDriver
            .run(JobExecutionContext::new(&snapshot))
            .expect("default run disposition should be explicit");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("execution not wired"));
            }
            other => panic!("default run should be deferred, got {other:?}"),
        }
    }

    #[derive(Default)]
    struct RecordingRunDriver {
        seen_job_ids: Arc<Mutex<Vec<JobId>>>,
    }

    impl RecordingRunDriver {
        fn seen_job_ids(&self) -> Arc<Mutex<Vec<JobId>>> {
            self.seen_job_ids.clone()
        }
    }

    impl JobDriver<()> for RecordingRunDriver {
        fn module(&self) -> &'static str {
            "test"
        }

        fn kind(&self) -> &'static str {
            "run"
        }

        fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
            Ok(RestoreDisposition::Resumed)
        }

        fn run(&self, context: JobExecutionContext<'_, ()>) -> AppResult<JobRunDisposition> {
            self.seen_job_ids
                .lock()
                .expect("recording driver mutex should not be poisoned")
                .push(context.job_id().clone());
            assert_eq!(context.snapshot().module, "test");
            assert_eq!(context.snapshot().kind, "run");
            Ok(JobRunDisposition::Accepted)
        }
    }

    #[test]
    fn registry_resolved_driver_can_accept_execution_turn_with_read_only_context() {
        let driver = Arc::new(RecordingRunDriver::default());
        let seen_job_ids = driver.seen_job_ids();
        let mut registry = super::JobDriverRegistry::new();
        registry.register(driver);
        let snapshot = test_snapshot("test", "run");
        let expected_job_id = snapshot.job_id.clone();

        let disposition = registry
            .resolve("test", "run")
            .expect("registered driver should resolve by module and kind")
            .run(JobExecutionContext::new(&snapshot))
            .expect("fake driver should accept the execution turn");

        assert_eq!(disposition, JobRunDisposition::Accepted);
        assert_eq!(
            seen_job_ids
                .lock()
                .expect("recording driver mutex should not be poisoned")
                .as_slice(),
            &[expected_job_id]
        );
    }

    #[test]
    fn execution_dispatch_calls_registered_driver_once_for_enqueued_snapshot() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(1));
        let job_id = JobId::generate();
        runtime
            .enqueue(EnqueueJobRequest {
                job_id: job_id.clone(),
                module: "test".into(),
                kind: "run".into(),
                priority: JobPriority::Normal,
                recoverable: true,
                extension: None,
            })
            .expect("dispatch fixture should enqueue a queued snapshot");

        let driver = Arc::new(RecordingRunDriver::default());
        let seen_job_ids = driver.seen_job_ids();
        let mut registry = super::JobDriverRegistry::new();
        registry.register(driver);

        let disposition = runtime
            .run_one_execution_turn(&job_id, &registry)
            .expect("one-shot dispatch should call the registered driver");

        assert_eq!(disposition, JobRunDisposition::Accepted);
        assert_eq!(
            seen_job_ids
                .lock()
                .expect("recording driver mutex should not be poisoned")
                .as_slice(),
            &[job_id.clone()]
        );
        assert_eq!(
            runtime
                .snapshot(&job_id)
                .expect("snapshot query should succeed")
                .expect("queued snapshot should still exist")
                .state,
            JobState::Queued,
            "one-shot dispatch must not mutate lifecycle state yet"
        );
    }

    #[test]
    fn execution_dispatch_defers_when_snapshot_is_missing() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(1));
        let registry = super::JobDriverRegistry::new();
        let missing_job_id = JobId::generate();

        let disposition = runtime
            .run_one_execution_turn(&missing_job_id, &registry)
            .expect("missing snapshot should be a non-terminal dispatch result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("snapshot missing"));
            }
            other => panic!("missing snapshot should defer dispatch, got {other:?}"),
        }
    }

    #[test]
    fn execution_dispatch_defers_when_driver_is_missing() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(1));
        let job_id = JobId::generate();
        runtime
            .enqueue(EnqueueJobRequest {
                job_id: job_id.clone(),
                module: "missing".into(),
                kind: "driver".into(),
                priority: JobPriority::Normal,
                recoverable: true,
                extension: None,
            })
            .expect("dispatch fixture should enqueue a queued snapshot");
        let registry = super::JobDriverRegistry::new();

        let disposition = runtime
            .run_one_execution_turn(&job_id, &registry)
            .expect("missing driver should be a non-terminal dispatch result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("driver not registered"));
            }
            other => panic!("missing driver should defer dispatch, got {other:?}"),
        }
    }

    fn test_snapshot(module: &str, kind: &str) -> JobSnapshot<()> {
        JobSnapshot {
            job_id: JobId::generate(),
            module: module.into(),
            kind: kind.into(),
            state: JobState::Queued,
            ui_state: JobUiState::Queued,
            progress: JobProgress::pending(),
            recoverable: true,
            updated_at: launcher_kernel_foundation::IsoDateTime::now(),
            extension: None,
        }
    }
}
