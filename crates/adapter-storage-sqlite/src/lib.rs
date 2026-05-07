//! SQLite storage adapter crate 的共享入口与仓储外壳集合。
//!
//! 这个 crate 负责把结构化本地事实落到 SQLite，并为 Fab、downloads 和 job
//! runtime 暴露各自的适配器外壳。当前这一刀只补文件头和共享配置声明注释，
//! 不改变既有 `rusqlite` 访问方式、schema 初始化或仓储行为。

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use launcher_kernel_foundation::{AppResult, AssetId, IsoDateTime, JobId, PageSlice};
use launcher_kernel_jobs::{JobPriority, JobProgress, JobSnapshot, JobSnapshotStore, JobState, JobUiState};
use launcher_module_downloads::{
    DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadJobRecord,
    DownloadJobRecordState, DownloadJobRepository,
};
use launcher_module_fab::{
    contracts::{FabAssetDetailDto, FabInventoryListQueryDto},
    facade::{FabInventoryProjectionPage, FabInventoryProjectionRepository},
};

/// 组装各个 SQLite 仓储适配器时共享的最小配置快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteStorageAdapterConfig {
    database_path: PathBuf,
}

impl SqliteStorageAdapterConfig {
    /// 用数据库文件路径创建 SQLite adapter 配置。
    pub fn new(database_path: impl Into<PathBuf>) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }

    /// 返回 SQLite 数据库文件的本地路径。
    pub fn database_path(&self) -> &Path {
        &self.database_path
    }
}

/// 基于 SQLite projection 的 Fab 库存读模型仓储外壳。
#[derive(Debug, Clone)]
pub struct SqliteFabInventoryProjectionRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabInventoryProjectionRepository {
    /// 用共享 SQLite 配置创建 Fab 库存 projection 仓储。
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    /// 暴露只读配置快照，供装配和诊断路径检查数据库绑定结果。
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

/// 基于 SQLite 游标表的 Fab 同步进度仓储外壳。
#[derive(Debug, Clone)]
pub struct SqliteFabSyncCursorRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabSyncCursorRepository {
    /// 用共享 SQLite 配置创建 Fab 同步游标仓储。
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    /// 暴露只读配置快照，供装配和诊断路径确认游标仓储绑定结果。
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
        let repo = Self { config };
        repo.ensure_table()
            .expect("SqliteDownloadJobRepository: failed to create jobs table");
        repo
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }

