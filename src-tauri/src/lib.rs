pub mod bootstrap;
pub mod commands {
	use launcher_composition_root::DesktopAppServices;
	use launcher_kernel_foundation::{AppError, AppResult, IsoDateTime};
	use launcher_kernel_jobs::AcceptedJob;

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct AppErrorDto {
		pub code: String,
		pub message: String,
		pub retryable: bool,
		pub severity: String,
		pub correlation_id: String,
	}

	impl From<AppError> for AppErrorDto {
		fn from(error: AppError) -> Self {
			Self {
				code: error.code,
				message: error.message,
				retryable: error.retryable,
				severity: format!("{:?}", error.severity).to_lowercase(),
				correlation_id: error.correlation_id.to_string(),
			}
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum CommandResultDto<T> {
		Success { data: T },
		Failure { error: AppErrorDto },
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum QueryResultDto<T> {
		Success { data: T, as_of: Option<IsoDateTime> },
		Failure { error: AppErrorDto },
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct AcceptedJobDto {
		pub accepted: bool,
		pub job_id: launcher_kernel_foundation::JobId,
		pub module: String,
		pub kind: String,
		pub queued_at: IsoDateTime,
	}

	impl From<AcceptedJob> for AcceptedJobDto {
		fn from(job: AcceptedJob) -> Self {
			Self {
				accepted: true,
				job_id: job.job_id,
				module: job.module,
				kind: job.kind,
				queued_at: job.queued_at,
			}
		}
	}

	pub type DesktopServices = DesktopAppServices;

	pub fn map_command_result<T>(result: AppResult<T>) -> CommandResultDto<T> {
		match result {
			Ok(data) => CommandResultDto::Success { data },
			Err(error) => CommandResultDto::Failure {
				error: error.into(),
			},
		}
	}

	pub fn map_query_result<T>(result: AppResult<T>) -> QueryResultDto<T> {
		match result {
			Ok(data) => QueryResultDto::Success {
				data,
				as_of: Some(IsoDateTime::now()),
			},
			Err(error) => QueryResultDto::Failure {
				error: error.into(),
			},
		}
	}

	pub fn map_query_result_or_stub<T>(
		result: AppResult<T>,
		not_wired_code: &str,
		stub: impl FnOnce() -> T,
	) -> QueryResultDto<T> {
		match result {
			Ok(data) => QueryResultDto::Success {
				data,
				as_of: Some(IsoDateTime::now()),
			},
			Err(error) if error.code == not_wired_code => QueryResultDto::Success {
				data: stub(),
				as_of: Some(IsoDateTime::now()),
			},
			Err(error) => QueryResultDto::Failure {
				error: error.into(),
			},
		}
	}

	pub fn map_accepted_job_result(
		result: AppResult<AcceptedJob>,
	) -> CommandResultDto<AcceptedJobDto> {
		match result {
			Ok(job) => CommandResultDto::Success { data: job.into() },
			Err(error) => CommandResultDto::Failure {
				error: error.into(),
			},
		}
	}

	#[path = "fab.rs"]
	pub mod fab;

	#[path = "downloads.rs"]
	pub mod downloads;
}
pub mod state;

pub use bootstrap::{build_desktop_host_bootstrap, run_desktop_host, DesktopHostBootstrap};
pub use state::DesktopAppServicesHandle;