//! Fab 模块的 facade 边界，负责投影读取与 accepted-job 交接。
//!
//! 该 surface 汇总当前 Fab 后端语义：从本地 projection 读取库存摘要、在详情缓存未热时
//! 返回后端拥有的 cold-start 占位结果，并通过 job runtime 接收 sync/prewarm 作业，
//! 同时不向 transport 暴露 adapter 或运行时细节。

use launcher_kernel_foundation::{AppResult, AssetId, IsoDateTime, JobId};
use launcher_kernel_jobs::{
    AcceptedJob, EnqueueJobRequest, JobPriority, JobRuntime, SharedJobRuntimeHost,
};

use crate::contracts::{
    FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryListQueryDto, FabInventoryPageDto,
    FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto,
};

/// 当前 Fab facade 边界所需的具体依赖束。
#[derive(Debug, Clone)]
pub struct FabModuleDeps<P, C, M, J, K> {
    /// 从本地 projection 读取库存摘要分页与已缓存的详情快照。
    pub projection_repo: P,
    /// 持久化增量同步 cursor，保证 Fab 同步可跨重启恢复。
    pub cursor_repo: C,
    /// 解析缩略图与预览元数据，避免把媒体细节塞进列表 read model。
    pub media_repo: M,
    /// 接收 Fab 长任务，并返回后端拥有的 accepted-job 结果。
    pub job_runtime: J,
    /// 在模块边界之后访问上游 Fab provider。
    pub catalog_provider: K,
}

/// 暴露给 composition-root 与宿主 transport 的 Fab 用例入口。
pub struct FabFacade<P, C, M, J, K> {
    deps: FabModuleDeps<P, C, M, J, K>,
}

/// 由本地 Fab read model 返回、以 projection 为事实来源的库存分页。
pub type FabInventoryProjectionPage = FabInventoryPageDto;

