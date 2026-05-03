# 活跃原子任务模板

在实现开始前，用它定义当前原子任务。
建议文件路径：.artifacts/ai/active-task.md。

## Identity

- task id:
- title:
- status: ready | in_progress | validating | committed | blocked

## Goal

- exact local outcome:

## Scope

- in scope:
- out of scope:

## Allowed Files

1. path/to/file-or-directory
2. path/to/file-or-directory

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. docs/...
2. docs/...

## Hypothesis

- falsifiable local hypothesis:

## Cheap Check

- narrowest check that can disconfirm the hypothesis:

## Validation Gate

1. focused compile, test, or validation command

## 需要更新的文档和日志

1. docs to refresh after success
2. task records to update after success

## 验证后的 Git 动作

1. commit message plan
2. push command plan

## 停止条件

1. missing or conflicting docs
2. out-of-scope change request
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: