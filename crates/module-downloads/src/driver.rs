use std::sync::Arc;

use launcher_kernel_foundation::{AppResult, JobId};
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadCheckpointRecord {
    pub job_id: JobId,
}

pub trait DownloadCheckpointRepository: Send + Sync {
    fn load(&self, job_id: &JobId) -> AppResult<Option<DownloadCheckpointRecord>>;
    fn save(&self, checkpoint: &DownloadCheckpointRecord) -> AppResult<()>;
}

/// Restore driver for the `downloads/download` job kind.
///
/// Stage-2 restore is only allowed to keep a download job resumable when
/// its persisted download checkpoint still exists. Staging-file verification
/// remains a later slice.
#[derive(Clone)]
pub struct DownloadJobDriver {
    checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
}

impl DownloadJobDriver {
    pub fn new(checkpoint_repo: Arc<dyn DownloadCheckpointRepository>) -> Self {
        Self { checkpoint_repo }
    }
}

impl JobDriver<()> for DownloadJobDriver {
    fn module(&self) -> &'static str {
        "downloads"
    }

    fn kind(&self) -> &'static str {
        "download"
    }

    fn restore(&self, snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        if self.checkpoint_repo.load(&snapshot.job_id)?.is_some() {
            return Ok(RestoreDisposition::Resumed);
        }

        Ok(RestoreDisposition::FailedAsUnrecoverable {
            reason: format!(
                "download checkpoint missing for queued job {}",
                snapshot.job_id
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Mutex;

    use launcher_kernel_foundation::{IsoDateTime, JobId};
    use launcher_kernel_jobs::{JobProgress, JobState, JobUiState};

    use super::{
        DownloadCheckpointRecord, DownloadCheckpointRepository, DownloadJobDriver,
    };

    #[derive(Default)]
    struct InMemoryCheckpointRepository {
        job_ids: Mutex<HashSet<JobId>>,
    }

    impl DownloadCheckpointRepository for InMemoryCheckpointRepository {
        fn load(&self, job_id: &JobId) -> launcher_kernel_foundation::AppResult<Option<DownloadCheckpointRecord>> {
            let has_checkpoint = self
                .job_ids
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .contains(job_id);

            Ok(has_checkpoint.then(|| DownloadCheckpointRecord {
                job_id: job_id.clone(),
            }))
        }

        fn save(&self, checkpoint: &DownloadCheckpointRecord) -> launcher_kernel_foundation::AppResult<()> {
            self.job_ids
                .lock()
                .expect("checkpoint mutex should not be poisoned")
                .insert(checkpoint.job_id.clone());
            Ok(())
        }
    }

    fn make_snapshot(job_id: JobId) -> launcher_kernel_jobs::JobSnapshot<()> {
        launcher_kernel_jobs::JobSnapshot {
            job_id,
            module: "downloads".into(),
            kind: "download".into(),
            state: JobState::Queued,
            ui_state: JobUiState::Queued,
            progress: JobProgress::pending(),
            recoverable: true,
            updated_at: IsoDateTime::now(),
            extension: None,
        }
    }

    #[test]
    fn restore_returns_failed_when_checkpoint_is_missing() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo);
        let snapshot = make_snapshot(JobId::generate());

        let disposition = driver
            .restore(&snapshot)
            .expect("restore should not error for missing checkpoint");

        assert!(
            matches!(
                disposition,
                launcher_kernel_jobs::RestoreDisposition::FailedAsUnrecoverable { .. }
            ),
            "missing checkpoint should make the queued download unrecoverable"
        );
    }

    #[test]
    fn restore_returns_resumed_when_checkpoint_exists() {
        let repo = Arc::new(InMemoryCheckpointRepository::default());
        let driver = DownloadJobDriver::new(repo.clone());
        let snapshot = make_snapshot(JobId::generate());

        repo.save(&DownloadCheckpointRecord {
            job_id: snapshot.job_id.clone(),
        })
        .expect("saving a synthetic checkpoint should succeed");

        let disposition = driver
            .restore(&snapshot)
            .expect("restore should not error for persisted checkpoint");

        assert_eq!(
            disposition,
            launcher_kernel_jobs::RestoreDisposition::Resumed,
            "persisted checkpoint should keep the queued download resumable"
        );
    }
}
