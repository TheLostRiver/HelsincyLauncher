# Active Atomic Task

## Identity

- task id: AT-2026-05-03-044
- title: Build shared job runtime bundle
- status: committed

## Goal

- exact local outcome: Replace the current `()` job-runtime placeholder with a real shared in-memory runtime host from `launcher-kernel-jobs`, wire it through composition-root, and let the existing Fab accepted-job paths enqueue into that runtime, while leaving drivers, persistence, recovery, and downloads behavior for later slices.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/kernel-jobs/src/lib.rs`
  - update `crates/kernel-jobs/src/runtime.rs`
  - update `crates/module-fab/src/facade/mod.rs`
  - update `crates/composition-root/src/bootstrap.rs`
  - update `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
- out of scope:
  - adding runtime persistence, lease recovery, or driver registry behavior
  - changing startup host execution order in `src-tauri`
  - changing downloads facade behavior beyond receiving the shared runtime dependency
  - introducing provider IO, media-cache IO, or real SQLite queries
  - changing downloads, startup, or frontend code
  - touching user-owned frontend worktree changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/kernel-jobs/src/lib.rs
6. crates/kernel-jobs/src/runtime.rs
7. crates/module-fab/src/facade/mod.rs
8. crates/composition-root/src/bootstrap.rs
9. crates/composition-root/tests/bootstrap_wiring_smoke.rs

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

- falsifiable local hypothesis: If `launcher-kernel-jobs` provides a minimal shared in-memory runtime host that records enqueued snapshots and composition-root injects it instead of `()`, then the current Fab accepted-job paths can stop using placeholder acceptance and move onto a real shared runtime bundle without reopening persistence, recovery, or driver execution.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named kernel-jobs test can prove the shared runtime host records a snapshot when a job is enqueued, then rerun `bootstrap_wiring_smoke` and the existing host `transport_wiring_smoke` to confirm the broader baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-kernel-jobs shared_job_runtime_host_records_enqueued_snapshot --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo test -p launcher-kernel-jobs shared_job_runtime_host_records_enqueued_snapshot --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and proved the new shared runtime host records a queued snapshot when a job is enqueued.
- The first `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` run surfaced a Rust coherence conflict from blanket `JobRuntime` acceptance impls in `module-fab`; narrowing those impls to the concrete `SharedJobRuntimeHost` repaired the slice locally.
- `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` then passed and confirmed composition-root now injects the shared runtime host while the Fab prewarm path records a queued snapshot.
- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed and confirmed the existing desktop host transport baseline still compiles and executes after the runtime-bundle change.
- `git diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md crates/kernel-jobs/src/lib.rs crates/kernel-jobs/src/runtime.rs crates/module-fab/src/facade/mod.rs crates/composition-root/src/bootstrap.rs crates/composition-root/tests/bootstrap_wiring_smoke.rs` produced no blocking output for the AT-044 runtime-bundle slice.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/kernel-jobs/src/lib.rs
6. crates/kernel-jobs/src/runtime.rs
7. crates/module-fab/src/facade/mod.rs
8. crates/composition-root/src/bootstrap.rs
9. crates/composition-root/tests/bootstrap_wiring_smoke.rs

## 验证后的 Git 动作

1. commit message plan: Build shared job runtime bundle
2. push command plan: git push

## 停止条件

1. wiring a real runtime bundle requires reopening persistence, recovery, driver registry, or broader downloads behavior beyond the allowed slice
2. the current module surfaces cannot express backend-owned job acceptance without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: AT-044 is closed and validated; decide whether the next backend slice should open runtime persistence/recovery, broader downloads runtime behavior, richer startup gating, or another narrow backend path.