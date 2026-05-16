# Active Atomic Task

## Identity

- task id: AT-2026-05-16-177
- title: Guard all-sealed resume from scheduler
- status: completed

## Goal

补上 all-sealed resume 的 scheduler 边界保护：当 `resume_download_outcome()` 判定所有 segment 已 sealed 并返回 `AlreadyComplete` 时，不能调用 downloads-owned scheduler，也不能调用 shared runtime enqueue。

本轮只覆盖 focused guard：

- add focused guard test around all-sealed resume
- use a failing scheduler fake to prove the scheduler boundary is not touched
- keep concrete scheduler execution, SQLite schema, frontend, host transport, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
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
6. .artifacts/ai/handoff.md

## 控制性文档

1. docs/modules/downloads/README_IMPL.md
2. docs/TauriDownloadRuntimeDesign.md
3. docs/TauriKernelJobsRuntimeDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md

## Hypothesis

- falsifiable local hypothesis: all-sealed resume already returns `AlreadyComplete` before scheduler/runtime work, and a focused guard can lock that ordering down without production behavior changes.

## Cheap Check

- add one focused guard test with a failing scheduler fake, then run focused and full downloads module tests.

## Validation Gate

1. Read focused implementation docs before editing Rust.
2. Add focused all-sealed/no-scheduler guard test.
3. Update README_IMPL current state.
4. Run focused test and full `launcher-module-downloads` test.
5. Run scoped `git diff --check`.
6. Commit the test/docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- README_IMPL call-order lines were read before adding the guard.
- Added focused guard `resume_download_all_sealed_does_not_touch_scheduler` using a failing scheduler fake to prove the scheduler boundary is not touched.
- Focused guard passed immediately, confirming existing call order already returned `AlreadyComplete` before scheduler/runtime work.
- Full module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` returned 21 passed, 0 failed.
- Scoped `git diff --check` passed for AT-177 files with CRLF warnings only.
- Concrete scheduler execution, SQLite schema, frontend, host transport, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-176 committed scheduler failure guard as `edec23d`.
- User approved four consecutive tasks without intermediate confirmation; this is task 4/4 in the current batch.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: the four-task batch is complete; reassess README_IMPL implementation map before choosing another downloads backend slice.
