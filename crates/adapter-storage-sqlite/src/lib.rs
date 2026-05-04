use std::path::{Path, PathBuf};
use std::sync::Mutex;

use launcher_kernel_foundation::{AppResult, AssetId, IsoDateTime, JobId, PageSlice};
use launcher_kernel_jobs::{JobProgress, JobSnapshot, JobSnapshotStore, JobState, JobUiState};
use launcher_module_fab::{
    contracts::{FabAssetDetailDto, FabInventoryListQueryDto},
    facade::{FabInventoryProjectionPage, FabInventoryProjectionRepository},
};

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

impl FabInventoryProjectionRepository for SqliteFabInventoryProjectionRepository {
    fn list_page(&self, _query: FabInventoryListQueryDto) -> AppResult<FabInventoryProjectionPage> {
        Ok(PageSlice::new(Vec::new(), None))
    }

    fn get_asset_detail_snapshot(&self, _asset_id: &AssetId) -> AppResult<Option<FabAssetDetailDto>> {
        Ok(None)
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

pub struct SqliteJobSnapshotStore {
    conn: Mutex<rusqlite::Connection>,
}

impl SqliteJobSnapshotStore {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        let conn = rusqlite::Connection::open(config.database_path())
            .expect("SqliteJobSnapshotStore: failed to open sqlite database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS job_snapshots (
                job_id       TEXT    PRIMARY KEY NOT NULL,
                module       TEXT    NOT NULL,
                kind         TEXT    NOT NULL,
                state        TEXT    NOT NULL,
                ui_state     TEXT    NOT NULL,
                progress     TEXT    NOT NULL,
                recoverable  INTEGER NOT NULL DEFAULT 1,
                updated_at   TEXT    NOT NULL,
                extension    TEXT
            );",
        )
        .expect("SqliteJobSnapshotStore: failed to create job_snapshots table");
        // Migration: add recoverable column to databases created before this column was added.
        // SQLite does not support IF NOT EXISTS for ALTER TABLE; ignore the error if the column exists.
        let _ = conn.execute(
            "ALTER TABLE job_snapshots ADD COLUMN recoverable INTEGER NOT NULL DEFAULT 1",
            [],
        );
        Self { conn: Mutex::new(conn) }
    }

