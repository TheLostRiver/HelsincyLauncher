use launcher_kernel_foundation::{AssetId, PageRequest};
use serde::{Deserialize, Serialize};

/// 描述读取 Fab 库存分页列表时提交给后端的查询条件。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryListQueryDto {
    pub page: PageRequest,
    pub search: Option<String>,
    pub category_key: Option<String>,
    pub include_incompatible: bool,
}

/// 描述按资产标识读取 Fab 资产详情时提交给后端的查询条件。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabAssetDetailQueryDto {
    pub asset_id: AssetId,
}