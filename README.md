# MyEpicLauncher

MyEpicLauncher 是一个面向桌面端的启动器重写仓库。

当前仓库状态不是“只有文档、没有后端”，而是：

1. 根目录已有可运行的 Next.js 前端原型。
2. `docs/` 下已经建立了较完整的 Tauri 重写架构文档基线。
3. 根目录已经落地 Rust workspace、`src-tauri/` 宿主和第一批 backend crates 骨架。
4. 当前重点从“能否起骨架”转到“继续推进后端集成切片，并补平协作者文档入口”。

换句话说，**这个仓库现在处于 docs-driven、frontend-prototype-plus-backend-skeleton 的阶段**。

---

## Current Status

当前已经明确的事实：

1. 前端原型技术栈是 Next.js 14 + React 18 + TypeScript + Tailwind。
2. 目标架构是 Tauri 2 + Rust stable + typed IPC + backend-owned business truth。
3. 后端骨架的落地顺序、测试门槛、安全边界、环境前提、发布/更新边界都已经写成独立文档并完成第一批落盘。
4. 当前仓库已经具备 `Cargo.toml`、`Cargo.lock`、`src-tauri/` 和 `crates/`，且核心 smoke baseline 已验证通过。
5. downloads 后端正在按模块实现文档推进 concrete segment execution：filesystem writer、length verifier、static fetcher、composition-root 静态执行器接线证明、`kernel-jobs` terminal disposition projection、downloads driver completion-first 终态判定，以及失败元数据持久化已完成；retry/backoff 与 failure-class 持久化边界正在收敛，下一步才是 Rust policy field 切片。

---

## Quick Start

前端原型运行：

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

1. 在 Windows PowerShell 5.1 下如果遇到 `npm.ps1` 执行策略限制，使用 `npm.cmd run build` 或 `npm.cmd run dev`。
2. 当前仓库前端仍然是原型层，后端真相和长任务逻辑不应回流到 UI。

如果要为后续 Tauri/Rust 开工准备本机环境，先看 [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)。

后端 baseline 验证：

```powershell
cargo check --workspace
cargo test -p launcher-kernel-foundation foundation_contract_smoke
cargo test -p launcher-composition-root bootstrap_wiring_smoke
cargo test -p my-epic-launcher-desktop transport_wiring_smoke
```

---

## Read First

如果你刚进入这个仓库，建议按下面顺序阅读：

1. [docs/TauriRewriteArchitectureBlueprint.md](docs/TauriRewriteArchitectureBlueprint.md)
2. [docs/TauriArchitecturePrinciplesDesign.md](docs/TauriArchitecturePrinciplesDesign.md)
3. [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md)
4. [docs/TauriTestingStrategyAndQualityGateDesign.md](docs/TauriTestingStrategyAndQualityGateDesign.md)
5. [docs/TauriAIDevelopmentTransactionProtocolDesign.md](docs/TauriAIDevelopmentTransactionProtocolDesign.md)
6. [docs/TauriDevelopmentEnvironmentBootstrapDesign.md](docs/TauriDevelopmentEnvironmentBootstrapDesign.md)
7. [docs/TauriDocumentationBenchmarkAgainstCodexManager.md](docs/TauriDocumentationBenchmarkAgainstCodexManager.md)

这几份文档分别回答：

1. 总体架构边界是什么。
2. 判断设计是否越界的原则是什么。
3. 当前仓库里的后端骨架是如何落地的。
4. 测试门槛是什么。
5. 协作者该如何落点、验证和规避高风险入口。
6. 复杂任务执行和恢复协议是什么。
7. 本地 Windows 开发环境要准备什么。
8. 当前文档体系相对外部同类项目还缺哪些入口层能力。

---

## Key Docs

### Architecture

- [docs/TauriRewriteArchitectureBlueprint.md](docs/TauriRewriteArchitectureBlueprint.md)
- [docs/TauriArchitecturePrinciplesDesign.md](docs/TauriArchitecturePrinciplesDesign.md)
- [docs/TauriCurrentRepoArchitectureOverview.md](docs/TauriCurrentRepoArchitectureOverview.md)
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

