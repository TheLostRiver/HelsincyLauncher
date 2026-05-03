# Active Atomic Task

## Identity

- task id: AT-2026-05-03-016
- title: Implement foundation contract surface
- status: committed

## Goal

- exact local outcome: Align the documented B2 slice with the current B1 baseline, then implement the minimal `launcher-kernel-foundation` error/clock/id/paging/time/result surface and a real `foundation_contract_smoke` test that passes.

## Scope

- in scope:
  - fix the B2 task-table file list drift in the backend skeleton doc
  - update `crates/kernel-foundation/Cargo.toml` with the minimal dependencies needed by the documented foundation surface
  - replace the B1 anchor with real `kernel-foundation` module exports
  - add `error.rs`, `result.rs`, `clock.rs`, `ids.rs`, `paging.rs`, `time.rs`
  - add `crates/kernel-foundation/tests/foundation_contract_smoke.rs`
  - update `.artifacts/ai` records for this slice
- out of scope:
  - adding business DTOs, provider payloads, or module state machines
  - starting `kernel-jobs`, composition-root, or `src-tauri` wiring work
  - widening to workspace-level `cargo check --workspace`

## Allowed Files

1. docs/TauriBackendSkeletonImplementationDesign.md
2. crates/kernel-foundation/Cargo.toml
3. crates/kernel-foundation/src/lib.rs
4. crates/kernel-foundation/src/error.rs
5. crates/kernel-foundation/src/result.rs
6. crates/kernel-foundation/src/clock.rs
7. crates/kernel-foundation/src/ids.rs
8. crates/kernel-foundation/src/paging.rs
9. crates/kernel-foundation/src/time.rs
10. crates/kernel-foundation/tests/foundation_contract_smoke.rs
11. .artifacts/ai/active-task.md
12. .artifacts/ai/task-plan.md
13. .artifacts/ai/progress.md
14. .artifacts/ai/findings.md

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
8. docs/TauriErrorHandlingAndProjectionDesign.md
9. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
10. .github/copilot-instructions.md
11. .github/skills/strict-doc-driven-development/SKILL.md
12. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-kernel-foundation` exports only the documented zero-business primitives (`AppError`, `AppResult`, `Clock`, stable IDs, paging, time) and the B2 task definition is corrected to match the real files needed for that surface, then `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling in any business-specific types.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-kernel-foundation foundation_contract_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the first real foundation primitives and named smoke test in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected dependency-driven `Cargo.lock` update.

## 需要更新的文档和日志

1. docs/TauriBackendSkeletonImplementationDesign.md
2. crates/kernel-foundation/Cargo.toml
3. crates/kernel-foundation/src/lib.rs
4. crates/kernel-foundation/src/error.rs
5. crates/kernel-foundation/src/result.rs
6. crates/kernel-foundation/src/clock.rs
7. crates/kernel-foundation/src/ids.rs
8. crates/kernel-foundation/src/paging.rs
9. crates/kernel-foundation/src/time.rs
10. crates/kernel-foundation/tests/foundation_contract_smoke.rs
11. .artifacts/ai/active-task.md
12. .artifacts/ai/task-plan.md
13. .artifacts/ai/progress.md
14. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Implement foundation contract surface
2. push command plan: git push

## 停止条件

1. the slice starts introducing business DTOs or cross-crate wiring into `launcher-kernel-foundation`
2. `cargo test -p launcher-kernel-foundation foundation_contract_smoke` fails for reasons outside the B2 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the B2 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.