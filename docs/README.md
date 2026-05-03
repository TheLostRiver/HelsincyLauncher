# Docs Map

这份文档只负责一件事：

帮你在当前仓库里快速找到“哪份文档该管什么”。

它不是蓝图，不是任务日志，也不是 README 的副本。

---

## 1. 先判断你要解决的问题类型

先问自己：

1. 我需要的是总体边界，还是当前 repo 的落点？
2. 我需要的是专题设计，还是模块级文档？
3. 我需要的是协作/任务协议，还是架构设计？

不同问题应该去不同层，不要把所有入口都重新堆回 `README.md`。

---

## 2. 文档分层

### 2.1 第一入口层

这些文档负责“先把人带对地方”：

1. `README.md`：仓库第一入口、当前状态、关键入口链接
2. `CONTRIBUTING.md`：协作边界、常用命令、最小验证矩阵、高风险文件、文档分工
3. `docs/README.md`：当前这份 docs map，负责解释 `docs/` 里的文档分层

### 2.2 原则层

这些文档负责“系统为什么必须长成这样”：

1. `docs/TauriRewriteArchitectureBlueprint.md`
2. `docs/TauriArchitecturePrinciplesDesign.md`

如果你需要的是：

1. 前后端边界
2. composition root 规则
3. contract first
4. long-running job ownership

优先看这一层。

### 2.3 当前 repo 导航层

这些文档负责“当前仓库现在长什么样、该往哪落”：

1. `docs/TauriCurrentRepoArchitectureOverview.md`
2. `docs/TauriBackendSkeletonImplementationDesign.md`

如果你需要的是：

1. 当前目录职责
2. 关键入口文件索引
3. 当前 host/composition/runtime 骨架链路
4. 骨架落地顺序

优先看这一层。

### 2.4 专题设计层

这些文档负责“某个专项问题应该怎样落地”：

1. `docs/TauriIPCAndStateContractsDesign.md`
2. `docs/TauriRepositoryPortsAndAdapterDesign.md`
3. `docs/TauriCompositionRootWiringDesign.md`
4. `docs/TauriTestingStrategyAndQualityGateDesign.md`
5. `docs/TauriStartupPipelineDesign.md`
6. `docs/TauriSecurityCredentialsAndPermissionsDesign.md`
7. `docs/TauriLoggingAndObservabilityDesign.md`
8. `docs/TauriDevelopmentEnvironmentBootstrapDesign.md`
9. `docs/TauriReleasePackagingAndUpdateDesign.md`
10. `docs/TauriStorageAndDatabaseDesign.md`
11. `docs/TauriErrorHandlingAndProjectionDesign.md`
12. `docs/TauriDownloadRuntimeDesign.md`
13. `docs/TauriEngineVerificationRepairDesign.md`
14. `docs/TauriFabInventoryLoadingDesign.md`
15. `docs/TauriKernelJobsRuntimeDesign.md`
16. `docs/TauriRustTsSchemaDesign.md`
17. `docs/TauriFirstCrateApiDrafts.md`
18. `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`

如果你已经知道自己在改哪个问题域，这一层通常最值钱。

### 2.5 协作与治理层

这些文档负责“如何协作、如何执行复杂任务、文档体系还缺什么”：

1. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
2. `docs/TauriAIContextManagementIntegrationDesign.md`
3. `docs/TauriDocumentationBenchmarkAgainstCodexManager.md`

如果你需要的是：

1. AI 原子任务协议
2. `.artifacts/ai` 与 planning-with-files 的关系
3. 文档治理改进方向

优先看这一层。

### 2.6 模块文档层

这些文档负责“某个业务模块自己的边界和流”：

1. `docs/ModuleDocumentationStandard.md`
2. `docs/modules/*/README_ARCH.md`
3. `docs/modules/*/README_API.md`
4. `docs/modules/*/README_FLOW.md`

如果你在改具体模块，不要只读蓝图，直接进模块文档。

---

## 3. 什么时候看哪份文档

### 3.1 想判断一个设计有没有越界

先看：

1. `docs/TauriArchitecturePrinciplesDesign.md`
2. `docs/TauriRewriteArchitectureBlueprint.md`

### 3.2 想知道当前改动该落在哪层

先看：

1. `CONTRIBUTING.md`
2. `docs/TauriCurrentRepoArchitectureOverview.md`

### 3.3 想继续推进 backend skeleton

先看：

1. `docs/TauriBackendSkeletonImplementationDesign.md`
2. `docs/TauriTestingStrategyAndQualityGateDesign.md`
3. `docs/TauriCompositionRootWiringDesign.md`

### 3.4 想执行复杂 AI 任务或恢复上下文

先看：

1. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
2. `.artifacts/ai/README.md`
3. `.artifacts/ai/active-task.md`
4. `.artifacts/ai/task-plan.md`

### 3.5 想知道当前文档治理缺什么

先看：

1. `docs/TauriDocumentationBenchmarkAgainstCodexManager.md`

---

## 4. 哪些内容不要再堆回 README

`README.md` 只应该承担：

1. 仓库简介
2. 当前状态
3. 第一入口链接

不要继续把这些内容全部堆回 root README：

1. 详细协作规则
2. 大段架构原则
3. 当前 repo 入口文件索引
4. 专题设计细节
5. 活跃任务状态和执行日志

这些内容已经分别属于：

1. `CONTRIBUTING.md`
2. `docs/TauriRewriteArchitectureBlueprint.md`
3. `docs/TauriCurrentRepoArchitectureOverview.md`
4. 各专题设计文档
5. `.artifacts/ai/`

---

## 5. 更新路由规则

如果你改的是：

1. 当前仓库状态摘要或第一入口链接：优先更新 `README.md`
2. 协作命令、验证矩阵、热点、文档分工：优先更新 `CONTRIBUTING.md`
3. 当前 repo 目录职责、入口文件索引、host/composition 链路：优先更新 `docs/TauriCurrentRepoArchitectureOverview.md`
4. 深度原则：优先更新 `docs/TauriArchitecturePrinciplesDesign.md` 或 blueprint
5. 专题问题：优先更新对应专题设计文档
6. 模块边界：优先更新 `docs/modules/*`
7. 活跃任务与恢复点：优先更新 `.artifacts/ai/`

如果一个改动需要同时更新很多层，通常说明：

1. 你在做跨层变更
2. 或者当前任务该先拆分

---

## 6. 一句话使用法

可以把整个 docs 体系记成一句话：

`README.md` 管第一入口，`CONTRIBUTING.md` 管协作，`docs/README.md` 管 docs 导航，blueprint/原则文档管边界，current-repo 文档管今天这棵树，专题文档管专项问题，模块文档管模块自己。