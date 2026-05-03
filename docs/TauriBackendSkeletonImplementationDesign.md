# Tauri Backend Skeleton Implementation Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriFirstCrateApiDrafts.md`
> Focus: current-repo backend scaffold target, first file set, phased implementation order, validation gates

---

## 1. Purpose

是的，**当前仓库还没有后端骨架代码**。

现有工作区仍然只有前端原型：`app/`、`components/`、`package.json` 和相关 Next.js 配置已经存在，但 Rust workspace、Tauri 宿主、composition root 和 module crates 还没有真正落盘。

同时，现有几份后端相关文档分别回答了：

1. crate 应如何拆分
2. composition root 应如何接线
3. 第一批 crate 的 API 草图是什么

它们仍然缺一块最关键的内容：

**如何基于当前这个仓库，把第一版可编译的后端骨架按顺序落出来。**

这份文档专门补这个缺口。

---

## 2. Current Repo Reality

| Item | Current State | Implication |
|------|---------------|-------------|
| frontend host | 仓库根是 Next.js 工程，已有 `app/`、`components/`、`package.json` | 当前前端原型可继续保留，不应一上来大搬家 |
| Rust workspace | 不存在根 `Cargo.toml` | 目前无法做 `cargo check --workspace` |
| Tauri host | 不存在 `src-tauri/` | 当前没有桌面宿主入口 |
| backend crates | 不存在 `crates/` | facade、kernel、adapter 都还只是文档概念 |
| implementation guide | 不存在针对当前仓库的骨架落地文档 | 下一步实现顺序不够稳定，容易一上来改成大迁移 |

结论：

1. 当前仓库没有后端骨架代码。
2. 现有后端文档更偏“目标结构设计”，还不是“当前仓库的骨架实现文档”。

---

## 3. Implementation Decision

为了让后端骨架尽快落地，第一版不直接把当前仓库改成蓝图里的 `apps/desktop/` monorepo 形态，而采用更小步、可回退的方案：

1. **保留当前仓库根的 Next.js 前端结构不动**。
2. 在仓库根新增 Rust workspace `Cargo.toml`。
3. 在仓库根新增 `src-tauri/` 作为桌面宿主。
4. 在仓库根新增 `crates/` 承载第一批 backend crates。
5. 等后端骨架真实跑通后，再决定是否把前端迁移到 `apps/desktop/`。

这样做的原因很直接：

1. 当前前端原型仍在快速迭代，不值得先做目录大迁移。
2. Tauri 标准结构本身就允许“前端在仓库根，Rust 在 `src-tauri/`”。
3. 先把 `cargo check`、composition root 和 facade 骨架跑起来，比先追求理想目录更重要。

---

## 4. Current-repo Target Layout

第一版建议采用下面这个**贴合当前仓库**的骨架：

```text
MyEpicLauncher/
├─ app/
├─ components/
├─ docs/
├─ package.json
├─ Cargo.toml
├─ Cargo.lock
├─ src-tauri/
│  ├─ Cargo.toml
│  ├─ tauri.conf.json
│  └─ src/
│     ├─ main.rs
│     ├─ commands/
│     └─ state.rs
└─ crates/
   ├─ composition-root/
   ├─ kernel-foundation/
   ├─ kernel-jobs/
   ├─ module-fab/
   ├─ module-downloads/
   ├─ adapter-storage-sqlite/
   └─ adapter-provider-fab/
```

这里的关键判断是：

1. 当前阶段的目标是“出现一个真实可编译的后端骨架”。
2. 当前阶段的目标不是“立刻把 repo 目录整理成最终理想态”。

---

## 5. First Implementation Slice

第一批真实创建的 crate 固定为：

| Kind | Directory | Package Name | Role |
|------|-----------|--------------|------|
| host | `src-tauri` | `my-epic-launcher-desktop` | Tauri 入口和命令注册 |
| composition | `crates/composition-root` | `launcher-composition-root` | 唯一 assembly owner |
| kernel | `crates/kernel-foundation` | `launcher-kernel-foundation` | 通用错误、时钟、ID、分页 |
| kernel | `crates/kernel-jobs` | `launcher-kernel-jobs` | 通用长任务协议 |
| module | `crates/module-fab` | `launcher-module-fab` | Fab facade、contracts、ports |
| module | `crates/module-downloads` | `launcher-module-downloads` | Downloads facade、contracts、ports |
| adapter | `crates/adapter-storage-sqlite` | `launcher-adapter-storage-sqlite` | SQLite repository stubs |
| adapter | `crates/adapter-provider-fab` | `launcher-adapter-provider-fab` | Fab provider adapter stub |

