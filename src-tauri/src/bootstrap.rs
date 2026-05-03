use crate::state::DesktopAppServicesHandle;

#[derive(Debug, Clone)]
pub struct DesktopHostBootstrap {
    pub services: DesktopAppServicesHandle,
    pub registered_commands: &'static [&'static str],
}

pub fn build_desktop_host_bootstrap() -> DesktopHostBootstrap {
    DesktopHostBootstrap {
        services: DesktopAppServicesHandle::placeholder(),
        registered_commands: &["desktop_health"],
    }
}

pub fn run_desktop_host() {
    let _bootstrap = build_desktop_host_bootstrap();
}