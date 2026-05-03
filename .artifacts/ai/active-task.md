# Active Atomic Task

## Identity

- task id: AT-2026-05-03-041
- title: Accept fab sync inventory job
- status: committed

## Goal

- exact local outcome: Replace the `FAB_NOT_WIRED` result on `FabFacade::sync_inventory()` with a backend-owned accepted-job response that stays within the current module boundary, while leaving real job-runtime enqueue wiring and startup prewarm orchestration for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/module-fab/src/facade/mod.rs`
- out of scope:
  - wiring `run_startup_prewarm`
  - introducing real job-runtime enqueue wiring, provider IO, media-cache IO, or real SQLite queries
  - changing downloads, startup, or frontend code
  - touching user-owned frontend worktree changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. .github/copilot-instructions.md
2. .github/skills/strict-doc-driven-development/SKILL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. docs/TauriTestingStrategyAndQualityGateDesign.md
8. docs/TauriFirstCrateApiDrafts.md
9. docs/TauriRepositoryPortsAndAdapterDesign.md
10. docs/TauriFabInventoryLoadingDesign.md
11. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
12. docs/TauriCompositionRootWiringDesign.md
13. docs/TauriCurrentRepoArchitectureOverview.md

## Hypothesis

- falsifiable local hypothesis: If `FabFacade::sync_inventory()` delegates job acceptance to a narrow sync-job boundary that has a placeholder implementation for the current `()` runtime dependency, then the Fab sync command can return a backend-owned `AcceptedJob` without reopening startup-prewarm orchestration or the still-missing real job runtime wiring.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named module-fab test can prove `sync_inventory()` returns a backend-owned `AcceptedJob` when the current placeholder runtime is injected, then rerun the existing host `transport_wiring_smoke` to confirm the broader host baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-module-fab sync_inventory_returns_backend_owned_accepted_job_with_placeholder_runtime --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-module-fab sync_inventory_returns_backend_owned_accepted_job_with_placeholder_runtime --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved the Fab sync-inventory path now returns a backend-owned `AcceptedJob` when the current placeholder runtime is injected.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing host transport baseline still compiles and executes after the Fab sync accepted-job change.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/module-fab/src/facade/mod.rs` produced no blocking output for the AT-041 sync-job slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs

## 验证后的 Git 动作

1. commit message plan: Accept fab sync inventory job
2. push command plan: git push

## 停止条件

1. wiring `sync_inventory` requires reopening real job-runtime, provider, or startup orchestration beyond the allowed module slice
2. the current module surfaces cannot express backend-owned job acceptance without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the AT-041 record and module-fab files, commit the slice, then decide whether the next Fab/backend task should move to startup prewarm, a real runtime bundle, or another narrow backend path.