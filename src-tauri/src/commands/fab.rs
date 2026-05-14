//! Fab 模块的宿主 transport handler。
//!
//! 该边界把 IPC 侧 Fab 查询和命令转接到 `DesktopServices` 暴露的后端 Fab facade。
//! 查询路径在 facade/read model 尚未完全接线时保留 `FAB_NOT_WIRED` fallback 投影，
//! 命令路径把后端 accepted job 投影进共享宿主 transport envelope。

use launcher_kernel_foundation::PageSlice;
use launcher_module_fab::contracts::{
    FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryListQueryDto, FabInventoryPageDto,
    FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto,
};

use super::{
    map_accepted_job_result, map_query_result_or_stub, CommandResultDto, DesktopServices,
    QueryResultDto,
};

/// 读取 Fab 库存摘要分页，并在当前 host stub 路径上回退为空分页。
pub async fn fab_list_inventory(
    services: &DesktopServices,
    query: FabInventoryListQueryDto,
) -> QueryResultDto<FabInventoryPageDto> {
    map_query_result_or_stub(services.fab.list_inventory(query), "FAB_NOT_WIRED", || {
        PageSlice::new(Vec::new(), None)
    })
}

/// 读取单个 Fab 资产详情投影，并在当前 transport stub 路径上回退为占位详情。
pub async fn fab_get_asset_detail(
    services: &DesktopServices,
    query: FabAssetDetailQueryDto,
) -> QueryResultDto<FabAssetDetailDto> {
    map_query_result_or_stub(services.fab.get_asset_detail(query.clone()), "FAB_NOT_WIRED", || {
        FabAssetDetailDto {
            asset_id: query.asset_id,
            title: "Fab asset detail not wired yet".into(),
            description: "E1 transport shell placeholder response.".into(),
            seller_name: "unknown".into(),
            compatibility_notes: Vec::new(),
            dependency_warnings: vec!["fab detail remains a stub until module wiring lands".into()],
        }
    })
}

/// 通过后端 Fab facade 启动库存预热作业，并投影 accepted-job envelope。
pub async fn fab_run_startup_prewarm(
    services: &DesktopServices,
    request: FabInventoryPrewarmRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.fab.run_startup_prewarm(request))
}

/// 通过后端 Fab facade 启动库存同步作业，并投影 accepted-job envelope。
pub async fn fab_sync_inventory(
    services: &DesktopServices,
    request: FabInventorySyncRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.fab.sync_inventory(request))
}
