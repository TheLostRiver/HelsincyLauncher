# Active Atomic Task

## Identity

- task id: AT-2026-05-03-039
- title: Wire fab list inventory query
- status: committed

## Goal

- exact local outcome: Replace the `FAB_NOT_WIRED` fallback on `FabFacade::list_inventory()` with a real projection-repository delegation path, keep the SQLite projection adapter on a cold-start empty-page stub, and prove the query path with one named module-level test while leaving detail, sync, and prewarm work for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/module-fab/src/facade/mod.rs`
  - update `crates/adapter-storage-sqlite/src/lib.rs`
- out of scope:
  - wiring `get_asset_detail`, `run_startup_prewarm`, or `sync_inventory`
  - introducing provider IO or real SQLite queries
  - changing downloads, startup, or frontend code
  - touching user-owned frontend worktree changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs
6. crates/adapter-storage-sqlite/src/lib.rs

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
11. docs/TauriCurrentRepoArchitectureOverview.md

## Hypothesis

- falsifiable local hypothesis: If `FabFacade::list_inventory()` delegates to a `FabInventoryProjectionRepository` and the current SQLite projection adapter returns a cold-start empty page instead of `FAB_NOT_WIRED`, then the already-wired host transport path becomes the first real backend-owned Fab inventory query without requiring provider sync or broader startup work.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named module-fab test can prove `list_inventory()` delegates to the projection repository, then rerun the existing host `transport_wiring_smoke` to confirm the transport path still compiles and executes through the real query chain.

## Validation Gate

1. `cargo test -p launcher-module-fab list_inventory_delegates_to_projection_repository --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-module-fab list_inventory_delegates_to_projection_repository --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved `FabFacade::list_inventory()` delegates to the projection repository.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing host transport baseline still executes through the updated Fab query path.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/module-fab/src/facade/mod.rs crates/adapter-storage-sqlite/src/lib.rs` produced no blocking output for the AT-039 query-wiring slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs
6. crates/adapter-storage-sqlite/src/lib.rs

## 验证后的 Git 动作

1. commit message plan: Wire fab list inventory query
2. push command plan: git push

## 停止条件

1. wiring `list_inventory` requires reopening broader Fab sync/provider/startup architecture instead of a local query slice
2. the current module/adpater surfaces cannot express a projection-query path without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the AT-039 record and query-wiring files, commit the slice, then decide whether the next backend task stays on Fab or moves to another narrow runtime path.