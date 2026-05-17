# Tauri Composition Root Wiring Design

> Status: local draft v2
> Date: 2026-05-04
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `docs/TauriFirstCrateApiDrafts.md`, `docs/TauriFabInventoryLoadingDesign.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriEngineVerificationRepairDesign.md`
> Focus: composition-root crate public API, adapter injection, DesktopAppServices assembly, startup pipeline staging, tauri integration boundary

---

## 1. Purpose

前面的文档已经分别说明了：

1. 模块边界和 ports / adapters 规则
2. 第一批真实 crate 清单
3. `kernel-foundation`、`kernel-jobs`、`module-fab`、`module-downloads` 的 crate-level API 草图

当前仍然缺一个最关键的问题：

**这些 crate 最终在谁那里完成接线，以及接线顺序如何固定。**

这份文档专门定义 `launcher-composition-root` 的职责、公共 API 和 wiring 规则，目标是防止后续实现时把依赖注入、startup 阶段和 Tauri glue 重新打散。

---

## 2. Wiring Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| One assembly owner | 只有 composition root 知道 concrete adapter 类型 |
| Facade-only exposure | 桌面宿主和 Tauri commands 只能拿到 facade 聚合，不拿 repository |
| Staged startup | 启动预热必须按阶段显式运行，不藏在构造函数里 |
| Testable wiring | `build_desktop_services()` 必须可做 smoke test 和失败路径测试 |
| Minimal current slice | 当前桌面基线装配 Fab、Downloads、Engines 的最小依赖，其他模块仍可后置 |
| No hidden background boot | 模块不会在构造时偷偷启动任务 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 让 composition root 承担业务编排 | 它只负责装配，不执行业务规则 |
| 在 Tauri handler 内手动 new adapter | 会让 transport 层重新知道基础设施细节 |
| 用全局 service locator 解决依赖传递 | 会把边界重新糊掉 |
| 在模块 facade 构造函数里自动触发 prewarm / resume | 会让启动顺序不可观测、不可测试 |

---

## 3. Composition Root Responsibility

`launcher-composition-root` 只负责五件事：

1. 创建基础 runtime 环境
2. 创建 concrete adapters
3. 用这些 adapters 构造 module facades
4. 组装 `DesktopAppServices`
5. 组装 `StartupPipelineFacade`

它不负责：

1. 执行 Fab inventory sync 业务
2. 执行 download resume 业务
3. 直接响应前端 IPC
4. 决定 UI 路由或窗口状态

---

## 4. Current Wiring Scope

当前桌面宿主基线接线覆盖以下能力：

### 4.1 Included

1. `launcher-kernel-foundation`
2. `launcher-kernel-jobs`
3. `launcher-module-fab`
4. `launcher-module-downloads`
5. `launcher-module-engines`
6. `launcher-adapter-storage-sqlite`
7. `launcher-adapter-provider-fab`

### 4.2 Deferred

1. `module-auth`
2. `module-installations`
3. `adapter-filesystem`
4. `adapter-secure-storage`
5. `adapter-platform-desktop`

说明：

1. 延后不代表没有边界，只代表 wiring 顺序后置。
2. `StartupPipelineFacade` 第一版仍允许为 Auth 预留空 hook；`Engines` 已进入 `DesktopAppServices`，但更宽的 verify / repair orchestration 仍可后置。

---

## 5. Public API of `launcher-composition-root`

### 5.1 Public Surface Rule

该 crate 对外只暴露：

1. `DesktopBootstrapConfig`
2. `DesktopAppServices`
3. `StartupPipelineFacade`
4. `build_desktop_services()`

不暴露：

1. repository concrete type
2. SQLite connection 细节
3. provider adapter concrete type
4. 任意 module 内部 ports

### 5.2 `lib.rs` Sketch

```rust
pub mod bootstrap;
pub mod startup;

pub use bootstrap::{build_desktop_services, DesktopAppServices, DesktopBootstrapConfig};
pub use startup::StartupPipelineFacade;
```

### 5.3 `DesktopBootstrapConfig`

```rust
pub struct DesktopBootstrapConfig {
    pub app_data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub sqlite_path: PathBuf,
    pub enable_fab: bool,
    pub enable_downloads: bool,
    pub enable_startup_prewarm: bool,
    pub default_download_slots: u16,
}
```

规则：

1. 这里只放 wiring 所需配置，不放业务 DTO。
2. provider token、账号状态等运行时业务数据不从这里传入。

### 5.4 `DesktopAppServices`

