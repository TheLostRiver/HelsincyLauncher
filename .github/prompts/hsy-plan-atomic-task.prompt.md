---
name: hsy-plan-atomic-task
description: "为当前工作创建或刷新严格文档驱动的原子任务记录。触发词：hsy-plan-atomic-task, 原子任务, active-task, AT, 拆任务, strict-doc."
argument-hint: "输入任务目标或待拆分的工作"
agent: agent
---

为 MyEpicLauncher 执行一次严格文档驱动的原子任务规划，只处理当前这一条任务。

执行要求：

1. 先读取 [active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md)、[progress](../../.artifacts/ai/progress.md)、[findings](../../.artifacts/ai/findings.md)。
2. 再读取控制文档：[blueprint](../../docs/TauriRewriteArchitectureBlueprint.md)、[principles](../../docs/TauriArchitecturePrinciplesDesign.md)、[AI protocol](../../docs/TauriAIDevelopmentTransactionProtocolDesign.md)、[testing gates](../../docs/TauriTestingStrategyAndQualityGateDesign.md)、[context integration](../../docs/TauriAIContextManagementIntegrationDesign.md)、[strict-doc skill](../skills/strict-doc-driven-development/SKILL.md)、[planning-with-files skill](../skills/planning-with-files/SKILL.md)。
3. 如果已经存在 `in_progress` 或 `validating` 的活跃原子任务，先判断是继续、提交还是阻塞，不要并行开启第二个活跃任务。
4. 把当前请求压缩成一个可证伪的局部假设、一个最便宜验证动作、清晰的 `allowed_files` 和 `next_step`。
5. 更新 [active task](../../.artifacts/ai/active-task.md) 与 [task plan](../../.artifacts/ai/task-plan.md)，必要时补充 [progress](../../.artifacts/ai/progress.md)。
6. 默认停在“可立即执行”的原子任务定义上，除非用户明确要求你继续实现。