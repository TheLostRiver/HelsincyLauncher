//! Composition-root assembly owner for the current desktop backend baseline.
//! 当前桌面后端基线的 composition-root 装配 owner。
//!
//! This module is the only place that knows the concrete adapter/runtime types used
//! by the desktop host. It assembles the shared job runtime, storage/provider
//! adapters, module facades, startup pipeline, and the final facade-only service
//! aggregation exposed to `src-tauri`.
//! 本模块是桌面宿主唯一了解具体 adapter/runtime 类型的地方；它负责组装共享任务
//! runtime、storage/provider adapter、模块 facade、启动管线，以及最终暴露给
//! `src-tauri` 的 facade-only 服务聚合。

use std::path::PathBuf;
use std::sync::Arc;

use launcher_adapter_provider_fab::{EpicFabCatalogProviderAdapter, EpicFabCatalogProviderConfig};
use launcher_adapter_storage_sqlite::{
    SqliteDownloadCheckpointRepository, SqliteDownloadJobRepository,
    SqliteFabInventoryProjectionRepository, SqliteFabMediaMetadataRepository,
    SqliteFabSyncCursorRepository, SqliteJobSnapshotStore, SqliteStorageAdapterConfig,
};
use launcher_kernel_foundation::{AppError, AppErrorSeverity, AppResult, CorrelationId};
use launcher_kernel_jobs::{
    JobDriverRegistry, JobSnapshotStore, RuntimeQueuePolicy, SharedJobRuntimeHost,
};
use launcher_module_downloads::{
    DownloadCheckpointRepository, DownloadFacade, DownloadJobDriver, DownloadModuleDeps,
    DownloadPendingResumeWorkSource, InMemoryDownloadResumeWorkScheduler,
};
use launcher_module_engines::{EngineFacade, EngineJobDriver, EngineModuleDeps};
use launcher_module_fab::{FabFacade, FabModuleDeps, FabPrewarmJobDriver, FabSyncJobDriver};

use crate::startup::StartupPipelineFacade;

// Concrete desktop aliases keep the public service aggregation readable while the
// concrete adapter/runtime graph remains private to composition-root.
// 这些具体桌面别名让公开服务聚合保持可读，同时把真实 adapter/runtime 图继续收在
// composition-root 内部。
type DesktopFabFacade = FabFacade<
    SqliteFabInventoryProjectionRepository,
    SqliteFabSyncCursorRepository,
    SqliteFabMediaMetadataRepository,
    SharedJobRuntimeHost,
    EpicFabCatalogProviderAdapter,
>;

type DesktopDownloadFacade = DownloadFacade<
    SqliteDownloadJobRepository,
    SqliteDownloadCheckpointRepository,
    (),
    (),
    InMemoryDownloadResumeWorkScheduler,
    SharedJobRuntimeHost,
>;

type DesktopEngineFacade = EngineFacade<(), (), SharedJobRuntimeHost>;

/// Wiring configuration owned by composition-root for the current desktop baseline.
/// 当前桌面基线中由 composition-root 持有的接线配置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopBootstrapConfig {
    /// Root directory for durable app-owned data.
    /// 应用自有持久化数据的根目录。
    pub app_data_dir: PathBuf,

    /// Root directory for cache data that can be rebuilt.
    /// 可重建缓存数据的根目录。
    pub cache_dir: PathBuf,

    /// Root directory for desktop host logs.
    /// 桌面宿主日志文件的根目录。
    pub logs_dir: PathBuf,

    /// SQLite database path used by the current storage adapters and snapshot store.
    /// 当前 storage adapter 与 snapshot store 共用的 SQLite 数据库路径。
    pub sqlite_path: PathBuf,

    /// Whether Fab wiring should be exposed in the assembled service graph.
    /// 控制装配后的服务图是否暴露 Fab 接线。
    pub enable_fab: bool,

    /// Whether downloads wiring should be exposed in the assembled service graph.
    /// 控制装配后的服务图是否暴露 downloads 接线。
    pub enable_downloads: bool,

    /// Whether startup stage 3 should schedule Fab prewarm when supported.
    /// 控制 startup stage 3 在能力可用时是否调度 Fab prewarm。
    pub enable_startup_prewarm: bool,

    /// Shared runtime queue width used for current download/job scheduling.
    /// 当前 download/job 调度使用的共享 runtime 队列宽度。
    pub default_download_slots: u16,
}

