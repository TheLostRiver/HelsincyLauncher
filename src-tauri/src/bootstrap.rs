//! Desktop host bootstrap entry for the current Tauri backend baseline.
//!
//! This module is intentionally small: it asks composition-root to assemble the
//! concrete service graph, projects that graph into a host-owned handle, and
//! exposes the registered command list that the desktop transport layer should
//! publish.

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};
use launcher_kernel_foundation::AppResult;

use crate::{commands, state::DesktopAppServicesHandle};

/// Host-owned bootstrap bundle returned after composition-root wiring succeeds.
#[derive(Clone)]
pub struct DesktopHostBootstrap {
    /// Desktop services projected into the host state handle consumed by commands.
    pub services: DesktopAppServicesHandle,

    /// Command names that the host should register on the desktop transport boundary.
    pub registered_commands: &'static [&'static str],
}

/// Builds the current desktop host bootstrap from the composition-root baseline.
pub fn build_desktop_host_bootstrap() -> AppResult<DesktopHostBootstrap> {
    let services = build_desktop_services(DesktopBootstrapConfig::default())?;

    Ok(DesktopHostBootstrap {
        services: DesktopAppServicesHandle::from_services(services),
        registered_commands: commands::REGISTERED_COMMANDS,
    })
}

/// Runs the current desktop host startup path.
///
/// The present baseline only verifies that host bootstrap wiring succeeds; the
/// wider Tauri runtime loop is intentionally deferred to a later slice.
pub fn run_desktop_host() -> AppResult<()> {
    let _bootstrap = build_desktop_host_bootstrap()?;
    Ok(())
}