    fn ensure_table(&self) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS download_jobs (
                job_id TEXT PRIMARY KEY NOT NULL,
                target_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                install_intent TEXT NULL,
                priority TEXT NOT NULL,
                state TEXT NOT NULL
            );",
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("download_jobs table init failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }

    fn open_connection(&self) -> AppResult<rusqlite::Connection> {
        rusqlite::Connection::open(self.config.database_path()).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_OPEN_ERROR",
                format!("failed to open sqlite database: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })
    }

    pub fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(
                "SELECT target_id, kind, install_intent, priority, state
                 FROM download_jobs
                 WHERE job_id = ?1",
            )
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("failed to prepare download job select: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        let mut rows = stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job query failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        let maybe_row = rows.next().map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job row read failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;

        let Some(row) = maybe_row else {
            return Ok(None);
        };

        let target_id: String = row.get(0).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job target_id decode failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        let kind: String = row.get(1).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job kind decode failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        let install_intent: Option<String> = row.get(2).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job install_intent decode failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        let priority_raw: String = row.get(3).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job priority decode failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        let state_raw: String = row.get(4).map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("download job state decode failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;

        Ok(Some(DownloadJobRecord {
            job_id: job_id.clone(),
            target_id,
            kind,
            install_intent,
            priority: decode_job_priority(&priority_raw)?,
            state: decode_download_job_state(&state_raw)?,
        }))
    }
}

impl DownloadJobRepository for SqliteDownloadJobRepository {
    fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO download_jobs
             (job_id, target_id, kind, install_intent, priority, state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                job.job_id.to_string(),
                job.target_id,
                job.kind,
                job.install_intent,
                encode_job_priority(job.priority),
                encode_download_job_state(job.state),
            ],
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("download job insert failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }

    fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
        self.get_job(job_id)
    }

    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "UPDATE download_jobs SET state = ?2 WHERE job_id = ?1",
            rusqlite::params![job_id.to_string(), encode_download_job_state(state)],
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("download job state update failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }
}

fn encode_job_priority(priority: JobPriority) -> &'static str {
    match priority {
        JobPriority::Low => "low",
        JobPriority::Normal => "normal",
        JobPriority::High => "high",
    }
}

fn decode_job_priority(priority: &str) -> AppResult<JobPriority> {
    match priority {
        "low" => Ok(JobPriority::Low),
        "normal" => Ok(JobPriority::Normal),
        "high" => Ok(JobPriority::High),
        _ => Err(launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("unsupported download job priority `{priority}`"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        )),
    }
}

fn encode_download_job_state(state: DownloadJobRecordState) -> &'static str {
    match state {
        DownloadJobRecordState::Queued => "queued",
        DownloadJobRecordState::Running => "running",
        DownloadJobRecordState::Paused => "paused",
        DownloadJobRecordState::Completed => "completed",
        DownloadJobRecordState::Failed => "failed",
        DownloadJobRecordState::Canceled => "canceled",
    }
}

fn decode_download_job_state(state: &str) -> AppResult<DownloadJobRecordState> {
    match state {
        "queued" => Ok(DownloadJobRecordState::Queued),
        "running" => Ok(DownloadJobRecordState::Running),
        "paused" => Ok(DownloadJobRecordState::Paused),
        "completed" => Ok(DownloadJobRecordState::Completed),
        "failed" => Ok(DownloadJobRecordState::Failed),
        "canceled" => Ok(DownloadJobRecordState::Canceled),
        _ => Err(launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("unsupported download job state `{state}`"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        )),
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDownloadCheckpointRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteDownloadCheckpointRepository {
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        let repo = Self { config };
        repo.ensure_table()
            .expect("SqliteDownloadCheckpointRepository: failed to create checkpoint table");
        repo
    }

    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }

    fn ensure_table(&self) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS download_job_checkpoints (
                job_id TEXT PRIMARY KEY NOT NULL
            );",
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("download_job_checkpoints table init failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }

    fn open_connection(&self) -> AppResult<rusqlite::Connection> {
        rusqlite::Connection::open(self.config.database_path()).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_OPEN_ERROR",
                format!("failed to open sqlite database: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })
    }

    pub fn load_checkpoint(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare("SELECT job_id FROM download_job_checkpoints WHERE job_id = ?1")
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("failed to prepare checkpoint select: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        let mut rows = stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("checkpoint query failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            ))?;

        let maybe_row = rows.next().map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_READ_ERROR",
            format!("checkpoint row read failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;

        Ok(maybe_row.map(|_| DownloadCheckpointRecord {
            job_id: job_id.clone(),
        }))
    }

    pub fn save_checkpoint(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO download_job_checkpoints (job_id) VALUES (?1)",
            rusqlite::params![checkpoint.job_id.to_string()],
        )
        .map_err(|e| launcher_kernel_foundation::AppError::new(
            "SQLITE_WRITE_ERROR",
            format!("checkpoint upsert failed: {e}"),
            false,
            launcher_kernel_foundation::AppErrorSeverity::Warning,
            launcher_kernel_foundation::CorrelationId::generate(),
        ))?;
        Ok(())
    }
}

impl DownloadCheckpointRepository for SqliteDownloadCheckpointRepository {
    fn load(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>> {
        self.load_checkpoint(job_id)
    }

    fn save(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()> {
        self.save_checkpoint(checkpoint)
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