impl DesktopBootstrapConfig {
    /// Creates a desktop bootstrap config with the current baseline feature toggles.
    /// 使用当前基线功能开关创建桌面 bootstrap 配置。
    pub fn new(
        app_data_dir: impl Into<PathBuf>,
        cache_dir: impl Into<PathBuf>,
        logs_dir: impl Into<PathBuf>,
        sqlite_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            app_data_dir: app_data_dir.into(),
            cache_dir: cache_dir.into(),
            logs_dir: logs_dir.into(),
            sqlite_path: sqlite_path.into(),
            enable_fab: true,
            enable_downloads: true,
            enable_startup_prewarm: true,
            default_download_slots: 3,
        }
    }
}

impl Default for DesktopBootstrapConfig {
    /// Returns the local baseline bootstrap config used by smoke tests and the host shell.
    /// 返回 smoke test 与宿主 shell 使用的本地基线 bootstrap 配置。
    fn default() -> Self {
        Self::new("app-data", "cache", "logs", "launcher.sqlite3")
    }
}

/// Facade-only desktop services exposed to the host after composition-root assembly.
/// composition-root 装配后暴露给宿主的 facade-only 桌面服务集合。
#[derive(Clone)]
pub struct DesktopAppServices<
    F = DesktopFabFacade,
    D = DesktopDownloadFacade,
    E = DesktopEngineFacade,
> {
    /// Fab module facade wired with the current desktop storage/provider/runtime stack.
    /// 使用当前桌面 storage/provider/runtime 栈接线的 Fab 模块 facade。
    pub fab: Arc<F>,

    /// Downloads module facade wired with the current desktop storage/runtime stack.
    /// 使用当前桌面 storage/runtime 栈接线的 Downloads 模块 facade。
    pub downloads: Arc<D>,

    /// Engines module facade wired with the shared runtime host.
    /// 使用共享 runtime host 接线的 Engines 模块 facade。
    pub engines: Arc<E>,

    /// Startup pipeline facade that owns staged restore/prewarm entry points.
    /// 持有分阶段 restore/prewarm 入口的启动管线 facade。
    pub startup: Arc<StartupPipelineFacade>,

    /// Shared snapshot store projected for host/runtime inspection surfaces.
    /// 投射给宿主/runtime 检视面的共享 snapshot store。
    pub snapshot_store: Arc<dyn JobSnapshotStore<()>>,
}

impl<F, D, E> DesktopAppServices<F, D, E> {
    /// Creates the facade-only desktop service aggregation returned to the host.
    /// 创建返回给宿主的 facade-only 桌面服务聚合。
    pub fn new(
        fab: Arc<F>,
        downloads: Arc<D>,
        engines: Arc<E>,
        startup: Arc<StartupPipelineFacade>,
        snapshot_store: Arc<dyn JobSnapshotStore<()>>,
    ) -> Self {
        Self {
            fab,
            downloads,
            engines,
            startup,
            snapshot_store,
        }
    }
}

/// Assembles the current desktop service graph without starting background work.
/// 装配当前桌面服务图，但不在构建过程中启动后台任务。
pub fn build_desktop_services(config: DesktopBootstrapConfig) -> AppResult<DesktopAppServices> {
    let sqlite_config = build_storage_config(&config)?;
    let fab_provider = build_fab_provider_adapter()?;
    let (job_runtime, snapshot_store) = build_job_runtime(&config)?;
    let download_checkpoint_repo = SqliteDownloadCheckpointRepository::new(sqlite_config.clone());
    let download_resume_scheduler = InMemoryDownloadResumeWorkScheduler::new();

    let fab = Arc::new(build_fab_module(
        sqlite_config.clone(),
        fab_provider,
        job_runtime.clone(),
    ));
    let downloads = Arc::new(build_downloads_module(
        sqlite_config,
        download_checkpoint_repo.clone(),
        job_runtime.clone(),
        download_resume_scheduler.clone(),
    ));
    let engines = Arc::new(build_engines_module(job_runtime));
    let registry = build_job_driver_registry(
        Arc::new(download_checkpoint_repo),
        Arc::new(download_resume_scheduler),
    );
    let snapshot_store_dyn: Arc<dyn JobSnapshotStore<()>> = snapshot_store.clone();
    let startup = Arc::new(build_startup_pipeline(
        &config,
        fab.clone(),
        snapshot_store,
        registry,
    ));

    Ok(DesktopAppServices::new(
        fab,
        downloads,
        engines,
        startup,
        snapshot_store_dyn,
    ))
}

