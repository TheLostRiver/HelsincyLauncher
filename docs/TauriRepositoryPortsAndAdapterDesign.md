# Tauri Repository Ports and Adapter Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Focus: repository ports, storage/file/secret/provider adapter boundaries, naming rules, orchestration ownership

---

## 1. Purpose

本文档把当前蓝图和专项稿中的“ports-and-adapters”继续往实现前推进一层，专门回答以下问题：

1. 哪些接口属于跨模块 / 前端可见 contracts
2. 哪些接口属于模块内部 application ports
3. 哪些接口应该由 SQLite、文件系统、安全存储、provider adapter 来实现
4. 哪些边界绝不能被提升成新的“万能仓储”或“万能服务”

这份文档的目标不是定义页面 DTO，而是定义 **后端模块内部如何通过端口与适配器协作**。

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Internal ports are explicit | repository / provider / object store / secret store 都必须显式建模 |
| Contracts stay small | 跨模块 contracts 不被内部存储细节污染 |
| Adapter ownership is narrow | 每个 adapter 只实现自己那类 port，不跨界偷做业务决策 |
| Module-local reasoning | 开发者只看一个模块的 ports 就能知道它依赖哪些外界能力 |
| Storage medium isolation | SQLite、文件系统、安全存储、provider API 之间职责分清 |
| Orchestrator-first coordination | 多端口协作由应用层 use case / orchestrator 编排，不让 adapters 自己互调 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 把所有存储接口抽成全局 mega repository | 会重新形成耦合中心 |
| 让 Presentation 或 Tauri command 直接依赖 repository ports | 这些端口是后端内部边界 |
| 让 provider adapter 自己决定何时写库 | 会打穿应用层编排 |
| 用泛型 CRUD trait 替代业务语义端口 | 可读性差，语义弱 |

---

## 3. Boundary Taxonomy

必须先区分四类东西：

### 3.1 Public Contracts

给以下调用方看的接口或 DTO：

1. 前端 IPC
2. 其他模块
3. Shell / 跨模块 facade

示例：

- `fab.inventory.list`
- `downloads.job.start`
- `EngineHealthSummaryDto`

### 3.2 Internal Application Ports

只给模块自身的 use case / orchestrator / application service 用。

示例：

- `FabInventoryProjectionRepository`
- `DownloadCheckpointRepository`
- `EngineManifestProviderPort`
- `SessionSecretStorePort`

### 3.3 Adapters

具体实现 internal ports 的基础设施边界。

示例：

- `SqliteFabInventoryProjectionRepository`
- `FsDownloadStagingObjectStore`
- `WindowsCredentialSessionSecretStore`
- `EpicFabCatalogProviderAdapter`

### 3.4 Persistence / Runtime Models

这些不是 contracts，也不是 ports，它们只是内部数据模型：

1. SQLite row / projection model
2. 文件系统对象元数据
3. runtime snapshot storage model

规则：

1. Public Contracts 不能直接暴露 persistence model。
2. Adapter 实现 ports，但不定义跨模块协议。
3. Application ports 是内部接口，不自动升级成 public contract。

---

## 4. Ownership Rules

### 4.1 Ports belong to the consuming module

谁需要某种能力，谁定义它所依赖的 port。

例如：

1. `module-fab` 需要“读取和写入 owned inventory projection”的能力，那么 `FabInventoryProjectionRepository` 归 `module-fab` 所有。
2. `module-engines` 需要“解析某版本引擎 manifest”，那么 `EngineManifestProviderPort` 归 `module-engines` 所有。

不要反过来让 `adapter-storage-sqlite` 或 `adapter-provider-fab` 主导接口长相。

### 4.2 Shared low-level adapters are allowed, shared business repositories are not

允许共享：

1. `adapter-storage-sqlite`
2. `adapter-filesystem`
3. `adapter-secure-storage`

但不允许共享这种东西：

1. `IGlobalRepository`
2. `IMegaPersistenceService`
3. `IStorageManager` 同时负责 SQL、文件、凭据、缓存

### 4.3 Contracts vs Ports

固定规则：

1. `Contracts` 只用于前端或跨模块可见能力。
2. `Ports` 只用于模块内部 application boundary。
3. 任何 repository port 默认不对前端、Presentation 或其他模块开放。

