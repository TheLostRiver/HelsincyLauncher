# Active Atomic Task

## Identity

- task id: AT-2026-05-16-176
- title: Guard runtime enqueue on scheduler failure
- status: completed

## Goal

补上 `DownloadResumeWorkScheduler` 失败路径的最小保护：当 downloads-owned scheduler/driver boundary 返回错误时，`resume_download_outcome()` 必须直接返回该错误，并且不能调用 shared `JobRuntime::enqueue()`。

本轮只覆盖失败保护：

- add focused RED test first
- make scheduler failure observable in the existing test fake
- prove runtime enqueue is skipped when scheduler preparation fails
- keep concrete fetch/write/verify execution, SQLite schema, frontend, host transport, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - concrete scheduler/fetch/write/verify execution
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_IMPL.md
5. docs/TauriDownloadRuntimeDesign.md
6. docs/TauriKernelJobsRuntimeDesign.md
7. docs/TauriTestingStrategyAndQualityGateDesign.md
8. docs/TauriAIDevelopmentTransactionProtocolDesign.md
9. docs/TauriCodeCommentStandard.md

## Hypothesis

- falsifiable local hypothesis: the existing scheduler-before-enqueue call order already supports failure propagation, and a focused test can prove no runtime enqueue happens when the scheduler port fails.

## Cheap Check

- add one focused failing test that configures the scheduler fake to fail, then run the focused test and full module suite.

## Validation Gate

1. Read required docs before editing Rust.
2. Add RED test proving scheduler error skips runtime enqueue.
3. Add the smallest test fake behavior / production adjustment needed.
4. Update README_IMPL current state.
5. Run focused test and full `launcher-module-downloads` test.
6. Run scoped `git diff --check`.
7. Commit the Rust/docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required README_IMPL scheduler failure behavior, kernel-jobs non-goals, and module facade test strategy were read before Rust edits.
- RED test `resume_download_skips_runtime_enqueue_when_scheduler_fails` first failed on missing failing scheduler fake behavior.
- Added the smallest test fake behavior to let `RecordingResumeWorkScheduler` return a configured `AppError`.
- Focused test passed: `resume_download_skips_runtime_enqueue_when_scheduler_fails` returned 1 passed, 0 failed.
- Full module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` returned 20 passed, 0 failed.
- Scoped `git diff --check` passed for AT-176 files with CRLF warnings only.
- Concrete scheduler execution, SQLite schema, frontend, host transport, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-175 committed scheduler port as `8846a40`.
- User approved four consecutive tasks without intermediate confirmation; this is task 3/4 in the current batch.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: start task 4/4 by adding a focused all-sealed/no-scheduler guard.