### Workflow And Governance

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/README.md](docs/README.md)
- [docs/TauriAIDevelopmentTransactionProtocolDesign.md](docs/TauriAIDevelopmentTransactionProtocolDesign.md)
- [docs/TauriDocumentationBenchmarkAgainstCodexManager.md](docs/TauriDocumentationBenchmarkAgainstCodexManager.md)
- [.artifacts/ai/README.md](.artifacts/ai/README.md)

### Module Docs

- [docs/ModuleDocumentationStandard.md](docs/ModuleDocumentationStandard.md)
- [docs/modules/shell/README_ARCH.md](docs/modules/shell/README_ARCH.md)
- [docs/modules/fab-inventory/README_ARCH.md](docs/modules/fab-inventory/README_ARCH.md)
- [docs/modules/downloads/README_ARCH.md](docs/modules/downloads/README_ARCH.md)
- [docs/modules/downloads/README_IMPL.md](docs/modules/downloads/README_IMPL.md)
- [docs/modules/engines/README_ARCH.md](docs/modules/engines/README_ARCH.md)
- [docs/modules/projects/README_ARCH.md](docs/modules/projects/README_ARCH.md)
- [docs/modules/settings/README_ARCH.md](docs/modules/settings/README_ARCH.md)
- [docs/modules/account-auth/README_ARCH.md](docs/modules/account-auth/README_ARCH.md)

---

## Repository Layout

当前仓库关键目录：

```text
MyEpicLauncher/
├─ .artifacts/ai/    # 本地任务协议与过程记录
├─ app/              # Next.js app router
├─ Cargo.toml        # Rust workspace manifest
├─ Cargo.lock        # Rust workspace lockfile
├─ components/       # 前端原型组件
├─ crates/           # backend kernel / module / adapter / composition crates
├─ docs/             # Tauri 重写架构与模块文档
├─ package.json      # 当前前端脚本入口
├─ README.md         # 仓库第一入口
└─ src-tauri/        # 桌面宿主与 transport wiring
```

---

## Near-term Roadmap

短期最明确的下一步不是继续扩写大蓝图，而是：

1. 在已经通过 smoke gate 的 backend skeleton 基线上继续推进更窄的集成切片。
2. downloads 后端已落 durable retry facts、纯 `DownloadSegmentRetryPolicy` 计算器，并已在 failed checkpoint mutation 中写入 policy-computed `next_retry_after`；下一步应先做到期 retry-ready segment 选择，再考虑 scheduler、`TerminalFailed` 或公开稳定 `DL_*` 执行错误。
3. 把 contributor-facing 的协作入口和 current-repo 导航补平。
4. 保持 README、`.artifacts/ai` 协议和深度设计文档之间的一致性。

在这之前，README 的职责是让任何人一眼分清：

1. 现在仓库能运行什么
2. 现在仓库还不能运行什么
3. 真正该去哪里读下一步实现约束

---

## Notes

1. 当前仓库文档较多，但核心策略不是“文档越多越好”，而是“深度设计文档和扁平入口文档都要各司其职”。
2. 如果你准备开始提交改动，先读 [CONTRIBUTING.md](CONTRIBUTING.md)。
3. 如果你准备开始写 Rust/Tauri 代码，先不要跳过 [docs/TauriBackendSkeletonImplementationDesign.md](docs/TauriBackendSkeletonImplementationDesign.md)、[docs/TauriTestingStrategyAndQualityGateDesign.md](docs/TauriTestingStrategyAndQualityGateDesign.md) 和 [docs/TauriAIDevelopmentTransactionProtocolDesign.md](docs/TauriAIDevelopmentTransactionProtocolDesign.md)。
4. 如果你想理解本仓库文档体系相对同类项目还缺什么入口层能力，先看 [docs/TauriDocumentationBenchmarkAgainstCodexManager.md](docs/TauriDocumentationBenchmarkAgainstCodexManager.md)。
5. 如果你准备改前端原型，先确认自己没有把后端职责重新塞回 UI。
