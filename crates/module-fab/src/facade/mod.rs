use launcher_kernel_foundation::{
    AppError, AppErrorSeverity, AppResult, AssetId, CorrelationId,
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

pub type FabInventoryProjectionPage = FabInventoryPageDto;

pub trait FabInventoryProjectionRepository {
    fn list_page(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryProjectionPage>;
    fn get_asset_detail_snapshot(&self, asset_id: &AssetId) -> AppResult<Option<FabAssetDetailDto>>;
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K> {
    pub fn new(deps: FabModuleDeps<P, C, M, J, K>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &FabModuleDeps<P, C, M, J, K> {
        &self.deps
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

impl<P, C, M, J, K> FabFacade<P, C, M, J, K>
where
    P: FabInventoryProjectionRepository,
{
    pub fn list_inventory(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryPageDto> {
        self.deps.projection_repo.list_page(query)
    }

    pub fn get_asset_detail(&self, query: FabAssetDetailQueryDto) -> AppResult<FabAssetDetailDto> {
        Ok(self
            .deps
            .projection_repo
            .get_asset_detail_snapshot(&query.asset_id)?
            .unwrap_or_else(|| cold_start_asset_detail(query.asset_id)))
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

fn cold_start_asset_detail(asset_id: AssetId) -> FabAssetDetailDto {
    FabAssetDetailDto {
        asset_id,
        title: "Fab asset detail not cached yet".into(),
        description: "Local detail projection is still cold; provider/detail hydration lands in a later Fab slice.".into(),
        seller_name: "unknown".into(),
        compatibility_notes: Vec::new(),
        dependency_warnings: vec!["fab detail currently falls back to a backend-owned cold-start placeholder until richer projection/provider wiring lands".into()],
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AssetId, PageCursor, PageRequest, PageSlice};

    use super::{FabFacade, FabInventoryProjectionPage, FabInventoryProjectionRepository, FabModuleDeps};
    use crate::contracts::{
        FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryItemDto, FabInventoryListQueryDto,
    };

    #[derive(Debug)]
    struct RecordingProjectionRepository {
        captured_query: Mutex<Option<FabInventoryListQueryDto>>,
        page: FabInventoryProjectionPage,
    }

    impl RecordingProjectionRepository {
        fn new(page: FabInventoryProjectionPage) -> Self {
            Self {
                captured_query: Mutex::new(None),
                page,
            }
        }
    }

    impl FabInventoryProjectionRepository for RecordingProjectionRepository {
        fn list_page(&self, query: FabInventoryListQueryDto) -> launcher_kernel_foundation::AppResult<FabInventoryProjectionPage> {
            *self.captured_query.lock().expect("query mutex should not be poisoned") = Some(query);
            Ok(self.page.clone())
        }

        fn get_asset_detail_snapshot(&self, _asset_id: &AssetId) -> launcher_kernel_foundation::AppResult<Option<FabAssetDetailDto>> {
            Ok(None)
        }
    }

    #[test]
    fn list_inventory_delegates_to_projection_repository() {
        let query = FabInventoryListQueryDto {
            page: PageRequest::new(20, Some(PageCursor::new("cursor-1"))),
            search: Some("fab".into()),
            category_key: Some("assets".into()),
            include_incompatible: false,
        };
        let expected_page = PageSlice::new(
            vec![FabInventoryItemDto {
                asset_id: AssetId::new("asset-1"),
                title: "Fab Asset".into(),
                seller_name: "Seller".into(),
                category_key: Some("assets".into()),
                engine_support_summary: vec!["UE 5.4".into()],
            }],
            Some(PageCursor::new("cursor-2")),
        );
        let facade = FabFacade::new(FabModuleDeps {
            projection_repo: RecordingProjectionRepository::new(expected_page.clone()),
            cursor_repo: (),
            media_repo: (),
            job_runtime: (),
            catalog_provider: (),
        });

        let actual_page = facade
            .list_inventory(query.clone())
            .expect("list_inventory should delegate to the projection repository");

        assert_eq!(actual_page, expected_page);
        assert_eq!(
            *facade
                .deps()
                .projection_repo
                .captured_query
                .lock()
                .expect("query mutex should not be poisoned"),
            Some(query)
        );
    }

    #[test]
    fn get_asset_detail_returns_cold_start_placeholder_when_projection_is_empty() {
        let asset_id = AssetId::new("asset-detail-1");
        let facade = FabFacade::new(FabModuleDeps {
            projection_repo: RecordingProjectionRepository::new(PageSlice::new(Vec::new(), None)),
            cursor_repo: (),
            media_repo: (),
            job_runtime: (),
            catalog_provider: (),
        });

        let detail = facade
            .get_asset_detail(FabAssetDetailQueryDto {
                asset_id: asset_id.clone(),
            })
            .expect("get_asset_detail should return a backend-owned cold-start placeholder when no local snapshot exists");

        assert_eq!(detail.asset_id, asset_id);
        assert_eq!(detail.title, "Fab asset detail not cached yet");
        assert_eq!(detail.seller_name, "unknown");
        assert!(detail.compatibility_notes.is_empty());
        assert_eq!(detail.dependency_warnings.len(), 1);
    }
}