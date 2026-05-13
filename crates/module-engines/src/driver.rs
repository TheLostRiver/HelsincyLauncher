//! Engines 模块的校验恢复 driver 入口。
//!
//! 该边界把 `engines/verification` 作业种类收口到当前基线的恢复实现，
//! 让占位恢复策略只在这一处声明，而不是分散到 facade 或宿主侧。

use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// 表示当前 `engines/verification` 作业种类的占位恢复 driver。
///
/// 当前基线总是返回 `Resumed`，真实恢复逻辑会在后续切片中检查缓存和验证状态。
pub struct EngineJobDriver;

impl JobDriver<()> for EngineJobDriver {
    fn module(&self) -> &'static str {
        "engines"
    }

    fn kind(&self) -> &'static str {
        "verification"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        // TODO: AT-054 - 检查引擎缓存和验证状态后再决定恢复处置。
        // 当前占位实现保持可恢复，以便 startup stage 2 能先验证 driver 接线。
        Ok(RestoreDisposition::Resumed)
    }
}
