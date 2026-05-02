# Tauri Backend Crate Layout and Use Case Stub Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriRepositoryPortsAndAdapterDesign.md`, `docs/TauriIPCAndStateContractsDesign.md`, `docs/TauriDownloadRuntimeDesign.md`
> Focus: Rust workspace layout, module crate boundaries, composition root, use case stub catalog, dependency rules

---

## 1. Purpose

前面的蓝图已经明确了系统原则、长任务模型、共享 schema 和 ports / adapters 边界。

当前还缺一层：**这些抽象最终如何在 Rust workspace 里落成可实现的模块骨架**。

这份文档只回答五件事：

1. Rust workspace 应该如何分 crate
2. 哪些 crate 属于模块核心，哪些属于 adapters，哪些属于桌面宿主
3. 模块内部目录如何组织 contracts / application / domain
4. 哪些 use case 值得先建最小可测试壳
5. Tauri transport、组合根、后台 job runtime 分别落在哪里

这份稿的目标不是立刻开始写生产代码，而是把“实现应该从哪里起脚”钉成统一骨架。

---

## 2. Design Outcome

结论先行：

1. 采用 **Rust workspace + 多个窄职责 crate**。
2. 每个业务模块拥有自己的核心 crate，里面公开 `contracts` 和对外 facade，内部保留 `application` / `domain`。
3. SQLite、文件系统、安全存储、provider、平台集成全部落在独立 adapter crates。
4. Tauri command handlers 只放在桌面宿主 crate，命令只转发到模块 facade / use case。
5. 下载、安装、验证、修复共享一个 job runtime crate，但每个模块保留自己的 job extension model 与 orchestration。

---

## 3. Workspace Layout

推荐骨架如下：

```text
epic-launcher-rewrite/
├─ Cargo.toml
├─ Cargo.lock
├─ apps/
│  └─ desktop/
│     ├─ package.json
│     ├─ src/                    # React + TS UI
│     └─ src-tauri/
│        ├─ Cargo.toml
│        ├─ tauri.conf.json
│        └─ src/
│           ├─ main.rs
│           ├─ commands/
│           ├─ events/
│           └─ windowing/
├─ crates/
│  ├─ composition-root/
│  ├─ kernel-foundation/
│  ├─ kernel-jobs/
│  ├─ module-auth/
│  ├─ module-fab/
│  ├─ module-downloads/
│  ├─ module-installations/
│  ├─ module-engines/
│  ├─ adapter-storage-sqlite/
│  ├─ adapter-filesystem/
│  ├─ adapter-secure-storage/
│  ├─ adapter-provider-fab/
│  ├─ adapter-provider-epic/
│  ├─ adapter-platform-desktop/
│  └─ adapter-observability/
└─ tests/
   ├─ contract-tests/
   ├─ integration-tests/
   └─ fixture-data/
```

---

## 4. Crate Roles

### 4.1 apps/desktop/src-tauri

职责：

1. Tauri 入口
2. window / tray / deep-link / updater glue
3. command handler 注册
4. event bridge 注册

限制：

1. 不直接 new SQLite repository
2. 不直接写业务编排
3. 不直接访问 provider API

### 4.2 composition-root

职责：

1. 组装模块 facade
2. 注入具体 adapters
3. 建立 app-level startup pipeline
4. 提供桌面宿主可消费的 `DesktopAppServices`

这个 crate 是唯一知道“哪些 trait 对应哪些 concrete adapter”的地方。

### 4.3 kernel-foundation

职责：

1. 通用错误 / result / clock / ids
2. 共享小型基础类型
3. 模块都可依赖的零业务基础设施抽象

限制：

1. 不放业务 DTO
2. 不放模块状态机
3. 不放 provider-specific 类型

### 4.4 kernel-jobs

职责：

1. 通用 job state machine
2. snapshot / progress / cancellation protocol
3. scheduler / queue / lease / checkpoint 协议
4. runtime host traits

说明：

1. 这是通用长任务内核，不是下载模块。
2. `module-downloads`、`module-installations`、`module-engines` 都可以复用它。

### 4.5 module-* core crates

每个模块 crate 的职责：

1. 对外 contracts
2. 对外 facade
3. application use cases / orchestrators
4. domain models / policies / state machines
5. 内部 ports 定义

