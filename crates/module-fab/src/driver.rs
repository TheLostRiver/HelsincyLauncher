//! Fab 当前已注册后台作业类型的恢复 driver。
//!
//! 这些 driver 只服务 startup stage-2 对既有 resumable snapshot 的判定。当前 Fab
//! baseline 里的两个作业类型都可以通过重新入队恢复，不需要在此处读取额外业务
//! checkpoint。

use launcher_kernel_foundation::AppResult;
use launcher_kernel_jobs::{JobDriver, JobSnapshot, RestoreDisposition};

/// `fab/inventory_startup_prewarm` 作业类型的恢复 driver。
///
/// 启动预热在当前 baseline 下保持幂等，因此已持久化的 queued job 可以在重启后
/// 重新进入 runtime，并标记为 `Resumed`。
pub struct FabPrewarmJobDriver;

impl JobDriver<()> for FabPrewarmJobDriver {
    fn module(&self) -> &'static str {
        "fab"
    }

    fn kind(&self) -> &'static str {
        "inventory_startup_prewarm"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        Ok(RestoreDisposition::Resumed)
    }
}

/// `fab/inventory_sync` 作业类型的恢复 driver。
///
/// 增量同步的 cursor 状态位于此 driver 边界以下的 Fab 持久化层；当前恢复判定只需要
/// 让共享 runtime 重新接管该 queued job，不在 driver 内复制业务 checkpoint。
pub struct FabSyncJobDriver;

impl JobDriver<()> for FabSyncJobDriver {
    fn module(&self) -> &'static str {
        "fab"
    }

    fn kind(&self) -> &'static str {
        "inventory_sync"
    }

    fn restore(&self, _snapshot: &JobSnapshot<()>) -> AppResult<RestoreDisposition> {
        Ok(RestoreDisposition::Resumed)
    }
}