---

## 5. Naming Rules

### 5.1 Public Contracts

建议命名：

- `FabInventoryReadService`
- `DownloadCommandFacade`
- `EngineVerificationFacade`

### 5.2 Internal Repositories

按语义命名，不要按技术名命名：

- `FabInventoryProjectionRepository`
- `FabSyncCursorRepository`
- `DownloadJobRepository`
- `DownloadCheckpointRepository`
- `InstallationRecordRepository`
- `EngineVerificationSummaryRepository`

不要使用：

- `DataRepository`
- `BaseRepository<T>`
- `SqlRepository`

### 5.3 Object / File Stores

用于大对象、缓存、staging、raw manifest：

- `DownloadStagingObjectStore`
- `ThumbnailObjectStore`
- `ManifestBlobStore`

### 5.4 Secret Stores

用于 token 或敏感凭据：

- `SessionSecretStorePort`
- `ProviderCredentialStorePort`

### 5.5 Provider Ports

用于远程 API / manifest / catalog 解析：

- `FabCatalogProviderPort`
- `DownloadManifestProviderPort`
- `EngineManifestProviderPort`

### 5.6 Adapter Implementations

实现类用技术前缀 + 业务语义：

- `SqliteDownloadCheckpointRepository`
- `FsManifestBlobStore`
- `WindowsCredentialSessionSecretStore`
- `EpicFabCatalogProviderAdapter`

---

## 6. Port Categories

### 6.1 Repository Ports

职责：

1. 访问结构化持久化事实
2. 封装事务边界内的数据读写
3. 返回业务语义模型，而不是裸 SQL 行

适合处理：

- projection
- checkpoint
- 安装记录
- health summary
- repair plan projection

### 6.2 Object Store Ports

职责：

1. 保存和读取大对象
2. 管理文件路径、对象 key、版本、清理
3. 不承担结构化查询

适合处理：

- raw manifest
- download staging
- thumbnail cache
- repair payloads

### 6.3 Secret Store Ports

职责：

1. 保存 access token / refresh token / provider secret
2. 提供系统级保护语义
3. 不与普通配置和 SQLite 混存

### 6.4 Provider Ports

职责：

1. 访问远程 HTTP / API / manifest 来源
2. 把 provider-specific payload 标准化为模块可用模型
3. 不直接写 SQLite 或文件系统业务状态

### 6.5 Platform Ports

职责：

1. 提供路径、通知、托盘、文件关联等平台能力
2. 只承载平台差异，不承载业务决策

---

## 7. Recommended Backend Layout

推荐把 ports 放在对应模块的 application 层，而不是集中到全局目录：

```text
backend/
  module-fab/
    src/
      application/
        ports/
          fab_inventory_projection_repository.rs
          fab_sync_cursor_repository.rs
          fab_catalog_provider_port.rs
          fab_media_metadata_repository.rs
  module-downloads/
    src/
      application/
        ports/
          download_job_repository.rs
          download_checkpoint_repository.rs
          download_staging_object_store.rs
          download_manifest_provider_port.rs
  module-engines/
    src/
      application/
        ports/
          engine_installation_repository.rs
          engine_manifest_provider_port.rs
          engine_verification_summary_repository.rs
  adapters/
    adapter-storage-sqlite/
    adapter-filesystem/
    adapter-secure-storage/
    adapter-provider-fab/
    adapter-provider-epic/
```

规则：

1. 端口靠近消费它的 use case。
2. adapter 作为独立 crate 或清晰目录存在。
3. 不把所有 ports 放成一个中心化“接口大仓库”。

---

## 8. Storage-related Port Design

### 8.1 Fab Module

Fab 至少需要这些内部 ports：

