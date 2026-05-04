//! Public contract entrypoint for the downloads module.
//!
//! Callers import downloads command/query inputs, read models, and event payloads
//! from this boundary instead of depending on the module's internal file layout.

/// Command input contracts that express downloads-side user intent.
pub mod commands;

/// Read-model and snapshot DTOs projected from backend-owned download state.
pub mod dto;

/// Event payloads broadcast when download facts change.
pub mod events;

/// Query input contracts for stable download reads.
pub mod queries;

pub use commands::{
    CancelDownloadRequestDto, PauseDownloadRequestDto, ResumeDownloadRequestDto,
    StartDownloadRequestDto, UpdateDownloadPolicyRequestDto,
};
pub use dto::{
    DownloadJobExtensionDto, DownloadJobListDto, DownloadJobListItemDto,
    DownloadJobSnapshotDto, DownloadPolicyDto,
};
pub use events::DownloadEventDto;
pub use queries::{GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto};