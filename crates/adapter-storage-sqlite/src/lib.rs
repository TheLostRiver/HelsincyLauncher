use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteStorageAdapterConfig {
    database_path: PathBuf,
}

impl SqliteStorageAdapterConfig {
    pub fn new(database_path: impl Into<PathBuf>) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }
}

#[derive(Debug, Clone)]
pub struct SqliteFabInventoryProjectionRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabInventoryProjectionRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct SqliteFabSyncCursorRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabSyncCursorRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct SqliteFabMediaMetadataRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabMediaMetadataRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDownloadJobRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteDownloadJobRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDownloadCheckpointRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteDownloadCheckpointRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}