use launcher_kernel_foundation::{AppError, AppErrorSeverity, AppResult, CorrelationId};
use launcher_kernel_jobs::JobRuntime;

use crate::contracts::{
    GetEngineStatusRequestDto, ListEnginesRequestDto, RunEngineVerificationRequestDto,
};

#[derive(Debug, Clone)]
pub struct EngineModuleDeps<J, C, R> {
    pub job_repo: J,
    pub checkpoint_repo: C,
    pub job_runtime: R,
}

pub struct EngineFacade<J, C, R> {
    deps: EngineModuleDeps<J, C, R>,
}

impl<J, C, R> EngineFacade<J, C, R> {
    pub fn new(deps: EngineModuleDeps<J, C, R>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &EngineModuleDeps<J, C, R> {
        &self.deps
    }
}

impl<J, C, R: JobRuntime<Extension = ()>> EngineFacade<J, C, R> {
    pub fn list_engines(&self, request: ListEnginesRequestDto) -> AppResult<Vec<String>> {
        let _ = request;
        Err(not_wired("list_engines"))
    }

    pub fn get_status(&self, request: GetEngineStatusRequestDto) -> AppResult<String> {
        let _ = request;
        Err(not_wired("get_status"))
    }

    pub fn run_verification(&self, request: RunEngineVerificationRequestDto) -> AppResult<()> {
        let _ = request;
        Err(not_wired("run_verification"))
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
