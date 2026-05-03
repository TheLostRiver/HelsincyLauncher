use launcher_kernel_foundation::{AppResult, JobId};

use crate::model::{AcceptedJob, EnqueueJobRequest, JobSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeQueuePolicy {
    pub max_concurrent_jobs: usize,
}

impl RuntimeQueuePolicy {
    pub fn new(max_concurrent_jobs: usize) -> Self {
        Self {
            max_concurrent_jobs,
        }
    }
}

pub trait JobRuntime: Send + Sync {
    type Extension;

    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob>;
    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>>;
    fn pause(&self, job_id: &JobId) -> AppResult<()>;
    fn resume(&self, job_id: &JobId) -> AppResult<()>;
    fn cancel(&self, job_id: &JobId) -> AppResult<()>;
}