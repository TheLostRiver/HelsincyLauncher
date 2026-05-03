use std::sync::Arc;

use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopBootstrapConfig {
    pub profile: String,
}

impl DesktopBootstrapConfig {
    pub fn new(profile: impl Into<String>) -> Self {
        Self {
            profile: profile.into(),
        }
    }
}

impl Default for DesktopBootstrapConfig {
    fn default() -> Self {
        Self::new("desktop")
    }
}

#[derive(Debug, Clone, Default)]
pub struct StartupPipelineFacade;

impl StartupPipelineFacade {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct DesktopAppServices<F = (), D = ()> {
    pub fab: Arc<F>,
    pub downloads: Arc<D>,
    pub startup: Arc<StartupPipelineFacade>,
}

impl<F, D> DesktopAppServices<F, D> {
    pub fn new(fab: Arc<F>, downloads: Arc<D>, startup: Arc<StartupPipelineFacade>) -> Self {
        Self {
            fab,
            downloads,
            startup,
        }
    }
}

pub fn build_desktop_services(config: DesktopBootstrapConfig) -> AppResult<DesktopAppServices> {
    let _ = config;
    Err(not_wired("build_desktop_services"))
}

fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "COMPOSITION_ROOT_NOT_WIRED",
        format!("composition-root operation `{operation}` is not wired in D1"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}