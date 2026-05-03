# Active Atomic Task

## Identity

- task id: AT-2026-05-03-015
- title: Persist kernel foundation lockfile
- status: committed

## Goal

- exact local outcome: Persist the `Cargo.lock` change introduced by the new `launcher-kernel-foundation` workspace member so the repo returns to a clean post-B1 baseline before B2 starts.

## Scope

- in scope:
  - stage and commit the generated `Cargo.lock` delta
  - update `.artifacts/ai` records for this cleanup slice
- out of scope:
  - modifying the committed B1 code files
  - starting B2 foundation modules or tests
  - changing any other workspace members

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
3. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
4. .github/copilot-instructions.md
5. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the generated `Cargo.lock` entry for `launcher-kernel-foundation` is persisted immediately after B1, then the repo will return to a clean baseline without widening the B1 code slice.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` and then inspect `git -C q:\DEV\MyEpicLauncher status --short`.

## Validation Gate

1. `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` still passed against the just-committed B1 baseline.
- `git status --short` showed only the expected lockfile and AT-015 record updates before commit.

## 需要更新的文档和日志

1. Cargo.lock
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Persist kernel foundation lockfile
2. push command plan: git push

## 停止条件

1. `Cargo.lock` changes beyond the expected new workspace package entry
2. `cargo check` stops matching the just-committed B1 baseline
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the lockfile cleanup slice, then open B2 for the first real foundation modules and named smoke test.