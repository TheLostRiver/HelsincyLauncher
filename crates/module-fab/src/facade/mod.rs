use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, CorrelationId,
};
use launcher_kernel_jobs::AcceptedJob;

use crate::contracts::{
    FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryListQueryDto, FabInventoryPageDto,
    FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto,
};

#[derive(Debug, Clone)]
pub struct FabModuleDeps<P, C, M, J, K> {
    pub projection_repo: P,
    pub cursor_repo: C,
    pub media_repo: M,
    pub job_runtime: J,
    pub catalog_provider: K,
}

pub struct FabFacade<P, C, M, J, K> {
    deps: FabModuleDeps<P, C, M, J, K>,
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K> {
    pub fn new(deps: FabModuleDeps<P, C, M, J, K>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &FabModuleDeps<P, C, M, J, K> {
        &self.deps
    }

    pub fn list_inventory(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryPageDto> {
        let _ = query;
        Err(not_wired("list_inventory"))
    }

    pub fn get_asset_detail(&self, query: FabAssetDetailQueryDto) -> AppResult<FabAssetDetailDto> {
        let _ = query;
        Err(not_wired("get_asset_detail"))
    }

    pub fn run_startup_prewarm(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("run_startup_prewarm"))
    }

    pub fn sync_inventory(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Err(not_wired("sync_inventory"))
    }
}

fn not_wired(operation: &str) -> AppError {
    AppError::new(
        "FAB_NOT_WIRED",
        format!("fab facade operation `{operation}` is not wired in C1"),
        false,
        AppErrorSeverity::Warning,
        CorrelationId::generate(),
    )
}