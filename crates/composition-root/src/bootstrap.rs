//! Composition-root assembly owner for the current desktop backend baseline.
//!
//! This module is the only place that knows the concrete adapter/runtime types used
//! by the desktop host. It assembles the shared job runtime, storage/provider
//! adapters, module facades, startup pipeline, and the final facade-only service
//! aggregation exposed to `src-tauri`.

use std::path::PathBuf;
use std::sync::Arc;

use launcher_adapter_provider_fab::{
    EpicFabCatalogProviderAdapter, EpicFabCatalogProviderConfig,
};
use launcher_adapter_storage_sqlite::{
    SqliteDownloadCheckpointRepository, SqliteDownloadJobRepository,
    SqliteFabInventoryProjectionRepository, SqliteFabMediaMetadataRepository,
    SqliteFabSyncCursorRepository, SqliteJobSnapshotStore, SqliteStorageAdapterConfig,
};
use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId,
};
use launcher_kernel_jobs::{JobDriverRegistry, JobSnapshotStore, RuntimeQueuePolicy, SharedJobRuntimeHost};
use launcher_module_downloads::{
    DownloadCheckpointRepository, DownloadFacade, DownloadJobDriver, DownloadModuleDeps,
};
use launcher_module_fab::{FabFacade, FabModuleDeps, FabPrewarmJobDriver, FabSyncJobDriver};
use launcher_module_engines::{EngineFacade, EngineJobDriver, EngineModuleDeps};

use crate::startup::StartupPipelineFacade;

// Concrete desktop aliases keep the public service aggregation readable while the
// concrete adapter/runtime graph remains private to composition-root.
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
    SharedJobRuntimeHost,
>;

type DesktopEngineFacade = EngineFacade<(), (), SharedJobRuntimeHost>;

/// Wiring configuration owned by composition-root for the current desktop baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopBootstrapConfig {
    /// Root directory for durable app-owned data.
    pub app_data_dir: PathBuf,

    /// Root directory for cache data that can be rebuilt.
    pub cache_dir: PathBuf,

    /// Root directory for desktop host logs.
    pub logs_dir: PathBuf,

    /// SQLite database path used by the current storage adapters and snapshot store.
    pub sqlite_path: PathBuf,

    /// Whether Fab wiring should be exposed in the assembled service graph.
    pub enable_fab: bool,

    /// Whether downloads wiring should be exposed in the assembled service graph.
    pub enable_downloads: bool,

    /// Whether startup stage 3 should schedule Fab prewarm when supported.
    pub enable_startup_prewarm: bool,

    /// Shared runtime queue width used for current download/job scheduling.
    pub default_download_slots: u16,
}

impl DesktopBootstrapConfig {
    /// Creates a desktop bootstrap config with the current baseline feature toggles.
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
    fn default() -> Self {
        Self::new("app-data", "cache", "logs", "launcher.sqlite3")
    }
}

/// Facade-only desktop services exposed to the host after composition-root assembly.
#[derive(Clone)]
pub struct DesktopAppServices<F = DesktopFabFacade, D = DesktopDownloadFacade, E = DesktopEngineFacade> {
    /// Fab module facade wired with the current desktop storage/provider/runtime stack.
    pub fab: Arc<F>,

    /// Downloads module facade wired with the current desktop storage/runtime stack.
    pub downloads: Arc<D>,

    /// Engines module facade wired with the shared runtime host.
    pub engines: Arc<E>,

    /// Startup pipeline facade that owns staged restore/prewarm entry points.
    pub startup: Arc<StartupPipelineFacade>,

    /// Shared snapshot store projected for host/runtime inspection surfaces.
    pub snapshot_store: Arc<dyn JobSnapshotStore<()>>,
}

impl<F, D, E> DesktopAppServices<F, D, E> {
    /// Creates the facade-only desktop service aggregation returned to the host.
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
pub fn build_desktop_services(config: DesktopBootstrapConfig) -> AppResult<DesktopAppServices> {
    let sqlite_config = build_storage_config(&config)?;
    let fab_provider = build_fab_provider_adapter()?;
    let (job_runtime, snapshot_store) = build_job_runtime(&config)?;
    let download_checkpoint_repo = SqliteDownloadCheckpointRepository::new(sqlite_config.clone());

    let fab = Arc::new(build_fab_module(
        sqlite_config.clone(),
        fab_provider,
        job_runtime.clone(),
    ));
    let downloads = Arc::new(build_downloads_module(
        sqlite_config,
        download_checkpoint_repo.clone(),
        job_runtime.clone(),
    ));
    let engines = Arc::new(build_engines_module(job_runtime));
    let registry = build_job_driver_registry(Arc::new(download_checkpoint_repo));
    let snapshot_store_dyn: Arc<dyn JobSnapshotStore<()>> = snapshot_store.clone();
    let startup = Arc::new(build_startup_pipeline(&config, fab.clone(), snapshot_store, registry));

    Ok(DesktopAppServices::new(fab, downloads, engines, startup, snapshot_store_dyn))
}

// Validate storage inputs before concrete adapters and snapshot stores are created.
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
fn build_fab_provider_adapter() -> AppResult<EpicFabCatalogProviderAdapter> {
    let provider_config = EpicFabCatalogProviderConfig::new(
        "https://www.fab.com",
        "my-epic-launcher-desktop",
    );

    if provider_config.base_url().is_empty() || provider_config.client_name().is_empty() {
        return Err(invalid_builder_input(
            "build_provider_adapters",
            "Fab provider config must not be empty",
        ));
    }

    Ok(EpicFabCatalogProviderAdapter::new(provider_config))
}

// Module builders pin concrete dependencies locally while the public service surface stays facade-only.
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
) -> DesktopDownloadFacade {
    DownloadFacade::new(DownloadModuleDeps {
        job_repo: SqliteDownloadJobRepository::new(sqlite_config.clone()),
        checkpoint_repo,
        manifest_provider: (),
        staging_store: (),
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
fn build_job_runtime(config: &DesktopBootstrapConfig) -> AppResult<(SharedJobRuntimeHost, Arc<SqliteJobSnapshotStore>)> {
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
fn build_job_driver_registry(
    download_checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
) -> Arc<JobDriverRegistry<()>> {
    let mut registry = JobDriverRegistry::new();
    registry.register(Arc::new(FabPrewarmJobDriver));
    registry.register(Arc::new(FabSyncJobDriver));
    registry.register(Arc::new(DownloadJobDriver::new(download_checkpoint_repo)));
    registry.register(Arc::new(EngineJobDriver));
    Arc::new(registry)
}

// Startup only depends on assembled facades/runtime surfaces, not concrete repository types.
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
fn invalid_builder_input(builder: &str, detail: &str) -> AppError {
    AppError::new(
        "COMPOSITION_ROOT_INVALID_CONFIG",
        format!("{builder} failed: {detail}"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}