```rust
pub struct DesktopAppServices {
    pub fab: Arc<FabFacade<...>>,
    pub downloads: Arc<DownloadFacade<...>>,
    pub engines: Arc<EngineFacade<...>>,
    pub startup: Arc<StartupPipelineFacade>,
}
```

固定规则：

1. 桌面宿主只拿这个聚合对象。
2. 它只暴露 facade，不暴露 repository、adapter、job host。

### 5.5 `build_desktop_services()`

```rust
pub fn build_desktop_services(
    config: DesktopBootstrapConfig,
) -> AppResult<DesktopAppServices>;
```

该函数必须：

1. 同步返回完整接线结果或明确错误
2. 不在构造过程中偷偷触发 prewarm / sync / resume
3. 可在测试中被直接调用

---

## 6. Internal Wiring Layers

建议在 composition root 内部拆成四层私有 builder，而不是一个巨型函数：

```text
bootstrap/
  config.rs
  foundation.rs
  adapters.rs
  modules.rs
  services.rs
```

### 6.1 Foundation Layer

负责创建：

1. `Clock`
2. `IdGenerator`
3. logging / tracing bootstrap handle
4. SQLite connection factory
5. shared Tokio runtime handle or runtime bindings

### 6.2 Adapter Layer

负责创建：

1. Fab 的 SQLite repositories
2. Downloads 的 SQLite repositories
3. Fab provider adapter
4. shared job runtime concrete implementation

### 6.3 Module Layer

负责：

1. 用 adapters 构造 `FabFacade`
2. 用 adapters 构造 `DownloadFacade`
3. 用 runtime / placeholder deps 构造 `EngineFacade`

### 6.4 Service Aggregation Layer

负责：

1. 组装 `StartupPipelineFacade`
2. 组装 `DesktopAppServices`

---

## 7. Concrete Wiring Sequence

推荐接线顺序必须固定，不允许每个调用方自行拼装：

```text
build_desktop_services(config)
  -> build_foundation_context(config)
  -> build_job_runtime(foundations)
  -> build_storage_adapters(foundations)
  -> build_provider_adapters(foundations)
  -> build_fab_module(storage, providers, jobs)
  -> build_download_module(storage, jobs)
    -> build_engines_module(jobs)
  -> build_startup_pipeline(fab, downloads)
  -> return DesktopAppServices
```

为什么这样排：

1. foundation 先于一切，因为 repository / provider / runtime 都依赖它。
2. job runtime 必须先有，Fab prewarm 和 Downloads start/resume 都需要。
3. `Engines` 当前虽然不依赖完整 storage/provider wiring，但它的 accepted-job 路径仍依赖共享 job runtime，所以仍应在 runtime 之后组装。
4. module facade 只能建立在 adapters 或 runtime 等已准备好的依赖之后。
5. startup pipeline 最后组装，因为它依赖模块 facade，而不是反过来。

---

## 8. Internal Builder Sketches

### 8.1 Foundation Context

```rust
struct FoundationContext {
    clock: Arc<SystemClock>,
    id_generator: Arc<UuidGenerator>,
    sqlite_factory: Arc<SqliteConnectionFactory>,
    runtime_handle: tokio::runtime::Handle,
}
```

### 8.2 Storage Bundle

```rust
struct StorageAdapterBundle {
    fab_projection_repo: Arc<SqliteFabInventoryProjectionRepository>,
    fab_cursor_repo: Arc<SqliteFabSyncCursorRepository>,
    fab_media_repo: Arc<SqliteFabMediaMetadataRepository>,
    download_job_repo: Arc<SqliteDownloadJobRepository>,
    download_checkpoint_repo: Arc<SqliteDownloadCheckpointRepository>,
}
```

### 8.3 Provider Bundle

```rust
struct ProviderAdapterBundle {
    fab_catalog_provider: Arc<EpicFabCatalogProviderAdapter>,
}
```

### 8.4 Runtime Bundle

```rust
struct RuntimeBundle {
    job_runtime: Arc<SharedJobRuntimeHost>,
}
```

### 8.5 Module Bundle

```rust
struct ModuleBundle {
    fab: Arc<FabFacade<...>>,
    downloads: Arc<DownloadFacade<...>>,
    engines: Arc<EngineFacade<...>>,
}
```

这些 bundle 都必须保持私有，不能穿透到桌面宿主。

---

## 9. `StartupPipelineFacade`

### 9.1 Purpose

它是启动阶段唯一公开入口，用来显式执行各阶段 hook。

