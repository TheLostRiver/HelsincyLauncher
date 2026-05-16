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
use launcher_module_downloads::facade::DownloadResumeOutcome;

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
/// 从后端 `AppError` 投影到 IPC 安全字段的稳定错误 envelope。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppErrorDto {
    /// Stable machine-readable error code.
    /// 稳定、机器可读的错误码。
    pub code: String,

    /// Human-readable message safe to surface at the transport boundary.
    /// 可安全暴露到 transport 边界的人类可读错误消息。
    pub message: String,

    /// Whether the caller can retry the command/query without changing inputs.
    /// 调用方是否可以在不修改输入的情况下重试该 command/query。
    pub retryable: bool,

    /// Lowercased severity label projected from backend error severity.
    /// 从后端错误严重级别投影出的 lowercase 严重性标签。
    pub severity: String,

    /// Correlation identifier used to join frontend failures with backend logs.
    /// 用于把前端失败与后端日志关联起来的 correlation 标识。
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
/// 供会修改后端拥有状态或作业的 IPC handler 使用的 command result envelope。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResultDto<T> {
    Success { data: T },
    Failure { error: AppErrorDto },
}

/// Query result envelope used by IPC handlers that return a read model snapshot.
/// 供返回 read model 快照的 IPC handler 使用的 query result envelope。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResultDto<T> {
    Success { data: T, as_of: Option<IsoDateTime> },
    Failure { error: AppErrorDto },
}

/// Accepted-job projection exposed to transport callers after long-running work is queued.
/// 长任务入队后暴露给 transport 调用方的 accepted-job 投影。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedJobDto {
    /// Marker that the backend accepted the request for asynchronous processing.
    /// 标记后端已接收该请求并将异步处理。
    pub accepted: bool,

    /// Backend-generated job identifier.
    /// 后端生成的作业标识。
    pub job_id: JobId,

    /// Module that owns the accepted job.
    /// 拥有该 accepted job 的模块。
    pub module: String,

    /// Job kind within the owning module.
    /// 拥有模块内部的作业类型。
    pub kind: String,

    /// Time when the backend queued the job.
    /// 后端将作业入队的时间。
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

/// Resume outcome projection used only by downloads resume transport.
/// 仅供 downloads resume transport 使用的恢复结果投影。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadResumeOutcomeDto {
    /// Resume produced runtime work and was accepted into the shared job queue.
    /// 恢复产生了 runtime 工作，并已进入 shared job 队列。
    RuntimeAccepted { job: AcceptedJobDto },
    /// Resume found the job already complete without queueing runtime work.
    /// 恢复发现作业已完成，且没有入队 runtime 工作。
    AlreadyComplete {
        /// Marker for the already-complete resume outcome.
        /// 标记恢复结果已经完成。
        completed: bool,

        /// Downloads job identifier that completed through all-sealed resume.
        /// 通过 all-sealed 恢复完成的 downloads 作业标识。
        job_id: JobId,

        /// Module that owns the completed resume outcome.
        /// 拥有该完成恢复结果的模块。
        module: String,

        /// Job kind within the owning module.
        /// 拥有模块内部的作业类型。
        kind: String,
    },
}

impl From<DownloadResumeOutcome> for DownloadResumeOutcomeDto {
    fn from(outcome: DownloadResumeOutcome) -> Self {
        match outcome {
            DownloadResumeOutcome::RuntimeAccepted(job) => {
                Self::RuntimeAccepted { job: job.into() }
            }
            DownloadResumeOutcome::AlreadyComplete {
                job_id,
                module,
                kind,
            } => Self::AlreadyComplete {
                completed: true,
                job_id,
                module,
                kind,
            },
        }
    }
}

/// Shared desktop service aggregation exposed to concrete command handlers.
/// 暴露给具体 command handler 的共享桌面服务聚合。
pub type DesktopServices = DesktopAppServices;

/// Maps a backend command result into the shared desktop command envelope.
/// 将后端 command 结果映射为共享桌面 command envelope。
pub fn map_command_result<T>(result: AppResult<T>) -> CommandResultDto<T> {
    match result {
        Ok(data) => CommandResultDto::Success { data },
        Err(error) => CommandResultDto::Failure {
            error: error.into(),
        },
    }
}

/// Maps a backend query result into the shared desktop query envelope.
/// 将后端 query 结果映射为共享桌面 query envelope。
///
/// Some transport queries remain callable before their read model is fully wired;
/// those paths can project a temporary backend-owned stub when the error code matches
/// the explicitly allowed `not_wired_code`.
/// 某些 transport query 在 read model 尚未完整接线时仍需保持可调用；
/// 当错误码匹配显式允许的 `not_wired_code` 时，这些路径可以投影临时的后端拥有 stub。
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
            // 在 read path 仍为 stub 时，保持稳定 transport contract。
            data: stub(),
            as_of: Some(IsoDateTime::now()),
        },
        Err(error) => QueryResultDto::Failure {
            error: error.into(),
        },
    }
}

/// Maps an accepted backend job into the shared desktop command envelope.
/// 将后端 accepted job 映射为共享桌面 command envelope。
pub fn map_accepted_job_result(result: AppResult<AcceptedJob>) -> CommandResultDto<AcceptedJobDto> {
    match result {
        Ok(job) => CommandResultDto::Success { data: job.into() },
        Err(error) => CommandResultDto::Failure {
            error: error.into(),
        },
    }
}

/// Maps a downloads resume outcome without treating already-complete as accepted work.
/// 映射 downloads 恢复结果，避免把已完成结果当成 accepted work。
pub fn map_download_resume_outcome_result(
    result: AppResult<DownloadResumeOutcome>,
) -> CommandResultDto<DownloadResumeOutcomeDto> {
    match result {
        Ok(outcome) => CommandResultDto::Success {
            data: outcome.into(),
        },
        Err(error) => CommandResultDto::Failure {
            error: error.into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use launcher_kernel_foundation::{IsoDateTime, JobId};
    use launcher_kernel_jobs::AcceptedJob;
    use launcher_module_downloads::facade::DownloadResumeOutcome;

    use super::*;

    #[test]
    fn maps_download_resume_already_complete_without_accepted_job_marker() {
        let job_id = JobId::generate();

        let result =
            map_download_resume_outcome_result(Ok(DownloadResumeOutcome::AlreadyComplete {
                job_id: job_id.clone(),
                module: "downloads".into(),
                kind: "download".into(),
            }));

        assert_eq!(
            result,
            CommandResultDto::Success {
                data: DownloadResumeOutcomeDto::AlreadyComplete {
                    completed: true,
                    job_id,
                    module: "downloads".into(),
                    kind: "download".into(),
                },
            }
        );
    }

    #[test]
    fn maps_download_resume_runtime_accepted_through_accepted_job_projection() {
        let job_id = JobId::generate();
        let accepted = AcceptedJob {
            job_id: job_id.clone(),
            module: "downloads".into(),
            kind: "download".into(),
            queued_at: IsoDateTime::now(),
        };

        let result = map_download_resume_outcome_result(Ok(
            DownloadResumeOutcome::RuntimeAccepted(accepted.clone()),
        ));

        match result {
            CommandResultDto::Success {
                data: DownloadResumeOutcomeDto::RuntimeAccepted { job },
            } => {
                assert!(job.accepted);
                assert_eq!(job.job_id, job_id);
                assert_eq!(job.module, accepted.module);
                assert_eq!(job.kind, accepted.kind);
                assert_eq!(job.queued_at, accepted.queued_at);
            }
            other => panic!("runtime-accepted resume should project accepted job, got {other:?}"),
        }
    }
}
