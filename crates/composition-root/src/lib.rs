//! composition-root crate 入口，集中导出桌面后端装配与启动管线边界。
//!
//! 该 crate 是当前桌面宿主的唯一装配 owner；上层只能拿到 facade-only 服务聚合，
//! 不能绕过这里直接接触具体 adapter、runtime 或 repository 类型。

pub mod bootstrap;
pub mod startup;

pub use bootstrap::{build_desktop_services, DesktopAppServices, DesktopBootstrapConfig};
pub use startup::StartupPipelineFacade;
