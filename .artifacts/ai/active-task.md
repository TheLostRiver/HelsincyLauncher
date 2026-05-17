# Active Atomic Task

## Identity

- task id: AT-2026-05-17-203
- title: Define downloads get-job snapshot query boundary
- status: completed

## Goal

After AT-202 finished fake local mixed-result checkpoint orchestration, document the next safe `list/get/policy surfaces` slice before coding.

The first slice should be `DownloadsFacade::get_job_snapshot(...)` because it can compose existing module job records with the existing shared runtime `snapshot(...)` query. It must not start list pagination, policy persistence, runtime list APIs, SQLite adapter work, transport changes, frontend changes, or concrete execution work.

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
  - Rust tests
  - `list_jobs`
  - `get_policy`
  - `update_policy`
  - runtime list/query API expansion
  - SQLite schema or adapter changes
  - host transport or frontend changes
  - retry/backoff, public segment execution projection, terminal runtime completion, or concrete IO
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md current-state table and sections 7.21 / 8 / 9
8. docs/TauriDownloadRuntimeDesign.md query contract section
9. docs/TauriIPCAndStateContractsDesign.md downloads query catalog and query envelope section
10. docs/TauriAIDevelopmentTransactionProtocolDesign.md atomic-task protocol
11. current `crates/module-downloads/src/contracts/*` query/read-model DTOs
12. current `crates/module-downloads/src/facade/mod.rs` list/get/policy stubs
13. current `crates/kernel-jobs/src/runtime.rs` snapshot query surface

## Hypothesis

- falsifiable local hypothesis: the implementation docs can define a narrow `get_job_snapshot` query boundary that reuses existing module job lookup and runtime snapshot lookup without requiring list pagination, policy storage, adapter schema changes, transport changes, frontend work, or concrete segment execution.

## Cheap Check

1. Add a README_IMPL section that chooses `get_job_snapshot` as the first query surface slice.
2. Record current Rust reality, first Rust slice, error semantics, validation, and explicit exclusions.
3. Update PWF records so AT-202 is published as `043f3f7` and AT-203 is the active docs task.
4. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL names `get_job_snapshot` as the next code slice and leaves `list_jobs` / policy surfaces for later.
2. The boundary requires a future RED test before Rust code.
3. The boundary uses existing `DownloadJobRepository::get_job(...)` and `JobRuntime::snapshot(...)`.
4. The boundary explicitly forbids runtime list API expansion, SQLite adapter/schema work, transport/frontend changes, concrete IO, and terminal runtime completion.
5. Scoped `git diff --check` passes.
6. Commit only AT-203 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.22 now chooses `DownloadsFacade::get_job_snapshot(...)` as the first `list/get/policy surfaces` code slice.
2. The documented boundary requires a future RED test before Rust code and keeps `list_jobs`, `get_policy`, and `update_policy` out of scope.
3. The boundary uses existing `DownloadJobRepository::get_job(...)` plus `JobRuntime::snapshot(...)` and reserves `DL_JOB_SNAPSHOT_MISSING` for the missing-runtime-snapshot branch.
4. Scoped `git diff --check` passed for the AT-203 files with CRLF normalization warnings only.
5. No Rust code, runtime APIs, adapters, transport handlers, frontend files, concrete IO, retry/backoff, or terminal runtime completion behavior were changed.

## Notes

- AT-2026-05-17-202 final local commit is `043f3f7` and was pushed to `origin/main`.
- AT-2026-05-17-203 initial local commit is `98c491b` before PWF hash backfill amend.