    fn upsert(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        let progress_json = serde_json::to_string(&snapshot.progress)
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobProgress: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;
        let state_str = serde_json::to_string(&snapshot.state)
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobState: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;
        let ui_state_str = serde_json::to_string(&snapshot.ui_state)
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobUiState: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;
        let conn = self.conn.lock().unwrap_or_else(|p| p.into_inner());
        conn.execute(
            "INSERT OR REPLACE INTO job_snapshots
                (job_id, module, kind, state, ui_state, progress, recoverable, updated_at, extension)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL)",
            rusqlite::params![
                snapshot.job_id.to_string(),
                snapshot.module,
                snapshot.kind,
                state_str,
                ui_state_str,
                progress_json,
                i64::from(snapshot.recoverable),
                snapshot.updated_at.to_string(),
            ],
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("job_snapshots upsert failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }
}

impl JobSnapshotStore<()> for SqliteJobSnapshotStore {
    fn create(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.upsert(snapshot)
    }

    fn update(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.upsert(snapshot)
    }

    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<()>>> {
        let conn = self.conn.lock().unwrap_or_else(|p| p.into_inner());
        let mut stmt = conn.prepare(
            "SELECT job_id, module, kind, state, ui_state, progress, recoverable, updated_at
             FROM job_snapshots WHERE job_id = ?1",
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("failed to prepare job_snapshots select: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;

        let mut rows = stmt.query(rusqlite::params![job_id.to_string()])
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("job_snapshots query failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        if let Some(row) = rows.next().map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("job_snapshots row read failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))? {
            let job_id_str: String = row.get(0).unwrap();
            let module: String = row.get(1).unwrap();
            let kind: String = row.get(2).unwrap();
            let state_str: String = row.get(3).unwrap();
            let ui_state_str: String = row.get(4).unwrap();
            let progress_str: String = row.get(5).unwrap();
            let recoverable_int: i64 = row.get(6).unwrap_or(1);
            let updated_at_str: String = row.get(7).unwrap();

            let state: JobState = serde_json::from_str(&state_str)
                .map_err(|e| launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobState: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                ))?;
            let ui_state: JobUiState = serde_json::from_str(&ui_state_str)
                .map_err(|e| launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobUiState: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                ))?;
            let progress: JobProgress = serde_json::from_str(&progress_str)
                .map_err(|e| launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobProgress: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                ))?;
            let updated_at: IsoDateTime = serde_json::from_str(&format!(r#""{updated_at_str}""#))
                .map_err(|e| launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to parse updated_at: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                ))?;
            let parsed_job_id = JobId::new(job_id_str);

            Ok(Some(JobSnapshot {
                job_id: parsed_job_id,
                module,
                kind,
                state,
                ui_state,
                progress,
                recoverable: recoverable_int != 0,
                updated_at,
                extension: None,
            }))
        } else {
            Ok(None)
        }
    }

    fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<()>>> {
        let resumable_state_jsons = [
            r#""queued""#,
            r#""claiming_lease""#,
            r#""restoring""#,
            r#""running""#,
        ];
        let placeholders = resumable_state_jsons.iter().enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT job_id, module, kind, state, ui_state, progress, recoverable, updated_at
             FROM job_snapshots WHERE state IN ({placeholders})"
        );
        let conn = self.conn.lock().unwrap_or_else(|p| p.into_inner());
        let mut stmt = conn.prepare(&sql).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("list_resumable prepare failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;

        let mut results = Vec::new();
        let mut rows = stmt.query(rusqlite::params_from_iter(resumable_state_jsons.iter()))
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("list_resumable query failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        while let Some(row) = rows.next().map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("list_resumable row read failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))? {
            let job_id_str: String = row.get(0).unwrap();
            let module: String = row.get(1).unwrap();
            let kind: String = row.get(2).unwrap();
            let state_str: String = row.get(3).unwrap();
            let ui_state_str: String = row.get(4).unwrap();
            let progress_str: String = row.get(5).unwrap();
            let recoverable_int: i64 = row.get(6).unwrap_or(1);
            let updated_at_str: String = row.get(7).unwrap();

            let state: JobState = serde_json::from_str(&state_str).map_err(|e| launcher_kernel_foundation::AppError::new("SQLITE_DESERIALIZE_ERROR", format!("state: {e}"), false, launcher_kernel_foundation::AppErrorSeverity::Warning, launcher_kernel_foundation::CorrelationId::generate()))?;
            let ui_state: JobUiState = serde_json::from_str(&ui_state_str).map_err(|e| launcher_kernel_foundation::AppError::new("SQLITE_DESERIALIZE_ERROR", format!("ui_state: {e}"), false, launcher_kernel_foundation::AppErrorSeverity::Warning, launcher_kernel_foundation::CorrelationId::generate()))?;
            let progress: JobProgress = serde_json::from_str(&progress_str).map_err(|e| launcher_kernel_foundation::AppError::new("SQLITE_DESERIALIZE_ERROR", format!("progress: {e}"), false, launcher_kernel_foundation::AppErrorSeverity::Warning, launcher_kernel_foundation::CorrelationId::generate()))?;
            let updated_at: IsoDateTime = serde_json::from_str(&format!(r#""{updated_at_str}""#)).map_err(|e| launcher_kernel_foundation::AppError::new("SQLITE_DESERIALIZE_ERROR", format!("updated_at: {e}"), false, launcher_kernel_foundation::AppErrorSeverity::Warning, launcher_kernel_foundation::CorrelationId::generate()))?;

            results.push(JobSnapshot {
                job_id: JobId::new(job_id_str),
                module,
                kind,
                state,
                ui_state,
                progress,
                recoverable: recoverable_int != 0,
                updated_at,
                extension: None,
            });
        }
        Ok(results)
    }
}