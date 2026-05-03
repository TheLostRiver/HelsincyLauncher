#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesktopAppServicesHandle {
    wired_to_composition_root: bool,
}

impl DesktopAppServicesHandle {
    pub fn placeholder() -> Self {
        Self {
            wired_to_composition_root: false,
        }
    }

    pub fn is_wired_to_composition_root(&self) -> bool {
        self.wired_to_composition_root
    }
}