它不是：

1. service locator
2. app lifetime manager
3. 模块内部端口聚合器

### 9.2 Public API Sketch

```rust
pub struct StartupPipelineFacade {
    fab: Arc<FabFacade<...>>,
    downloads: Arc<DownloadFacade<...>>,
}

impl StartupPipelineFacade {
    pub async fn run_stage1_shell_ready(&self) -> AppResult<()>;
    pub async fn run_stage2_restore_runtime_state(&self) -> AppResult<()>;
    pub async fn run_stage3_background_prewarm(&self) -> AppResult<()>;
}
```

补充说明：

1. 当前基线不要求把 `Engines` facade 纳入 `StartupPipelineFacade` 的阶段成员。
2. `engines.verify.start` 当前通过 host command 直接进入 `EngineFacade` 和共享 job runtime。
3. 是否把 engines repair / verification restore orchestration 纳入 startup pipeline，属于后续更宽切片。

### 9.3 Stage Ownership

#### Stage 1: shell ready

目标：

1. 应用壳可交互
2. 不运行重任务

该阶段不调用任何模块业务 facade。

#### Stage 2: restore runtime state

目标：

1. 恢复轻量运行态
2. 允许恢复未完成 job 的只读快照或 runtime registration

第一批建议包含：

1. Downloads 的 runtime snapshot / queued job registration

不包含：

1. Fab 远程同步
2. 大量 provider 请求

#### Stage 3: background prewarm

目标：

1. 在不阻塞 shell 的前提下启动后台预热
2. 统一调度 Fab inventory prewarm 和后续后台校准

第一批建议包含：

1. `fab.run_startup_prewarm(...)`

规则：

1. 只有 capability enabled 且会话可用时才执行。
2. 通过 facade 明确调用，不允许模块构造器里自己启动。

### 9.4 One-shot Runtime Execution Helper

Current Rust reality:

1. `SharedJobRuntimeHost::run_next_execution_turn(...)` can select and dispatch one queued job while respecting the coarse runtime capacity policy.
2. `build_desktop_services(...)` already assembles both the shared runtime host and `JobDriverRegistry<()>`.
3. `StartupPipelineFacade` already owns staged restore/prewarm entry points and receives the driver registry for stage 2 restore, but it does not yet hold the shared runtime host.
4. No composition-owned public helper currently combines runtime plus registry to run exactly one queued execution turn.

First Rust slice:

1. add an explicit one-shot helper on the composition-owned startup/runtime surface, tentatively `StartupPipelineFacade::run_one_runtime_execution_turn(...)`;
2. wire a cloned `SharedJobRuntimeHost` into that surface from `build_startup_pipeline(...)` without exposing concrete repositories or changing module facade constructors;
3. when runtime or registry wiring is absent, return an explicit non-terminal deferred execution result rather than silently succeeding;
4. when both are present, delegate exactly once to `SharedJobRuntimeHost::run_next_execution_turn(&registry)`;
5. do not call this helper automatically from `build_desktop_services(...)`, `run_stage2_restore_runtime_state(...)`, or `run_stage3_background_prewarm(...)`;
6. keep scheduler loops/background tasks, durable leases, precise active-slot accounting, terminal completion/failure projection, downloads concrete IO, host transport, frontend, and SQLite schema changes out of this slice.

Validation should stay narrow:

1. focused composition-root tests prove the helper defers when runtime execution wiring is absent;
2. focused composition-root tests prove the helper dispatches one queued fake-driver snapshot when runtime and registry are wired;
3. existing startup restore and prewarm tests keep passing and do not accidentally invoke execution;
4. affected `launcher-composition-root` tests, `launcher-composition-root` check, scoped rustfmt, and scoped diff checks pass.

---

## 10. Startup Flow End-to-End

建议桌面宿主启动顺序：

```text
desktop main
  -> load DesktopBootstrapConfig
  -> build_desktop_services(config)
  -> register tauri commands with DesktopAppServices
  -> show shell window
  -> await startup.run_stage1_shell_ready()
  -> spawn startup.run_stage2_restore_runtime_state()
  -> spawn startup.run_stage3_background_prewarm()
```

关键规则：

1. `build_desktop_services()` 只装配，不做业务启动。
2. `run_stage2_*` 和 `run_stage3_*` 的调用时机由桌面宿主决定。
3. 任何阶段失败都应返回结构化错误并写日志，但不能把阶段副作用藏到其他地方。

---

## 11. Tauri Integration Boundary

