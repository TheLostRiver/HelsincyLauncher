# Active Atomic Task

## Identity

- task id: AT-2026-05-03-014
- title: Bootstrap kernel foundation crate
- status: committed

## Goal

- exact local outcome: Create the minimal `crates/kernel-foundation` crate shell and register it in the root workspace so `cargo check -p launcher-kernel-foundation` passes without pulling in B2 foundation types yet.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/kernel-foundation/Cargo.toml`
  - add `crates/kernel-foundation/src/lib.rs`
  - update `.artifacts/ai` records for the B1 slice
- out of scope:
  - adding `error.rs`, `clock.rs`, `ids.rs`, `paging.rs`, `time.rs`, or any tests from B2
  - changing `src-tauri` host files
  - introducing `kernel-jobs`, module crates, adapter crates, or composition root

## Allowed Files

1. Cargo.toml
2. crates/kernel-foundation/Cargo.toml
3. crates/kernel-foundation/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

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
8. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md
11. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the root workspace gains a real `crates/kernel-foundation` member with a target-bearing `src/lib.rs`, then the documented B1 gate `cargo check -p launcher-kernel-foundation` will pass without needing any B2 foundation modules yet.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the minimal B1 shell in place.
- The crate remains B1-sized: only workspace registration, manifest, and target-bearing `src/lib.rs` exist; B2 types and tests are still deferred.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/kernel-foundation/Cargo.toml
3. crates/kernel-foundation/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap kernel foundation crate
2. push command plan: git push

## 停止条件

1. the crate shell starts leaking B2 content or business types into B1
2. `cargo check -p launcher-kernel-foundation` fails because the workspace member shape is still invalid
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the B1 shell, then open B2 for the first real foundation modules and named smoke test.