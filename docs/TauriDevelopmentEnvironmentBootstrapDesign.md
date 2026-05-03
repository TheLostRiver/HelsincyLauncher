# Tauri Development Environment Bootstrap Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriBackendSkeletonImplementationDesign.md`, `docs/TauriStartupPipelineDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, `docs/TauriSecurityCredentialsAndPermissionsDesign.md`
> Focus: local machine prerequisites, Windows toolchain setup, repo bootstrap commands, phased verification, first-day developer workflow

---

## 1. Purpose

当前仓库并不是完全没有环境前提信息。

问题在于：**Node、Next.js、未来 Tauri 2、Rust、Windows Build Tools、WebView2、以及“现在能跑什么命令/什么时候开始能跑 cargo 命令”的信息，目前只散落在 package.json、蓝图、后端骨架文档和外部官方文档里，还没有一份仓库级开发环境 bootstrap 文档。**

这会直接带来几个问题：

1. 新环境很容易只装 Node，不装未来真正需要的 Rust / Windows 依赖。
2. 开发者会分不清哪些命令现在就该成功，哪些要等 `src-tauri/` 和 `Cargo.toml` 落地后才有意义。
3. 后端骨架一开工，环境问题会和代码问题混在一起，定位成本很高。

---

## 2. Existing State

当前仓库现实状态如下：

| Item | Current State | Implication |
|------|---------------|-------------|
| frontend toolchain | 根目录已有 `package.json`，当前脚本是 `dev` / `build` / `start` / `lint` | 现在已经可以作为纯 Next.js 前端开发环境运行 |
| Rust workspace | 尚不存在根 `Cargo.toml` | 现在还不能跑 `cargo check --workspace` |
| Tauri host | 尚不存在 `src-tauri/` | 现在还不能跑 `cargo tauri dev` |
| docs | 已有后端骨架、startup、testing、安全等文档 | 缺的是“机器如何准备好开始这些步骤”的统一入口 |

结论：

1. 当前环境 bootstrap 的主要缺口不是前端，而是**为后端骨架开工做准备**。
2. 当前真正缺的是**独立的本地开发环境 bootstrap 总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 在 Windows 上开始本项目开发前，机器至少要装哪些依赖。
2. 现在作为纯前端原型阶段，应先跑哪些命令验证环境。
3. 当后端骨架 Phase A 开始后，应新增哪些命令验证 Rust/Tauri 环境。
4. 哪些移动端或打包依赖当前不需要，不要提前扩展。

这份文档不负责：

1. 替代真正的代码 scaffolding 文档。
2. 给出所有平台的完整安装截图教程。
3. 定义发布签名、CI 和正式分发流程。

---

## 4. Bootstrap Principles

1. prepare for the next slice, not the final universe：先满足当前仓库和即将开始的 Phase A，不提前装一整套无关工具。
2. verify each layer separately：先验证 Node/前端，再验证 Rust，再验证 Tauri host，不把所有失败混在一起。
3. repo commands reflect repo reality：不存在的 `Cargo.toml` 和 `src-tauri/` 不能被写成“现在就应该通过”的命令。
4. desktop-first：当前只准备桌面开发环境，不提前把 Android/iOS 依赖引进来。
5. windows is first-class：当前用户环境是 Windows，因此优先明确 Windows 的 Build Tools 和 WebView2 前提。

---

## 5. Toolchain Baseline

当前仓库建议的本地环境基线如下：

| Layer | Baseline | Why |
|------|----------|-----|
| Node.js | LTS | 当前前端原型依赖 Next.js 14 和 npm 脚本 |
| npm | 随 Node LTS 提供即可 | 当前仓库已经使用 npm scripts |
| Rust | stable toolchain via rustup | Tauri 2 和未来 workspace 依赖 Rust stable |
| Windows C++ Build Tools | Desktop development with C++ | Tauri on Windows 的本地构建前提 |
| WebView2 Runtime | Windows 桌面 WebView 宿主 | Tauri Windows 渲染前提 |
| Git | 已安装即可 | 用于拉取、提交和后续依赖检出 |

说明：

1. 当前没有证据表明仓库需要 pnpm、yarn 或 bun，默认继续使用 npm。
2. 当前没有证据表明仓库必须依赖 nightly Rust，默认继续使用 stable。

---

## 6. Windows Prerequisites

### 6.1 Node.js

安装：

1. 安装 Node.js LTS。
2. 安装完成后重开终端。

最小验证：

```powershell
node -v
npm -v
```

### 6.2 Rust

安装：

1. 使用 rustup 安装 Rust stable。
2. 在 Windows 上默认使用 MSVC toolchain。
3. 安装后重开终端。

最小验证：

```powershell
rustc -V
cargo -V
rustup show
```

### 6.3 Microsoft C++ Build Tools

安装要求：

1. 安装 Microsoft C++ Build Tools。
2. 勾选 `Desktop development with C++`。

这不是可选项；在 Windows 上做 Tauri 本地构建必须依赖它。

### 6.4 WebView2 Runtime

规则：

