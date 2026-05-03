# Active Atomic Task

## Identity

- task id: AT-2026-05-03-042
- title: Accept fab startup prewarm job
- status: committed

## Goal

- exact local outcome: Replace the `FAB_NOT_WIRED` result on `FabFacade::run_startup_prewarm()` with a backend-owned accepted-job response that stays within the current module boundary, while leaving stage-3 startup orchestration and real job-runtime enqueue wiring for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/module-fab/src/facade/mod.rs`
- out of scope:
  - wiring `StartupPipelineFacade::run_stage3_background_prewarm()` or startup host execution order
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

- falsifiable local hypothesis: If `FabFacade::run_startup_prewarm()` delegates job acceptance to a narrow prewarm-job boundary that has a placeholder implementation for the current `()` runtime dependency, then the Fab startup-prewarm command can return a backend-owned `AcceptedJob` without reopening stage-3 startup orchestration or the still-missing real job runtime wiring.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named module-fab test can prove `run_startup_prewarm()` returns a backend-owned `AcceptedJob` when the current placeholder runtime is injected, then rerun the existing host `transport_wiring_smoke` to confirm the broader host baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-module-fab run_startup_prewarm_returns_backend_owned_accepted_job_with_placeholder_runtime --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-module-fab run_startup_prewarm_returns_backend_owned_accepted_job_with_placeholder_runtime --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved the Fab startup-prewarm path now returns a backend-owned `AcceptedJob` when the current placeholder runtime is injected.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing host transport baseline still compiles and executes after the Fab startup-prewarm accepted-job change.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/module-fab/src/facade/mod.rs` produced no blocking output for the AT-042 prewarm-job slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/module-fab/src/facade/mod.rs

## 验证后的 Git 动作

1. commit message plan: Accept fab startup prewarm job
2. push command plan: git push

## 停止条件

1. wiring `run_startup_prewarm` requires reopening stage-3 startup orchestration, real job-runtime, provider, or startup host execution beyond the allowed module slice
2. the current module surfaces cannot express backend-owned job acceptance without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the AT-042 record and module-fab files, commit the slice, then decide whether the next Fab/backend task should move to startup stage-3 orchestration, a real runtime bundle, or another narrow backend path.