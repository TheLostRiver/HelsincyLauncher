use serde::{Deserialize, Serialize};

/// 描述启动阶段触发 Fab 库存预热时提交给后端的命令负载。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventoryPrewarmRequestDto {
    pub reason: String,
}

/// 描述手动或后台触发 Fab 库存同步时提交给后端的命令负载。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabInventorySyncRequestDto {
    pub trigger: String,
    pub force_full_sync: bool,
}