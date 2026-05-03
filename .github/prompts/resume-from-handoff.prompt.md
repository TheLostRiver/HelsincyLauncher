---
name: resume-from-handoff
description: "从 .artifacts/ai/handoff.md 和相关记录恢复当前工作。触发词：继续上次, resume, handoff, 恢复上下文."
argument-hint: "可选：输入恢复后要优先处理的方向"
agent: agent
---

从仓库的 handoff 和任务记录中恢复当前工作，并只恢复一个活跃任务路径。

执行要求：

1. 先读取 [handoff](../../.artifacts/ai/handoff.md)、[active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md)、[progress](../../.artifacts/ai/progress.md)、[findings](../../.artifacts/ai/findings.md)。
2. 如果存在 [session-catchup](../skills/planning-with-files/scripts/session-catchup.py)，在恢复前用它检查未同步上下文。
3. 识别当前唯一应该继续的原子任务；如果 `active-task.md` 已经是终态，就回退到 [task plan](../../.artifacts/ai/task-plan.md) 选择下一条切片，不要平行恢复多个任务。
4. 先把过期的状态、验证结果或下一步入口补齐到 [active task](../../.artifacts/ai/active-task.md)、[task plan](../../.artifacts/ai/task-plan.md) 或 [progress](../../.artifacts/ai/progress.md)，再继续执行。
5. 如果用户只是要求恢复上下文，停在简洁的恢复摘要和下一步入口；如果用户明确要求继续实现，再执行当前切片的第一步。