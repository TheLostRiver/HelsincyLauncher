use std::ops::Deref;
use std::sync::Arc;

use crate::commands::DesktopServices;

#[derive(Clone)]
pub struct DesktopAppServicesHandle {
    services: Arc<DesktopServices>,
}

impl DesktopAppServicesHandle {
    pub fn from_services(services: DesktopServices) -> Self {
        Self {
            services: Arc::new(services),
        }
    }

    pub fn services(&self) -> &DesktopServices {
        self.services.as_ref()
    }

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