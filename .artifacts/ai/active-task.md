# Active Atomic Task

## Identity

- task id: AT-2026-05-16-179
- title: Add downloads pending resume work scheduler shell
- status: completed

## Goal

实现 downloads 模块本地 pending resume work queue / scheduler shell，让 `DownloadResumeWorkScheduler` 的真实模块内实现可以登记 `DownloadResumeWorkPlan`，并保持登记发生在 shared runtime job-level enqueue 之前。

本轮只覆盖 TDD-backed module-local shell：

- add focused RED test for pending work registration before runtime enqueue
- add minimal in-memory/module-local scheduler shell behind `DownloadResumeWorkScheduler`
- keep `DownloadResumeWorkPlan` transient and module-owned
- keep actual fetch/write/verify execution, checkpoint mutation, SQLite schema, host transport, frontend, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - concrete scheduler/fetch/write/verify execution
  - checkpoint mutation or new checkpoint repository methods
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

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

Related architecture/collaboration docs read in scoped snippets:

1. docs/modules/downloads/README_IMPL.md
2. docs/TauriDownloadRuntimeDesign.md
3. docs/TauriKernelJobsRuntimeDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md
6. docs/TauriRewriteArchitectureBlueprint.md
7. docs/TauriArchitecturePrinciplesDesign.md
8. docs/TauriCurrentRepoArchitectureOverview.md
9. docs/TauriCompositionRootWiringDesign.md

## Hypothesis

- falsifiable local hypothesis: a minimal module-local scheduler shell can record pending resume work plans before runtime enqueue without touching fetch/write/verify, checkpoint persistence, host transport, frontend, SQLite schema, or `kernel-jobs`.

## Cheap Check

- focused RED/GREEN test: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download_registers_pending_work_before_runtime_enqueue`

## Validation Gate

1. Read required docs in scoped snippets before editing README_IMPL.
2. Write focused RED test for pending resume work registration before runtime enqueue.
3. Observe the focused test fail for missing scheduler shell API.
4. Add the minimal module-local scheduler shell and exports.
5. Update README_IMPL current state and PWF records.
6. Run focused test and full `launcher-module-downloads` suite.
7. Run scoped `git diff --check` and path-limited `git status --short`.
8. Commit only the AT-179 slice locally.

## Validation Result

- passed
- Required README, docs index, CONTRIBUTING, downloads module docs, README_IMPL, download runtime, kernel-jobs runtime, testing strategy, AI transaction protocol, crate API, and composition snippets were read before coding.
- RED: focused test `resume_download_registers_pending_work_before_runtime_enqueue` failed on missing `DownloadPendingResumeWork` and `InMemoryDownloadResumeWorkScheduler` imports.
- GREEN: added `DownloadPendingResumeWork` and `InMemoryDownloadResumeWorkScheduler` in `crates/module-downloads/src/facade/mod.rs`.
- The scheduler shell stores pending work in memory only, implements `DownloadResumeWorkScheduler`, and exposes `pending_work()` / `drain_pending_work()` for later module-owned driver use.
- Exported the new scheduler shell and pending work type through `crates/module-downloads/src/lib.rs`.
- Focused test passed: 1 passed, 0 failed.
- Full `launcher-module-downloads` suite passed: 22 passed, 0 failed.
- `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
- Scoped `git diff --check` passed for AT-179 files with CRLF warnings only.
- Path-limited status shows AT-179 files plus the unrelated pre-existing `crates/composition-root/src/startup.rs` formatting side effect.
- Concrete fetch/write/verify, checkpoint mutation, SQLite schema, host transport, frontend, composition wiring, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-178 committed scheduler execution boundary docs as `41c3be4`.
- User approved starting AT-179 with "开始实现".
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Current unrelated worktree state: preserve `crates/composition-root/src/startup.rs` formatting side effect and other unrelated dirty files.
- Shell note: an `rg` command with nested double quotes failed PowerShell parsing while checking Mutex style; reran the query with single quotes successfully.