第一版明确不做：

1. `module-auth`
2. `module-installations`
3. `module-engines`
4. `adapter-filesystem`
5. `adapter-secure-storage`
6. 真正的 OAuth、下载、修复、库存同步 IO

这些边界已经在其他文档里定义，但实现顺序后置。

---

## 6. Phase-by-phase Implementation Order

### Phase A: Workspace Bootstrap

先创建：

1. 根 `Cargo.toml`
2. `src-tauri/Cargo.toml`
3. `src-tauri/tauri.conf.json`
4. `src-tauri/src/main.rs`
5. `src-tauri/src/state.rs`

目标：

1. 让工作区第一次具备 Rust workspace 入口。
2. 让桌面宿主有地方挂 `DesktopAppServices`。

限制：

1. 此阶段不直接 new 任何 SQLite repository。
2. `main.rs` 只做 bootstrap、state 注册和 command 注册。

### Phase B: Kernel Crates

先补：

1. `launcher-kernel-foundation`
2. `launcher-kernel-jobs`

最小可用 API：

1. `AppError`
2. `AppResult<T>`
3. `Clock`
4. `JobId`
5. `JobSnapshot`
6. `JobState`
7. `JobRuntime` trait

目标：

1. 给后续 module 和 composition root 提供稳定底座。
2. 不让任何业务 crate 自己发明错误模型和 job 协议。

### Phase C: Module and Adapter Stubs

然后补：

1. `launcher-module-fab`
2. `launcher-module-downloads`
3. `launcher-adapter-storage-sqlite`
4. `launcher-adapter-provider-fab`

每个 module crate 只要求先具备：

1. `contracts`
2. `facade`
3. `application/ports`
4. 最小空实现或 stub use case

每个 adapter crate 只要求先具备：

1. port 实现壳
2. 构造函数
3. compile-time 依赖方向正确

此阶段不要求：

1. 完整 SQL schema
2. 完整 HTTP 协议
3. 真正的断点续传实现

### Phase D: Composition Root Wiring

再补：

1. `build_desktop_services()`
2. `DesktopBootstrapConfig`
3. `DesktopAppServices`
4. `StartupPipelineFacade`

此阶段的重点是：

1. 真正把 concrete adapters 收口到 composition root。
2. 让 `src-tauri` 只依赖 facade 聚合，不碰 repository。

### Phase E: Tauri Transport Integration

最后接：

1. `src-tauri/src/commands/fab.rs`
2. `src-tauri/src/commands/downloads.rs`
3. `tauri::State<DesktopAppServices>`

第一版命令只要求：

1. 能调用 facade
2. 能返回空列表或 stub snapshot
3. 能证明 transport -> composition root -> facade 这条链是通的

第一版不要求：

1. 真实 provider 登录
2. 真正下载任务执行
3. 完整事件流

---

## 7. Atomic Task Breakdown

上面的 Phase A-E 仍然偏阶段视角。真正开工时，应该继续压缩成**原子任务**：每个任务只碰少量文件，只解决一个明确目标，并绑定一个最小验证动作。

### 7.1 Workspace Membership Rule

这里必须先固定一个实现约束：

1. 根 `Cargo.toml` 不能预先写入不存在的 package path，否则最早几步就会让 workspace 失效。
2. 因此，**凡是第一次引入某个 workspace member 的任务，都必须同时更新根 `Cargo.toml`**。
3. 这意味着 A2、B1、B3、C1、C2、C3、C4、D1 都要把根 `Cargo.toml` 视为共享修改文件，而不是只有 A1 改它一次。
4. A1 只负责建立 workspace root 和共享依赖策略，不假装第一天就把所有 future crate 路径都注册完成。

这样做的原因是：

1. Cargo workspace 的成员路径必须和当前真实存在的 package 对齐。
2. 后续每个包级 `cargo check -p ...` 都依赖它已经在 root workspace 中被真实接入。

### 7.2 Atomic Task Table

建议拆成下面这 14 个原子任务：