### 11.1 Command Registration Rule

Tauri command handler 只允许依赖：

1. `State<DesktopAppServices>`
2. 前端 IPC DTO

不允许依赖：

1. `SqliteFabInventoryProjectionRepository`
2. `EpicFabCatalogProviderAdapter`
3. `SharedJobRuntimeHost`

### 11.2 Command Example

```rust
#[tauri::command]
pub async fn downloads_resume(
    state: State<'_, DesktopAppServices>,
    request: ResumeDownloadRequestDto,
) -> CommandResultDto<AcceptedJobDto> {
    map_command_result(state.downloads.resume_download(request))
}
```

### 11.3 Event Bridge Rule

事件桥只负责：

1. 订阅 facade / runtime 暴露的稳定事件源
2. 转换成前端可消费 event DTO
3. emit 给对应窗口或全局事件通道

事件桥不负责：

1. 更新数据库
2. 调用 provider
3. 触发业务重试

---

## 12. Failure and Observability Rules

### 12.1 Build-time Failure

如果 wiring 缺口导致 `build_desktop_services()` 失败，必须：

1. 返回结构化 `AppError`
2. 明确指出失败的 builder 段，例如 `build_storage_adapters` 或 `build_fab_module`

### 12.2 Startup-stage Failure

如果 `run_stage2_*` 或 `run_stage3_*` 失败：

1. 不使整个 `DesktopAppServices` 失效
2. 记录结构化日志
3. 允许 UI 显示可恢复提示或局部错误状态

### 12.3 Recommended Logs

建议至少有这些日志点：

1. `composition_root.build.started`
2. `composition_root.build.completed`
3. `startup.stage2.restore_runtime_state.started`
4. `startup.stage2.restore_runtime_state.failed`
5. `startup.stage3.background_prewarm.started`
6. `startup.stage3.background_prewarm.completed`

---

## 13. Testing Strategy

### 13.1 Composition Smoke Test

验证：

1. `build_desktop_services()` 能返回完整 `DesktopAppServices`
2. `fab`、`downloads`、`engines`、`startup` 四个 facade 都存在

### 13.2 Builder Failure Test

验证：

1. SQLite path 非法时，`build_storage_adapters` 失败路径清晰
2. provider 配置缺失时，`build_provider_adapters` 失败路径清晰

### 13.3 Startup Ordering Test

验证：

1. `build_desktop_services()` 不触发任何 prewarm
2. `run_stage2_*` 先于 `run_stage3_*`
3. Stage 3 才允许触发 Fab inventory prewarm

### 13.4 Handler Boundary Test

验证：

1. command handler 只通过 `DesktopAppServices` 调 facade
2. handler 测试中不需要 concrete adapter 类型即可完成 mock

---

## 14. Recommended Internal File Layout

建议 `launcher-composition-root` 采用以下结构：

```text
src/
  lib.rs
  bootstrap/
    mod.rs
    config.rs
    foundation.rs
    adapters.rs
    modules.rs
    services.rs
  startup/
    mod.rs
    pipeline.rs
    stages.rs
```

说明：

1. `bootstrap/*` 只关注装配。
2. `startup/*` 只关注阶段化调用。
3. 不把 Tauri command 放进 composition root。

---

## 15. Anti-patterns

以下做法视为偏离本稿：

1. 在 `main.rs` 里手动创建全部 repository、provider、runtime，再逐个传给 command handler。
2. 让 `FabFacade::new(...)` 内部自动启动 startup prewarm。
3. 把 `StartupPipelineFacade` 设计成能直接访问所有内部 repository 的巨型对象。
4. 让 Stage 2 或 Stage 3 的启动任务从 command handler 第一次调用时才懒触发。
5. 为了省事把 concrete adapter 类型直接挂到 `DesktopAppServices` 上。

---

## 16. Acceptance Criteria

满足以下条件，才算这份 wiring 稿可进入实现前阶段：

1. 已明确 `launcher-composition-root` 的公共 API 只暴露 `DesktopBootstrapConfig`、`DesktopAppServices`、`StartupPipelineFacade` 和 `build_desktop_services()`。
2. 已明确 adapter 注入和 module facade 构造的固定顺序。
3. 已明确 startup pipeline 的 Stage 1 / Stage 2 / Stage 3 边界。
4. 已明确 Tauri command / event bridge 只能通过 `DesktopAppServices` 接触业务。
5. 已明确 smoke test、失败路径测试和启动顺序测试的最小矩阵。
