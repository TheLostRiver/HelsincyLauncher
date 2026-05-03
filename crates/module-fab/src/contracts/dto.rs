use launcher_kernel_foundation::{AssetId, PageSlice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryItemDto {
    pub asset_id: AssetId,
    pub title: String,
    pub seller_name: String,
    pub category_key: Option<String>,
    pub engine_support_summary: Vec<String>,
}

pub type FabInventoryPageDto = PageSlice<FabInventoryItemDto>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabAssetDetailDto {
    pub asset_id: AssetId,
    pub title: String,
    pub description: String,
    pub seller_name: String,
    pub compatibility_notes: Vec<String>,
    pub dependency_warnings: Vec<String>,
}