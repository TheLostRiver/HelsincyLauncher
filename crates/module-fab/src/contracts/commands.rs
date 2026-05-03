use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryPrewarmRequestDto {
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventorySyncRequestDto {
    pub trigger: String,
    pub force_full_sync: bool,
}