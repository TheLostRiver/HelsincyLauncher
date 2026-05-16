# Active Atomic Task

## Identity

- task id: AT-2026-05-16-174
- title: Document downloads resume scheduler boundary
- status: completed

## Goal

在继续写 scheduler/driver 代码前，先把 `DownloadResumeWorkPlan` 的消费者边界写清楚：谁拥有调度端口、`resume_download_outcome()` 何时生成并交付 work plan、调度失败如何阻止 runtime enqueue，以及哪些内容仍不能进入 `kernel-jobs`、host transport、frontend、SQLite schema 或具体 fetch/write/verify 执行。

本轮是实现文档切片，属于用户要求的“任务不确定就先规划好实现文档再编码”：

- define the module-owned scheduler/driver port boundary
- define call order relative to work-plan derivation and job-level runtime enqueue
- define failure behavior before runtime enqueue
- define the next minimal TDD Rust slice
- keep Rust production code out of scope for AT-174

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

- falsifiable local hypothesis: a docs-only implementation guide slice can make the scheduler/driver boundary precise enough that the next Rust task can add a TDD-backed scheduler port and call it before runtime enqueue without touching concrete execution, persistence, host transport, frontend, or `kernel-jobs` payloads.

## Cheap Check

- update README_IMPL, then run scoped `git diff --check` and read back the scheduler/driver boundary wording.

## Validation Gate

1. Read required root, docs index, downloads module, runtime, kernel-jobs, crate layout/API, testing, and collaboration docs before editing README_IMPL.
2. Update README_IMPL with the scheduler/driver boundary, call order, failure behavior, out-of-scope list, and next Rust slice.
3. Record findings and recovery state in `.artifacts/ai`.
4. Run scoped `git diff --check` for AT-174 files.
5. Commit the docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required root, docs index, downloads module, runtime, kernel-jobs, crate layout/API, testing, and collaboration docs were read in scoped snippets before editing README_IMPL.
- README_IMPL now defines the downloads-owned `DownloadResumeWorkScheduler` boundary and its `schedule_resume_work(&self, job_id, plan)` method.
- README_IMPL defines the facade call order: reject mismatch and all-sealed before scheduling; build/schedule work plan before shared job runtime enqueue.
- README_IMPL defines failure behavior: scheduler errors return from `resume_download_outcome()` and must skip runtime enqueue.
- Scoped `git diff --check` passed for AT-174 files with CRLF warnings only.
- Runtime production code, frontend, host transport, SQLite schema, concrete scheduler execution, and `kernel-jobs` payloads remained unchanged.

## Notes

- AT-2026-05-16-172 documented the scheduler/driver payload boundary in README_IMPL section 7.6.
- AT-2026-05-16-173 committed local work-plan derivation as `1a698f9`.
- User approved four consecutive tasks without intermediate confirmation; proceed directly when the next task is determined.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: start the next Rust TDD slice by adding `DownloadResumeWorkScheduler`, wiring it into `DownloadModuleDeps`, and proving `resume_download_outcome()` schedules the derived work plan before runtime enqueue.
