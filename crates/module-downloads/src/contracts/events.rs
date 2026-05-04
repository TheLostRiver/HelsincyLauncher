//! Event payloads emitted when download facts change.
//!
//! These events are transport-safe notifications derived from backend state
//! transitions rather than internal runtime messages.

use launcher_kernel_foundation::JobId;
use serde::{Deserialize, Serialize};

/// Stable event union for download lifecycle notifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DownloadEventDto {
    /// Emitted when a download job finishes and produces a concrete artifact reference.
    JobCompleted { job_id: JobId, artifact_ref: String },

    /// Emitted when a download job fails and reports whether retry remains possible.
    JobFailed { job_id: JobId, retryable: bool },

    /// Emitted when a download job is canceled before completion.
    JobCanceled { job_id: JobId },
}