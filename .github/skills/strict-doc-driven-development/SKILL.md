---
name: strict-doc-driven-development
description: "用于实现、规划、评审或拆分 MyEpicLauncher 任务，且这些任务必须严格遵循仓库开发规范文档。触发词：严格按照开发规范文档, 开发规范, 架构文档, 设计文档, blueprint, 模块文档, 后端骨架."
user-invocable: true
allowed-tools: "Read Write Edit Bash Glob Grep"
---

# 严格文档驱动开发

这个仓库不接受泛化的默认实现行为。
仓库文档是有约束力的工程规范，不是可选参考。

## 核心规则

1. 开始编辑前，先识别本次改动由哪些文档约束。
2. 如果任务涉及业务模块，改代码前先读对应模块文档。
3. 如果文档缺失、过期或冲突，不要即兴发挥架构；先暴露缺口，或有意识地更新文档。
4. 拆分或委派子任务时，必须写清精确文档路径、作用域、允许文件和验证门槛。
5. 必须使用控制性文档要求的最窄验证，不要用更松的检查替代。
6. 文档有预算：任务日志、验证输出、提交号和 handoff 写入 `.artifacts/ai/`；只有长期边界、公开契约、数据模型、错误语义、wiring 规则或验证规则才写入 `docs/`。

## 原子任务循环

1. 先读取本地任务记录：.artifacts/ai/active-task.md、.artifacts/ai/task-plan.md、.artifacts/ai/progress.md、.artifacts/ai/findings.md。
2. 只有在读完任务记录和相关架构或模块文档后，才能开始编码。
3. 任一时刻只允许一个活跃原子任务处于进行中，直到它进入验证、提交或阻塞状态。
4. 每个原子任务成功后，先跑文档要求的最窄编译、测试或验证门槛。
5. 验证通过后，更新 .artifacts/ai/ 下的任务记录和相关文档，创建 git commit，并立刻尝试 push。
6. 如果 push 失败，不要丢弃 commit；把 pending push 详情持久化到 .artifacts/ai/ 任务记录里，后续再推。
7. 如果同一阻塞修了 5 次仍失败，就把 bug 详情、尝试历史和当前状态持久化，然后停止，等待用户决断。

## 基础必读文档

大多数实现任务先读这些文档：

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md

## 按任务补充的文档映射

按任务形态补读相应文档：

- Security, credentials, permissions: docs/TauriSecurityCredentialsAndPermissionsDesign.md
- Startup, restore, warmup: docs/TauriStartupPipelineDesign.md
- Error model and user-facing projection: docs/TauriErrorHandlingAndProjectionDesign.md
- Release, packaging, updater: docs/TauriReleasePackagingAndUpdateDesign.md
- Backend skeleton, crate layout, transport wiring: docs/TauriBackendSkeletonImplementationDesign.md
- Environment bootstrap and prerequisites: docs/TauriDevelopmentEnvironmentBootstrapDesign.md
- Module work: docs/modules/<module-name>/README_ARCH.md, README_API.md, README_FLOW.md

## 实施协议

1. 做决定前先说清本次改动由哪些文档控制。
2. 把任务压缩成一条可证伪的局部假设和一个最便宜的验证动作。
3. 只做满足文档边界的最小改动。
4. 用文档明确要求的门槛验证受影响切片。
5. 如果代码和文档冲突，不要偷偷选一边；要显式指出冲突并解决它。
6. 不要把每个 AT 的完成流水、长排除清单和验证细节追加到 `README_IMPL.md`；需要写模块实现文档时，优先写短表或 3-5 行长期状态。

## 子任务简报契约

拆分任务时，不能只给一个模糊目标。
每个子任务都必须携带仓库级约束。

子任务必须包含这些字段：

1. goal
2. scope and non-goals
3. allowed files
4. controlling docs
5. task-specific constraints
6. validation gates
7. done-when criteria

使用 bundled 的 subtask-template.md。
如果一个子任务连控制性文档和最便宜验证都说不清，它就还不能执行。

## 委派检查清单

委派前逐条确认：

1. 子任务写明了精确的控制性文档。
2. 子任务限制了文件作用域，避免执行者到处游走。
3. 子任务写清了编辑后立即要跑的最窄验证。
4. 子任务明确列出禁止的架构动作。
5. 当文档把真相归属到 backend、host 或 composition root 时，子任务不会默认走 frontend-only 捷径。
6. 子任务写清了 commit 前必须更新哪些任务记录和文档。
7. 子任务写明验证成功后是否必须立刻 git commit 和 push。

## 工作流模板

使用这些模板保持工作流结构化：

1. active-atomic-task-template.md：在编码前定义当前原子任务，落到 .artifacts/ai/active-task.md，写清作用域、控制性文档、cheap check 和停止条件。
2. docs-update-log-template.md：记录改了哪些文档、为什么改、它们对应哪段验证或代码切片。
3. pending-push-template.md：记录已经准备好但还没 push 的提交，包括失败原因和下次执行命令。
4. blocked-bug-template.md：记录同一阻塞修了 5 次仍失败的情况，包括尝试历史、最新错误、受影响文件和停止点。

不要把 active-task 状态、文档更新、push 失败或 5 次修复失败的阻塞只留在临时对话上下文里。
要用这些模板把它们落到 .artifacts/ai/ 任务记录里。

## 语言模式

1. 默认语言模式是中文，配置文件是 .artifacts/ai/language-mode.txt。
2. 支持值目前为 zh-CN 和 en。
3. 启动提醒会优先读取环境变量 MYEPIC_WORKFLOW_LANG；如果没有，再回退到 .artifacts/ai/language-mode.txt。
4. 为了兼容 Windows PowerShell 5.1，脚本文件尽量保持 ASCII，把中文文案放在外部文本资产里。

## 禁止行为

- 不要因为当前原型偏 UI，就把业务真相移到 frontend。
- 不要发明绕过 facade、port、contract 或 projection 边界的跨模块耦合。
- 没有明确文档依据时，不要擅自放宽权限、secret storage 或 startup blocking 规则。
- 不要因为更大的 build 绿了，就跳过本该跑的窄验证。
- 不要把仓库文档当成可选建议。
- 不要再把根目录 legacy task_plan.md / progress.md / findings.md 当作新任务的事实来源。
