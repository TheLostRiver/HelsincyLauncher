//! Read-model DTOs projected from backend-owned download state.
//!
//! These shapes are stable transport contracts for list/detail reads and policy
//! snapshots exposed outside the downloads module.

use launcher_kernel_foundation::{JobId, PageSlice};
use launcher_kernel_jobs::{JobSnapshot, JobUiState};
use serde::{Deserialize, Serialize};

/// Download-specific extension facts attached to a generic job snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobExtensionDto {
    /// Backend-known identifier of the asset or engine payload being downloaded.
    pub target_id: String,

    /// Optional install follow-up captured when the job was created.
    pub install_intent: Option<String>,

    /// Number of payload bytes already materialized locally.
    pub completed_bytes: u64,

    /// Total payload size when the backend can determine it.
    pub total_bytes: Option<u64>,

    /// Whether the latest failure mode allows another backend retry attempt.
    pub retryable: bool,
}

/// Full projected snapshot of one download job.
pub type DownloadJobSnapshotDto = JobSnapshot<DownloadJobExtensionDto>;

/// Summary row used by paged download job listings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadJobListItemDto {
    pub job_id: JobId,

    /// User-facing title resolved for the item shown in download lists.
    pub title: String,

    /// Projected UI state consumed by the shell and frontend filters.
    pub ui_state: JobUiState,

    /// Optional human-readable progress message when raw bytes are not enough.
    pub progress_label: Option<String>,

    /// Latest observed transfer rate, if the runtime currently exposes one.
    pub throughput_bytes_per_sec: Option<u64>,
}

/// Paged download job list read model.
pub type DownloadJobListDto = PageSlice<DownloadJobListItemDto>;

/// Snapshot of the downloads policy currently enforced by the backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadPolicyDto {
    pub concurrency_slots: u32,
    pub bandwidth_limit_bytes_per_sec: Option<u64>,
    pub auto_resume: bool,
}