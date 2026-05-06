use launcher_kernel_foundation::{AssetId, PageSlice};
use serde::{Deserialize, Serialize};

/// 描述 Fab 库存列表里单个资产摘要项的后端投影快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryItemDto {
    pub asset_id: AssetId,
    pub title: String,
    pub seller_name: String,
    pub category_key: Option<String>,
    pub engine_support_summary: Vec<String>,
}

/// 表示按页返回的 Fab 库存摘要结果。
pub type FabInventoryPageDto = PageSlice<FabInventoryItemDto>;

/// 描述单个 Fab 资产详情页读取路径返回的后端投影快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabAssetDetailDto {
    pub asset_id: AssetId,
    pub title: String,
    pub description: String,
    pub seller_name: String,
    pub compatibility_notes: Vec<String>,
    pub dependency_warnings: Vec<String>,
}