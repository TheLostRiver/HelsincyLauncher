# Tauri Current-repo Architecture Overview

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriArchitecturePrinciplesDesign.md`, `docs/TauriBackendSkeletonImplementationDesign.md`, `docs/TauriDocumentationBenchmarkAgainstCodexManager.md`
> Focus: current repo shape, live entrypoints, bootstrap/request chain, hotspots, and contributor landing zones

---

## 1. Purpose

这份文档不是再讲一次蓝图原则。

它只负责回答协作者最常见的当前态问题：

1. 这个仓库现在到底长什么样。
2. 当前关键入口文件在哪里。
3. 宿主、composition-root、模块和测试现在是怎么串起来的。
4. 新改动应该优先落在哪一层。

如果你需要的是深层边界原则，回到：

1. `docs/TauriRewriteArchitectureBlueprint.md`
2. `docs/TauriArchitecturePrinciplesDesign.md`

---

## 2. 当前仓库形态

当前仓库是一个“前端原型仍在根目录，Rust/Tauri backend skeleton 已经落地”的渐进式结构。

```text
MyEpicLauncher/
├─ app/                  # Next.js app router
├─ components/           # 前端原型组件
├─ src-tauri/            # 桌面宿主与 transport wiring
├─ crates/               # kernel / module / adapter / composition root
├─ docs/                 # 架构、专题设计与治理文档
├─ .artifacts/ai/        # 活跃任务协议与执行记录
├─ Cargo.toml            # Rust workspace
├─ Cargo.lock            # workspace lockfile
├─ package.json          # 前端脚本入口
└─ README.md             # 仓库第一入口
```

关键判断：

1. 这不是最终理想 monorepo 形态的完整实现。
2. 这也不再是“只有前端原型”的阶段。
3. 当前最重要的是守住 live repo shape，不把前端原型、宿主 transport、composition root 和 deep docs 混成一层。

---

## 3. 目录职责总览

### 3.1 `app/` 与 `components/`

这里仍然是前端原型和展示层。

当前职责：

1. 页面壳层和 UI 结构
2. 原型交互
3. 面向未来 typed IPC 的展示位

当前不应承接：

1. backend-owned business truth
2. 下载、修复、校验、恢复等长任务逻辑
3. 宿主 wiring 或 provider 接入逻辑

### 3.2 `src-tauri/`

这是桌面宿主边界。

当前职责：

1. 宿主 bootstrap surface
2. transport command surfaces
3. shared host state handle
4. host-level smoke test

它的定位不是业务核心，而是 host/transport glue。

### 3.3 `crates/`

这里是 Rust backend skeleton 的主体。

当前已落地的子层：

1. `kernel-foundation/`
2. `kernel-jobs/`
3. `module-fab/`
4. `module-downloads/`
5. `adapter-storage-sqlite/`
6. `adapter-provider-fab/`
7. `composition-root/`

### 3.4 `docs/`

这里负责架构蓝图、原则层、专题设计、模块文档和治理补充文档。

这里不应承载：

1. 活跃任务状态
2. 会话恢复点
3. 一次性执行日志

### 3.5 `.artifacts/ai/`

这里是运行中的任务协议面，不是长期架构说明面。

它负责：

1. 当前活跃原子任务
2. phase/AT ledger
3. append-only 进度
4. 发现与恢复点

---

## 4. 关键入口文件索引

### 4.1 仓库与工作流入口

1. `README.md`：项目第一入口和当前状态摘要
2. `CONTRIBUTING.md`：协作者入口、落点、验证与热点
3. `Cargo.toml`：workspace 入口
4. `package.json`：前端脚本入口
5. `.artifacts/ai/active-task.md`：当前原子任务边界
6. `.artifacts/ai/task-plan.md`：阶段和 AT ledger

### 4.2 宿主入口

1. `src-tauri/src/lib.rs`：宿主公开边界，导出 bootstrap、commands、state
2. `src-tauri/src/bootstrap.rs`：宿主可测试 bootstrap surface
3. `src-tauri/src/commands/mod.rs`：command 名称注册和 DTO/result 映射
4. `src-tauri/src/commands/fab.rs`：Fab transport handlers
5. `src-tauri/src/commands/downloads.rs`：Downloads transport handlers
6. `src-tauri/src/state.rs`：shared `DesktopAppServicesHandle`
7. `src-tauri/src/main.rs`：保持很薄，只进入宿主 bootstrap

### 4.3 composition root 入口

1. `crates/composition-root/src/bootstrap.rs`：当前 concrete assembly owner
2. `crates/composition-root/src/startup.rs`：startup pipeline facade

### 4.4 当前验证入口

1. `crates/kernel-foundation/tests/foundation_contract_smoke.rs`
2. `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
3. `src-tauri/tests/transport_wiring_smoke.rs`

---

## 5. 当前启动与调用链

### 5.1 宿主 bootstrap 链

当前宿主的最重要链路是：

