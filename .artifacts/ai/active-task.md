# Active Atomic Task

## Identity

- task id: AT-2026-05-03-012
- title: Ignore Rust target artifacts
- status: committed

## Goal

- exact local outcome: Add the missing root `.gitignore` rule for Rust `target/` outputs so package-scoped backend validation no longer leaves the repo dirty after `cargo check`.

## Scope

- in scope:
  - update the root `.gitignore` to ignore `target/`
  - update `.artifacts/ai` records for the cleanup slice
- out of scope:
  - changing the committed AT-011 host shell files
  - adding new backend crates or host wiring
  - deleting any existing local build outputs manually

## Allowed Files

1. .gitignore
2. .artifacts/ai/**

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

- falsifiable local hypothesis: If the repo root ignores `target/`, then rerunning `cargo check -p my-epic-launcher-desktop` will no longer leave untracked Rust incremental artifacts in the worktree.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` and then inspect `git -C q:\DEV\MyEpicLauncher status --short`.

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` still passed after adding `target/` to the root `.gitignore`.
- `git status --short` showed only the intended AT-012 record updates and `.gitignore`; the Rust incremental build outputs stopped appearing in the worktree.

## 需要更新的文档和日志

1. .gitignore
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Ignore Rust target artifacts
2. push command plan: git push

## 停止条件

1. `target/` is already tracked, which would require a different cleanup path
2. `cargo check` still leaves visible worktree noise after `.gitignore` is updated
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: choose the next backend bootstrap slice now that package-scoped cargo validation leaves the repo clean by default.