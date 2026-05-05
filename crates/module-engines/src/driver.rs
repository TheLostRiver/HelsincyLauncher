//! Engines 模块的校验恢复 driver 入口。
//!
//! 该边界把 `engines/verification` 作业种类收口到当前基线的恢复实现，
//! 让占位恢复策略只在这一处声明，而不是分散到 facade 或宿主侧。

use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// Stub restore driver for the `engines/verification` job kind.
///
/// Always returns `Resumed` for engine verification jobs.
/// Real restoration logic will check cache and verification state in a later slice.
pub struct EngineJobDriver;

impl JobDriver<()> for EngineJobDriver {
    fn module(&self) -> &'static str {
        "engines"
    }

    fn kind(&self) -> &'static str {
        "verification"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        // TODO: AT-054 — Check engine cache and verification state for recovery
        // For now, stub returns Resumed.
        Ok(RestoreDisposition::Resumed)
    }
}
