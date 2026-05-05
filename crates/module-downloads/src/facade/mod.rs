//! downloads 模块的公开 facade 边界。
//!
//! 这里集中暴露下载任务的命令入口、最小持久化记录契约和依赖束，供
//! composition root 与宿主传输层装配。当前只有 `start_download` 已返回
//! 后端拥有的 accepted job；其余查询、暂停和策略入口仍保留 C2 阶段的
//! `DOWNLOADS_NOT_WIRED` stub 语义。

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

/// 下载任务在模块仓储里的粗粒度持久化状态。
///
/// 这层状态只服务于 downloads 自身的 intake 与恢复边界，不试图完整复制
/// 共享 job runtime 的更细粒度快照状态机。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadJobRecordState {
    /// 已创建并持久化，但尚未真正进入运行时执行。
    Queued,
    /// 已由运行时接管并处于执行中。
    Running,
    /// 已被显式暂停，等待后续恢复。
    Paused,
    /// 已完成并产出预期下载结果。
    Completed,
    /// 已失败，后续是否重试由更高层策略决定。
    Failed,
    /// 已被取消，不再继续推进。
    Canceled,
}

/// 下载任务在模块仓储中的最小持久化记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadJobRecord {
    /// 后端拥有的稳定 job 标识，用来关联 runtime 快照与模块记录。
    pub job_id: JobId,
    /// 用户请求要下载的业务目标标识，例如资产或引擎条目。
    pub target_id: String,
    /// 下载内容类型快照，供后续投影和安装链路区分用途。
    pub kind: String,
    /// 启动下载时携带的安装意图，避免 intake 元数据在入队前丢失。
    pub install_intent: Option<String>,
    /// 调度优先级快照，由 facade intake 固化到仓储记录中。
    pub priority: JobPriority,
    /// downloads 模块自己维护的粗粒度持久化状态。
    pub state: DownloadJobRecordState,
}

/// 持久化 downloads intake 记录的最小仓储边界。
pub trait DownloadJobRepository: Send + Sync {
    /// 在任务真正入队前写入最小记录，保证 intake 元数据先落盘。
    fn create_job(&self, job: &DownloadJobRecord) -> AppResult<()>;
    /// 按 job 标识读取模块自己的最小下载记录。
    fn get_job(&self, job_id: &JobId) -> AppResult<Option<DownloadJobRecord>>;
    /// 同步 downloads 模块拥有的粗粒度状态迁移结果。
    fn update_state(&self, job_id: &JobId, state: DownloadJobRecordState) -> AppResult<()>;
}

/// 组装 downloads facade 所需的依赖束。
#[derive(Debug, Clone)]
pub struct DownloadModuleDeps<J, C, M, S, R> {
    /// 保存 intake 元数据与粗粒度下载状态的模块仓储。
    pub job_repo: J,
    /// 为暂停、恢复和重启场景保留的 checkpoint 能力。
    pub checkpoint_repo: C,
    /// 负责解析 provider manifest 的适配器能力，留给后续切片接线。
    pub manifest_provider: M,
    /// 管理本地 staging 产物落盘位置的能力，留给后续切片接线。
    pub staging_store: S,
    /// 真正拥有 accepted job 生命周期的共享 job runtime。
    pub job_runtime: R,
}

/// downloads 模块对外暴露的 use-case facade。
pub struct DownloadFacade<J, C, M, S, R> {
    deps: DownloadModuleDeps<J, C, M, S, R>,
}

impl<J, C, M, S, R> DownloadFacade<J, C, M, S, R> {
    /// 用已经装配好的依赖束创建 downloads facade。
    pub fn new(deps: DownloadModuleDeps<J, C, M, S, R>) -> Self {
        Self { deps }
    }

    /// 暴露依赖束，主要服务于 composition-root smoke test 与局部诊断。
    pub fn deps(&self) -> &DownloadModuleDeps<J, C, M, S, R> {
        &self.deps
    }
}

impl<J: DownloadJobRepository, C, M, S, R: JobRuntime<Extension = ()>> DownloadFacade<J, C, M, S, R> {
    /// 记录用户下载意图并向共享 job runtime 提交后端拥有的下载任务。
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

    /// 预留暂停入口；当前仍由模块边界返回未接线错误。
    pub fn pause_download(&self, request: PauseDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("pause_download"))
    }

    /// 预留恢复入口；当前仍由模块边界返回未接线错误。
    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("resume_download"))
    }

    /// 预留取消入口；当前仍由模块边界返回未接线错误。
    pub fn cancel_download(&self, request: CancelDownloadRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("cancel_download"))
    }

    /// 预留任务列表查询；当前仍由模块边界返回未接线错误。
    pub fn list_jobs(&self, query: ListDownloadJobsQueryDto) -> AppResult<DownloadJobListDto> {
        let _ = query;
        Err(not_wired("list_jobs"))
    }

    /// 预留单任务快照查询；当前仍由模块边界返回未接线错误。
    pub fn get_job_snapshot(
        &self,
        query: GetDownloadJobQueryDto,
    ) -> AppResult<Option<DownloadJobSnapshotDto>> {
        let _ = query;
        Err(not_wired("get_job_snapshot"))
    }

    /// 预留下载策略读取；当前仍由模块边界返回未接线错误。
    pub fn get_policy(
        &self,
        query: GetDownloadPolicyQueryDto,
    ) -> AppResult<DownloadPolicyDto> {
        let _ = query;
        Err(not_wired("get_policy"))
    }

    /// 预留下载策略更新；当前仍由模块边界返回未接线错误。
    pub fn update_policy(&self, request: UpdateDownloadPolicyRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("update_policy"))
    }
}

// 在 facade 边界统一保留 C2 阶段的未接线错误码，避免宿主层自己发明 fallback。
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