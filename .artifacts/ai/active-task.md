# Active Atomic Task

## Identity

- task id: AT-2026-05-17-204
- title: Implement downloads get-job snapshot query
- status: completed

## Goal

Implement the first `list/get/policy surfaces` code slice defined by AT-203: `DownloadsFacade::get_job_snapshot(...)`.

The method should verify the downloads module record exists, read the shared runtime snapshot, and project a `DownloadJobSnapshotDto` with conservative downloads extension facts. It must leave `list_jobs`, `get_policy`, and `update_policy` as `DOWNLOADS_NOT_WIRED`.

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
  - `list_jobs`
  - `get_policy`
  - `update_policy`
  - runtime list APIs
  - policy storage
  - SQLite schema or adapter changes
  - host transport or frontend changes
  - concrete HTTP/file/hash execution
  - retry/backoff
  - terminal runtime completion
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md section 7.22 and validation section
8. docs/TauriDownloadRuntimeDesign.md query contract section
9. docs/TauriIPCAndStateContractsDesign.md downloads query catalog and query envelope section
10. docs/TauriAIDevelopmentTransactionProtocolDesign.md atomic-task protocol
11. current downloads contracts DTOs
12. current downloads facade stubs/test helpers
13. current kernel-jobs runtime snapshot surface
14. superpowers TDD skill

## Hypothesis

- falsifiable local hypothesis: focused facade tests can prove `get_job_snapshot(...)` returns a downloads-owned snapshot by composing `DownloadJobRepository::get_job(...)` with `JobRuntime::snapshot(...)`, and returns `DL_JOB_SNAPSHOT_MISSING` when the runtime snapshot is absent after the module record exists.

## Cheap Check

1. Add focused RED tests for successful snapshot projection and missing-runtime-snapshot error.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml get_job_snapshot`.
3. Implement only the facade method and smallest helper/error needed.
4. Update README_IMPL current state.
5. Run focused tests, full downloads module tests, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails on current `DOWNLOADS_NOT_WIRED` behavior.
2. GREEN returns `DownloadJobSnapshotDto` with runtime snapshot facts and downloads extension facts from the module job record.
3. Missing module record reuses `DL_JOB_NOT_FOUND`.
4. Missing runtime snapshot after a present module record returns `DL_JOB_SNAPSHOT_MISSING`.
5. `list_jobs`, `get_policy`, and `update_policy` remain `DOWNLOADS_NOT_WIRED`.
6. Public comments added or changed in code are bilingual and existing English comments are preserved.
7. Focused and full module tests pass.
8. Formatting and scoped diff checks pass.
9. Commit only AT-204 files locally, then push `main` to `origin`.

## Validation Result

1. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml get_job_snapshot` failed with 0 passed and 3 failed because the current facade stub returned `DOWNLOADS_NOT_WIRED`.
2. GREEN focused validation passed with 3 passed, 0 failed after implementing only `get_job_snapshot(...)`, `DL_JOB_SNAPSHOT_MISSING`, and the local snapshot projection helper.
3. Full downloads module validation passed after formatting: 41 passed, 0 failed.
4. `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed after applying rustfmt.
5. `list_jobs(...)`, `get_policy(...)`, and `update_policy(...)` remain `DOWNLOADS_NOT_WIRED`.
6. No runtime list API, policy storage, SQLite adapter/schema, host transport, frontend, concrete IO, retry/backoff, or terminal runtime completion behavior was added.
7. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-203 final local commit is `fb5a94e` and was pushed to `origin/main`.
- AT-2026-05-17-204 initial local commit is `d1de743` before PWF hash backfill amend.
