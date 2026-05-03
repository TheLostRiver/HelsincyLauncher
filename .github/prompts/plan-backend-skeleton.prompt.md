---
name: plan-backend-skeleton
description: "为 MyEpicLauncher 后端骨架生成下一条可执行的原子任务计划。触发词：后端骨架, backend skeleton, src-tauri, crates, cargo workspace."
argument-hint: "输入后端骨架目标、阶段或当前阻塞点"
agent: agent
---

为 MyEpicLauncher 后端骨架规划下一条可执行的原子任务，不要直接扩展成多条并行任务。

执行要求：

1. 先读取 [active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md)、[progress](../../.artifacts/ai/progress.md)、[findings](../../.artifacts/ai/findings.md)。
2. 再读取控制文档：[blueprint](../../docs/TauriRewriteArchitectureBlueprint.md)、[principles](../../docs/TauriArchitecturePrinciplesDesign.md)、[AI protocol](../../docs/TauriAIDevelopmentTransactionProtocolDesign.md)、[testing gates](../../docs/TauriTestingStrategyAndQualityGateDesign.md)、[backend skeleton](../../docs/TauriBackendSkeletonImplementationDesign.md)、[crate layout](../../docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md)、[composition root](../../docs/TauriCompositionRootWiringDesign.md)、[environment bootstrap](../../docs/TauriDevelopmentEnvironmentBootstrapDesign.md)。
3. 以“保留根目录前端原型、后端真相归 Rust/Tauri、一次只开一个原子任务”为硬约束。
4. 选择当前最小可落地的后端骨架切片，明确 `allowed_files`、`required_context`、`hypothesis`、`cheap_check` 和文档要求的验证门槛。
5. 更新 [active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md) 和 [progress](../../.artifacts/ai/progress.md)。
6. 如果缺文档、缺前置条件或存在规范冲突，就把它记成阻塞，不要即兴发挥架构。