# Active Atomic Task

## Identity

- task id: AT-2026-05-16-175
- title: Add downloads resume scheduler port
- status: completed

## Goal

按照 AT-174 写清楚的实现边界，在 `module-downloads` 内引入最小 `DownloadResumeWorkScheduler` 端口，并让 `resume_download_outcome()` 在 shared `JobRuntime` job-level enqueue 之前，把已派生的 `DownloadResumeWorkPlan` 交给 downloads-owned scheduler/driver boundary。

本轮只覆盖端口和调用顺序：

- add focused RED test first
- add `DownloadResumeWorkScheduler` trait and `()` placeholder
- wire scheduler dependency into `DownloadModuleDeps`
- call scheduler after work-plan derivation and before runtime enqueue
- update composition/test construction sites as needed
- keep concrete fetch/write/verify execution, SQLite schema, frontend, host transport, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/lib.rs`
  - `crates/composition-root/src/bootstrap.rs`
  - `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/lib.rs
3. crates/composition-root/src/bootstrap.rs
4. crates/composition-root/tests/bootstrap_wiring_smoke.rs
5. docs/modules/downloads/README_IMPL.md
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md
10. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
10. docs/TauriFirstCrateApiDrafts.md
11. docs/TauriKernelJobsRuntimeDesign.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md
14. docs/TauriCodeCommentStandard.md

## Hypothesis

- falsifiable local hypothesis: `resume_download_outcome()` can schedule the module-owned `DownloadResumeWorkPlan` before shared runtime enqueue by adding a narrow scheduler port, without changing concrete execution, persistence schema, host transport, frontend, or `kernel-jobs` payloads.

## Cheap Check

- add one focused failing module test that records scheduler calls before runtime enqueue, then add the smallest scheduler port implementation and rerun focused + module tests.

## Validation Gate

1. Read required docs before editing Rust.
2. Add RED test proving `resume_download_outcome()` schedules the derived work plan before runtime enqueue.
3. Add minimal scheduler port, placeholder impl, dependency wiring, and crate export with bilingual comments.
4. Update README_IMPL current state.
5. Run focused module test, full `launcher-module-downloads` test, and narrow composition smoke if composition wiring changes.
6. Run scoped `git diff --check`.
7. Commit the Rust/docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required docs, README_IMPL section 7.7, composition wiring boundaries, and comment-language rules were read before Rust edits.
- RED test `resume_download_schedules_work_plan_before_runtime_enqueue` first failed on missing `DownloadResumeWorkScheduler`.
- Minimal GREEN implementation added `DownloadResumeWorkScheduler`, no-op `()` placeholder, `DownloadModuleDeps` scheduler dependency, facade scheduling call before runtime enqueue, crate export, and composition placeholder wiring.
- Focused test passed: `resume_download_schedules_work_plan_before_runtime_enqueue` returned 1 passed, 0 failed.
- Full module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` returned 19 passed, 0 failed.
- Composition smoke passed: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml bootstrap_wiring_smoke` returned 1 passed, 0 failed for the integration test.
- Scoped `git diff --check` passed for AT-175 files with CRLF warnings only.
- Concrete scheduler execution, SQLite schema, frontend, host transport, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-174 committed scheduler boundary docs as `6929fa9`.
- User approved four consecutive tasks without intermediate confirmation; this is task 2/4 in the current batch.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: start task 3/4 by adding a focused scheduler-failure guard proving scheduler errors skip runtime enqueue.