```text
src-tauri/src/main.rs
  -> run_desktop_host()
  -> build_desktop_host_bootstrap()
  -> launcher_composition_root::build_desktop_services()
  -> DesktopAppServicesHandle
  -> REGISTERED_COMMANDS
```

这条链说明：

1. `main.rs` 不拥有真正的装配逻辑。
2. `src-tauri` 通过 composition-root 获取 concrete services。
3. host state 只持有已装配好的 `DesktopAppServices` handle。

### 5.2 composition-root 装配链

当前 `crates/composition-root/src/bootstrap.rs` 负责：

1. 生成 `DesktopBootstrapConfig`
2. 构造 storage config
3. 构造 Fab provider adapter
4. 装配 Fab facade
5. 装配 Downloads facade
6. 装配 `StartupPipelineFacade`

这里是当前 concrete dependency assembly 的唯一 owner。

### 5.3 transport 调用链

当前 transport 层的基本形态是：

```text
command handler
  -> DesktopServices
  -> module facade
  -> result mapper
  -> CommandResultDto / QueryResultDto
```

`src-tauri/src/commands/mod.rs` 已经把当前公开 command 名称集中到 `REGISTERED_COMMANDS`，并定义了共享 DTO/result envelope。

这意味着：

1. transport surface 已经有统一形状。
2. 当前还不需要真实 Tauri runtime 才能验证 host 调用链。

### 5.4 startup facade 当前状态

`crates/composition-root/src/startup.rs` 当前是 no-op shell：

1. `run_stage1_shell_ready()`
2. `run_stage2_restore_runtime_state()`
3. `run_stage3_background_prewarm()`

这说明 startup 顺序已经有边界，但具体恢复和预热逻辑还未落地。

---

## 6. 当前验证链路

当前 repo 的最关键验证入口不是宽泛的“全都跑一次”，而是三条命名 smoke/contract 线：

1. `foundation_contract_smoke`：证明基础契约面存在
2. `bootstrap_wiring_smoke`：证明 composition-root 能真实装配服务
3. `transport_wiring_smoke`：证明宿主 bootstrap、state 和 handler 调用链是通的

其中 `src-tauri/tests/transport_wiring_smoke.rs` 直接展示了当前宿主验证思路：

1. build host bootstrap
2. 断言 registered commands
3. 调一个真实 handler
4. 不启动真实 runtime

这也是当前仓库最值得保持的验证风格。

---

## 7. 当前结构热点

当前最需要克制继续堆逻辑的热点包括：

1. `README.md`：仓库入口，容易再次发生状态漂移
2. `CONTRIBUTING.md`：新的 contributor-facing 入口，不应长成第二份蓝图
3. `src-tauri/src/lib.rs`：宿主公开入口，不应塞进过多流程
4. `src-tauri/src/bootstrap.rs`：宿主 bootstrap 面，不应退化成新的 God layer
5. `src-tauri/src/commands/mod.rs`：共享 transport surface，容易被无序追加
6. `crates/composition-root/src/bootstrap.rs`：assembly owner，最容易被误塞业务逻辑
7. `Cargo.toml`：workspace 入口，改动会波及多个 slice
8. `.artifacts/ai/active-task.md` 与 `task-plan.md`：任务协议面，误改会影响恢复路径

热点的共同特征不是“不能改”，而是：

1. 它们要么是入口。
2. 要么是组合面。
3. 要么是协议控制面。

---

## 8. 建议的改动落点

为了减少结构污染，新增需求尽量按下面落点：

1. 新 UI 壳层或展示组件：`app/`、`components/`
2. 新宿主命令、host state、transport glue：`src-tauri/`
3. 新模块契约或 facade：`crates/module-*`
4. 新 adapter：`crates/adapter-*`
5. 新 concrete wiring：`crates/composition-root/`
6. 新原则层或专项设计：`docs/`
7. 新当前态协作入口：`README.md`、`CONTRIBUTING.md`、current-repo overview 这类入口层文档

不要这样落点：

1. 为了省事把 module 行为塞进 `src-tauri`
2. 为了省事把 concrete wiring 塞进模块内部
3. 为了省事把活跃任务状态堆回 `docs/`

---

## 9. 这份文档不负责什么

它不负责：

1. 取代蓝图
2. 取代原则文档
3. 取代模块文档
4. 记录活跃任务状态

如果你需要的是：

1. 深层架构原则：看 `docs/TauriArchitecturePrinciplesDesign.md`
2. 当前协作者落点与验证：看 `CONTRIBUTING.md`
3. 活跃任务恢复：看 `.artifacts/ai/`

---

## 10. 一句话总结

当前 repo 的核心形状可以压成一句话：

前端原型仍在根目录，宿主 transport 在 `src-tauri/`，concrete assembly 在 `crates/composition-root/`，模块与 adapter 在 `crates/`，深度约束在 `docs/`，活跃任务协议在 `.artifacts/ai/`。