| Task | Files | Goal | Minimal Validation |
|------|-------|------|--------------------|
| A1 | `Cargo.toml` | 建立 workspace root、统一依赖版本和 member 接入规则，不预写不存在的 package path | `cargo metadata --format-version 1` |
| A2 | `Cargo.toml`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` | 建立桌面宿主 manifest 和配置壳，并把 `src-tauri` 第一次接入 workspace | `cargo metadata --format-version 1` |
| A3 | `src-tauri/src/main.rs`, `src-tauri/src/state.rs` | 建立桌面入口和共享 state 类型 | `cargo check -p my-epic-launcher-desktop` |
| B1 | `Cargo.toml`, `crates/kernel-foundation/Cargo.toml`, `crates/kernel-foundation/src/lib.rs` | 建立 foundation crate 外壳，并把它第一次接入 workspace | `cargo check -p launcher-kernel-foundation` |
| B2 | `crates/kernel-foundation/src/error.rs`, `clock.rs`, `ids.rs`, `paging.rs`, `time.rs` | 补齐最小基础类型和导出面 | `cargo test -p launcher-kernel-foundation` |
| B3 | `Cargo.toml`, `crates/kernel-jobs/Cargo.toml`, `crates/kernel-jobs/src/lib.rs`, `model.rs`, `runtime.rs` | 建立通用 job 协议壳，并把它第一次接入 workspace | `cargo check -p launcher-kernel-jobs` |
| C1 | `Cargo.toml`, `crates/module-fab/Cargo.toml`, `crates/module-fab/src/lib.rs`, `contracts/*`, `facade/*` | 建立 Fab 模块公共边界，并把它第一次接入 workspace | `cargo check -p launcher-module-fab` |
| C2 | `Cargo.toml`, `crates/module-downloads/Cargo.toml`, `crates/module-downloads/src/lib.rs`, `contracts/*`, `facade/*` | 建立 Downloads 模块公共边界，并把它第一次接入 workspace | `cargo check -p launcher-module-downloads` |
| C3 | `Cargo.toml`, `crates/adapter-storage-sqlite/Cargo.toml`, `crates/adapter-storage-sqlite/src/lib.rs` | 建立 SQLite adapter stub，并把它第一次接入 workspace | `cargo check -p launcher-adapter-storage-sqlite` |
| C4 | `Cargo.toml`, `crates/adapter-provider-fab/Cargo.toml`, `crates/adapter-provider-fab/src/lib.rs` | 建立 Fab provider adapter stub，并把它第一次接入 workspace | `cargo check -p launcher-adapter-provider-fab` |
| D1 | `Cargo.toml`, `crates/composition-root/Cargo.toml`, `crates/composition-root/src/lib.rs` | 建立 composition root 公共 API 壳，并把它第一次接入 workspace | `cargo check -p launcher-composition-root` |
| D2 | `crates/composition-root/src/bootstrap.rs`, `startup.rs` | 接出 `build_desktop_services()` 和 startup facade | `cargo test -p launcher-composition-root` |
| E1 | `src-tauri/src/commands/fab.rs`, `src-tauri/src/commands/downloads.rs` | 建立 transport -> facade 的最小命令面 | `cargo check -p my-epic-launcher-desktop` |
| E2 | `src-tauri/src/main.rs`, `src-tauri/src/commands/mod.rs`, `src-tauri/tests/transport_wiring_smoke.rs` | 注册 commands 和 shared state，并补一条宿主级 smoke test 来证明 wiring 已真正打通 | `cargo test -p my-epic-launcher-desktop transport_wiring_smoke` |

原子任务规则：

1. 一个任务失败后，不要继续开下一个 crate，再修同一层。
2. 一个任务默认不同时改 module、adapter、host 三层。
3. 一个任务完成后立即跑对应的最小验证，不等到最后一起检查。
4. 如果某个任务自然会扩成十几个文件，说明任务拆分还不够原子。
5. E 阶段要证明的是“host wiring 生效”，因此不能只靠 `cargo check`；至少要有一条不依赖真实 provider IO 的宿主 smoke test。

推荐的执行节奏：

1. A1-A3 解决“工作区与宿主存在性”。
2. B1-B3 解决“通用内核存在性”。
3. C1-C4 解决“模块与适配器边界存在性”。
4. D1-D2 解决“接线存在性”。
5. E1-E2 解决“桌面 transport 连通性”。

---

## 8. Minimum File Checklist

第一批最少要真正落地的文件如下：

| Path | Purpose |
|------|---------|
| `Cargo.toml` | 定义 workspace members 和统一依赖版本 |
| `src-tauri/Cargo.toml` | 桌面宿主 manifest |
| `src-tauri/tauri.conf.json` | Tauri 桌面配置 |
| `src-tauri/src/main.rs` | 创建 app 并注册 commands |
| `src-tauri/src/state.rs` | 存放共享 `DesktopAppServices` 挂载类型 |
| `src-tauri/tests/transport_wiring_smoke.rs` | 验证 commands 注册、shared state 注入和 facade 调用路径 |
| `crates/kernel-foundation/src/lib.rs` | 导出基础错误、时间、ID、分页类型 |
| `crates/kernel-jobs/src/lib.rs` | 导出 job 协议和 runtime traits |
| `crates/module-fab/src/lib.rs` | 导出 Fab facade 和 contracts |
| `crates/module-downloads/src/lib.rs` | 导出 Downloads facade 和 contracts |
| `crates/adapter-storage-sqlite/src/lib.rs` | 导出 SQLite adapter 壳 |
| `crates/adapter-provider-fab/src/lib.rs` | 导出 Fab provider adapter 壳 |
| `crates/composition-root/src/lib.rs` | 导出 `build_desktop_services()` |

若上述文件都还不存在，就不能说“后端骨架已经建立”。

---

## 9. Public Surface Rules

第一版骨架的公开边界固定如下：

| Crate | Must Expose | Must Not Expose |
|------|-------------|-----------------|
| `launcher-kernel-foundation` | `AppError`, `AppResult`, `Clock`, `JobId` | 任意业务 DTO |
| `launcher-kernel-jobs` | `JobState`, `JobSnapshot`, `JobRuntime` | 下载或库存业务规则 |
| `launcher-module-fab` | `contracts`, `FabFacade` | SQLite row model |
| `launcher-module-downloads` | `contracts`, `DownloadFacade` | scheduler 内部实现细节 |
| `launcher-adapter-storage-sqlite` | adapter constructors | Tauri state 或 window 对象 |
| `launcher-adapter-provider-fab` | provider adapter constructor | 前端 view state |
| `launcher-composition-root` | `DesktopBootstrapConfig`, `DesktopAppServices`, `build_desktop_services()` | repository concrete type |
| `src-tauri` | command handlers | 业务 use case 或 SQL 细节 |

---

## 10. Validation Gates

后端骨架第一版完成时，至少要过以下检查：

1. `cargo check --workspace`
2. `cargo test -p launcher-kernel-foundation`
3. `cargo test -p launcher-composition-root`
4. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke`
5. `npm run build` 仍然不被新增 Rust/Tauri 文件破坏

这五项都不依赖 Tauri CLI，因此都应作为第一版骨架的硬门槛。

额外可选验证：

1. `cargo check -p my-epic-launcher-desktop`
2. `cargo tauri dev` 能拉起空桌面壳并注册 stub commands

---

## 11. Non-goals of the First Skeleton

第一版后端骨架明确不追求：

1. 真正可用的登录流程
2. 真正可用的下载与断点续传
3. 真实 SQLite schema 和迁移系统
4. 自动代码生成的 TS contracts
5. 立即把前端搬进 `apps/desktop/`

第一版只回答一个问题：

**我们是否已经把“后端能从哪儿长出来”这件事变成了真实代码骨架，而不是继续停留在抽象图纸。**

---

## 12. Recommended Commit Boundaries

如果下一步要真正开工，建议按下面顺序分提交：

1. `chore: add cargo workspace and tauri host scaffold`
2. `feat: add kernel foundation and job runtime crates`
3. `feat: add fab and downloads module stubs`
4. `feat: wire composition root into tauri host`

这样拆的原因是：

1. 每次提交都能对应一个清晰可验证的骨架层。
2. 出问题时更容易定位是 workspace、kernel、module 还是 wiring 失效。

---

## 13. Exit Criteria

当满足下面五条时，可以认为“后端骨架已经真正出现”：

1. 仓库里已经存在 `Cargo.toml`、`src-tauri/`、`crates/`
2. `src-tauri` 只通过 `DesktopAppServices` 消费后端能力
3. `module-*` 与 `adapter-*` 依赖方向满足既有文档规则
4. 至少有一条宿主级 smoke test 能验证 command 注册和 shared state 注入
5. 至少 `cargo check --workspace` 可以稳定通过

在这之前，都只能说“我们有后端架构设计”，不能说“我们有后端骨架实现”。