//! Shared desktop transport contracts and mapping helpers.
//! 共享桌面 transport contract 与映射 helper。
//!
//! This module is the narrow host-facing boundary between Rust backend services
//! and the Tauri command layer: it owns registered command names, stable result
//! envelopes, and the common mapping helpers that keep command/query handlers from
//! leaking raw internal errors across IPC.
//! 该模块是 Rust 后端服务与 Tauri command 层之间的窄宿主边界，集中拥有命令名注册、
//! 稳定结果 envelope，以及避免 handler 泄露内部错误的公共映射 helper。

use launcher_composition_root::DesktopAppServices;
use launcher_kernel_foundation::{AppError, AppResult, IsoDateTime, JobId};
use launcher_kernel_jobs::AcceptedJob;

pub mod downloads;
pub mod engines;
pub mod fab;
pub mod jobs;

/// Command names exposed by the current desktop host transport boundary.
/// 当前桌面宿主 transport 边界公开的命令名集合。
pub const REGISTERED_COMMANDS: &[&str] = &[
    "fab_list_inventory",
    "fab_get_asset_detail",
    "fab_run_startup_prewarm",
    "fab_sync_inventory",
    "engines_run_verification",
    "downloads_start",
    "downloads_pause",
    "downloads_resume",
    "downloads_cancel",
    "downloads_list_jobs",
    "downloads_get_job_snapshot",
    "downloads_get_policy",
    "downloads_update_policy",
    "jobs_list_active",
];

/// Stable error envelope projected from backend `AppError` values into IPC-safe fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppErrorDto {
    /// Stable machine-readable error code.
    pub code: String,

    /// Human-readable message safe to surface at the transport boundary.
    pub message: String,

    /// Whether the caller can retry the command/query without changing inputs.
    pub retryable: bool,

    /// Lowercased severity label projected from backend error severity.
    pub severity: String,

    /// Correlation identifier used to join frontend failures with backend logs.
    pub correlation_id: String,
}

impl From<AppError> for AppErrorDto {
    fn from(error: AppError) -> Self {
        Self {
            code: error.code,
            message: error.message,
            retryable: error.retryable,
            severity: format!("{:?}", error.severity).to_lowercase(),
            correlation_id: error.correlation_id.to_string(),
        }
    }
}

/// Command result envelope used by IPC handlers that mutate backend-owned state or jobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResultDto<T> {
    Success { data: T },
    Failure { error: AppErrorDto },
}

/// Query result envelope used by IPC handlers that return a read model snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResultDto<T> {
    Success { data: T, as_of: Option<IsoDateTime> },
    Failure { error: AppErrorDto },
}

/// Accepted-job projection exposed to transport callers after long-running work is queued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedJobDto {
    /// Marker that the backend accepted the request for asynchronous processing.
    pub accepted: bool,

    /// Backend-generated job identifier.
    pub job_id: JobId,

    /// Module that owns the accepted job.
    pub module: String,

    /// Job kind within the owning module.
    pub kind: String,

    /// Time when the backend queued the job.
    pub queued_at: IsoDateTime,
}

impl From<AcceptedJob> for AcceptedJobDto {
    fn from(job: AcceptedJob) -> Self {
        Self {
            accepted: true,
            job_id: job.job_id,
            module: job.module,
            kind: job.kind,
            queued_at: job.queued_at,
        }
    }
}

/// Shared desktop service aggregation exposed to concrete command handlers.
pub type DesktopServices = DesktopAppServices;

/// Maps a backend command result into the shared desktop command envelope.
pub fn map_command_result<T>(result: AppResult<T>) -> CommandResultDto<T> {
    match result {
        Ok(data) => CommandResultDto::Success { data },
        Err(error) => CommandResultDto::Failure {
            error: error.into(),
        },
    }
}

/// Maps a backend query result into the shared desktop query envelope.
///
/// Some transport queries remain callable before their read model is fully wired;
/// those paths can project a temporary backend-owned stub when the error code matches
/// the explicitly allowed `not_wired_code`.
pub fn map_query_result_or_stub<T>(
    result: AppResult<T>,
    not_wired_code: &str,
    stub: impl FnOnce() -> T,
) -> QueryResultDto<T> {
    match result {
        Ok(data) => QueryResultDto::Success {
            data,
            as_of: Some(IsoDateTime::now()),
        },
        Err(error) if error.code == not_wired_code => QueryResultDto::Success {
            // Preserve a stable transport contract while the read path is still stubbed.
            data: stub(),
            as_of: Some(IsoDateTime::now()),
        },
        Err(error) => QueryResultDto::Failure {
            error: error.into(),
        },
    }
}

/// Maps an accepted backend job into the shared desktop command envelope.
pub fn map_accepted_job_result(result: AppResult<AcceptedJob>) -> CommandResultDto<AcceptedJobDto> {
    match result {
        Ok(job) => CommandResultDto::Success { data: job.into() },
        Err(error) => CommandResultDto::Failure {
            error: error.into(),
        },
    }
}