每个模块 crate 不包含：

1. SQLite 实现
2. 文件系统实现
3. OS 安全存储实现
4. HTTP client 实现

### 4.6 adapter-* crates

adapter crates 只负责实现 ports：

1. `adapter-storage-sqlite` 实现 repository ports
2. `adapter-filesystem` 实现 object store / artifact store / path ports
3. `adapter-secure-storage` 实现 secret store ports
4. `adapter-provider-fab` / `adapter-provider-epic` 实现 provider ports
5. `adapter-platform-desktop` 实现托盘、通知、目录解析等平台端口

---

## 5. Module Internal Layout

推荐每个业务模块按同一套目录组织：

```text
crates/module-fab/
└─ src/
   ├─ lib.rs
   ├─ contracts/
   │  ├─ commands.rs
   │  ├─ queries.rs
   │  ├─ events.rs
   │  └─ dto.rs
   ├─ facade/
   │  └─ mod.rs
   ├─ application/
   │  ├─ use_cases/
   │  ├─ queries/
   │  ├─ orchestrators/
   │  ├─ ports/
   │  ├─ mappers/
   │  └─ services/
   ├─ domain/
   │  ├─ models/
   │  ├─ value_objects/
   │  ├─ policies/
   │  └─ state_machines/
   └─ internal/
      └─ mod.rs
```

`lib.rs` 只公开：

1. `contracts`
2. `facade`
3. 少量必要的 facade 构造参数类型

`application` 和 `domain` 默认不对其他 crate 公开。

---

## 6. Dependency Rules

### 6.1 Allowed Dependencies

允许的依赖方向：

1. `apps/desktop/src-tauri` -> `composition-root`
2. `composition-root` -> `module-*`, `adapter-*`, `kernel-*`
3. `module-*` -> `kernel-foundation`, `kernel-jobs`（需要长任务时）
4. `adapter-*` -> `module-*`（只为了实现其 ports）

### 6.2 Forbidden Dependencies

禁止：

1. `module-*` -> `adapter-*`
2. `module-fab` -> `module-downloads` 内部 application / domain
3. `apps/desktop/src-tauri` -> `adapter-storage-sqlite`
4. `frontend` -> Rust 内部 ports

### 6.3 Cross-module Rule

跨模块只允许依赖：

1. 对方的 `contracts`
2. 对方公开 facade

禁止依赖：

1. 对方的 repository ports
2. 对方的 domain internals
3. 对方的 persistence models

---

## 7. Composition Root Model

组合根建议只做装配，不做业务：

```text
Desktop main.rs
  -> build_desktop_services()
       -> create sqlite pool / connection factory
       -> create provider adapters
       -> create object stores
       -> create module facades
       -> create startup pipeline
  -> register tauri commands
  -> launch app
```

建议暴露一个聚合对象：

```rust
pub struct DesktopAppServices {
    pub auth: AuthFacade,
    pub fab: FabFacade,
    pub downloads: DownloadFacade,
    pub installations: InstallationFacade,
    pub engines: EngineFacade,
    pub startup: StartupPipelineFacade,
}
```

这里的 facade 是 transport-facing 的稳定入口，不是 UI state store。

---

## 8. Tauri Command Routing

command handlers 只承担三件事：

1. 接收 IPC DTO
2. 调用 facade / use case
3. 转成统一 `CommandResultDto<T>` 或错误 envelope

示意：

```text
tauri command
  -> parse request dto
  -> services.fab.list_inventory(query)
  -> map to QueryResultDto<FabInventoryPageDto>
```

以下行为不允许进 command handler：

1. 手动开事务
2. 直接访问 SQLite
3. 自己拼 provider URL
4. 组合多个模块内部 ports

---

## 9. Recommended Use Case Stub Catalog

这部分定义第一批最值得落地的最小 use case 壳。它们都应该先做到“可测试、可注入、可委托”，而不是一步写满实现。

### 9.1 Auth

| Use case | Input | Output | Depends on |
|----------|-------|--------|------------|
| `HydrateSessionUseCase` | `HydrateSessionRequest` | `SessionStatusDto` | `SessionSecretStorePort`, `AuthProviderPort` |
| `SignInWithDeviceCodeUseCase` | `DeviceCodeRequest` | `AcceptedJobDto` or `SessionStatusDto` | `AuthProviderPort`, `SessionSecretStorePort` |
| `SignOutUseCase` | `SignOutRequest` | `CommandResultDto<()>` | `SessionSecretStorePort` |

