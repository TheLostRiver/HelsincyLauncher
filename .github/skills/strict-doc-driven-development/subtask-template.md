# 严格文档驱动子任务模板

在拆分子任务、subagent 工作，或后续实现切片时使用这个模板。

## Goal

写清精确的局部结果。

## Scope

- In scope:
- Out of scope:

## Allowed Files

- path/to/file-or-directory

## 优先读取的本地任务记录

- .artifacts/ai/active-task.md
- .artifacts/ai/task-plan.md
- .artifacts/ai/progress.md
- .artifacts/ai/findings.md

## 控制性文档

1. docs/...
2. docs/...

## 任务约束

1. 写清不能打破的边界规则。
2. 写清谁拥有业务真相。
3. 写清禁止的捷径。

## Cheap Check

写清最窄、最能证伪当前假设的验证动作。

## Validation Gates

1. command or focused check
2. command or focused check

## 需要更新的任务记录和文档

1. .artifacts/ai/task-plan.md / .artifacts/ai/progress.md / .artifacts/ai/findings.md / .artifacts/ai/handoff.md updates as needed
2. repository or module docs to refresh

## 验证后的 Git 动作

1. commit message plan
2. push immediately after validation, or record pending push details if push fails

## Done When

1. 具体且可观察的结果
2. 具体的验证结果