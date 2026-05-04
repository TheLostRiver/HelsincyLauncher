//! Host transport handlers for the Fab module.
//!
//! This boundary adapts IPC-facing Fab queries and commands onto the backend
//! Fab facade exposed through `DesktopServices`. The query handlers still own
//! temporary `FAB_NOT_WIRED` fallback projections, while command handlers
//! project backend-owned accepted jobs into the shared host transport envelope.

use launcher_kernel_foundation::PageSlice;
use launcher_module_fab::contracts::{
    FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryListQueryDto, FabInventoryPageDto,
    FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto,
};

use super::{
    map_accepted_job_result, map_query_result_or_stub, CommandResultDto, DesktopServices,
    QueryResultDto,
};

/// Lists Fab inventory summaries and falls back to an empty page on the current host stub path.
pub async fn fab_list_inventory(
    services: &DesktopServices,
    query: FabInventoryListQueryDto,
) -> QueryResultDto<FabInventoryPageDto> {
    map_query_result_or_stub(services.fab.list_inventory(query), "FAB_NOT_WIRED", || {
        PageSlice::new(Vec::new(), None)
    })
}

/// Gets one Fab asset detail projection and falls back to the current transport-owned placeholder detail.
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

/// Starts startup prewarm through the backend Fab facade and projects the accepted job envelope.
pub async fn fab_run_startup_prewarm(
    services: &DesktopServices,
    request: FabInventoryPrewarmRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.fab.run_startup_prewarm(request))
}

/// Starts inventory sync through the backend Fab facade and projects the accepted job envelope.
pub async fn fab_sync_inventory(
    services: &DesktopServices,
    request: FabInventorySyncRequestDto,
) -> CommandResultDto<super::AcceptedJobDto> {
    map_accepted_job_result(services.fab.sync_inventory(request))
}