use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};
use launcher_kernel_foundation::AppResult;

use crate::{commands, state::DesktopAppServicesHandle};

#[derive(Clone)]
pub struct DesktopHostBootstrap {
    pub services: DesktopAppServicesHandle,
    pub registered_commands: &'static [&'static str],
}

pub fn build_desktop_host_bootstrap() -> AppResult<DesktopHostBootstrap> {
    let services = build_desktop_services(DesktopBootstrapConfig::default())?;

    Ok(DesktopHostBootstrap {
        services: DesktopAppServicesHandle::from_services(services),
        registered_commands: commands::REGISTERED_COMMANDS,
    })
}

pub fn run_desktop_host() -> AppResult<()> {
    let _bootstrap = build_desktop_host_bootstrap()?;
    Ok(())
}