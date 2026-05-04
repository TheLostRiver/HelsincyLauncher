use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId, JobId,
};
use launcher_kernel_jobs::{AcceptedJob, EnqueueJobRequest, JobRuntime, JobPriority};

use crate::contracts::{
    CancelDownloadRequestDto, DownloadJobListDto, DownloadJobSnapshotDto, DownloadPolicyDto,
    GetDownloadJobQueryDto, GetDownloadPolicyQueryDto, ListDownloadJobsQueryDto,
    PauseDownloadRequestDto, ResumeDownloadRequestDto, StartDownloadRequestDto,
    UpdateDownloadPolicyRequestDto,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadJobRecordState {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadJobRecord {
    pub job_id: JobId,
    pub target_id: String,
    pub kind: String,
    pub install_intent: Option<String>,
    pub priority: JobPriority,
    pub state: DownloadJobRecordState,
}

pub trait DownloadJobRepository: Send + Sync {
    fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()>;
    fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>>;
    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct DownloadModuleDeps<J, C, M, S, R> {
    pub job_repo: J,
    pub checkpoint_repo: C,
    pub manifest_provider: M,
    pub staging_store: S,
    pub job_runtime: R,
}

pub struct DownloadFacade<J, C, M, S, R> {
    deps: DownloadModuleDeps<J, C, M, S, R>,
}

impl<J, C, M, S, R> DownloadFacade<J, C, M, S, R> {
    pub fn new(deps: DownloadModuleDeps<J, C, M, S, R>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &DownloadModuleDeps<J, C, M, S, R> {
        &self.deps
    }
}

impl<J: DownloadJobRepository, C, M, S, R: JobRuntime<Extension = ()>> DownloadFacade<J, C, M, S, R> {
    pub fn start_download(&self, request: StartDownloadRequestDto) -> AppResult<AcceptedJob> {
        let job_id = JobId::generate();
        let StartDownloadRequestDto {
            target_id,
            kind,
            install_intent,
            priority,
        } = request;

        self.deps.job_repo.create_job(&DownloadJobRecord {
            job_id: job_id.clone(),
            target_id,
            kind,
            install_intent,
            priority,
            state: DownloadJobRecordState::Queued,
        })?;

        let enqueue_request = EnqueueJobRequest {
            job_id,
            module: "downloads".to_string(),
            kind: "download".to_string(),
            priority,
            recoverable: true,
            extension: None,
        };
        self.deps.job_runtime.enqueue(enqueue_request)
    }

    pub fn pause_download(&self, request: PauseDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("pause_download"))
    }

    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("resume_download"))
    }

    pub fn cancel_download(&self, request: CancelDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("cancel_download"))
    }

    pub fn list_jobs(&self, query: ListDownloadJobsQueryDto) -> AppResult<DownloadJobListDto> {
        let _ = query;
        Err(not_wired("list_jobs"))
    }

    pub fn get_job_snapshot(
        &self,
        query: GetDownloadJobQueryDto,
    ) -> AppResult<Option<DownloadJobSnapshotDto>> {
        let _ = query;
        Err(not_wired("get_job_snapshot"))
    }

    pub fn get_policy(
        &self,
        query: GetDownloadPolicyQueryDto,
    ) -> AppResult<DownloadPolicyDto> {
        let _ = query;
        Err(not_wired("get_policy"))
    }

    pub fn update_policy(&self, request: UpdateDownloadPolicyRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("update_policy"))
    }
}

fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "DOWNLOADS_NOT_WIRED",
        format!("downloads facade operation `{operation}` is not wired in C2"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};
    use launcher_kernel_jobs::{JobPriority, JobProgress, JobSnapshot, JobState, JobUiState};

    use super::{
        DownloadFacade, DownloadJobRecord, DownloadJobRecordState, DownloadJobRepository,
        DownloadModuleDeps,
    };
    use crate::contracts::StartDownloadRequestDto;

    #[derive(Default)]
    struct RecordingDownloadJobRepository {
        created_jobs: Mutex<Vec<DownloadJobRecord>>,
    }

    impl RecordingDownloadJobRepository {
        fn created_jobs(&self) -> Vec<DownloadJobRecord> {
            self.created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .clone()
        }
    }

    impl DownloadJobRepository for RecordingDownloadJobRepository {
        fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()> {
            self.created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .push(job.clone());
            Ok(())
        }

        fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>> {
            Ok(self
                .created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .iter()
                .find(|job| &job.job_id == job_id)
                .cloned())
        }

        fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()> {
            if let Some(job) = self
                .created_jobs
                .lock()
                .expect("created jobs mutex should not be poisoned")
                .iter_mut()
                .find(|job| &job.job_id == job_id)
            {
                job.state = state;
            }

            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingJobRuntime {
        enqueued_requests: Mutex<Vec<launcher_kernel_jobs::EnqueueJobRequest<()>>>,
    }

    impl RecordingJobRuntime {
        fn enqueued_requests(&self) -> Vec<launcher_kernel_jobs::EnqueueJobRequest<()>> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .clone()
        }
    }

    impl launcher_kernel_jobs::JobRuntime for RecordingJobRuntime {
        type Extension = ();

        fn enqueue(&self, request: launcher_kernel_jobs::EnqueueJobRequest<Self::Extension>) -> AppResult<launcher_kernel_jobs::AcceptedJob> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .push(request.clone());

            Ok(launcher_kernel_jobs::AcceptedJob {
                job_id: request.job_id,
                module: request.module,
                kind: request.kind,
                queued_at: IsoDateTime::now(),
            })
        }

        fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
            Ok(Some(JobSnapshot {
                job_id: job_id.clone(),
                module: "downloads".into(),
                kind: "download".into(),
                state: JobState::Queued,
                ui_state: JobUiState::Queued,
                progress: JobProgress::pending(),
                recoverable: true,
                updated_at: IsoDateTime::now(),
                extension: None,
            }))
        }

        fn pause(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }

        fn resume(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }

        fn cancel(&self, _job_id: &JobId) -> AppResult<()> {
            Ok(())
        }
    }

    #[test]
    fn start_download_persists_request_metadata_and_enqueue_priority() {
        let job_repo = RecordingDownloadJobRepository::default();
        let runtime = RecordingJobRuntime::default();
        let facade = DownloadFacade::new(DownloadModuleDeps {
            job_repo,
            checkpoint_repo: (),
            manifest_provider: (),
            staging_store: (),
            job_runtime: runtime,
        });

        let request = StartDownloadRequestDto {
            target_id: "asset-123".into(),
            kind: "engine".into(),
            install_intent: Some("install".into()),
            priority: JobPriority::High,
        };

        let accepted = facade
            .start_download(request.clone())
            .expect("start_download should accept the job");

        let created_jobs = facade.deps().job_repo.created_jobs();
        let enqueued_requests = facade.deps().job_runtime.enqueued_requests();

        assert_eq!(created_jobs.len(), 1, "start_download should persist a download job record");
        assert_eq!(enqueued_requests.len(), 1, "start_download should enqueue one runtime job");

        let created_job = &created_jobs[0];
        assert_eq!(created_job.job_id, accepted.job_id);
        assert_eq!(created_job.target_id, request.target_id);
        assert_eq!(created_job.kind, request.kind);
        assert_eq!(created_job.install_intent, request.install_intent);
        assert_eq!(created_job.priority, request.priority);
        assert_eq!(created_job.state, DownloadJobRecordState::Queued);

        let enqueued_request = &enqueued_requests[0];
        assert_eq!(enqueued_request.job_id, accepted.job_id);
        assert_eq!(enqueued_request.priority, request.priority);
        assert_eq!(enqueued_request.kind, "download");
    }
}