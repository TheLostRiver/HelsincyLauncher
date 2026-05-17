//! Startup pipeline boundary for staged restore and optional background warmup.
//! 分阶段 restore 与可选后台 warmup 的 startup pipeline 边界。
//!
//! This module keeps startup policy explicit inside composition-root: stage 1 keeps
//! the shell/backend baseline callable, stage 2 restores resumable runtime state,
//! and stage 3 triggers optional background prewarm without hiding that work in
//! constructors or module assembly.
//! 本模块把启动策略显式收在 composition-root 内：stage 1 保持 shell/backend 基线
//! 可调用，stage 2 恢复可续跑 runtime 状态，stage 3 触发可选后台预热，同时避免把
//! 这些工作藏进构造函数或模块装配过程。

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use launcher_kernel_foundation::{AppResult, IsoDateTime};
use launcher_kernel_jobs::{
    AcceptedJob, JobDriverRegistry, JobRunDisposition, JobSnapshotStore, JobState, JobUiState,
    RestoreDisposition, SharedJobRuntimeHost,
};
use launcher_module_fab::{
    contracts::FabInventoryPrewarmRequestDto, facade::FabStartupPrewarmJobAcceptance, FabFacade,
};

/// Startup-stage port that can enqueue the Fab prewarm job without exposing concrete wiring.
/// startup 阶段使用的端口，用于排入 Fab prewarm job，同时不暴露具体接线细节。
pub trait FabStartupPrewarmPort: Send + Sync {
    /// Triggers the startup Fab prewarm path when stage 3 decides it is allowed.
    /// 在 stage 3 判定允许时触发 Fab startup prewarm 路径。
    fn run_startup_prewarm(&self, request: FabInventoryPrewarmRequestDto)
        -> AppResult<AcceptedJob>;
}

impl<P, C, M, J, K> FabStartupPrewarmPort for FabFacade<P, C, M, J, K>
where
    P: Send + Sync,
    C: Send + Sync,
    M: Send + Sync,
    J: FabStartupPrewarmJobAcceptance + Send + Sync,
    K: Send + Sync,
{
    fn run_startup_prewarm(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob> {
        FabFacade::run_startup_prewarm(self, request)
    }
}

/// Staged startup facade exposed by composition-root to the desktop host.
/// composition-root 暴露给桌面宿主的分阶段 startup facade。
#[derive(Clone)]
pub struct StartupPipelineFacade {
    enable_startup_prewarm: bool,
    fab_prewarm: Option<Arc<dyn FabStartupPrewarmPort>>,
    job_runtime: Option<SharedJobRuntimeHost>,
    snapshot_store: Option<Arc<dyn JobSnapshotStore<()>>>,
    driver_registry: Option<Arc<JobDriverRegistry<()>>>,
}

impl Debug for StartupPipelineFacade {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StartupPipelineFacade")
            .field("enable_startup_prewarm", &self.enable_startup_prewarm)
            .field("has_fab_prewarm", &self.fab_prewarm.is_some())
            .field("has_job_runtime", &self.job_runtime.is_some())
            .field("has_snapshot_store", &self.snapshot_store.is_some())
            .field("has_driver_registry", &self.driver_registry.is_some())
            .finish()
    }
}

impl Default for StartupPipelineFacade {
    /// Returns a no-op startup facade suitable for placeholder or test-only wiring.
    /// 返回适合占位或测试专用接线的 no-op startup facade。
    fn default() -> Self {
        Self::new(false, None, None, None)
    }
}

impl StartupPipelineFacade {
    /// Creates a startup facade over the already-assembled startup-stage dependencies.
    /// 基于已经装配好的 startup-stage 依赖创建 startup facade。
    pub fn new(
        enable_startup_prewarm: bool,
        fab_prewarm: Option<Arc<dyn FabStartupPrewarmPort>>,
        snapshot_store: Option<Arc<dyn JobSnapshotStore<()>>>,
        driver_registry: Option<Arc<JobDriverRegistry<()>>>,
    ) -> Self {
        Self {
            enable_startup_prewarm,
            fab_prewarm,
            job_runtime: None,
            snapshot_store,
            driver_registry,
        }
    }