```rust
pub trait FabInventoryProjectionRepository {
    fn list_page(&self, query: FabInventoryListQuery) -> Result<FabInventoryProjectionPage>;
    fn upsert_batch(&self, rows: &[FabInventoryProjectionRow]) -> Result<()>;
    fn get_freshness(&self, account_id: &str) -> Result<FabInventoryFreshnessModel>;
}

pub trait FabSyncCursorRepository {
    fn load(&self, account_id: &str) -> Result<Option<FabSyncCursorModel>>;
    fn save(&self, cursor: &FabSyncCursorModel) -> Result<()>;
    fn acquire_lease(&self, account_id: &str, lease_id: &str) -> Result<bool>;
}

pub trait FabCatalogProviderPort {
    fn fetch_owned_page(&self, request: FabCatalogPageRequest) -> Result<FabCatalogPagePayload>;
    fn fetch_asset_details(&self, asset_id: &str) -> Result<FabAssetDetailsPayload>;
}
```

说明：

1. `FabInventoryProjectionRepository` 不关心 HTTP。
2. `FabCatalogProviderPort` 不关心 SQLite。
3. use case / sync orchestrator 同时依赖两者并负责编排。

### 8.2 Downloads Module

```rust
pub trait DownloadJobRepository {
    fn create_job(&self, job: &DownloadJobModel) -> Result<()>;
    fn get_job(&self, job_id: &str) -> Result<Option<DownloadJobModel>>;
    fn update_state(&self, job_id: &str, state: DownloadJobStateModel) -> Result<()>;
}

pub trait DownloadCheckpointRepository {
    fn load(&self, job_id: &str) -> Result<Option<DownloadCheckpointModel>>;
    fn save(&self, checkpoint: &DownloadCheckpointModel) -> Result<()>;
}

pub trait DownloadStagingObjectStore {
    fn ensure_staging_root(&self, job_id: &str) -> Result<StagingRootRef>;
    fn write_segment(&self, segment: &SegmentWriteRequest) -> Result<()>;
    fn cleanup_job(&self, job_id: &str) -> Result<()>;
}

pub trait DownloadManifestProviderPort {
    fn fetch_manifest(&self, target: &DownloadTargetRef) -> Result<ManifestPayload>;
}
```

### 8.3 Installations Module

```rust
pub trait InstallationRecordRepository {
    fn get_by_subject(&self, subject_id: &str) -> Result<Option<InstallationRecordModel>>;
    fn upsert(&self, record: &InstallationRecordModel) -> Result<()>;
}

pub trait InstallationManifestIndexRepository {
    fn get_manifest_ref(&self, subject_id: &str) -> Result<Option<ManifestRefModel>>;
    fn save_manifest_ref(&self, manifest_ref: &ManifestRefModel) -> Result<()>;
}

pub trait InstallationArtifactStore {
    fn resolve_install_root(&self, subject_id: &str) -> Result<InstallRootRef>;
    fn replace_file(&self, request: &ReplaceFileRequest) -> Result<()>;
}
```

### 8.4 Engines Module

```rust
pub trait EngineInstallationRepository {
    fn list_installed(&self) -> Result<Vec<EngineInstallationModel>>;
    fn get(&self, installation_id: &str) -> Result<Option<EngineInstallationModel>>;
}

pub trait EngineManifestProviderPort {
    fn get_engine_manifest(&self, version_id: &str, platform: PlatformKey) -> Result<EngineManifestPayload>;
    fn get_component_catalog(&self, version_id: &str) -> Result<ComponentCatalogPayload>;
}

pub trait EngineVerificationSummaryRepository {
    fn get_summary(&self, installation_id: &str) -> Result<Option<EngineVerificationSummaryModel>>;
    fn save_summary(&self, summary: &EngineVerificationSummaryModel) -> Result<()>;
}
```

### 8.5 Auth and Settings

```rust
pub trait SessionSecretStorePort {
    fn load(&self, account_id: &str) -> Result<Option<SessionSecretModel>>;
    fn save(&self, secret: &SessionSecretModel) -> Result<()>;
    fn clear(&self, account_id: &str) -> Result<()>;
}

pub trait AppSettingsStorePort {
    fn load(&self) -> Result<AppSettingsModel>;
    fn save(&self, settings: &AppSettingsModel) -> Result<()>;
}
```

---

## 9. Adapter Boundary Rules

### 9.1 SQLite Adapters

SQLite adapters 只负责：

1. SQL
2. transaction
3. row <-> model mapping
4. index-aware query

SQLite adapters 不负责：

