//! Query input contracts for stable downloads read paths.
//!
//! These DTOs let callers ask for paged job lists, one concrete job snapshot, or
//! the current downloads policy without leaking backend internals.

use launcher_kernel_foundation::{JobId, PageRequest};
use launcher_kernel_jobs::JobUiState;
use serde::{Deserialize, Serialize};

/// Requests a paged list of download jobs, optionally filtered by projected UI state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDownloadJobsQueryDto {
    /// Requested page window for the stable download jobs read model.
    pub page: PageRequest,

    /// Optional UI-level state filter applied to the projected job list.
    pub ui_state: Option<JobUiState>,
}

/// Requests one projected download job snapshot by stable job identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetDownloadJobQueryDto {
    pub job_id: JobId,
}

/// Requests the current downloads policy snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GetDownloadPolicyQueryDto;