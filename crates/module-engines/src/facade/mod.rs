use launcher_kernel_foundation::{AppError, AppErrorSeverity, AppResult, CorrelationId, JobId};
use launcher_kernel_jobs::{AcceptedJob, EnqueueJobRequest, JobPriority, JobRuntime};

use crate::contracts::{
    GetEngineStatusRequestDto, ListEnginesRequestDto, RunEngineVerificationRequestDto,
};

#[derive(Debug, Clone)]
/// Engines 模块在 facade 层消费的依赖集合。
pub struct EngineModuleDeps<J, C, R> {
    /// 预留给引擎作业事实持久化的仓储依赖。
    pub job_repo: J,
    /// 预留给引擎校验检查点持久化的仓储依赖。
    pub checkpoint_repo: C,
    /// 接受引擎校验作业并返回受理结果的运行时依赖。
    pub job_runtime: R,
}

/// Engines 模块对外暴露的查询与校验受理边界。
pub struct EngineFacade<J, C, R> {
    deps: EngineModuleDeps<J, C, R>,
}

impl<J, C, R> EngineFacade<J, C, R> {
    /// 用已组装好的依赖集合创建 facade 实例。
    pub fn new(deps: EngineModuleDeps<J, C, R>) -> Self {
        Self { deps }
    }

    /// 暴露当前 facade 持有的依赖集合，供外层装配或测试读取。
    pub fn deps(&self) -> &EngineModuleDeps<J, C, R> {
        &self.deps
    }
}

impl<J, C, R: JobRuntime<Extension = ()>> EngineFacade<J, C, R> {
    /// 列出当前可见引擎；现阶段仍保留 `ENGINES_NOT_WIRED` 占位返回。
    pub fn list_engines(&self, request: ListEnginesRequestDto) -> AppResult<Vec<String>> {
        let _ = request;
        Err(not_wired("list_engines"))
    }

    /// 读取单个引擎状态；现阶段仍保留 `ENGINES_NOT_WIRED` 占位返回。
    pub fn get_status(&self, request: GetEngineStatusRequestDto) -> AppResult<String> {
        let _ = request;
        Err(not_wired("get_status"))
    }

    /// 提交引擎校验作业并返回后端拥有的受理结果。
    pub fn run_verification(&self, request: RunEngineVerificationRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        self.deps.job_runtime.enqueue(EnqueueJobRequest {
            job_id: JobId::generate(),
            module: "engines".into(),
            kind: "verification".into(),
            priority: JobPriority::Normal,
            recoverable: true,
            extension: None,
        })
    }
}

fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "ENGINES_NOT_WIRED",
        format!("engines facade operation `{operation}` is not wired in C2"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AppResult, IsoDateTime, JobId};
    use launcher_kernel_jobs::{
        AcceptedJob, EnqueueJobRequest, JobRuntime, JobSnapshot,
    };

    use super::{EngineFacade, EngineModuleDeps};
    use crate::contracts::RunEngineVerificationRequestDto;

    #[derive(Default)]
    struct RecordingJobRuntime {
        enqueued_requests: Mutex<Vec<EnqueueJobRequest<()>>>,
    }

    impl RecordingJobRuntime {
        fn enqueued_requests(&self) -> Vec<EnqueueJobRequest<()>> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .clone()
        }
    }

    impl JobRuntime for RecordingJobRuntime {
        type Extension = ();

        fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob> {
            self.enqueued_requests
                .lock()
                .expect("enqueued requests mutex should not be poisoned")
                .push(request.clone());

            Ok(AcceptedJob {
                job_id: request.job_id,
                module: request.module,
                kind: request.kind,
                queued_at: IsoDateTime::now(),
            })
        }

        fn snapshot(&self, _job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>> {
            Ok(None)
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
    fn run_verification_returns_backend_owned_accepted_job() {
        let facade = EngineFacade::new(EngineModuleDeps {
            job_repo: (),
            checkpoint_repo: (),
            job_runtime: RecordingJobRuntime::default(),
        });

        let accepted = facade
            .run_verification(RunEngineVerificationRequestDto {
                engine_id: "ue-5.4".into(),
            })
            .expect("run_verification should enqueue an engine verification job");

        let enqueued_requests = facade.deps().job_runtime.enqueued_requests();
        assert_eq!(enqueued_requests.len(), 1, "one engine verification job should be enqueued");

        let request = &enqueued_requests[0];
        assert_eq!(request.job_id, accepted.job_id);
        assert_eq!(request.module, "engines");
        assert_eq!(request.kind, "verification");
        assert_eq!(request.priority, JobPriority::Normal);
        assert!(request.recoverable, "engine verification jobs should stay recoverable by default");
    }
}
