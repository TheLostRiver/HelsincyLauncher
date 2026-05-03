# Active Atomic Task

## Identity

- task id: AT-2026-05-03-017
- title: Persist foundation dependency lockfile
- status: committed

## Goal

- exact local outcome: Persist the `Cargo.lock` expansion introduced by the first dependency-bearing `launcher-kernel-foundation` B2 slice so the repo returns to a clean post-B2 baseline before `kernel-jobs` B3 starts.

## Scope

- in scope:
  - stage and commit the generated `Cargo.lock` delta
  - update `.artifacts/ai` records for this cleanup slice
- out of scope:
  - modifying the committed B2 foundation code files
  - starting `kernel-jobs` or any other crate work
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
3. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
4. .github/copilot-instructions.md
5. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the dependency-resolved `Cargo.lock` produced by the validated foundation B2 slice is persisted immediately, then the repo will return to a clean baseline without widening the foundation contract task itself.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` and then inspect `git -C q:\DEV\MyEpicLauncher status --short`.

## Validation Gate

1. `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` still passed against the just-committed B2 baseline.
- `git status --short` showed only the expected lockfile and AT-017 record updates before commit.

## 需要更新的文档和日志

1. Cargo.lock
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Persist foundation dependency lockfile
2. push command plan: git push

## 停止条件

1. `Cargo.lock` contains unrelated package changes that were not introduced by the validated B2 slice
2. the foundation smoke test stops matching the just-committed B2 baseline
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the lockfile cleanup slice, then open `kernel-jobs` B3 from the clean post-B2 baseline.