1. Windows 10 1803 及以上版本通常已内置或可直接使用 WebView2。
2. 若环境缺失，则安装 Evergreen Runtime / Bootstrapper。

说明：

1. 当前仓库还没进入 Tauri 运行阶段，因此这一步不会立即通过仓库命令显现。
2. 但如果现在不装，后面进入 `src-tauri` 阶段时会把平台依赖错误误判成代码错误。

### 6.5 Optional: VBSCRIPT for MSI

仅当后续需要在 Windows 上构建 MSI 安装包时才相关。

当前阶段：

1. 不作为第一版后端骨架的硬前提。
2. 可以在真正开始打 MSI 时再检查。

---

## 7. Current-repo Bootstrap Flow

### 7.1 Phase 0: Frontend-only Reality

在后端骨架尚未落地之前，当前仓库只应验证前端环境：

```powershell
npm install
npm run dev
npm run build
npm run lint
```

这些命令的语义：

1. `npm install`：拉下当前前端依赖。
2. `npm run dev`：验证 Next.js 前端本地开发可启动。
3. `npm run build`：验证当前前端原型可生产构建。
4. `npm run lint`：验证当前前端基础规则可运行。

### 7.2 Phase 1: Workspace Bootstrap Begins

只有当根 `Cargo.toml` 和 `src-tauri/` 真正出现后，下面这些命令才应被加入验证链：

```powershell
cargo metadata --format-version 1
cargo check -p my-epic-launcher-desktop
```

### 7.3 Phase 2: First Rust Slice After Kernel / Composition Exists

当最小 crates 和 smoke tests 落地后，再把这些命令纳入日常验证：

```powershell
cargo check --workspace
cargo test -p launcher-kernel-foundation foundation_contract_smoke
cargo test -p launcher-composition-root bootstrap_wiring_smoke
cargo test -p my-epic-launcher-desktop transport_wiring_smoke
```

规则：

1. 不要在仓库还没有这些文件时就把这些命令写进“当前必过检查”。
2. 但要提前把这些命令写进环境文档，让开发者知道自己机器未来需要支撑什么。

---

## 8. Environment Variables and Paths

当前桌面优先的第一版骨架中：

1. 不要求 `JAVA_HOME`
2. 不要求 `ANDROID_HOME`
3. 不要求 `NDK_HOME`

因为：

1. 当前没有移动端目标。
2. 当前没有 Android/iOS 构建任务。

可能后续需要但当前不应提前复杂化的路径包括：

1. `app-data/`
2. `cache/`
3. `logs/`
4. `sqlite path`

这些路径应由后续 `DesktopBootstrapConfig` 和宿主实现决定，而不是靠开发者手工散落配置。

---

## 9. What Developers Should Not Do Yet

当前阶段禁止把下面这些动作写成“默认第一天环境安装步骤”：

1. 安装 Android Studio
2. 安装 CocoaPods
3. 配置移动端 targets
4. 配置完整 MSI 分发链
5. 引入和仓库现实无关的替代包管理器或 monorepo 工具

原因很简单：

1. 当前仓库还没进入这些阶段。
2. 过早引入这些依赖会模糊真正的桌面骨架启动路径。

---

## 10. First-day Verification Matrix

| Stage | Command | Expected Result |
|------|---------|-----------------|
| current frontend | `node -v` | Node 可用 |
| current frontend | `npm -v` | npm 可用 |
| current frontend | `npm install` | 依赖安装成功 |
| current frontend | `npm run build` | Next.js 构建通过 |
| pre-Rust readiness | `rustc -V` | Rust 编译器可用 |
| pre-Rust readiness | `cargo -V` | Cargo 可用 |
| after Phase A | `cargo metadata --format-version 1` | workspace 元数据可读 |
| after host scaffold | `cargo check -p my-epic-launcher-desktop` | 宿主 crate 可编译 |
| after first smoke tests | `cargo check --workspace` | 整体 Rust workspace 可编译 |

---

## 11. Ownership and Update Rules

| Concern | Owner |
|---------|-------|
| Node / npm baseline | repo root frontend setup |
| Rust / cargo baseline | backend skeleton rollout |
| Build Tools / WebView2 | Windows desktop host prerequisites |
| future secure storage and host permission edge cases | security and host docs |

更新这份文档时，至少要同步确认：

1. 当前 package manager 是否仍然是 npm。
2. 当前 `package.json` scripts 是否发生变化。
3. 当前后端骨架阶段是否已经让更多 `cargo` 命令变成“当前必过”。
4. 当前平台目标是否仍然是 desktop-first。

---

## 12. Exit Criteria

当满足下面几条时，可以认为“开发环境 bootstrap 文档缺口已经补上”：

1. 仓库里已经存在独立的本地开发环境 bootstrap 文档。
2. Windows 下的 Node、Rust、Build Tools 和 WebView2 前提已被统一定义。
3. 当前阶段能跑的命令与未来 Phase A 之后能跑的命令已被清楚区分。
4. 新开发者不必再从 package.json、后端骨架文档和外部 Tauri prerequisites 页面里拼凑第一天环境要求。