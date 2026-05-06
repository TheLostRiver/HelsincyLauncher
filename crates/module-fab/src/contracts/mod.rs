//! Fab 模块的公开 contracts 聚合入口。
//!
//! 该文件把命令 DTO、查询 DTO、事件 DTO 与只读投影 DTO 收口到同一条稳定边界，
//! 让 facade、宿主 transport 与上层装配只依赖模块公开 contracts 表面，而不必分散导入内部子模块路径。

pub mod commands;
pub mod dto;
pub mod events;
pub mod queries;

pub use commands::{FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto};
pub use dto::{FabAssetDetailDto, FabInventoryItemDto, FabInventoryPageDto};
pub use events::FabInventoryEventDto;
pub use queries::{FabAssetDetailQueryDto, FabInventoryListQueryDto};