# Active Atomic Task

## Identity

- task id: AT-2026-05-03-018
- title: Bootstrap kernel jobs crate
- status: committed

## Goal

- exact local outcome: Create the first `launcher-kernel-jobs` crate shell, wire it into the root workspace, and expose a minimal zero-business job protocol surface (`JobState`, `JobSnapshot`, `JobRuntime`) that passes the documented B3 package check.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/kernel-jobs/Cargo.toml`
  - add `crates/kernel-jobs/src/lib.rs`
  - add `crates/kernel-jobs/src/model.rs`
  - add `crates/kernel-jobs/src/runtime.rs`
  - update `.artifacts/ai` records for the B3 slice
- out of scope:
  - adding module-specific checkpoint logic or provider-specific job details
  - creating tests for `kernel-jobs` before the later dedicated test slice
  - touching `src-tauri`, composition-root, or module crates

## Allowed Files

1. Cargo.toml
2. crates/kernel-jobs/Cargo.toml
3. crates/kernel-jobs/src/lib.rs
4. crates/kernel-jobs/src/model.rs
5. crates/kernel-jobs/src/runtime.rs
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriBackendSkeletonImplementationDesign.md
6. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
7. docs/TauriFirstCrateApiDrafts.md
8. docs/TauriKernelJobsRuntimeDesign.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md
11. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-kernel-jobs` is introduced as a workspace member with only shared job models and a minimal `JobRuntime` trait backed by `launcher-kernel-foundation` types, then `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without introducing any module-specific runtime logic.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the minimal shared job protocol shell in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected workspace-member `Cargo.lock` update.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/kernel-jobs/Cargo.toml
3. crates/kernel-jobs/src/lib.rs
4. crates/kernel-jobs/src/model.rs
5. crates/kernel-jobs/src/runtime.rs
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap kernel jobs crate
2. push command plan: git push

## 停止条件

1. the slice starts leaking download/Fab/engine-specific runtime rules into `launcher-kernel-jobs`
2. `cargo check -p launcher-kernel-jobs` fails for reasons outside the B3 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the B3 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.