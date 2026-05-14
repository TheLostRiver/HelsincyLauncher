//! 当前 Tauri 后端基线的桌面宿主 bootstrap 入口。
//!
//! 该模块刻意保持很薄：它请求 composition-root 装配具体服务图，
//! 再把服务图投影成宿主拥有的句柄，并暴露桌面 transport 层需要发布的 command 列表。

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};
use launcher_kernel_foundation::AppResult;

use crate::{commands, state::DesktopAppServicesHandle};

/// 表示 composition-root 接线成功后返回给宿主的 bootstrap 聚合结果。
#[derive(Clone)]
pub struct DesktopHostBootstrap {
    /// 投影成宿主 state handle 的桌面服务集合，供 command handler 消费。
    pub services: DesktopAppServicesHandle,

    /// 宿主应在桌面 transport 边界注册的 command 名称列表。
    pub registered_commands: &'static [&'static str],
}

/// 基于当前 composition-root 基线构造桌面宿主 bootstrap。
pub fn build_desktop_host_bootstrap() -> AppResult<DesktopHostBootstrap> {
    let services = build_desktop_services(DesktopBootstrapConfig::default())?;

    Ok(DesktopHostBootstrap {
        services: DesktopAppServicesHandle::from_services(services),
        registered_commands: commands::REGISTERED_COMMANDS,
    })
}

/// 运行当前桌面宿主启动路径。
///
/// 当前基线只验证宿主 bootstrap 接线可以成功；更完整的 Tauri runtime loop
/// 仍按计划留给后续切片接入。
pub fn run_desktop_host() -> AppResult<()> {
    let _bootstrap = build_desktop_host_bootstrap()?;
    Ok(())
}
