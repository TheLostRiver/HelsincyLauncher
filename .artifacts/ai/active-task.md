# Active Atomic Task

## Identity

- task id: AT-2026-05-03-040
- title: Wire fab asset detail query
- status: committed

## Goal

- exact local outcome: Replace the `FAB_NOT_WIRED` fallback on `FabFacade::get_asset_detail()` with a projection-first detail path that consults the local projection repository and returns a backend-owned cold-start detail placeholder when no local snapshot exists, while leaving provider-backed detail hydration and startup prewarm for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/module-fab/src/facade/mod.rs`
  - update `crates/adapter-storage-sqlite/src/lib.rs`
- out of scope:
  - wiring `run_startup_prewarm` or `sync_inventory`
  - introducing provider IO, media-cache IO, or real SQLite queries
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

- falsifiable local hypothesis: If `FabFacade::get_asset_detail()` consults the local projection repository first and returns a backend-owned cold-start placeholder when the projection is empty, then the Fab detail path stops depending on the transport-layer fallback without pulling in provider sync or startup-prewarm architecture.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named module-fab test can prove `get_asset_detail()` consults the projection repository and returns the cold-start detail placeholder when no snapshot exists, then rerun the existing host `transport_wiring_smoke` to confirm the broader host baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-module-fab get_asset_detail_returns_cold_start_placeholder_when_projection_is_empty --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-module-fab get_asset_detail_returns_cold_start_placeholder_when_projection_is_empty --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved the Fab detail path returns a backend-owned cold-start placeholder when the local projection is empty.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing host transport baseline still compiles and executes after the Fab detail change.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/module-fab/src/facade/mod.rs crates/adapter-storage-sqlite/src/lib.rs` produced no blocking output for the AT-040 detail-query slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs
6. crates/adapter-storage-sqlite/src/lib.rs

## 验证后的 Git 动作

1. commit message plan: Wire fab asset detail query
2. push command plan: git push

## 停止条件

1. wiring `get_asset_detail` requires reopening broader Fab provider/startup architecture instead of a local detail-query slice
2. the current module/adpater surfaces cannot express a projection-query path without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the AT-040 record and detail-query files, commit the slice, then decide whether the next Fab/backend task should move to startup prewarm, sync, or another narrow backend path.