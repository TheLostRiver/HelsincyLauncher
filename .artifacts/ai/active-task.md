# Active Atomic Task

## Identity

- task id: AT-2026-05-17-206
- title: Implement downloads list-jobs query
- status: completed

## Goal

Implement `DownloadsFacade::list_jobs(...)` using the AT-205 repository-backed list boundary.

This slice should add a downloads-owned repository page method, project conservative `DownloadJobListItemDto` rows, and keep policy surfaces, runtime list APIs, live snapshot joins, transport, frontend, concrete IO, retry/backoff, and terminal runtime completion out of scope.

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `get_policy`
  - `update_policy`
  - runtime list APIs
  - live runtime snapshot joins
  - host transport or frontend changes
  - SQLite schema changes
  - concrete HTTP/file/hash execution
  - retry/backoff
  - terminal runtime completion
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/adapter-storage-sqlite/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_IMPL.md section 7.23 and validation section
5. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md downloads use-case table
6. docs/TauriRepositoryPortsAndAdapterDesign.md downloads repository boundary
7. docs/TauriKernelJobsRuntimeDesign.md runtime list gap
8. docs/TauriDownloadRuntimeDesign.md downloads query contract section
9. current downloads contracts DTOs
10. current downloads facade/test helper surfaces
11. current SQLite download job repository surface
12. superpowers TDD skill

## Hypothesis

- falsifiable local hypothesis: focused facade tests can prove `list_jobs(...)` maps repository records into conservative list DTO rows and applies optional `ui_state` filtering through the repository page method, without adding runtime list APIs or policy surfaces.

## Cheap Check

1. Add focused RED tests for `list_jobs(...)` projection and `ui_state` filtering.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml list_jobs`.
3. Implement the repository page method, facade projection helper, in-memory test repository behavior, and SQLite adapter compile support.
4. Update README_IMPL current state.
5. Run focused tests, full downloads module tests, `cargo check -p launcher-adapter-storage-sqlite`, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails on current `DOWNLOADS_NOT_WIRED` behavior.
2. GREEN returns conservative `DownloadJobListItemDto` rows from module job records.
3. Optional `ui_state` filtering is covered.
4. `get_policy(...)` and `update_policy(...)` remain `DOWNLOADS_NOT_WIRED`.
5. No runtime list API, live snapshot join, policy storage, host transport, frontend, concrete IO, retry/backoff, or terminal runtime completion is added.
6. Focused and full module tests pass.
7. SQLite adapter compiles after trait expansion.
8. Formatting and scoped diff checks pass.
9. Commit only AT-206 files locally, then push `main` to `origin`.

## Validation Result

1. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml list_jobs` failed with 0 passed and 2 failed because the current facade stub returned `DOWNLOADS_NOT_WIRED`.
2. GREEN focused validation passed with 2 passed, 0 failed after adding the repository page method, conservative facade projection, in-memory test repository behavior, and SQLite adapter compile support.
3. Full downloads module validation passed: 43 passed, 0 failed.
4. SQLite adapter compile validation passed: `cargo check -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
5. Formatting validation passed: `cargo fmt -p launcher-module-downloads -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check`.
6. `get_policy(...)` and `update_policy(...)` remain `DOWNLOADS_NOT_WIRED`.
7. No runtime list API, live snapshot join, policy storage, host transport, frontend, concrete IO, retry/backoff, or terminal runtime completion was added.
8. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-205 final local commit is `17e0bb4` and was pushed to `origin/main`.
- AT-2026-05-17-206 initial local commit is `87b09ab` before PWF hash backfill amend.
