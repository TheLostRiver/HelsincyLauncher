# MyEpicLauncher

MyEpicLauncher 是一个面向桌面端的启动器重写仓库。

当前仓库状态不是“已经有完整 Tauri/Rust 应用”，而是：

1. 根目录已有可运行的 Next.js 前端原型。
2. `docs/` 下已经建立了较完整的 Tauri 重写架构文档基线。
3. 真实的 Rust workspace、`src-tauri/` 宿主和 backend crates 还没有开始落盘。

换句话说，**这个仓库现在处于 docs-first、frontend-prototype-first 的阶段**。

---

## Current Status

当前已经明确的事实：

1. 前端原型技术栈是 Next.js 14 + React 18 + TypeScript + Tailwind。
2. 目标架构是 Tauri 2 + Rust stable + typed IPC + backend-owned business truth。
3. 后端骨架的落地顺序、测试门槛、安全边界、环境前提、发布/更新边界都已经写成独立文档。
4. 真实后端代码还未开始，因此目前能直接运行的是前端原型，而不是桌面宿主。

---

## Quick Start

当前阶段只需要前端运行环境：

```powershell
npm install
npm run dev
```

常用检查：

```powershell
npm run build
npm run lint
```

说明：

1. 当前仓库还没有根 `Cargo.toml`。
2. 当前仓库还没有 `src-tauri/`。
3. 因此现在不要期待 `cargo check --workspace` 或 `cargo tauri dev` 已经可用。

如果要为后续 Tauri/Rust 开工准备本机环境，先看 [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)。

---

## Read First

如果你刚进入这个仓库，建议按下面顺序阅读：

1. [docs/TauriRewriteArchitectureBlueprint.md](docs/TauriRewriteArchitectureBlueprint.md)
2. [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md)
3. [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)
4. [docs/TauriTestingStrategyAndQualityGateDesign.md](docs/TauriTestingStrategyAndQualityGateDesign.md)
5. [docs/TauriSecurityCredentialsAndPermissionsDesign.md](docs/TauriSecurityCredentialsAndPermissionsDesign.md)
6. [docs/TauriReleasePackagingAndUpdateDesign.md](docs/TauriReleasePackagingAndUpdateDesign.md)

这几份文档分别回答：

1. 总体架构边界是什么。
2. 当前仓库里的第一版后端骨架应该怎么落。
3. 本地 Windows 开发环境要准备什么。
4. 测试门槛是什么。
5. 安全、凭据和权限边界是什么。
6. 发布、打包和应用更新边界是什么。

---

## Key Docs

### Architecture

- [docs/TauriRewriteArchitectureBlueprint.md](docs/TauriRewriteArchitectureBlueprint.md)
- [docs/TauriArchitecturePrinciplesDesign.md](docs/TauriArchitecturePrinciplesDesign.md)
- [docs/TauriIPCAndStateContractsDesign.md](docs/TauriIPCAndStateContractsDesign.md)

### Backend Rollout

- [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md)
- [docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md](docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md)
- [docs/TauriCompositionRootWiringDesign.md](docs/TauriCompositionRootWiringDesign.md)

### Quality And Ops

- [docs/TauriTestingStrategyAndQualityGateDesign.md](docs/TauriTestingStrategyAndQualityGateDesign.md)
- [docs/TauriLoggingAndObservabilityDesign.md](docs/TauriLoggingAndObservabilityDesign.md)
- [docs/TauriStartupPipelineDesign.md](docs/TauriStartupPipelineDesign.md)
- [docs/TauriErrorHandlingAndProjectionDesign.md](docs/TauriErrorHandlingAndProjectionDesign.md)
- [docs/TauriSecurityCredentialsAndPermissionsDesign.md](docs/TauriSecurityCredentialsAndPermissionsDesign.md)
- [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)
- [docs/TauriReleasePackagingAndUpdateDesign.md](docs/TauriReleasePackagingAndUpdateDesign.md)

### Module Docs

- [docs/ModuleDocumentationStandard.md](docs/ModuleDocumentationStandard.md)
- [docs/modules/shell/README_ARCH.md](docs/modules/shell/README_ARCH.md)
- [docs/modules/fab-inventory/README_ARCH.md](docs/modules/fab-inventory/README_ARCH.md)
- [docs/modules/downloads/README_ARCH.md](docs/modules/downloads/README_ARCH.md)
- [docs/modules/engines/README_ARCH.md](docs/modules/engines/README_ARCH.md)
- [docs/modules/projects/README_ARCH.md](docs/modules/projects/README_ARCH.md)
- [docs/modules/settings/README_ARCH.md](docs/modules/settings/README_ARCH.md)
- [docs/modules/account-auth/README_ARCH.md](docs/modules/account-auth/README_ARCH.md)

---

## Repository Layout

当前仓库关键目录：

```text
MyEpicLauncher/
├─ app/              # Next.js app router
├─ components/       # 前端原型组件
├─ docs/             # Tauri 重写架构与模块文档
├─ package.json      # 当前前端脚本入口
├─ task_plan.md      # 任务规划记录
├─ findings.md       # 研究发现与决策记录
└─ progress.md       # 过程与验证记录
```

后续进入后端骨架阶段后，仓库预计会新增：

```text
Cargo.toml
src-tauri/
crates/
```

但这些目录目前还不存在。

---

## Near-term Roadmap

短期最明确的下一步不是继续抽象架构，而是按 [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md) 的 Phase A 开始落第一版后端骨架：

1. 根 `Cargo.toml`
2. `src-tauri/`
3. 最薄宿主 bootstrap surface
4. 第一批 kernel / module / adapter / composition-root crates

在这之前，README 的职责是让任何人一眼分清：

1. 现在仓库能运行什么
2. 现在仓库还不能运行什么
3. 真正该去哪里读下一步实现约束

---

## Notes

1. 当前仓库文档较多，但这不是为了堆文档，而是为了先把重写边界钉死，再开始写后端代码。
2. 如果你准备开始写 Rust/Tauri 代码，先不要跳过 [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md) 和 [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)。
3. 如果你准备改前端原型，先确认自己没有把后端职责重新塞回 UI。