1. 远程 API 调用
2. provider fallback 策略
3. 页面 freshness 策略决策
4. 发布跨模块事件

### 9.2 File System Adapters

文件系统 adapters 只负责：

1. 路径解析
2. 读写文件 / 目录
3. 原子替换 / 清理
4. 大对象生命周期

文件系统 adapters 不负责：

1. 判断某个 repair action 是否应该执行
2. 决定何时发起下载

### 9.3 Secure Storage Adapters

安全存储 adapters 只负责凭据安全读写，不负责：

1. token 刷新策略
2. 登录状态机
3. UI 登录流程

### 9.4 Provider Adapters

provider adapters 只负责：

1. 发请求
2. 解析响应
3. 转成 provider payload model

provider adapters 不负责：

1. 把响应直接写入 projection 表
2. 更新缓存命中策略
3. 决定 route return 是否复用 warm cache

---

## 10. Orchestration Ownership

多 port 协作必须由 application use case / orchestrator 显式编排。

### 10.1 Fab Startup Prewarm

```text
FabInventoryStartupPrewarmUseCase
  -> FabSyncCursorRepository.load
  -> FabInventoryProjectionRepository.get_freshness
  -> FabCatalogProviderPort.fetch_owned_page
  -> FabInventoryProjectionRepository.upsert_batch
  -> FabSyncCursorRepository.save
  -> publish fab.inventory.changed
```

这里没有任何 adapter 彼此互调。

### 10.2 Download Resume

```text
ResumeDownloadUseCase
  -> DownloadJobRepository.get_job
  -> DownloadCheckpointRepository.load
  -> DownloadStagingObjectStore.ensure_staging_root
  -> DownloadManifestProviderPort.fetch_manifest
  -> scheduler.enqueue
```

### 10.3 Engine Repair

```text
EngineRepairOrchestrator
  -> EngineManifestProviderPort.get_engine_manifest
  -> InstallationRecordRepository.get_by_subject
  -> EngineVerificationSummaryRepository.save_summary
  -> Downloads public contract / facade
  -> InstallationArtifactStore.replace_file
```

---

## 11. Transactions and Consistency

### 11.1 Intra-SQLite Consistency

同一模块、同一 SQLite 数据库内，如果多个 repository 需要原子更新，可以引入模块内 transaction boundary，但不要过早全局化成万能 UoW。

可接受：

1. `FabProjectionWriteTx`
2. `DownloadCheckpointWriteTx`

不建议一开始就引入：

1. `GlobalUnitOfWork`
2. 跨所有模块、跨文件系统、跨 provider 的统一事务抽象

### 11.2 Cross-medium Consistency

跨 SQLite + 文件系统 + provider 的流程，必须用：

1. state machine
2. checkpoint
3. retry / compensation

来保证一致性，而不是假装存在真正的分布式事务。

---

## 12. Port Review Checklist

为任何新 port 评审时，至少检查六件事：

1. 它是不是内部 port，而不是应该成为 public contract？
2. 它是不是只表达单一能力，而不是又查库又请求远程又写文件？
3. 它的命名是不是业务语义，而不是技术细节？
4. 它有没有泄露具体表结构、具体路径布局或第三方响应体？
5. 它的 adapter 是否会反向做业务编排？
6. 它会不会诱导 Presentation 或别的模块跨界直接依赖？

---

## 13. Acceptance Criteria

满足以下条件，才算这份设计稿达标：

1. 已明确 public contracts 与 internal ports 的边界。
2. 已明确 repository、object store、secret store、provider port 四类内部端口。
3. 已明确 SQLite、文件系统、安全存储、provider adapters 各自只实现哪类端口。
4. 已给出 Fab、Downloads、Installations、Engines 四条链的最小端口样例。
5. 已明确多端口协作只能由 application use case / orchestrator 编排。

---

## 14. Anti-patterns

以下做法视为违规：

1. `Presentation` 或前端直接依赖 repository ports。
2. provider adapter 请求完远程后直接写 SQLite projection。
3. 文件系统 adapter 自己决定 repair plan 或 sync 策略。
4. 把所有数据访问都塞进一个 `StorageManager` / `RepositoryHub`。
5. 用无语义的通用 CRUD trait 替代业务语义 repository。
