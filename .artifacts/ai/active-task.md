# Active Atomic Task

## Identity

- task id: AT-2026-05-03-027
- title: Persist fab provider adapter lockfile
- status: committed

## Goal

- exact local outcome: Persist the `Cargo.lock` delta introduced by the validated `launcher-adapter-provider-fab` C4 slice so the repo returns to a clean post-C4 baseline before the next slice starts.

## Scope

- in scope:
  - stage and commit the generated `Cargo.lock` delta
  - update `.artifacts/ai` records for this cleanup slice
- out of scope:
  - modifying the committed C4 adapter files
  - starting the next phase or composition slice
  - changing workspace manifests beyond the existing lockfile output

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

- falsifiable local hypothesis: If the small `Cargo.lock` delta produced by the validated `launcher-adapter-provider-fab` C4 slice is persisted immediately, then the repo will return to a clean baseline without widening the C4 code task.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` and then inspect `git -C q:\DEV\MyEpicLauncher status --short`.

## Validation Gate

1. `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed against the just-committed C4 baseline.
- `git diff --check` passed, and `git status --short` showed only the expected `Cargo.lock` plus AT-027 record updates before commit.

## 需要更新的文档和日志

1. Cargo.lock
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Persist fab provider adapter lockfile
2. push command plan: git push

## 停止条件

1. `Cargo.lock` contains unrelated package changes that were not introduced by the validated C4 slice
2. the C4 package check stops matching the just-committed provider adapter baseline
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage only `Cargo.lock` plus the AT-027 record files, commit the cleanup slice, then choose the next documented backend task from the clean post-C4 baseline.