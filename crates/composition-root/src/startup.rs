use launcher_kernel_foundation::AppResult;

#[derive(Debug, Clone, Default)]
pub struct StartupPipelineFacade;

impl StartupPipelineFacade {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_stage1_shell_ready(&self) -> AppResult<()> {
        Ok(())
    }

    pub async fn run_stage2_restore_runtime_state(&self) -> AppResult<()> {
        Ok(())
    }

    pub async fn run_stage3_background_prewarm(&self) -> AppResult<()> {
        Ok(())
    }
}