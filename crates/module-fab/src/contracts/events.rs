use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FabInventoryEventDto {
    SyncRequested { trigger: String },
    InventoryProjectionRefreshed { item_count: usize },
}