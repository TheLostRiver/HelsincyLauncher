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
use launcher_kernel_jobs::{RuntimeQueuePolicy, SharedJobRuntimeHost};
use launcher_module_downloads::{DownloadFacade, DownloadModuleDeps};
use launcher_module_fab::{FabFacade, FabModuleDeps};

use crate::startup::StartupPipelineFacade;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopBootstrapConfig {
    pub app_data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub sqlite_path: PathBuf,
    pub enable_fab: bool,
    pub enable_downloads: bool,
    pub enable_startup_prewarm: bool,
    pub default_download_slots: u16,
}

impl DesktopBootstrapConfig {
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
    fn default() -> Self {
        Self::new("app-data", "cache", "logs", "launcher.sqlite3")
    }
}

#[derive(Clone)]
pub struct DesktopAppServices<F = DesktopFabFacade, D = DesktopDownloadFacade> {
    pub fab: Arc<F>,
    pub downloads: Arc<D>,
    pub startup: Arc<StartupPipelineFacade>,
}

impl<F, D> DesktopAppServices<F, D> {
    pub fn new(fab: Arc<F>, downloads: Arc<D>, startup: Arc<StartupPipelineFacade>) -> Self {
        Self {
            fab,
            downloads,
            startup,
        }
    }
}

pub fn build_desktop_services(config: DesktopBootstrapConfig) -> AppResult<DesktopAppServices> {
    let sqlite_config = build_storage_config(&config)?;
    let fab_provider = build_fab_provider_adapter()?;
    let job_runtime = build_job_runtime(&config)?;

    let fab = Arc::new(build_fab_module(
        sqlite_config.clone(),
        fab_provider,
        job_runtime.clone(),
    ));
    let downloads = Arc::new(build_downloads_module(sqlite_config, job_runtime));
    let startup = Arc::new(build_startup_pipeline(&config, fab.clone()));

    Ok(DesktopAppServices::new(fab, downloads, startup))
}

fn build_storage_config(config: &DesktopBootstrapConfig) -> AppResult<SqliteStorageAdapterConfig> {
    if config.sqlite_path.as_os_str().is_empty() {
        return Err(invalid_builder_input(
            "build_storage_adapters",
            "sqlite_path must not be empty",
        ));
    }

    Ok(SqliteStorageAdapterConfig::new(config.sqlite_path.clone()))
}

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
    job_runtime: SharedJobRuntimeHost,
) -> DesktopDownloadFacade {
    DownloadFacade::new(DownloadModuleDeps {
        job_repo: SqliteDownloadJobRepository::new(sqlite_config.clone()),
        checkpoint_repo: SqliteDownloadCheckpointRepository::new(sqlite_config),
        manifest_provider: (),
        staging_store: (),
        job_runtime,
    })
}

fn build_job_runtime(config: &DesktopBootstrapConfig) -> AppResult<SharedJobRuntimeHost> {
    if config.default_download_slots == 0 {
        return Err(invalid_builder_input(
            "build_job_runtime",
            "default_download_slots must not be zero",
        ));
    }

    let store = Arc::new(SqliteJobSnapshotStore::new(
        SqliteStorageAdapterConfig::new(config.sqlite_path.clone()),
    ));
    Ok(SharedJobRuntimeHost::with_store(
        RuntimeQueuePolicy::new(usize::from(config.default_download_slots)),
        store,
    ))
}

fn build_startup_pipeline(
    config: &DesktopBootstrapConfig,
    fab: Arc<DesktopFabFacade>,
) -> StartupPipelineFacade {
    StartupPipelineFacade::new(config.enable_fab && config.enable_startup_prewarm, Some(fab))
}

fn invalid_builder_input(builder: &str, detail: &str) -> AppError {
    AppError::new(
        "COMPOSITION_ROOT_INVALID_CONFIG",
        format!("{builder} failed: {detail}"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}