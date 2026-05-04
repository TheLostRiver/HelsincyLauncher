---
name: hsy-plan-doc-review
description: "为文档驱动的 review 或审计任务生成原子计划。触发词：hsy-plan-doc-review, review, 审计, 规范检查, workflow review, doc review."
argument-hint: "输入要审查的模块、工作流或文档范围"
agent: agent
---

为指定范围生成一次严格文档驱动的 review 计划，只规划这一次 review 切片。

执行要求：

1. 先读取 [active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md)、[progress](../../.artifacts/ai/progress.md)、[findings](../../.artifacts/ai/findings.md)。
2. 识别 review 目标对应的控制文档；最少包含 [blueprint](../../docs/TauriRewriteArchitectureBlueprint.md)、[principles](../../docs/TauriArchitecturePrinciplesDesign.md)、[AI protocol](../../docs/TauriAIDevelopmentTransactionProtocolDesign.md) 和 [testing gates](../../docs/TauriTestingStrategyAndQualityGateDesign.md)，再补读目标范围的专题文档。
3. 把 review 工作压成一个原子切片，写清目标范围、非目标范围、风险假设、最便宜验证动作和需要读取的关键文件。
4. 更新 [active task](../../.artifacts/ai/active-task.md) 与 [task plan](../../.artifacts/ai/task-plan.md)，必要时在 [progress](../../.artifacts/ai/progress.md) 记录 review 启动原因。
5. 默认只产出 review 计划与执行入口；除非用户明确要求，先不要直接开始大范围 review。