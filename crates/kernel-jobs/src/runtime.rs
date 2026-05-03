use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};

use crate::model::{
    AcceptedJob, EnqueueJobRequest, JobProgress, JobSnapshot, JobState, JobUiState,
};

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

#[derive(Debug, Clone)]
pub struct SharedJobRuntimeHost {
    policy: RuntimeQueuePolicy,
    snapshots: Arc<Mutex<HashMap<JobId, JobSnapshot<()>>>>,
}

impl SharedJobRuntimeHost {
    pub fn new(policy: RuntimeQueuePolicy) -> Self {
        Self {
            policy,
            snapshots: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn policy(&self) -> RuntimeQueuePolicy {
        self.policy
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

impl JobRuntime for SharedJobRuntimeHost {
    type Extension = ();

    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob> {
        let queued_at = IsoDateTime::now();
        let accepted = AcceptedJob {
            job_id: request.job_id.clone(),
            module: request.module.clone(),
            kind: request.kind.clone(),
            queued_at: queued_at.clone(),
        };
        let snapshot = JobSnapshot {
            job_id: request.job_id.clone(),
            module: request.module,
            kind: request.kind,
            state: JobState::Queued,
            ui_state: JobUiState::Queued,
            progress: JobProgress::pending(),
            updated_at: queued_at,
            extension: request.extension,
        };

        let mut snapshots = self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        snapshots.insert(snapshot.job_id.clone(), snapshot);

        Ok(accepted)
    }

    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
        let snapshots = self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        Ok(snapshots.get(job_id).cloned())
    }

    fn pause(&self, job_id: &JobId) -> AppResult<()> {
        let mut snapshots = self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(snapshot) = snapshots.get_mut(job_id) {
            snapshot.state = JobState::Paused;
            snapshot.ui_state = JobUiState::Paused;
            snapshot.updated_at = IsoDateTime::now();
        }

        Ok(())
    }

    fn resume(&self, job_id: &JobId) -> AppResult<()> {
        let mut snapshots = self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(snapshot) = snapshots.get_mut(job_id) {
            snapshot.state = JobState::Running;
            snapshot.ui_state = JobUiState::Running;
            snapshot.updated_at = IsoDateTime::now();
        }

        Ok(())
    }

    fn cancel(&self, job_id: &JobId) -> AppResult<()> {
        let mut snapshots = self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(snapshot) = snapshots.get_mut(job_id) {
            snapshot.state = JobState::Canceled;
            snapshot.ui_state = JobUiState::Canceled;
            snapshot.updated_at = IsoDateTime::now();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use launcher_kernel_foundation::JobId;

    use super::{JobRuntime, RuntimeQueuePolicy, SharedJobRuntimeHost};
    use crate::{EnqueueJobRequest, JobPriority, JobState, JobUiState};

    #[test]
    fn shared_job_runtime_host_records_enqueued_snapshot() {
        let runtime = SharedJobRuntimeHost::new(RuntimeQueuePolicy::new(3));
        let job_id = JobId::generate();

        let accepted = runtime
            .enqueue(EnqueueJobRequest {
                job_id: job_id.clone(),
                module: "fab".into(),
                kind: "inventory_sync".into(),
                priority: JobPriority::Normal,
                extension: None,
            })
            .expect("shared runtime host should accept enqueued jobs");

        let snapshot = runtime
            .snapshot(&job_id)
            .expect("shared runtime host should expose enqueued snapshots")
            .expect("shared runtime host should store the queued snapshot");

        assert_eq!(accepted.job_id, job_id);
        assert_eq!(snapshot.state, JobState::Queued);
        assert_eq!(snapshot.ui_state, JobUiState::Queued);
        assert_eq!(snapshot.module, "fab");
        assert_eq!(snapshot.kind, "inventory_sync");
    }
}