// Validate storage inputs before concrete adapters and snapshot stores are created.
// 在创建具体 adapter 与 snapshot store 前，先校验 storage 输入。
fn build_storage_config(config: &DesktopBootstrapConfig) -> AppResult<SqliteStorageAdapterConfig> {
    if config.sqlite_path.as_os_str().is_empty() {
        return Err(invalid_builder_input(
            "build_storage_adapters",
            "sqlite_path must not be empty",
        ));
    }

    Ok(SqliteStorageAdapterConfig::new(config.sqlite_path.clone()))
}

// Provider wiring stays centralized here so host/transport layers never see adapter details.
// Provider 接线集中在这里，确保 host/transport 层永远不接触 adapter 细节。
fn build_fab_provider_adapter() -> AppResult<EpicFabCatalogProviderAdapter> {
    let provider_config =
        EpicFabCatalogProviderConfig::new("https://www.fab.com", "my-epic-launcher-desktop");

    if provider_config.base_url().is_empty() || provider_config.client_name().is_empty() {
        return Err(invalid_builder_input(
            "build_provider_adapters",
            "Fab provider config must not be empty",
        ));
    }

    Ok(EpicFabCatalogProviderAdapter::new(provider_config))
}

// Module builders pin concrete dependencies locally while the public service surface stays facade-only.
// 模块 builder 在本地固定具体依赖，同时让公开服务面保持 facade-only。
fn build_fab_module(
    sqlite_config: SqliteStorageAdapterConfig,
    fab_provider: EpicFabCatalogProviderAdapter,
    job_runtime: SharedJobRuntimeHost,
) -> DesktopFabFacade {
    FabFacade::new(FabModuleDeps {
        projection_repo: SqliteFabInventoryProjectionRepository::new(sqlite_config.clone()),
        cursor_repo: SqliteFabSyncCursorRepository::new(sqlite_config.clone()),
        media_repo: SqliteFabMediaMetadataRepository::new(sqlite_config),
        job_runtime,
        catalog_provider: fab_provider,
    })
}

fn build_downloads_module(
    sqlite_config: SqliteStorageAdapterConfig,
    checkpoint_repo: SqliteDownloadCheckpointRepository,
    job_runtime: SharedJobRuntimeHost,
    resume_scheduler: InMemoryDownloadResumeWorkScheduler,
) -> DesktopDownloadFacade {
    DownloadFacade::new(DownloadModuleDeps {
        job_repo: SqliteDownloadJobRepository::new(sqlite_config.clone()),
        checkpoint_repo,
        manifest_provider: (),
        staging_store: (),
        resume_scheduler,
        job_runtime,
    })
}

fn build_engines_module(job_runtime: SharedJobRuntimeHost) -> DesktopEngineFacade {
    EngineFacade::new(EngineModuleDeps {
        job_repo: (),
        checkpoint_repo: (),
        job_runtime,
    })
}

// The shared runtime host is assembled once and then fanned out to all queued-job modules.
// 共享 runtime host 只装配一次，然后分发给所有 queued-job 模块。
fn build_job_runtime(
    config: &DesktopBootstrapConfig,
) -> AppResult<(SharedJobRuntimeHost, Arc<SqliteJobSnapshotStore>)> {
    if config.default_download_slots == 0 {
        return Err(invalid_builder_input(
            "build_job_runtime",
            "default_download_slots must not be zero",
        ));
    }

    let store = Arc::new(SqliteJobSnapshotStore::new(
        SqliteStorageAdapterConfig::new(config.sqlite_path.clone()),
    ));
    let runtime = SharedJobRuntimeHost::with_store(
        RuntimeQueuePolicy::new(usize::from(config.default_download_slots)),
        store.clone(),
    );
    Ok((runtime, store))
}