    /// Adds the shared runtime handle used by the explicit one-shot execution helper.
    /// 娣诲姞鏄惧紡 one-shot 鎵ц helper 浣跨敤鐨勫叡浜?runtime 鍙ユ焺銆?
    pub fn with_runtime_execution(mut self, job_runtime: SharedJobRuntimeHost) -> Self {
        self.job_runtime = Some(job_runtime);
        self
    }

    /// Runs one queued runtime execution turn when runtime and registry wiring are present.
    /// 鍦?runtime 涓?registry 鎺ョ嚎瀛樺湪鏃讹紝鏄惧紡杩愯涓€娆℃帓闃熶綔涓氭墽琛屽洖鍚堛€?
    pub fn run_one_runtime_execution_turn(&self) -> AppResult<JobRunDisposition> {
        let Some(job_runtime) = self.job_runtime.as_ref() else {
            return Ok(JobRunDisposition::Deferred {
                reason: "runtime execution not wired".into(),
            });
        };
        let Some(driver_registry) = self.driver_registry.as_ref() else {
            return Ok(JobRunDisposition::Deferred {
                reason: "runtime driver registry not wired".into(),
            });
        };

        // Keep execution explicit; startup stages decide when, or whether, to call this helper.
        // 鎵ц淇濇寔鏄惧紡锛涘惎鍔ㄩ樁娈电敱璋冪敤鏂瑰喅瀹氭槸鍚︽垨浣曟椂璋冪敤璇?helper銆?
        job_runtime.run_next_execution_turn(driver_registry.as_ref())
    }

    /// Runs stage 1 of startup, which is currently a no-op once the shell/backend baseline exists.
    /// 运行 startup stage 1；当前在 shell/backend 基线已存在后保持 no-op。
    pub async fn run_stage1_shell_ready(&self) -> AppResult<()> {
        Ok(())
    }

