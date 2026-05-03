# Active Atomic Task

## Identity

- task id: AT-2026-05-03-043
- title: Orchestrate startup stage-3 prewarm
- status: committed

## Goal

- exact local outcome: Replace the stage-3 no-op in `StartupPipelineFacade::run_stage3_background_prewarm()` with explicit, config-gated Fab prewarm orchestration that calls the already-wired `FabFacade::run_startup_prewarm()` path, while leaving real job-runtime execution and startup host ordering for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/composition-root/src/bootstrap.rs`
  - update `crates/composition-root/src/startup.rs`
  - update `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
- out of scope:
  - changing startup host execution order in `src-tauri`
  - introducing real job-runtime enqueue wiring, provider IO, media-cache IO, or real SQLite queries
  - adding auth/session-availability gating beyond current config capability gating
  - changing downloads, startup, or frontend code
  - touching user-owned frontend worktree changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/composition-root/src/bootstrap.rs
6. crates/composition-root/src/startup.rs
7. crates/composition-root/tests/bootstrap_wiring_smoke.rs

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

- falsifiable local hypothesis: If `StartupPipelineFacade::run_stage3_background_prewarm()` explicitly calls the already-wired Fab prewarm facade path when startup prewarm is enabled, then stage-3 startup prewarm stops being a no-op without reopening real job-runtime execution or host-side startup sequencing.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named composition-root test can prove `run_stage3_background_prewarm()` triggers Fab prewarm when the config gate is enabled, then rerun `bootstrap_wiring_smoke` and the existing host `transport_wiring_smoke` to confirm the broader baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-composition-root run_stage3_background_prewarm_triggers_fab_prewarm_when_enabled --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-composition-root run_stage3_background_prewarm_triggers_fab_prewarm_when_enabled --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved stage-3 startup prewarm now triggers the Fab prewarm path when the config gate is enabled.
- `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the composition root still assembles services while stage-3 prewarm stays callable.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing host transport baseline still compiles and executes after the stage-3 startup change.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/composition-root/src/bootstrap.rs crates/composition-root/src/startup.rs crates/composition-root/tests/bootstrap_wiring_smoke.rs` produced no blocking output for the AT-043 stage-3 slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/composition-root/src/bootstrap.rs
6. crates/composition-root/src/startup.rs
7. crates/composition-root/tests/bootstrap_wiring_smoke.rs

## 验证后的 Git 动作

1. commit message plan: Orchestrate startup stage-3 prewarm
2. push command plan: git push

## 停止条件

1. wiring stage-3 prewarm requires reopening real job-runtime execution, provider IO, or host-side startup ordering beyond the allowed composition-root slice
2. the current module surfaces cannot express backend-owned job acceptance without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the AT-043 record and composition-root files, commit the slice, then decide whether the next backend task should move to a real runtime bundle, richer startup gating, or another narrow backend path.