/// 从本地 projection 读取 Fab 库存摘要与已缓存详情快照。
pub trait FabInventoryProjectionRepository {
    /// 返回稳定库存分页，读取路径不直接访问上游 provider。
    fn list_page(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryProjectionPage>;

    /// 在本地 projection 已完成水合时返回缓存详情快照。
    fn get_asset_detail_snapshot(&self, asset_id: &AssetId) -> AppResult<Option<FabAssetDetailDto>>;
}

/// 接收 Fab 库存同步请求，并返回后端拥有的 accepted job。
pub trait FabSyncJobAcceptance {
    fn accept_sync_job(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob>;
}

/// 接收启动阶段库存预热请求，并返回后端拥有的 accepted job。
pub trait FabStartupPrewarmJobAcceptance {
    fn accept_startup_prewarm_job(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob>;
}

// The unit fallback keeps the facade callable before a real runtime host is injected.
// `()` fallback 让真实 runtime host 注入前的 facade 命令路径仍可调用。
impl FabSyncJobAcceptance for () {
    fn accept_sync_job(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        Ok(cold_start_sync_job())
    }
}

impl FabSyncJobAcceptance for SharedJobRuntimeHost {
    fn accept_sync_job(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob> {
        let _ = request;
        self.enqueue(EnqueueJobRequest {
            job_id: JobId::generate(),
            module: "fab".into(),
            kind: "inventory_sync".into(),
            priority: JobPriority::Normal,
            recoverable: true,
            extension: None,
        })
    }
}

// The unit fallback lets startup prewarm stay backend-owned before runtime wiring lands.
// `()` fallback 让启动预热在 runtime wiring 落地前仍保持后端拥有的 accepted-job 语义。
impl FabStartupPrewarmJobAcceptance for () {
    fn accept_startup_prewarm_job(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob> {
        let _ = request;
        Ok(cold_start_startup_prewarm_job())
    }
}

impl FabStartupPrewarmJobAcceptance for SharedJobRuntimeHost {
    fn accept_startup_prewarm_job(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob> {
        let _ = request;
        self.enqueue(EnqueueJobRequest {
            job_id: JobId::generate(),
            module: "fab".into(),
            kind: "inventory_startup_prewarm".into(),
            priority: JobPriority::Low,
            recoverable: true,
            extension: None,
        })
    }
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K> {
    /// Creates a Fab facade over already-assembled module dependencies.
    /// 使用已经完成装配的模块依赖束创建 Fab facade。
    pub fn new(deps: FabModuleDeps<P, C, M, J, K>) -> Self {
        Self { deps }
    }

    /// Exposes the assembled dependency bundle for composition-root smoke tests and wiring checks.
    /// 暴露已装配的依赖束，服务 composition-root smoke test 与 wiring 检查。
    pub fn deps(&self) -> &FabModuleDeps<P, C, M, J, K> {
        &self.deps
    }
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K>
where
    J: FabStartupPrewarmJobAcceptance,
{
    /// Accepts a startup prewarm request through the configured job runtime boundary.
    /// 通过已配置的 job runtime 边界接收启动预热请求。
    pub fn run_startup_prewarm(
        &self,
        request: FabInventoryPrewarmRequestDto,
    ) -> AppResult<AcceptedJob> {
        self.deps.job_runtime.accept_startup_prewarm_job(request)
    }
}

impl<P, C, M, J, K> FabFacade<P, C, M, J, K>
where
    P: FabInventoryProjectionRepository,
    J: FabSyncJobAcceptance,
{
    /// Returns an inventory page from the local Fab projection.
    /// 从本地 Fab projection 返回库存分页。
    pub fn list_inventory(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryPageDto> {
        self.deps.projection_repo.list_page(query)
    }

    /// Returns the cached detail snapshot or a backend-owned cold-start placeholder.
    /// 返回已缓存的详情快照，或后端拥有的 cold-start 占位详情。
    pub fn get_asset_detail(&self, query: FabAssetDetailQueryDto) -> AppResult<FabAssetDetailDto> {
        Ok(self
            .deps
            .projection_repo
            .get_asset_detail_snapshot(&query.asset_id)?
            .unwrap_or_else(|| cold_start_asset_detail(query.asset_id)))
    }

    /// Accepts an inventory sync job without leaking runtime details to callers.
    /// 接收库存同步作业，同时不向调用方泄露 runtime 细节。
    pub fn sync_inventory(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob> {
        self.deps.job_runtime.accept_sync_job(request)
    }
}

// The detail read path must stay backend-owned even when the local projection is still cold.
// 即使本地 projection 仍未热起来，详情读取路径也必须保持由后端拥有。
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

fn cold_start_sync_job() -> AcceptedJob {
    AcceptedJob {
        job_id: JobId::generate(),
        module: "fab".into(),
        kind: "inventory_sync".into(),
        queued_at: IsoDateTime::now(),
    }
}

fn cold_start_startup_prewarm_job() -> AcceptedJob {
    AcceptedJob {
        job_id: JobId::generate(),
        module: "fab".into(),
        kind: "inventory_startup_prewarm".into(),
        queued_at: IsoDateTime::now(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use launcher_kernel_foundation::{AssetId, PageCursor, PageRequest, PageSlice};

    use super::{
        FabFacade, FabInventoryProjectionPage, FabInventoryProjectionRepository,
        FabModuleDeps,
    };
    use crate::contracts::{
        FabAssetDetailDto, FabAssetDetailQueryDto, FabInventoryItemDto,
        FabInventoryListQueryDto, FabInventoryPrewarmRequestDto, FabInventorySyncRequestDto,
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

    #[test]
    fn sync_inventory_returns_backend_owned_accepted_job_with_placeholder_runtime() {
        let facade = FabFacade::new(FabModuleDeps {
            projection_repo: RecordingProjectionRepository::new(PageSlice::new(Vec::new(), None)),
            cursor_repo: (),
            media_repo: (),
            job_runtime: (),
            catalog_provider: (),
        });

        let accepted_job = facade
            .sync_inventory(FabInventorySyncRequestDto {
                trigger: "manual-refresh".into(),
                force_full_sync: false,
            })
            .expect("sync_inventory should return a backend-owned accepted job when the current runtime dependency is still a placeholder");

        assert_eq!(accepted_job.module, "fab");
        assert_eq!(accepted_job.kind, "inventory_sync");
        assert!(!accepted_job.job_id.as_str().is_empty());
    }

    #[test]
    fn run_startup_prewarm_returns_backend_owned_accepted_job_with_placeholder_runtime() {
        let facade = FabFacade::new(FabModuleDeps {
            projection_repo: RecordingProjectionRepository::new(PageSlice::new(Vec::new(), None)),
            cursor_repo: (),
            media_repo: (),
            job_runtime: (),
            catalog_provider: (),
        });

        let accepted_job = facade
            .run_startup_prewarm(FabInventoryPrewarmRequestDto {
                reason: "startup-stage-3".into(),
            })
            .expect("run_startup_prewarm should return a backend-owned accepted job when the current runtime dependency is still a placeholder");

        assert_eq!(accepted_job.module, "fab");
        assert_eq!(accepted_job.kind, "inventory_startup_prewarm");
        assert!(!accepted_job.job_id.as_str().is_empty());
    }
}
