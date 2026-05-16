# Active Atomic Task

## Identity

- task id: AT-2026-05-16-172
- title: Document downloads resume scheduler driver payload boundary
- status: completed

## Goal

在 `docs/modules/downloads/README_IMPL.md` 中把 `resume_partial` / `queue_remaining` 后续如何进入 downloads-owned scheduler/driver 写清楚：shared `JobRuntime` 仍只做 job-level enqueue，segment plan/checkpoint 仍留在 `module-downloads` 内部，下一步 Rust 代码只能先引入模块内的 resume work payload/plan 边界，不能把 segment payload 塞进 `kernel-jobs`、host transport、frontend 或 SQLite schema。

本轮只覆盖文档和 PWF：

- define downloads-owned resume work payload boundary
- define what runtime enqueue can and cannot carry
- define how `resume_partial` and `queue_remaining` should be translated for the later scheduler/driver
- define the next minimal TDD Rust slice
- keep Rust code, frontend, host transport, SQLite schema, scheduler execution, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

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

## Hypothesis

- falsifiable local hypothesis: a docs-only implementation guide slice can define the downloads-owned resume work payload boundary precisely enough that the next Rust task can add focused module tests without touching `kernel-jobs`, host transport, frontend, SQLite schema, or concrete scheduler execution.

## Cheap Check

- update README_IMPL, then run scoped doc/Git validation and read back the scheduler/driver payload boundary wording.

## Validation Gate

1. Read required root, docs index, downloads module, download runtime, kernel-jobs runtime, crate layout/API, testing, and collaboration docs before editing README_IMPL.
2. Update README_IMPL with the scheduler/driver payload boundary and next Rust slice.
3. Record findings and recovery state in `.artifacts/ai`.
4. Run scoped `git diff --check` for AT-172 files.
5. Commit the docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required root, docs index, downloads module, runtime, kernel-jobs, crate layout/API, testing, and collaboration documents were read in scoped snippets before editing.
- `docs/modules/downloads/README_IMPL.md` now defines the scheduler/driver payload boundary: shared `JobRuntime` receives only job-level enqueue, while segment work items remain downloads-owned.
- `resume_partial` and `queue_remaining` map to future downloads-owned work items; `seal_completed` and `reject_mismatch` do not create scheduler work.
- Scoped `git diff --check` passed for AT-172 files with CRLF warnings only.
- AT-172 is ready to be included in the local docs/PWF commit.
- Resume point: start the minimal Rust slice for module-local `DownloadResumeWorkPlan` / `DownloadResumeWorkItem` derivation before any scheduler execution, transport, frontend, SQLite schema, or `kernel-jobs` payload changes.

## Notes

- AT-2026-05-16-171 projected `DownloadResumeOutcome` through host transport.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