### 9.2 Fab

| Use case | Input | Output | Depends on |
|----------|-------|--------|------------|
| `ListFabInventoryQuery` | `FabInventoryListQueryDto` | `FabInventoryPageDto` | `FabInventoryProjectionRepository` |
| `RunFabInventoryPrewarmUseCase` | `FabInventoryPrewarmRequest` | `AcceptedJobDto` or `CommandResultDto<()>` | `FabInventoryProjectionRepository`, `FabSyncCursorRepository`, `FabCatalogProviderPort` |
| `SyncFabInventoryUseCase` | `FabSyncRequest` | `AcceptedJobDto` | `FabCatalogProviderPort`, `FabInventoryProjectionRepository`, `FabSyncCursorRepository` |
| `GetFabAssetDetailQuery` | `FabAssetDetailQueryDto` | `FabAssetDetailDto` | `FabInventoryProjectionRepository`, `FabMediaMetadataRepository` |

### 9.3 Downloads

| Use case | Input | Output | Depends on |
|----------|-------|--------|------------|
| `StartDownloadUseCase` | `StartDownloadRequestDto` | `AcceptedJobDto` | `DownloadJobRepository`, `DownloadManifestProviderPort`, `JobRuntimePort` |
| `PauseDownloadUseCase` | `PauseDownloadRequestDto` | `CommandResultDto<()>` | `DownloadJobRepository`, `JobRuntimeControlPort` |
| `ResumeDownloadUseCase` | `ResumeDownloadRequestDto` | `AcceptedJobDto` | `DownloadJobRepository`, `DownloadCheckpointRepository`, `DownloadStagingObjectStore`, `DownloadManifestProviderPort`, `JobRuntimePort` |
| `ListDownloadJobsQuery` | `ListDownloadJobsQueryDto` | `DownloadJobListDto` | `DownloadJobRepository` |

### 9.4 Installations

| Use case | Input | Output | Depends on |
|----------|-------|--------|------------|
| `RegisterInstallationUseCase` | `RegisterInstallationRequestDto` | `InstallationSummaryDto` | `InstallationRecordRepository` |
| `ScanInstallationsUseCase` | `ScanInstallationsRequestDto` | `AcceptedJobDto` | `InstallationArtifactStore`, `InstallationRecordRepository`, `JobRuntimePort` |
| `GetInstallationSummaryQuery` | `InstallationSummaryQueryDto` | `InstallationSummaryDto` | `InstallationRecordRepository` |

### 9.5 Engines

| Use case | Input | Output | Depends on |
|----------|-------|--------|------------|
| `VerifyEngineInstallationUseCase` | `EngineVerifyRequestDto` | `AcceptedJobDto` | `EngineInstallationRepository`, `EngineManifestProviderPort`, `EngineVerificationSummaryRepository`, `JobRuntimePort` |
| `PlanEngineRepairUseCase` | `EngineRepairPlanRequestDto` | `EngineRepairPlanDto` | `EngineInstallationRepository`, `EngineManifestProviderPort`, `EngineVerificationSummaryRepository` |
| `ApplyEngineRepairUseCase` | `EngineRepairApplyRequestDto` | `AcceptedJobDto` | `EngineVerificationSummaryRepository`, `InstallationArtifactStore`, `DownloadFacade`, `JobRuntimePort` |

---

## 10. Example Use Case Skeletons

### 10.1 Query-style use case

```rust
pub struct ListFabInventoryQueryHandler<R> {
    repository: R,
}

impl<R> ListFabInventoryQueryHandler<R>
where
    R: FabInventoryProjectionRepository,
{
    pub fn execute(&self, query: FabInventoryListQuery) -> AppResult<FabInventoryPageDto> {
        let page = self.repository.list_page(query)?;
        Ok(FabInventoryPageDto::from(page))
    }
}
```

要点：

1. 只依赖 port
2. 不知道 SQLite 怎么实现
3. 输出是 contract DTO，不是 persistence row

### 10.2 Orchestrator-style use case

