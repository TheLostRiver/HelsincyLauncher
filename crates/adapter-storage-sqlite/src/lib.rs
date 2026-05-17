//! SQLite storage adapter crate 的共享入口与仓储外壳集合。
//!
//! 这个 crate 负责把结构化本地事实落到 SQLite，并为 Fab、downloads 和 job
//! runtime 暴露各自的适配器外壳。当前这一刀只补文件头和共享配置声明注释，
//! 不改变既有 `rusqlite` 访问方式、schema 初始化或仓储行为。

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use launcher_kernel_foundation::{AppResult, AssetId, IsoDateTime, JobId, PageCursor, PageSlice};
use launcher_kernel_jobs::{
    JobPriority, JobProgress, JobSnapshot, JobSnapshotStore, JobState, JobUiState,
};
use launcher_module_downloads::{
    contracts::ListDownloadJobsQueryDto, DownloadCheckpointRecord, DownloadCheckpointRepository,
    DownloadJobRecord, DownloadJobRecordState, DownloadJobRepository,
    DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus,
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

    fn get_asset_detail_snapshot(
        &self,
        _asset_id: &AssetId,
    ) -> AppResult<Option<FabAssetDetailDto>> {
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

/// 基于 SQLite 媒体投影的 Fab 媒体元数据仓储外壳。
#[derive(Debug, Clone)]
pub struct SqliteFabMediaMetadataRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteFabMediaMetadataRepository {
    /// 用共享 SQLite 配置创建 Fab 媒体元数据仓储。
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        Self { config }
    }

    /// 暴露只读配置快照，供装配和诊断路径确认媒体仓储绑定结果。
    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }
}

