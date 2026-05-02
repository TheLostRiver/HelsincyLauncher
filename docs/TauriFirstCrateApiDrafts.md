# Tauri First Crate API Drafts

> Status: local draft v1
> Date: 2026-05-03
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `.artifacts/docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `.artifacts/docs/TauriRepositoryPortsAndAdapterDesign.md`, `.artifacts/docs/TauriDownloadRuntimeDesign.md`, `.artifacts/docs/TauriFabInventoryLoadingDesign.md`, `.artifacts/docs/TauriRustTsSchemaDesign.md`
> Focus: first implementation slice crate list, Cargo workspace members, crate-level public API sketches, module dependency surface

---

## 1. Purpose

上一轮已经把模块骨架、crate 职责和 use case 壳定下来了。

这一轮继续往前一步，不再停留在“有哪些 crate”，而是要回答：

1. 第一批真实要创建哪些 crate
2. 这些 crate 各自的 `Cargo.toml` 成员关系是什么
3. 每个 crate 的公共 API 应该暴露到什么粒度
4. `kernel-foundation`、`kernel-jobs`、`module-fab`、`module-downloads` 四个核心 crate 之间如何接线

这份文档是**实现前的 crate-level API 草图**，目标是降低真正开工时的命名漂移和边界回退风险。

---

## 2. First Slice Decision

第一批建议真实落地的 crate 清单如下：

| Kind | Directory | Package name | Why first |
|------|-----------|--------------|-----------|
| kernel | `crates/kernel-foundation` | `launcher-kernel-foundation` | 给所有后续 crate 提供稳定基础类型 |
| kernel | `crates/kernel-jobs` | `launcher-kernel-jobs` | 给下载/同步/验证统一长任务 runtime 协议 |
| module | `crates/module-fab` | `launcher-module-fab` | 验证 projection-first + prewarm + sync facade |
| module | `crates/module-downloads` | `launcher-module-downloads` | 验证 start/resume + checkpoint + runtime 闭环 |
| adapter | `crates/adapter-storage-sqlite` | `launcher-adapter-storage-sqlite` | 为 Fab / Downloads 提供最小 repository 实现 |
| adapter | `crates/adapter-provider-fab` | `launcher-adapter-provider-fab` | 为 Fab inventory sync 提供 provider adapter |
| composition | `crates/composition-root` | `launcher-composition-root` | 把上述 crate 装配成桌面可运行服务 |

说明：

1. `adapter-filesystem`、`adapter-secure-storage`、`module-engines`、`module-installations` 暂时不进入第一批真实创建。
2. 但它们的接口边界仍然保留在前几份文档中，不代表被否定，只是实现顺序后置。

---

## 3. Workspace Cargo Members

建议 workspace 根清单从第一天就显式写出来：

```toml
[workspace]
resolver = "2"
members = [
  "apps/desktop/src-tauri",
  "crates/composition-root",
  "crates/kernel-foundation",
  "crates/kernel-jobs",
  "crates/module-fab",
  "crates/module-downloads",
  "crates/adapter-storage-sqlite",
  "crates/adapter-provider-fab",
]

