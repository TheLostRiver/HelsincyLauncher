//! Host-owned wrapper around the composition-root desktop service aggregation.
//!
//! The desktop command layer keeps an `Arc`-backed handle instead of owning or
//! reconstructing services itself, so transport handlers can share one stable
//! service graph without introducing a second source of backend truth.

use std::ops::Deref;
use std::sync::Arc;

use crate::commands::DesktopServices;

/// Cloneable host state handle projected from the composition-root desktop services.
#[derive(Clone)]
pub struct DesktopAppServicesHandle {
    services: Arc<DesktopServices>,
}

impl DesktopAppServicesHandle {
    /// Wraps the assembled desktop services in a shareable host handle.
    pub fn from_services(services: DesktopServices) -> Self {
        Self {
            services: Arc::new(services),
        }
    }

    /// Returns the underlying desktop services without transferring ownership.
    pub fn services(&self) -> &DesktopServices {
        self.services.as_ref()
    }

    /// Indicates that this host handle is backed by the composition-root service graph.
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