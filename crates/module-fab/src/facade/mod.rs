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

pub type FabInventoryProjectionPage = FabInventoryPageDto;

pub trait FabInventoryProjectionRepository {
    fn list_page(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryProjectionPage>;
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K> {
    pub fn new(deps: FabModuleDeps<P, C, M, J, K>) -> Self {
        Self { deps }
    }

    pub fn deps(&self) -> &FabModuleDeps<P, C, M, J, K> {
        &self.deps
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

impl<P, C, M, J, K> FabFacade<P, C, M, J, K>
where
    P: FabInventoryProjectionRepository,
{
    pub fn list_inventory(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryPageDto> {
        self.deps.projection_repo.list_page(query)
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AssetId, PageCursor, PageRequest, PageSlice};

    use super::{FabFacade, FabInventoryProjectionPage, FabInventoryProjectionRepository, FabModuleDeps};
    use crate::contracts::{FabInventoryItemDto, FabInventoryListQueryDto};

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
}