    /// Runs stage 2 restore before any optional warmup work begins.
    /// 在任何可选 warmup 开始前运行 stage 2 restore。
    ///
    /// The current baseline repairs orphaned runtime state, consults registered
    /// restore drivers for queued jobs, and persists any resulting state changes
    /// back to the shared snapshot store.
    /// 当前基线会修复 orphaned runtime 状态，为 queued jobs 咨询已注册的 restore
    /// driver，并把由此产生的状态变化写回共享 snapshot store。
    pub async fn run_stage2_restore_runtime_state(&self) -> AppResult<()> {
        let orphaned_states = [
            JobState::Running,
            JobState::ClaimingLease,
            JobState::Restoring,
        ];

        if let Some(store) = self.snapshot_store.as_ref() {
            let resumable = store.list_resumable()?;
            tracing::info!(
                stage = "stage2_restore",
                resumable_count = resumable.len(),
                "stage-2 runtime state restore: found {} resumable job(s)",
                resumable.len()
            );
            for mut snapshot in resumable {
                if orphaned_states.contains(&snapshot.state) {
                    if snapshot.recoverable {
                        tracing::info!(
                            job_id = %snapshot.job_id,
                            prev_state = ?snapshot.state,
                            "stage-2: resetting orphaned recoverable job to Queued"
                        );
                        snapshot.state = JobState::Queued;
                        snapshot.ui_state = JobUiState::Queued;
                    } else {
                        tracing::info!(
                            job_id = %snapshot.job_id,
                            prev_state = ?snapshot.state,
                            "stage-2: marking orphaned non-recoverable job as Failed"
                        );
                        snapshot.state = JobState::Failed;
                        snapshot.ui_state = JobUiState::Failed;
                    }
                    snapshot.updated_at = IsoDateTime::now();
                    store.update(&snapshot)?;
                }

                // For Queued jobs, ask the registered driver whether they are
                // actually resumable (e.g. their business checkpoint still exists).
                // 对 Queued jobs，需要询问已注册 driver 是否确实可续跑，例如业务
                // checkpoint 是否仍然存在。
                if snapshot.state == JobState::Queued {
                    if let Some(registry) = self.driver_registry.as_ref() {
                        match registry.resolve(&snapshot.module, &snapshot.kind) {
                            Some(driver) => match driver.restore(&snapshot)? {
                                RestoreDisposition::Resumed => {
                                    tracing::info!(
                                        job_id = %snapshot.job_id,
                                        "stage-2: driver confirmed job resumed"
                                    );
                                }
                                RestoreDisposition::FailedAsUnrecoverable { reason } => {
                                    tracing::info!(
                                        job_id = %snapshot.job_id,
                                        %reason,
                                        "stage-2: driver marked job as unrecoverable"
                                    );
                                    snapshot.state = JobState::Failed;
                                    snapshot.ui_state = JobUiState::Failed;
                                    snapshot.updated_at = IsoDateTime::now();
                                    store.update(&snapshot)?;
                                }
                            },
                            None => {
                                tracing::warn!(
                                    job_id = %snapshot.job_id,
                                    module = %snapshot.module,
                                    kind = %snapshot.kind,
                                    "stage-2: no driver registered for this job kind, leaving as Queued"
                                );
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Runs stage 3 optional background prewarm after restore has completed.
    /// 在 restore 完成后运行 stage 3 的可选后台 prewarm。
    ///
    /// This stage must remain capability-gated and non-blocking for shell-first startup.
    /// 该阶段必须保持能力门控，并且不能阻塞 shell-first 启动。
    pub async fn run_stage3_background_prewarm(&self) -> AppResult<()> {
        if !self.enable_startup_prewarm {
            return Ok(());
        }

        if let Some(fab_prewarm) = self.fab_prewarm.as_ref() {
            let _ = fab_prewarm.run_startup_prewarm(FabInventoryPrewarmRequestDto {
                reason: "startup-stage-3".into(),
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};
    use launcher_kernel_jobs::{
        AcceptedJob, JobDriver, JobDriverRegistry, JobExecutionContext, JobProgress,
        JobRunDisposition, JobSnapshot, JobSnapshotStore, JobState, JobUiState, RestoreDisposition,
        RuntimeQueuePolicy, SharedJobRuntimeHost,
    };
    use launcher_module_fab::contracts::FabInventoryPrewarmRequestDto;

    use super::{FabStartupPrewarmPort, StartupPipelineFacade};

    // ── minimal in-test snapshot store ──────────────────────────────────────
    // 测试内最小 snapshot store：支撑 stage-2 恢复分支的纯内存断言。
    #[derive(Debug, Default, Clone)]
    struct TestSnapshotStore {
        snapshots: Arc<Mutex<HashMap<JobId, JobSnapshot<()>>>>,
    }

    impl TestSnapshotStore {
        fn get_state(&self, job_id: &JobId) -> Option<JobState> {
            self.snapshots.lock().unwrap().get(job_id).map(|s| s.state)
        }
    }

    impl JobSnapshotStore<()> for TestSnapshotStore {
        fn create(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
            self.snapshots
                .lock()
                .unwrap()
                .insert(snapshot.job_id.clone(), snapshot.clone());
            Ok(())
        }

        fn update(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
            self.snapshots
                .lock()
                .unwrap()
                .insert(snapshot.job_id.clone(), snapshot.clone());
            Ok(())
        }

        fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<()>>> {
            Ok(self.snapshots.lock().unwrap().get(job_id).cloned())
        }

        fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<()>>> {
            let resumable = [
                JobState::Queued,
                JobState::ClaimingLease,
                JobState::Restoring,
                JobState::Running,
            ];
            Ok(self
                .snapshots
                .lock()
                .unwrap()
                .values()
                .filter(|s| resumable.contains(&s.state))
                .cloned()
                .collect())
        }
    }

    fn make_running_snapshot(job_id: JobId, recoverable: bool) -> JobSnapshot<()> {
        JobSnapshot {
            job_id,
            module: "test".into(),
            kind: "test_job".into(),
            state: JobState::Running,
            ui_state: JobUiState::Running,
            progress: JobProgress::pending(),
            recoverable,
            updated_at: IsoDateTime::now(),
            extension: None,
        }
    }

    // ── orphaned lease reset tests ───────────────────────────────────────────
    // orphaned lease reset 测试：覆盖可恢复与不可恢复 Running 作业的降级路径。
    #[test]
    fn stage2_resets_orphaned_recoverable_running_job_to_queued() {
        let store = Arc::new(TestSnapshotStore::default());
        let job_id = JobId::generate();
        store
            .create(&make_running_snapshot(job_id.clone(), true))
            .unwrap();

        let facade = StartupPipelineFacade::new(false, None, Some(store.clone()), None);
        block_on_ready(facade.run_stage2_restore_runtime_state()).expect("stage-2 should succeed");

        assert_eq!(
            store.get_state(&job_id),
            Some(JobState::Queued),
            "recoverable Running job should be reset to Queued"
        );
    }

    #[test]
    fn stage2_marks_orphaned_nonrecoverable_running_job_as_failed() {
        let store = Arc::new(TestSnapshotStore::default());
        let job_id = JobId::generate();
        store
            .create(&make_running_snapshot(job_id.clone(), false))
            .unwrap();

        let facade = StartupPipelineFacade::new(false, None, Some(store.clone()), None);
        block_on_ready(facade.run_stage2_restore_runtime_state()).expect("stage-2 should succeed");

        assert_eq!(
            store.get_state(&job_id),
            Some(JobState::Failed),
            "non-recoverable Running job should become Failed"
        );
    }

    // ── driver registry test ──────────────────────────────────────────────────
    // 驱动注册表测试：验证 queued job 的 driver restore 处置。
    struct AlwaysFailDriver;
    impl JobDriver<()> for AlwaysFailDriver {
        fn module(&self) -> &'static str {
            "test"
        }
        fn kind(&self) -> &'static str {
            "test_job"
        }
        fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
            Ok(RestoreDisposition::FailedAsUnrecoverable {
                reason: "business checkpoint missing".into(),
            })
        }
    }

    #[test]
    fn stage2_driver_marks_queued_job_failed_when_checkpoint_missing() {
        let store = Arc::new(TestSnapshotStore::default());
        let job_id = JobId::generate();
        // Seed a Queued job (already in clean state, no orphan reset needed).
        // 写入一个 Queued job；它已经处于干净状态，不需要 orphan reset。
        store
            .create(&JobSnapshot {
                job_id: job_id.clone(),
                module: "test".into(),
                kind: "test_job".into(),
                state: JobState::Queued,
                ui_state: JobUiState::Queued,
                progress: JobProgress::pending(),
                recoverable: true,
                updated_at: IsoDateTime::now(),
                extension: None,
            })
            .unwrap();

        let mut registry = JobDriverRegistry::new();
        registry.register(Arc::new(AlwaysFailDriver));
        let facade =
            StartupPipelineFacade::new(false, None, Some(store.clone()), Some(Arc::new(registry)));
        block_on_ready(facade.run_stage2_restore_runtime_state()).expect("stage-2 should succeed");

        assert_eq!(
            store.get_state(&job_id),
            Some(JobState::Failed),
            "driver returning FailedAsUnrecoverable should mark job as Failed"
        );
    }

    // ── existing stage-3 tests ────────────────────────────────────────────────
    // 既有 stage-3 测试：验证能力门控后的后台 prewarm 触发行为。
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
            "test_job"
        }

        fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
            Ok(RestoreDisposition::Resumed)
        }

        fn run(&self, context: JobExecutionContext<'_, ()>) -> AppResult<JobRunDisposition> {
            self.seen_job_ids
                .lock()
                .expect("recording driver mutex should not be poisoned")
                .push(context.job_id().clone());
            Ok(JobRunDisposition::Accepted)
        }
    }

    #[test]
    fn runtime_execution_helper_defers_when_runtime_is_not_wired() {
        let facade = StartupPipelineFacade::new(false, None, None, None);

        let disposition = facade
            .run_one_runtime_execution_turn()
            .expect("missing runtime wiring should be a non-terminal execution result");

        match disposition {
            JobRunDisposition::Deferred { reason } => {
                assert!(reason.contains("runtime execution not wired"));
            }
            other => panic!("missing runtime wiring should defer execution, got {other:?}"),
        }
    }

    #[test]
    fn runtime_execution_helper_dispatches_one_queued_job_when_wired() {
        let store = Arc::new(TestSnapshotStore::default());
        let runtime = SharedJobRuntimeHost::with_store(RuntimeQueuePolicy::new(1), store.clone());
        let job_id = JobId::generate();
        store
            .create(&JobSnapshot {
                job_id: job_id.clone(),
                module: "test".into(),
                kind: "test_job".into(),
                state: JobState::Queued,
                ui_state: JobUiState::Queued,
                progress: JobProgress::pending(),
                recoverable: true,
                updated_at: IsoDateTime::now(),
                extension: None,
            })
            .expect("fixture should store a queued snapshot");
        let driver = Arc::new(RecordingRunDriver::default());
        let seen_job_ids = driver.seen_job_ids();
        let mut registry = JobDriverRegistry::new();
        registry.register(driver);
        let facade =
            StartupPipelineFacade::new(false, None, Some(store.clone()), Some(Arc::new(registry)))
                .with_runtime_execution(runtime);

        let disposition = facade
            .run_one_runtime_execution_turn()
            .expect("wired helper should run one queued execution turn");

        assert_eq!(disposition, JobRunDisposition::Accepted);
        assert_eq!(
            seen_job_ids
                .lock()
                .expect("recording driver mutex should not be poisoned")
                .as_slice(),
            &[job_id.clone()]
        );
        assert_eq!(store.get_state(&job_id), Some(JobState::Running));
    }

    #[derive(Debug, Default)]
    struct RecordingFabPrewarmPort {
        captured_requests: Mutex<Vec<FabInventoryPrewarmRequestDto>>,
    }

    impl FabStartupPrewarmPort for RecordingFabPrewarmPort {
        fn run_startup_prewarm(
            &self,
            request: FabInventoryPrewarmRequestDto,
        ) -> AppResult<AcceptedJob> {
            self.captured_requests
                .lock()
                .expect("prewarm request mutex should not be poisoned")
                .push(request);

            Ok(AcceptedJob {
                job_id: JobId::generate(),
                module: "fab".into(),
                kind: "inventory_startup_prewarm".into(),
                queued_at: IsoDateTime::now(),
            })
        }
    }

    #[test]
    fn run_stage3_background_prewarm_triggers_fab_prewarm_when_enabled() {
        let fab_prewarm = Arc::new(RecordingFabPrewarmPort::default());
        let facade = StartupPipelineFacade::new(true, Some(fab_prewarm.clone()), None, None);

        block_on_ready(facade.run_stage3_background_prewarm())
            .expect("stage-3 background prewarm should trigger the Fab prewarm path when enabled");

        assert_eq!(
            *fab_prewarm
                .captured_requests
                .lock()
                .expect("prewarm request mutex should not be poisoned"),
            vec![FabInventoryPrewarmRequestDto {
                reason: "startup-stage-3".into(),
            }]
        );
    }

    #[test]
    fn run_stage3_background_prewarm_skips_fab_prewarm_when_disabled() {
        let fab_prewarm = Arc::new(RecordingFabPrewarmPort::default());
        let facade = StartupPipelineFacade::new(false, Some(fab_prewarm.clone()), None, None);

        block_on_ready(facade.run_stage3_background_prewarm()).expect(
            "stage-3 background prewarm should stay a no-op when the capability gate is disabled",
        );

        assert!(fab_prewarm
            .captured_requests
            .lock()
            .expect("prewarm request mutex should not be poisoned")
            .is_empty());
    }

    fn block_on_ready<F>(future: F) -> F::Output
    where
        F: Future,
    {
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = pin!(future);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("startup prewarm future should be ready without a runtime"),
        }
    }

    fn noop_waker() -> Waker {
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }

    fn noop_raw_waker() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            noop_raw_waker()
        }

        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}

        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }
}
