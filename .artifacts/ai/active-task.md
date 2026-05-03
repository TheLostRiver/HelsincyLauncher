# Active Atomic Task

## Identity

- task id: AT-2026-05-03-032
- title: Persist desktop host lockfile
- status: committed

## Goal

- exact local outcome: Persist the small `Cargo.lock` delta introduced by the validated E1 desktop host command slice while leaving unrelated user frontend worktree changes untouched.

## Scope

- in scope:
  - stage and commit the generated `Cargo.lock` delta for the desktop host crate
  - update `.artifacts/ai` records for this cleanup slice
- out of scope:
  - touching the committed E1 host command files
  - touching unrelated frontend worktree changes
  - starting E2 registration or host smoke-test work

## Allowed Files

1. Cargo.lock
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. docs/TauriAIDevelopmentTransactionProtocolDesign.md
2. docs/TauriTestingStrategyAndQualityGateDesign.md
3. docs/TauriBackendSkeletonImplementationDesign.md
4. .github/copilot-instructions.md
5. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the small `Cargo.lock` delta caused by adding E1 desktop-host dependencies is persisted immediately after the validated host command slice, then backend work can continue into E2 without dragging this lockfile noise forward, even while unrelated user frontend edits remain unstaged.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` and inspect `git status --short -- .artifacts/ai Cargo.lock src-tauri`.

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai Cargo.lock src-tauri`

## Validation Result

- `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed against the just-committed E1 host command baseline.
- `git diff --check` surfaced only the existing CRLF warnings from untouched files, and `git status --short -- .artifacts/ai Cargo.lock src-tauri` confirmed that this cleanup slice contains only `Cargo.lock` plus the AT-032 record updates.

## 需要更新的文档和日志

1. Cargo.lock
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Persist desktop host lockfile
2. push command plan: git push

## 停止条件

1. `Cargo.lock` contains changes beyond the expected desktop host dependency list delta
2. the validated E1 package check stops matching the just-committed host command baseline
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage only `Cargo.lock` plus the AT-032 record files, commit the cleanup slice, then continue to E2 while leaving the unrelated frontend edits untouched.