# Active Atomic Task

## Identity

- task id: AT-2026-05-03-013
- title: Clarify active workflow records
- status: committed

## Goal

- exact local outcome: Add an explicit `.artifacts/ai/README.md` that distinguishes active workflow records from the archived `legacy-root-planning` files so future sessions do not confuse archive history with the live source of truth.

## Scope

- in scope:
  - add `.artifacts/ai/README.md`
  - update `.artifacts/ai` task records for this clarification slice
- out of scope:
  - modifying the archived files under `.artifacts/ai/legacy-root-planning/`
  - changing workflow hooks or planning-with-files behavior
  - starting the next backend crate slice before this clarification lands

## Allowed Files

1. .artifacts/ai/README.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. .artifacts/ai/legacy-root-planning/README.md

## 控制性文档

1. .github/copilot-instructions.md
2. .github/skills/planning-with-files/SKILL.md
3. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the active record directory carries its own README that explicitly names the live source-of-truth files and the archive boundary, then users and later sessions will have a local, unambiguous explanation without needing to inspect the skill file or archive README first.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Confirm `.artifacts/ai/README.md` exists with explicit active-vs-archive guidance and run `git -C q:\DEV\MyEpicLauncher diff --check`.

## Validation Gate

1. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `git -C q:\DEV\MyEpicLauncher diff --check` passed after adding `.artifacts/ai/README.md` and the accompanying record updates.
- The new README documents the existing source-of-truth boundary only; it does not redefine workflow behavior.

## 需要更新的文档和日志

1. .artifacts/ai/README.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Clarify active workflow records
2. push command plan: git push

## 停止条件

1. the clarification starts redefining workflow behavior instead of documenting the existing source-of-truth boundary
2. the new README conflicts with the current planning-with-files or strict-doc instructions
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the clarification slice, then open the next backend slice for `kernel-foundation`.