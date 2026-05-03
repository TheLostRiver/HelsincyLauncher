use launcher_kernel_foundation::{AssetId, PageRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryListQueryDto {
    pub page: PageRequest,
    pub search: Option<String>,
    pub category_key: Option<String>,
    pub include_incompatible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabAssetDetailQueryDto {
    pub asset_id: AssetId,
}