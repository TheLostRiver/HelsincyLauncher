//! 桌面宿主的公开 crate 入口。
//!
//! 该入口重新导出可测试的 bootstrap 入口和共享宿主状态句柄，
//! 供二进制入口与宿主 transport smoke tests 使用。

/// 桌面宿主 bootstrap 装配与启动入口。
pub mod bootstrap;

/// typed IPC command/query handler 与共享 transport envelope。
pub mod commands;

/// 围绕 composition-root 服务聚合的共享宿主状态包装。
pub mod state;

/// 为调用方和测试重新导出可测试的桌面宿主 bootstrap surface。
pub use bootstrap::{build_desktop_host_bootstrap, run_desktop_host, DesktopHostBootstrap};

/// 重新导出挂载到 state 的共享桌面宿主服务句柄。
pub use state::DesktopAppServicesHandle;