/// 基于 SQLite 作业表的下载任务仓储外壳。
#[derive(Debug, Clone)]
pub struct SqliteDownloadJobRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteDownloadJobRepository {
    /// 用共享 SQLite 配置创建下载任务仓储，并确保作业表可用。
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        let repo = Self { config };
        repo.ensure_table()
            .expect("SqliteDownloadJobRepository: failed to create jobs table");
        repo
    }

    /// 暴露只读配置快照，供装配和诊断路径确认下载任务仓储绑定结果。
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
        .map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("download_jobs table init failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
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

    /// 按作业标识读取当前持久化的下载任务快照。
    pub fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(
                "SELECT target_id, kind, install_intent, priority, state
                 FROM download_jobs
                 WHERE job_id = ?1",
            )
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("failed to prepare download job select: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        let mut rows = stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("download job query failed: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        let maybe_row = rows.next().map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job row read failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;

        let Some(row) = maybe_row else {
            return Ok(None);
        };

        let target_id: String = row.get(0).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job target_id decode failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let kind: String = row.get(1).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job kind decode failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let install_intent: Option<String> = row.get(2).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job install_intent decode failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let priority_raw: String = row.get(3).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job priority decode failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let state_raw: String = row.get(4).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("download job state decode failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;

        Ok(Some(DownloadJobRecord {
            job_id: job_id.clone(),
            target_id,
            kind,
            install_intent,
            priority: decode_job_priority(&priority_raw)?,
            state: decode_download_job_state(&state_raw)?,
        }))
    }

    /// Lists module-owned download job records with repository-local offset cursors.
    /// 使用仓储本地 offset 游标列出 downloads 模块拥有的任务记录。
    pub fn list_jobs(
        &self,
        query: &ListDownloadJobsQueryDto,
    ) -> AppResult<PageSlice<DownloadJobRecord>> {
        let limit = query.page.limit as usize;
        if limit == 0 {
            return Ok(PageSlice::new(Vec::new(), None));
        }

        let offset = decode_page_offset(query.page.cursor.as_ref())?;
        let fetch_limit = limit + 1;
        let conn = self.open_connection()?;
        let mut jobs = Vec::new();

        if let Some(ui_state) = query.ui_state {
            let Some(record_state) = download_record_state_for_ui_state(ui_state) else {
                return Ok(PageSlice::new(Vec::new(), None));
            };
            let mut stmt = conn
                .prepare(
                    "SELECT job_id, target_id, kind, install_intent, priority, state
                     FROM download_jobs
                     WHERE state = ?1
                     ORDER BY job_id
                     LIMIT ?2 OFFSET ?3",
                )
                .map_err(|e| {
                    sqlite_read_error(format!("failed to prepare download job list: {e}"))
                })?;
            let mut rows = stmt
                .query(rusqlite::params![
                    encode_download_job_state(record_state),
                    fetch_limit as i64,
                    offset as i64
                ])
                .map_err(|e| sqlite_read_error(format!("download job list query failed: {e}")))?;
            while let Some(row) = rows
                .next()
                .map_err(|e| sqlite_read_error(format!("download job list row read failed: {e}")))?
            {
                jobs.push(read_download_job_record_row(row)?);
            }
        } else {
            let mut stmt = conn
                .prepare(
                    "SELECT job_id, target_id, kind, install_intent, priority, state
                     FROM download_jobs
                     ORDER BY job_id
                     LIMIT ?1 OFFSET ?2",
                )
                .map_err(|e| {
                    sqlite_read_error(format!("failed to prepare download job list: {e}"))
                })?;
            let mut rows = stmt
                .query(rusqlite::params![fetch_limit as i64, offset as i64])
                .map_err(|e| sqlite_read_error(format!("download job list query failed: {e}")))?;
            while let Some(row) = rows
                .next()
                .map_err(|e| sqlite_read_error(format!("download job list row read failed: {e}")))?
            {
                jobs.push(read_download_job_record_row(row)?);
            }
        }

        let has_more = jobs.len() > limit;
        if has_more {
            jobs.truncate(limit);
        }
        let next_cursor = has_more.then(|| PageCursor::new((offset + limit).to_string()));

        Ok(PageSlice::new(jobs, next_cursor))
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
        .map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("download job insert failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        Ok(())
    }

    fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
        self.get_job(job_id)
    }

    fn list_jobs(
        &self,
        query: &ListDownloadJobsQueryDto,
    ) -> AppResult<PageSlice<DownloadJobRecord>> {
        self.list_jobs(query)
    }

    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "UPDATE download_jobs SET state = ?2 WHERE job_id = ?1",
            rusqlite::params![job_id.to_string(), encode_download_job_state(state)],
        )
        .map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("download job state update failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
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

fn decode_page_offset(cursor: Option<&PageCursor>) -> AppResult<usize> {
    match cursor {
        Some(cursor) => cursor
            .as_str()
            .parse::<usize>()
            .map_err(|e| sqlite_read_error(format!("download job list cursor decode failed: {e}"))),
        None => Ok(0),
    }
}

fn download_record_state_for_ui_state(ui_state: JobUiState) -> Option<DownloadJobRecordState> {
    match ui_state {
        JobUiState::Queued => Some(DownloadJobRecordState::Queued),
        JobUiState::Running => Some(DownloadJobRecordState::Running),
        JobUiState::Paused => Some(DownloadJobRecordState::Paused),
        JobUiState::Completed => Some(DownloadJobRecordState::Completed),
        JobUiState::Failed => Some(DownloadJobRecordState::Failed),
        JobUiState::Canceled => Some(DownloadJobRecordState::Canceled),
        JobUiState::AwaitingUser => None,
    }
}

fn read_download_job_record_row(row: &rusqlite::Row<'_>) -> AppResult<DownloadJobRecord> {
    let job_id_raw: String = row
        .get(0)
        .map_err(|e| sqlite_read_error(format!("download job id decode failed: {e}")))?;
    let target_id: String = row
        .get(1)
        .map_err(|e| sqlite_read_error(format!("download job target_id decode failed: {e}")))?;
    let kind: String = row
        .get(2)
        .map_err(|e| sqlite_read_error(format!("download job kind decode failed: {e}")))?;
    let install_intent: Option<String> = row.get(3).map_err(|e| {
        sqlite_read_error(format!("download job install_intent decode failed: {e}"))
    })?;
    let priority_raw: String = row
        .get(4)
        .map_err(|e| sqlite_read_error(format!("download job priority decode failed: {e}")))?;
    let state_raw: String = row
        .get(5)
        .map_err(|e| sqlite_read_error(format!("download job state decode failed: {e}")))?;

    Ok(DownloadJobRecord {
        job_id: JobId::new(job_id_raw),
        target_id,
        kind,
        install_intent,
        priority: decode_job_priority(&priority_raw)?,
        state: decode_download_job_state(&state_raw)?,
    })
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

/// 基于 SQLite checkpoint 表的下载断点仓储外壳。
#[derive(Debug, Clone)]
pub struct SqliteDownloadCheckpointRepository {
    config: SqliteStorageAdapterConfig,
}

impl SqliteDownloadCheckpointRepository {
    /// 用共享 SQLite 配置创建下载 checkpoint 仓储，并确保断点表可用。
    pub fn new(config: SqliteStorageAdapterConfig) -> Self {
        let repo = Self { config };
        repo.ensure_table()
            .expect("SqliteDownloadCheckpointRepository: failed to create checkpoint table");
        repo
    }

    /// 暴露只读配置快照，供装配和诊断路径确认 checkpoint 仓储绑定结果。
    pub fn config(&self) -> &SqliteStorageAdapterConfig {
        &self.config
    }

    fn ensure_table(&self) -> AppResult<()> {
        let conn = self.open_connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS download_job_checkpoints (
                job_id TEXT PRIMARY KEY NOT NULL
            );
            CREATE TABLE IF NOT EXISTS download_segment_checkpoints (
                job_id TEXT NOT NULL,
                segment_index INTEGER NOT NULL,
                segment_id TEXT NOT NULL,
                file_id TEXT NOT NULL,
                segment_offset TEXT NOT NULL,
                segment_length TEXT NOT NULL,
                downloaded_bytes TEXT NOT NULL,
                status TEXT NOT NULL,
                partial_path TEXT NULL,
                etag TEXT NULL,
                hash_state_ref TEXT NULL,
                PRIMARY KEY (job_id, segment_id),
                FOREIGN KEY (job_id) REFERENCES download_job_checkpoints(job_id) ON DELETE CASCADE
            );",
        )
        .map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("download checkpoint tables init failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
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

    /// 按作业标识读取当前持久化的下载断点记录。
    pub fn load_checkpoint(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>> {
        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare("SELECT job_id FROM download_job_checkpoints WHERE job_id = ?1")
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("failed to prepare checkpoint select: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        let mut rows = stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("checkpoint query failed: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        let maybe_row = rows.next().map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("checkpoint row read failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;

        if maybe_row.is_none() {
            return Ok(None);
        }

        drop(rows);
        drop(stmt);

        let mut segment_stmt = conn
            .prepare(
                "SELECT job_id, segment_id, file_id, segment_offset, segment_length,
                        downloaded_bytes, status, partial_path, etag, hash_state_ref
                 FROM download_segment_checkpoints
                 WHERE job_id = ?1
                 ORDER BY segment_index ASC",
            )
            .map_err(|e| {
                sqlite_read_error(format!("failed to prepare segment checkpoint select: {e}"))
            })?;

        let mut segment_rows = segment_stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| sqlite_read_error(format!("segment checkpoint query failed: {e}")))?;

        let mut segments = Vec::new();
        while let Some(row) = segment_rows
            .next()
            .map_err(|e| sqlite_read_error(format!("segment checkpoint row read failed: {e}")))?
        {
            let segment_job_id: String = row
                .get(0)
                .map_err(|e| sqlite_read_error(format!("segment job_id decode failed: {e}")))?;
            let segment_id: String = row
                .get(1)
                .map_err(|e| sqlite_read_error(format!("segment_id decode failed: {e}")))?;
            let file_id: String = row
                .get(2)
                .map_err(|e| sqlite_read_error(format!("segment file_id decode failed: {e}")))?;
            let offset_raw: String = row
                .get(3)
                .map_err(|e| sqlite_read_error(format!("segment offset decode failed: {e}")))?;
            let length_raw: String = row
                .get(4)
                .map_err(|e| sqlite_read_error(format!("segment length decode failed: {e}")))?;
            let downloaded_bytes_raw: String = row.get(5).map_err(|e| {
                sqlite_read_error(format!("segment downloaded_bytes decode failed: {e}"))
            })?;
            let status_raw: String = row
                .get(6)
                .map_err(|e| sqlite_read_error(format!("segment status decode failed: {e}")))?;
            let partial_path: Option<String> = row.get(7).map_err(|e| {
                sqlite_read_error(format!("segment partial_path decode failed: {e}"))
            })?;
            let etag: Option<String> = row
                .get(8)
                .map_err(|e| sqlite_read_error(format!("segment etag decode failed: {e}")))?;
            let hash_state_ref: Option<String> = row.get(9).map_err(|e| {
                sqlite_read_error(format!("segment hash_state_ref decode failed: {e}"))
            })?;

            segments.push(DownloadSegmentCheckpointRecord {
                job_id: JobId::new(segment_job_id),
                segment_id,
                file_id,
                offset: decode_u64_text("segment offset", &offset_raw)?,
                length: decode_u64_text("segment length", &length_raw)?,
                downloaded_bytes: decode_u64_text(
                    "segment downloaded_bytes",
                    &downloaded_bytes_raw,
                )?,
                status: decode_segment_checkpoint_status(&status_raw)?,
                partial_path,
                etag,
                hash_state_ref,
            });
        }

        Ok(Some(DownloadCheckpointRecord {
            job_id: job_id.clone(),
            segments,
        }))
    }

    /// 持久化最新的下载断点记录，供暂停和恢复链路复用。
    pub fn save_checkpoint(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()> {
        let mut conn = self.open_connection()?;
        let tx = conn.transaction().map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("checkpoint transaction start failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;

        tx.execute(
            "INSERT OR REPLACE INTO download_job_checkpoints (job_id) VALUES (?1)",
            rusqlite::params![checkpoint.job_id.to_string()],
        )
        .map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_WRITE_ERROR",
                format!("checkpoint upsert failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        // Replace the segment facts as one job-scoped checkpoint snapshot.
        // 将 segment facts 作为同一个 job-scoped checkpoint 快照整体替换。
        tx.execute(
            "DELETE FROM download_segment_checkpoints WHERE job_id = ?1",
            rusqlite::params![checkpoint.job_id.to_string()],
        )
        .map_err(|e| sqlite_write_error(format!("segment checkpoint delete failed: {e}")))?;

        for (segment_index, segment) in checkpoint.segments.iter().enumerate() {
            tx.execute(
                "INSERT INTO download_segment_checkpoints
                 (job_id, segment_index, segment_id, file_id, segment_offset, segment_length,
                  downloaded_bytes, status, partial_path, etag, hash_state_ref)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                rusqlite::params![
                    segment.job_id.to_string(),
                    i64::try_from(segment_index).map_err(|e| {
                        sqlite_write_error(format!("segment index conversion failed: {e}"))
                    })?,
                    segment.segment_id,
                    segment.file_id,
                    segment.offset.to_string(),
                    segment.length.to_string(),
                    segment.downloaded_bytes.to_string(),
                    encode_segment_checkpoint_status(segment.status),
                    segment.partial_path,
                    segment.etag,
                    segment.hash_state_ref,
                ],
            )
            .map_err(|e| sqlite_write_error(format!("segment checkpoint insert failed: {e}")))?;
        }

        tx.commit().map_err(|e| {
            sqlite_write_error(format!("checkpoint transaction commit failed: {e}"))
        })?;
        Ok(())
    }
}

fn encode_segment_checkpoint_status(status: DownloadSegmentCheckpointStatus) -> &'static str {
    match status {
        DownloadSegmentCheckpointStatus::Pending => "pending",
        DownloadSegmentCheckpointStatus::InProgress => "in_progress",
        DownloadSegmentCheckpointStatus::Completed => "completed",
        DownloadSegmentCheckpointStatus::Failed => "failed",
    }
}

fn decode_segment_checkpoint_status(raw: &str) -> AppResult<DownloadSegmentCheckpointStatus> {
    match raw {
        "pending" => Ok(DownloadSegmentCheckpointStatus::Pending),
        "in_progress" => Ok(DownloadSegmentCheckpointStatus::InProgress),
        "completed" => Ok(DownloadSegmentCheckpointStatus::Completed),
        "failed" => Ok(DownloadSegmentCheckpointStatus::Failed),
        _ => Err(sqlite_read_error(format!(
            "unsupported segment checkpoint status `{raw}`"
        ))),
    }
}

fn decode_u64_text(field: &str, raw: &str) -> AppResult<u64> {
    raw.parse::<u64>()
        .map_err(|e| sqlite_read_error(format!("{field} parse failed: {e}")))
}

fn sqlite_read_error(message: String) -> launcher_kernel_foundation::AppError {
    launcher_kernel_foundation::AppError::new(
        "SQLITE_READ_ERROR",
        message,
        false,
        launcher_kernel_foundation::AppErrorSeverity::Warning,
        launcher_kernel_foundation::CorrelationId::generate(),
    )
}

fn sqlite_write_error(message: String) -> launcher_kernel_foundation::AppError {
    launcher_kernel_foundation::AppError::new(
        "SQLITE_WRITE_ERROR",
        message,
        false,
        launcher_kernel_foundation::AppErrorSeverity::Warning,
        launcher_kernel_foundation::CorrelationId::generate(),
    )
}

impl DownloadCheckpointRepository for SqliteDownloadCheckpointRepository {
    fn load(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>> {
        self.load_checkpoint(job_id)
    }

    fn save(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()> {
        self.save_checkpoint(checkpoint)
    }
}

/// 基于 SQLite 快照表的共享作业快照存储外壳。
pub struct SqliteJobSnapshotStore {
    conn: Mutex<rusqlite::Connection>,
}

impl SqliteJobSnapshotStore {
    /// 用共享 SQLite 配置创建作业快照存储，并确保快照表可用。
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
        // 兼容旧库：为早于 `recoverable` 字段的 job_snapshots 表补列。
        // SQLite 的 ALTER TABLE 不支持 IF NOT EXISTS；列已存在时保留忽略错误的迁移策略。
        let _ = conn.execute(
            "ALTER TABLE job_snapshots ADD COLUMN recoverable INTEGER NOT NULL DEFAULT 1",
            [],
        );
        Self {
            conn: Mutex::new(conn),
        }
    }

    fn upsert(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        let progress_json = serde_json::to_string(&snapshot.progress).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobProgress: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let state_str = serde_json::to_string(&snapshot.state).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobState: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
        let ui_state_str = serde_json::to_string(&snapshot.ui_state).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_SERIALIZE_ERROR",
                format!("failed to serialize JobUiState: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;
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
        let mut stmt = conn
            .prepare(
                "SELECT job_id, module, kind, state, ui_state, progress, recoverable, updated_at
             FROM job_snapshots WHERE job_id = ?1",
            )
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("failed to prepare job_snapshots select: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        let mut rows = stmt
            .query(rusqlite::params![job_id.to_string()])
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("job_snapshots query failed: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        if let Some(row) = rows.next().map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("job_snapshots row read failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })? {
            let job_id_str: String = row.get(0).unwrap();
            let module: String = row.get(1).unwrap();
            let kind: String = row.get(2).unwrap();
            let state_str: String = row.get(3).unwrap();
            let ui_state_str: String = row.get(4).unwrap();
            let progress_str: String = row.get(5).unwrap();
            let recoverable_int: i64 = row.get(6).unwrap_or(1);
            let updated_at_str: String = row.get(7).unwrap();

            let state: JobState = serde_json::from_str(&state_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobState: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let ui_state: JobUiState = serde_json::from_str(&ui_state_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobUiState: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let progress: JobProgress = serde_json::from_str(&progress_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to deserialize JobProgress: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let updated_at: IsoDateTime = serde_json::from_str(&format!(r#""{updated_at_str}""#))
                .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("failed to parse updated_at: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
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
        let placeholders = resumable_state_jsons
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT job_id, module, kind, state, ui_state, progress, recoverable, updated_at
             FROM job_snapshots WHERE state IN ({placeholders})"
        );
        let conn = self.conn.lock().unwrap_or_else(|p| p.into_inner());
        let mut stmt = conn.prepare(&sql).map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("list_resumable prepare failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })?;

        let mut results = Vec::new();
        let mut rows = stmt
            .query(rusqlite::params_from_iter(resumable_state_jsons.iter()))
            .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_READ_ERROR",
                    format!("list_resumable query failed: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

        while let Some(row) = rows.next().map_err(|e| {
            launcher_kernel_foundation::AppError::new(
                "SQLITE_READ_ERROR",
                format!("list_resumable row read failed: {e}"),
                false,
                launcher_kernel_foundation::AppErrorSeverity::Warning,
                launcher_kernel_foundation::CorrelationId::generate(),
            )
        })? {
            let job_id_str: String = row.get(0).unwrap();
            let module: String = row.get(1).unwrap();
            let kind: String = row.get(2).unwrap();
            let state_str: String = row.get(3).unwrap();
            let ui_state_str: String = row.get(4).unwrap();
            let progress_str: String = row.get(5).unwrap();
            let recoverable_int: i64 = row.get(6).unwrap_or(1);
            let updated_at_str: String = row.get(7).unwrap();

            let state: JobState = serde_json::from_str(&state_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("state: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let ui_state: JobUiState = serde_json::from_str(&ui_state_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("ui_state: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let progress: JobProgress = serde_json::from_str(&progress_str).map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("progress: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;
            let updated_at: IsoDateTime = serde_json::from_str(&format!(r#""{updated_at_str}""#))
                .map_err(|e| {
                launcher_kernel_foundation::AppError::new(
                    "SQLITE_DESERIALIZE_ERROR",
                    format!("updated_at: {e}"),
                    false,
                    launcher_kernel_foundation::AppErrorSeverity::Warning,
                    launcher_kernel_foundation::CorrelationId::generate(),
                )
            })?;

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

#[cfg(test)]
mod tests {
    use super::*;

    use launcher_module_downloads::{
        DownloadSegmentCheckpointRecord, DownloadSegmentCheckpointStatus,
    };

    #[test]
    fn sqlite_download_checkpoint_round_trips_segment_facts() {
        let tmp_path =
            std::env::temp_dir().join("at188_download_segment_checkpoint_round_trip.sqlite3");
        let _ = std::fs::remove_file(&tmp_path);
        let repo = SqliteDownloadCheckpointRepository::new(SqliteStorageAdapterConfig::new(
            tmp_path.clone(),
        ));
        let job_id = JobId::generate();
        let checkpoint = DownloadCheckpointRecord {
            job_id: job_id.clone(),
            segments: vec![
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-partial".into(),
                    file_id: "file-a".into(),
                    offset: 128,
                    length: 1024,
                    downloaded_bytes: 512,
                    status: DownloadSegmentCheckpointStatus::InProgress,
                    partial_path: Some("staging/file-a.part".into()),
                    etag: Some("etag-a".into()),
                    hash_state_ref: Some("hash-a".into()),
                },
                DownloadSegmentCheckpointRecord {
                    job_id: job_id.clone(),
                    segment_id: "segment-complete".into(),
                    file_id: "file-b".into(),
                    offset: 2048,
                    length: 4096,
                    downloaded_bytes: 4096,
                    status: DownloadSegmentCheckpointStatus::Completed,
                    partial_path: None,
                    etag: None,
                    hash_state_ref: None,
                },
            ],
        };

        repo.save(&checkpoint)
            .expect("saving a segment checkpoint record should succeed");

        let loaded = repo
            .load(&job_id)
            .expect("loading a segment checkpoint record should not error")
            .expect("saved checkpoint should be present");

        assert_eq!(loaded, checkpoint);

        let _ = std::fs::remove_file(&tmp_path);
    }
}