// Restore drivers are registered centrally so startup stage 2 can rehydrate queued jobs.
// Restore driver 在这里集中注册，让 startup stage 2 能恢复 queued jobs。
fn build_job_driver_registry(
    download_checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
    download_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
) -> Arc<JobDriverRegistry<()>> {
    let mut registry = JobDriverRegistry::new();
    registry.register(Arc::new(FabPrewarmJobDriver));
    registry.register(Arc::new(FabSyncJobDriver));
    registry.register(Arc::new(build_download_job_driver(
        download_checkpoint_repo,
        download_resume_work_source,
    )));
    registry.register(Arc::new(EngineJobDriver));
    Arc::new(registry)
}

// Downloads driver construction is kept private so public services stay facade-only.
// Downloads driver 构造保持私有，确保公开服务面仍然只暴露 facade。
fn build_download_job_driver(
    download_checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
    download_resume_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
) -> DownloadJobDriver {
    DownloadJobDriver::with_pending_resume_work_source(
        download_checkpoint_repo,
        download_resume_work_source,
    )
}

// Startup only depends on assembled facades/runtime surfaces, not concrete repository types.
// Startup 只依赖已装配的 facade/runtime 表面，不依赖具体 repository 类型。
fn build_startup_pipeline(
    config: &DesktopBootstrapConfig,
    fab: Arc<DesktopFabFacade>,
    snapshot_store: Arc<SqliteJobSnapshotStore>,
    driver_registry: Arc<JobDriverRegistry<()>>,
) -> StartupPipelineFacade {
    StartupPipelineFacade::new(
        config.enable_fab && config.enable_startup_prewarm,
        Some(fab),
        Some(snapshot_store),
        Some(driver_registry),
    )
}

// Builder validation failures are normalized into one composition-root owned error shape.
// Builder 校验失败会统一归一化为 composition-root 持有的错误形状。
fn invalid_builder_input(builder: &str, detail: &str) -> AppError {
    AppError::new(
        "COMPOSITION_ROOT_INVALID_CONFIG",
        format!("{builder} failed: {detail}"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use launcher_kernel_foundation::JobId;
    use launcher_kernel_jobs::RuntimeQueuePolicy;
    use launcher_module_downloads::{
        DownloadResumeWorkItem, DownloadResumeWorkMode, DownloadResumeWorkPlan,
        DownloadResumeWorkScheduler,
    };

    #[test]
    fn download_driver_drains_work_scheduled_through_shared_facade_scheduler() {
        let tmp_path = std::env::temp_dir().join("at186_downloads_shared_scheduler_wiring.sqlite3");
        let _ = std::fs::remove_file(&tmp_path);
        let sqlite_config = SqliteStorageAdapterConfig::new(tmp_path.clone());
        let checkpoint_repo = SqliteDownloadCheckpointRepository::new(sqlite_config.clone());
        let job_runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(1));
        // Use one scheduler handle for both facade preparation and driver source wiring.
        // 使用同一个 scheduler 句柄同时接入 facade 准备路径与 driver source 接线。
        let scheduler = InMemoryDownloadResumeWorkScheduler::new();
        let downloads = build_downloads_module(
            sqlite_config,
            checkpoint_repo.clone(),
            job_runtime,
            scheduler.clone(),
        );
        let driver = build_download_job_driver(Arc::new(checkpoint_repo), Arc::new(scheduler));
        let job_id = JobId::generate();
        let plan = make_resume_work_plan("segment-shared");

        downloads
            .deps()
            .resume_scheduler
            .schedule_resume_work(&job_id, &plan)
            .expect("facade scheduler should register pending resume work");

        // The driver must drain the exact work registered through the facade dependency graph.
        // driver 必须取走通过 facade 依赖图登记的同一份 pending work。
        let drained = driver
            .drain_pending_resume_work(&job_id)
            .expect("driver should drain pending work from the shared scheduler source");

        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].job_id, job_id);
        assert_eq!(drained[0].plan, plan);
        assert!(
            downloads.deps().resume_scheduler.pending_work().is_empty(),
            "driver drain should consume the work registered through the facade scheduler"
        );

        let _ = std::fs::remove_file(&tmp_path);
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
}
