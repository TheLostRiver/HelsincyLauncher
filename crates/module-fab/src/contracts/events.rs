use serde::{Deserialize, Serialize};

/// 表示 Fab 库存同步链路对外广播的事件联合。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FabInventoryEventDto {
    /// 表示后端已接受一次库存同步触发请求。
    SyncRequested { trigger: String },
    /// 表示库存投影已经按最新同步结果刷新。
    InventoryProjectionRefreshed { item_count: usize },
}