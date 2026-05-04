# Active Atomic Task

## Identity

- task id: AT-2026-05-03-046
- title: Stage-2 restore runtime state summary
- status: in_progress

## Goal

- exact local outcome: Implement `run_stage2_restore_runtime_state()` in `StartupPipelineFacade` so it reads all resumable job snapshots (states: Queued, ClaimingLease, Restoring, Running) from the shared `JobSnapshotStore`, counts them, and returns `Ok(())` with a tracing log — no real re-execution, no lease acquisition, just a verified summary read so the stage-2 path is no longer a no-op stub. Add `list_resumable()` to the `JobSnapshotStore` trait and implement it in both the in-memory and sqlite adapters. Wire the store into `StartupPipelineFacade` from composition-root. Validate with a named test `stage2_restore_reads_resumable_snapshots` plus the two existing smoke tests.

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `crates/kernel-jobs/src/runtime.rs`
  - update `crates/adapter-storage-sqlite/src/lib.rs`
  - update `crates/composition-root/src/startup.rs`
  - update `crates/composition-root/src/bootstrap.rs`
  - update `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
- out of scope:
  - real job re-execution during restore
  - lease acquisition/release
  - driver registry or module callbacks
  - frontend IPC changes
  - touching user-owned frontend files

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. crates/kernel-jobs/src/runtime.rs
6. crates/adapter-storage-sqlite/src/lib.rs
7. crates/composition-root/src/startup.rs
8. crates/composition-root/src/bootstrap.rs
9. crates/composition-root/tests/bootstrap_wiring_smoke.rs

## Hypothesis

- falsifiable local hypothesis: If `StartupPipelineFacade` receives a `Arc<dyn JobSnapshotStore<()>>` and calls `list_resumable()` inside `run_stage2_restore_runtime_state`, then a named test that enqueues a job and calls stage-2 restore will see a non-empty resumable list, confirming the path is no longer a no-op.

## Cheap Check

- narrowest check: named test `stage2_restore_reads_resumable_snapshots` passes, then `bootstrap_wiring_smoke` and `transport_wiring_smoke` still pass.
  - changing downloads facade behavior beyond receiving persisted shared snapshots
  - introducing provider IO or media-cache IO
  - changing downloads, startup, or frontend production code
  - touching user-owned frontend worktree changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. Cargo.toml
6. crates/adapter-storage-sqlite/Cargo.toml
7. crates/kernel-jobs/src/lib.rs
8. crates/kernel-jobs/src/runtime.rs
9. crates/adapter-storage-sqlite/src/lib.rs
10. crates/composition-root/src/bootstrap.rs
11. crates/composition-root/tests/bootstrap_wiring_smoke.rs

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

- falsifiable local hypothesis: If `SharedJobRuntimeHost` routes snapshots through an explicit snapshot store and composition-root injects a sqlite-backed implementation, then a job accepted before dropping `DesktopAppServices` will still be readable after rebuilding services against the same sqlite path, without opening stage-2 restore orchestration or lease handling yet.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify a named composition-root test can enqueue a Fab prewarm job, rebuild `DesktopAppServices` with the same sqlite path, and still read that job snapshot from the new runtime host, then rerun `bootstrap_wiring_smoke` and the existing host `transport_wiring_smoke` to confirm the broader baseline still compiles.

## Validation Gate

1. `cargo test -p launcher-composition-root runtime_snapshot_persists_across_rebuilds --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- pending

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. Cargo.toml
6. crates/adapter-storage-sqlite/Cargo.toml
7. crates/kernel-jobs/src/lib.rs
8. crates/kernel-jobs/src/runtime.rs
9. crates/adapter-storage-sqlite/src/lib.rs
10. crates/composition-root/src/bootstrap.rs
11. crates/composition-root/tests/bootstrap_wiring_smoke.rs

## 验证后的 Git 动作

1. commit message plan: Persist runtime snapshots to sqlite
2. push command plan: git push

## 停止条件

1. persisting runtime snapshots requires opening full stage-2 recovery orchestration, lease handling, or driver registry behavior beyond the allowed slice
2. the current runtime host cannot adopt a pluggable snapshot store without changing files outside the allowed set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: run the named composition-root persistence test first, then rerun `bootstrap_wiring_smoke` and the existing host transport smoke before recording AT-045 closeout.