[workspace.dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
specta = "2"
thiserror = "2"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
tracing = "0.1"
uuid = { version = "1", features = ["v4", "serde"] }
rusqlite = { version = "0.35", features = ["bundled"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
```

规则：

1. workspace 层统一锁依赖版本。
2. module crates 默认只拉自己真正需要的最小依赖。
3. provider / sqlite 依赖不进入无关 module crate。

---

## 4. Crate Dependency Graph

第一批 crate 的依赖图建议固定为：

```text
apps/desktop/src-tauri
  -> launcher-composition-root

launcher-composition-root
  -> launcher-kernel-foundation
  -> launcher-kernel-jobs
  -> launcher-module-fab
  -> launcher-module-downloads
  -> launcher-adapter-storage-sqlite
  -> launcher-adapter-provider-fab

launcher-module-fab
  -> launcher-kernel-foundation
  -> launcher-kernel-jobs

launcher-module-downloads
  -> launcher-kernel-foundation
  -> launcher-kernel-jobs

launcher-adapter-storage-sqlite
  -> launcher-kernel-foundation
  -> launcher-module-fab
  -> launcher-module-downloads

launcher-adapter-provider-fab
  -> launcher-kernel-foundation
  -> launcher-module-fab
```

禁止反向依赖：

1. `launcher-module-fab` 不得依赖 `launcher-adapter-storage-sqlite`
2. `launcher-module-downloads` 不得依赖 `launcher-adapter-provider-fab`
3. `apps/desktop/src-tauri` 不得直接依赖任一 adapter crate

---

## 5. `launcher-kernel-foundation`

### 5.1 Crate Purpose

只放“所有后续 crate 都能依赖，但不带业务语义”的基础类型。

建议内容：

1. `AppError`
2. `AppResult<T>`
3. `Clock` trait
4. `IdGenerator` trait
5. 稳定 ID newtypes 或 type aliases
6. 少量分页、游标、时间戳基础模型

不允许放：

1. `FabInventoryPageDto`
2. `DownloadJobSnapshotDto`
3. provider payload
4. SQLite row model

### 5.2 Public Modules

```text
src/
  lib.rs
  error.rs
  result.rs
  clock.rs
  ids.rs
  paging.rs
  time.rs
```

### 5.3 `lib.rs` Sketch

```rust
pub mod clock;
pub mod error;
pub mod ids;
pub mod paging;
pub mod result;
pub mod time;

pub use clock::{Clock, SystemClock};
pub use error::AppError;
pub use ids::{AccountId, AssetId, CorrelationId, JobId};
pub use paging::{PageCursor, PageRequest, PageSlice};
pub use result::AppResult;
pub use time::IsoDateTime;
```

### 5.4 Minimal API Sketch

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSlice<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}
```

### 5.5 `Cargo.toml` Sketch

```toml
[package]
name = "launcher-kernel-foundation"
version = "0.1.0"
edition = "2024"

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde.workspace = true
thiserror.workspace = true
uuid.workspace = true
```

---

## 6. `launcher-kernel-jobs`

### 6.1 Crate Purpose

这是统一长任务内核，服务于下载、Fab 预热同步、引擎验证、安装扫描等后台任务。

建议公共 API 只暴露：

1. job identity
2. runtime-facing trait
3. 通用 snapshot / progress model
4. queue / lease / cancellation control 协议

不暴露：

1. 下载分段细节
2. Fab cursor 细节
3. 引擎修复计划细节

### 6.2 Public Modules

```text
src/
  lib.rs
  contracts/
    snapshot.rs
    progress.rs
    accepted.rs
  runtime/
    ports.rs
    events.rs
    policy.rs
  state/
    job_state.rs
  internal/
    mod.rs
```

### 6.3 `lib.rs` Sketch

```rust
pub mod contracts;
pub mod runtime;
pub mod state;

pub use contracts::{AcceptedJob, JobProgress, JobSnapshot, JobUiState};
pub use runtime::{JobRuntimeControlPort, JobRuntimePort, RuntimeQueuePolicy};
pub use state::JobState;
```

### 6.4 Minimal API Sketch

```rust
pub trait JobRuntimePort: Send + Sync {
    type Extension;

    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob>;
    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>>;
}

pub trait JobRuntimeControlPort: Send + Sync {
    fn pause(&self, job_id: &JobId) -> AppResult<()>;
    fn resume(&self, job_id: &JobId) -> AppResult<()>;
    fn cancel(&self, job_id: &JobId) -> AppResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedJob {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub queued_at: IsoDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSnapshot<E> {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub ui_state: JobUiState,
    pub progress: JobProgress,
    pub updated_at: IsoDateTime,
    pub extension: Option<E>,
}
```

### 6.5 Design Rule

`launcher-kernel-jobs` 不应该直接依赖 `specta` 导出的前端 DTO 名字，但它可以承载与之同构的后端内部快照模型。真正 IPC 导出的 DTO 可以在各 module facade 或共享 schema 层做 thin mapping。

### 6.6 `Cargo.toml` Sketch

```toml
[package]
name = "launcher-kernel-jobs"
version = "0.1.0"
edition = "2024"

[dependencies]
launcher-kernel-foundation = { path = "../kernel-foundation" }
serde.workspace = true
tokio.workspace = true
tracing.workspace = true
```

---

## 7. `launcher-module-fab`

### 7.1 Crate Purpose

它是 Fab 业务核心，不是 provider client，也不是 SQLite implementation。

它应该公开：

1. Fab contracts
2. `FabFacade`
3. facade 构造所需的依赖参数结构

它不应该公开：

1. SQLite row
2. provider payload
3. projection table schema
4. sync orchestrator 的内部 helper

### 7.2 Public Modules

```text
src/
  lib.rs
  contracts/
    dto.rs
    queries.rs
    commands.rs
    events.rs
  facade/
    mod.rs
  application/
    queries/
      list_inventory.rs
      get_asset_detail.rs
    use_cases/
      run_prewarm.rs
      sync_inventory.rs
    orchestrators/
      inventory_sync_orchestrator.rs
    ports/
      fab_inventory_projection_repository.rs
      fab_sync_cursor_repository.rs
      fab_catalog_provider_port.rs
      fab_media_metadata_repository.rs
  domain/
    models/
    policies/
```

### 7.3 `lib.rs` Sketch

```rust
pub mod contracts;
pub mod facade;

pub use facade::{FabFacade, FabModuleDeps};
```

### 7.4 Public Facade Sketch

```rust
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

impl<P, C, M, J, K> FabFacade<P, C, M, J, K>
where
    P: FabInventoryProjectionRepository,
    C: FabSyncCursorRepository,
    M: FabMediaMetadataRepository,
    J: JobRuntimePort,
    K: FabCatalogProviderPort,
{
    pub fn list_inventory(&self, query: FabInventoryListQueryDto) -> AppResult<FabInventoryPageDto>;
    pub fn get_asset_detail(&self, query: FabAssetDetailQueryDto) -> AppResult<FabAssetDetailDto>;
    pub fn run_startup_prewarm(&self, request: FabInventoryPrewarmRequestDto) -> AppResult<AcceptedJob>;
    pub fn sync_inventory(&self, request: FabInventorySyncRequestDto) -> AppResult<AcceptedJob>;
}
```

### 7.5 Internal Port Surface

最小端口集建议固定为：

1. `FabInventoryProjectionRepository`
2. `FabSyncCursorRepository`
3. `FabMediaMetadataRepository`
4. `FabCatalogProviderPort`

其中：

1. 列表 query 只强依赖 projection repository。
2. prewarm / sync use case 同时依赖 projection + cursor + provider + job runtime。
3. 详情 query 可以优先命中 projection / media projection，再决定是否做按需 provider 查询。

### 7.6 `Cargo.toml` Sketch

```toml
[package]
name = "launcher-module-fab"
version = "0.1.0"
edition = "2024"

[dependencies]
launcher-kernel-foundation = { path = "../kernel-foundation" }
launcher-kernel-jobs = { path = "../kernel-jobs" }
serde.workspace = true
specta.workspace = true
tracing.workspace = true
```

---

## 8. `launcher-module-downloads`

### 8.1 Crate Purpose

它是下载命令入口、编排、resume 恢复和只读投影的拥有者。

它应该公开：

1. Downloads contracts
2. `DownloadFacade`
3. 第一批 start / pause / resume / list use case

它不应该公开：

1. scheduler 内部队列结构
2. segment writer 内部状态
3. HTTP client / range request 细节

### 8.2 Public Modules

```text
src/
  lib.rs
  contracts/
    dto.rs
    commands.rs
    queries.rs
    events.rs
  facade/
    mod.rs
  application/
    use_cases/
      start_download.rs
      pause_download.rs
      resume_download.rs
    queries/
      list_download_jobs.rs
      get_download_job_snapshot.rs
    orchestrators/
      download_orchestrator.rs
    ports/
      download_job_repository.rs
      download_checkpoint_repository.rs
      download_manifest_provider_port.rs
      download_staging_object_store.rs
  domain/
    models/
    policies/
    state_machines/
```

### 8.3 `lib.rs` Sketch

```rust
pub mod contracts;
pub mod facade;

pub use facade::{DownloadFacade, DownloadModuleDeps};
```

### 8.4 Public Facade Sketch

```rust
pub struct DownloadModuleDeps<J, C, M, S, R> {
    pub job_repo: J,
    pub checkpoint_repo: C,
    pub manifest_provider: M,
    pub staging_store: S,
    pub job_runtime: R,
}

pub struct DownloadFacade<J, C, M, S, R> {
    deps: DownloadModuleDeps<J, C, M, S, R>,
}

impl<J, C, M, S, R> DownloadFacade<J, C, M, S, R>
where
    J: DownloadJobRepository,
    C: DownloadCheckpointRepository,
    M: DownloadManifestProviderPort,
    S: DownloadStagingObjectStore,
    R: JobRuntimePort + JobRuntimeControlPort,
{
    pub fn start_download(&self, request: StartDownloadRequestDto) -> AppResult<AcceptedJob>;
    pub fn pause_download(&self, request: PauseDownloadRequestDto) -> AppResult<()>;
    pub fn resume_download(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJob>;
    pub fn list_jobs(&self, query: ListDownloadJobsQueryDto) -> AppResult<DownloadJobListDto>;
    pub fn get_job_snapshot(&self, job_id: &JobId) -> AppResult<Option<DownloadJobSnapshotDto>>;
}
```

### 8.5 Internal Port Surface

最小端口集建议固定为：

1. `DownloadJobRepository`
2. `DownloadCheckpointRepository`
3. `DownloadManifestProviderPort`
4. `DownloadStagingObjectStore`
5. `JobRuntimePort`
6. `JobRuntimeControlPort`

规则：

1. `start_download` 只负责创建 job、解析 manifest、把执行请求送进 runtime。
2. `resume_download` 必须显式读取 checkpoint，而不是让 runtime 自己猜。
3. `pause_download` / `cancel_download` 通过 runtime control port 执行，而不是直接篡改 repository 状态。

### 8.6 `Cargo.toml` Sketch

```toml
[package]
name = "launcher-module-downloads"
version = "0.1.0"
edition = "2024"

[dependencies]
launcher-kernel-foundation = { path = "../kernel-foundation" }
launcher-kernel-jobs = { path = "../kernel-jobs" }
serde.workspace = true
specta.workspace = true
tracing.workspace = true
```

---

## 9. Required Adapter Surface for the First Slice

虽然这一稿聚焦四个核心 crate，但第一批要能闭环，至少还需要两类 adapter：

### 9.1 `launcher-adapter-storage-sqlite`

第一批需要实现：

1. `SqliteFabInventoryProjectionRepository`
2. `SqliteFabSyncCursorRepository`
3. `SqliteFabMediaMetadataRepository`
4. `SqliteDownloadJobRepository`
5. `SqliteDownloadCheckpointRepository`

### 9.2 `launcher-adapter-provider-fab`

第一批需要实现：

1. `EpicFabCatalogProviderAdapter`

说明：

1. Downloads 的 manifest provider 第一阶段可以先用 stub / fixture adapter，避免第一批同时打开太多实现面。
2. 这不会改变 `module-downloads` 的端口边界，只是实现顺序后置。

---

## 10. `launcher-composition-root`

### 10.1 Crate Purpose

这是第一批真实装配中心。

它应负责：

1. 创建 SQLite connection factory
2. 创建 Fab provider adapter
3. 创建共享 job runtime 实现
4. 组装 `FabFacade`
5. 组装 `DownloadFacade`
6. 暴露 `DesktopAppServices`

### 10.2 Public API Sketch

```rust
pub struct DesktopAppServices {
    pub fab: Arc<FabFacade<...>>,
    pub downloads: Arc<DownloadFacade<...>>,
}

pub fn build_desktop_services(config: DesktopBootstrapConfig) -> AppResult<DesktopAppServices>;
```

### 10.3 Wiring Sequence

```text
build_desktop_services
  -> create clock / id generator
  -> create sqlite repositories
  -> create fab provider adapter
  -> create job runtime host
  -> create fab facade
  -> create download facade
  -> return DesktopAppServices
```

组合根不应该：

1. 直接暴露 repository 给 Tauri command
2. 自己执行 startup prewarm 业务逻辑
3. 混入窗口、托盘、更新器 glue

---

## 11. Tauri Integration Boundary

桌面宿主第一阶段建议只注册最小命令集：

### 11.1 Fab Commands

1. `fab_list_inventory`
2. `fab_get_asset_detail`
3. `fab_run_startup_prewarm`

### 11.2 Download Commands

1. `downloads_start`
2. `downloads_pause`
3. `downloads_resume`
4. `downloads_list_jobs`
5. `downloads_get_job_snapshot`

command handler 草图：

```rust
#[tauri::command]
pub async fn fab_list_inventory(
    state: State<'_, DesktopAppServices>,
    query: FabInventoryListQueryDto,
) -> QueryResultDto<FabInventoryPageDto> {
    map_query_result(state.fab.list_inventory(query))
}
```

固定规则：

1. handler 只拿 `DesktopAppServices`。
2. handler 不知道 SQLite repository 具体类型。
3. handler 不接触 provider adapter。

---

## 12. First Test Matrix

为了让这批 crate 真正可开工，建议第一批测试就对齐 crate 边界：

### 12.1 `launcher-kernel-foundation`

1. `AppError` 映射测试
2. `PageSlice<T>` 序列化测试
3. `JobId` / `AccountId` round-trip 测试

### 12.2 `launcher-kernel-jobs`

1. `AcceptedJob` 序列化测试
2. `JobSnapshot<E>` 状态收敛测试
3. runtime trait stub smoke tests

### 12.3 `launcher-module-fab`

1. `list_inventory` 只调用 projection repository 的单测
2. `run_startup_prewarm` 同时调用 cursor/provider/runtime 的单测
3. DTO 到 facade 输出映射测试

### 12.4 `launcher-module-downloads`

1. `start_download` 调用 manifest provider + runtime 的单测
2. `resume_download` 必须读取 checkpoint 的单测
3. `pause_download` 走 control port 的单测

### 12.5 `launcher-composition-root`

1. `build_desktop_services` smoke test
2. 缺失依赖时的构建失败路径测试

---

## 13. Recommended Next Slice After This Draft

完成本稿后，下一轮最合理的推进顺序是：

1. 单独写 `launcher-composition-root` wiring 设计稿
2. 再把 `launcher-kernel-jobs` 压成更细的 runtime host / queue policy / snapshot store 草图
3. 然后才开始真实建 workspace 目录和空 crate

原因：

1. 现在 crate 名称和 facade 形状已经足够稳定。
2. 真正剩下还容易漂移的是组合根装配和 job runtime 细化。
3. 先把它们写实，再开始建空 crate，能减少“建了再改”的返工。

---

## 14. Anti-patterns

以下做法视为偏离本稿：

1. 先创建一个 `launcher-backend-core` 巨型 crate，后续再慢慢拆。
2. 把 `JobSnapshotDto`、`FabInventoryPageDto` 之类所有前端 DTO 塞进 `launcher-kernel-foundation`。
3. 让 `launcher-kernel-jobs` 直接知道下载 segment、Fab cursor、engine manifest 细节。
4. 让 `launcher-composition-root` 直接承担 startup prewarm、resume download 等业务流程。
5. 为了减少泛型参数，回到全局 service locator 或 `ManagerHub`。

---

## 15. Acceptance Criteria

满足以下条件，才算这份 crate-level API 草图可用：

1. 已明确第一批真实 crate 清单与 package name。
2. 已明确四个核心 crate 的 `lib.rs` 公开面和最小 API 草图。
3. 已明确第一批必需 adapter surface 与组合根装配责任。
4. 已明确桌面宿主只通过 `DesktopAppServices` 调用 facade。
5. 已明确第一批测试矩阵和下一轮细化顺序。