```rust
pub struct ResumeDownloadUseCase<J, C, S, M, R> {
    jobs: J,
    checkpoints: C,
    staging: S,
    manifests: M,
    runtime: R,
}

impl<J, C, S, M, R> ResumeDownloadUseCase<J, C, S, M, R>
where
    J: DownloadJobRepository,
    C: DownloadCheckpointRepository,
    S: DownloadStagingObjectStore,
    M: DownloadManifestProviderPort,
    R: JobRuntimePort,
{
    pub fn execute(&self, request: ResumeDownloadRequestDto) -> AppResult<AcceptedJobDto> {
        let job = self.jobs.get_job(&request.job_id)?.ok_or(AppError::not_found("download_job"))?;
        let checkpoint = self.checkpoints.load(&request.job_id)?;
        let staging_root = self.staging.ensure_staging_root(&request.job_id)?;
        let manifest = self.manifests.fetch_manifest(&job.target_ref)?;
        self.runtime.enqueue_resume(job, checkpoint, staging_root, manifest)
    }
}
```

要点：

1. 多端口协作发生在 use case 中
2. provider / repository / object store 不互调
3. 真正的并发执行下沉到 job runtime

---

## 11. Module-specific Startup Hooks

不是每个模块都应该暴露一个“初始化 service”。建议只暴露少量 startup hooks：

| Hook | Owner | Purpose |
|------|-------|---------|
| `hydrate_session` | Auth | 恢复本地凭据与登录态 |
| `prewarm_fab_inventory` | Fab | 已登录启动后的库存预热 |
| `resume_download_runtime` | Downloads | 恢复未完成任务 |
| `load_engine_health_projection` | Engines | 恢复上次验证结果投影 |

这些 hook 由 `StartupPipelineFacade` 按阶段调用，而不是让每个模块自行偷偷启动后台逻辑。

---

## 12. Testing Strategy for the Skeleton

第一阶段不追求全实现，先追求三类测试：

### 12.1 Use Case Unit Tests

验证点：

1. 输入校验
2. 正确调用了哪些 ports
3. 输出 contract DTO 是否稳定

### 12.2 Adapter Contract Tests

验证点：

1. `Sqlite*Repository` 是否满足对应 repository port 行为
2. `Fs*ObjectStore` 是否满足对象生命周期语义
3. `*ProviderAdapter` 是否把远程 payload 转成预期 provider model

### 12.3 Composition Smoke Tests

验证点：

1. 组合根是否能装配完整 `DesktopAppServices`
2. 所有必需 facade 是否有 concrete implementation
3. 启动阶段的最小装配不会因为依赖缺口而失败

---

## 13. First Implementation Slice Recommendation

如果按“最小但正确”的顺序起步，建议先做这四块：

1. `kernel-foundation`
2. `kernel-jobs`
3. `module-fab` 的 query facade + prewarm use case 壳
4. `module-downloads` 的 start/resume use case 壳

原因：

1. 这四块可以最快验证“module + ports + adapters + job runtime”的闭环是否成立。
2. Fab 预热和下载恢复正好覆盖 query、orchestrator、background job 三类不同 use case。
3. 这条路径最能检验之前关于缓存、长任务、端口边界的设计是否真的可接线。

---

## 14. Anti-patterns

以下做法视为偏离本稿：

1. 建一个 `backend-core` 巨型 crate，把所有模块、所有 ports、所有 facades 全塞进去。
2. 让 `apps/desktop/src-tauri` 直接创建 SQLite repository 或 HTTP client。
3. 让 `module-downloads` 直接依赖 `adapter-storage-sqlite`。
4. 让 `module-engines` 直接 import `module-downloads` 的 repository port，而不是消费其 facade / contract。
5. 用 `Manager`、`ServiceHub`、`RepositoryFactory` 一类名字把边界重新糊掉。

---

## 15. Acceptance Criteria

满足以下条件，才算这份骨架稿可进入实现前阶段：

1. 已明确 workspace、composition root、module crates、adapter crates 的职责切分。
2. 已明确模块内部目录结构与可见性原则。
3. 已明确 Tauri command handler 只能调用 facade / use case。
4. 已列出第一批最值得落地的 use case 壳。
5. 已给出最小测试策略和首个实现切片顺序。
