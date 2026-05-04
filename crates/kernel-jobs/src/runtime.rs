use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
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

pub trait JobSnapshotStore<E>: Send + Sync {
    fn create(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    fn update(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<E>>>;
    fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<E>>>;
}

#[derive(Debug, Clone, Default)]
struct InMemoryJobSnapshotStore {
    snapshots: Arc<Mutex<HashMap<JobId, JobSnapshot<()>>>>,
}

impl JobSnapshotStore<()> for InMemoryJobSnapshotStore {
    fn create(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(snapshot.job_id.clone(), snapshot.clone());
        Ok(())
    }

    fn update(&self, snapshot: &JobSnapshot<()>) -> AppResult<()> {
        self.snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(snapshot.job_id.clone(), snapshot.clone());
        Ok(())
    }

    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<()>>> {
        Ok(self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(job_id)
            .cloned())
    }

    fn list_resumable(&self) -> AppResult<Vec<JobSnapshot<()>>> {
        let resumable_states = [
            JobState::Queued,
            JobState::ClaimingLease,
            JobState::Restoring,
            JobState::Running,
        ];
        Ok(self
            .snapshots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .values()
            .filter(|s| resumable_states.contains(&s.state))
            .cloned()
            .collect())
    }
}

#[derive(Clone)]
pub struct SharedJobRuntimeHost {
    policy: RuntimeQueuePolicy,
    snapshot_store: Arc<dyn JobSnapshotStore<()>>,
}

impl SharedJobRuntimeHost {
    pub fn new(policy: RuntimeQueuePolicy) -> Self {
        Self::with_store(policy, Arc::new(InMemoryJobSnapshotStore::default()))
    }

    pub fn with_store(policy: RuntimeQueuePolicy, snapshot_store: Arc<dyn JobSnapshotStore<()>>) -> Self {
        Self {
            policy,
            snapshot_store,
        }
    }

    pub fn policy(&self) -> RuntimeQueuePolicy {
        self.policy
    }
}

impl Debug for SharedJobRuntimeHost {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SharedJobRuntimeHost")
            .field("policy", &self.policy)
            .finish_non_exhaustive()
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
            recoverable: request.recoverable,
            updated_at: queued_at,
            extension: request.extension,
        };

        self.snapshot_store.create(&snapshot)?;

        Ok(accepted)
    }

    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
        self.snapshot_store.get(job_id)
    }

    fn pause(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Paused;
            snapshot.ui_state = JobUiState::Paused;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
        }

        Ok(())
    }

    fn resume(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Running;
            snapshot.ui_state = JobUiState::Running;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
        }

        Ok(())
    }

    fn cancel(&self, job_id: &JobId) -> AppResult<()> {
        if let Some(mut snapshot) = self.snapshot_store.get(job_id)? {
            snapshot.state = JobState::Canceled;
            snapshot.ui_state = JobUiState::Canceled;
            snapshot.updated_at = IsoDateTime::now();
            self.snapshot_store.update(&snapshot)?;
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
                recoverable: true,
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