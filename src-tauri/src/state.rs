//! 宿主拥有的 composition-root 桌面服务聚合包装。
//!
//! 桌面 command 层只保存 `Arc` 支撑的句柄，不自行拥有或重建服务，
//! 这样 transport handler 可以共享同一份稳定服务图，而不会引入第二套后端真相。

use std::ops::Deref;
use std::sync::Arc;

use crate::commands::DesktopServices;

/// 表示从 composition-root 桌面服务聚合投影出来的可克隆宿主状态句柄。
#[derive(Clone)]
pub struct DesktopAppServicesHandle {
    services: Arc<DesktopServices>,
}

impl DesktopAppServicesHandle {
    /// 把已经装配好的桌面服务包装成可共享的宿主句柄。
    pub fn from_services(services: DesktopServices) -> Self {
        Self {
            services: Arc::new(services),
        }
    }

    /// 返回底层桌面服务引用，不转移服务图所有权。
    pub fn services(&self) -> &DesktopServices {
        self.services.as_ref()
    }

    /// 表示当前宿主句柄由 composition-root 服务图支撑。
    pub fn is_wired_to_composition_root(&self) -> bool {
        true
    }
}

impl Deref for DesktopAppServicesHandle {
    type Target = DesktopServices;

    fn deref(&self) -> &Self::Target {
        self.services.as_ref()
    }
}
