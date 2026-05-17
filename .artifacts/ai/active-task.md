# Active Atomic Task

## Identity

- task id: AT-2026-05-17-205
- title: Define downloads list-jobs query boundary
- status: completed

## Goal

After AT-204 implemented `get_job_snapshot(...)`, document the next safe query slice for `DownloadsFacade::list_jobs(...)` before coding.

The boundary must choose an explicit read source and projection shape. It should not start policy storage, runtime list APIs, host transport changes, frontend work, concrete IO, retry/backoff, or terminal runtime completion.

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
  - `get_policy`
  - `update_policy`
  - runtime list API expansion
  - live runtime snapshot joining for list rows
  - host transport or frontend changes
  - concrete HTTP/file/hash execution
  - retry/backoff
  - terminal runtime completion
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
4. docs/modules/downloads/README_IMPL.md sections 7.22 and 9
5. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md downloads use-case table
6. docs/TauriRepositoryPortsAndAdapterDesign.md downloads repository boundary
7. docs/TauriKernelJobsRuntimeDesign.md runtime list design gap
8. docs/TauriDownloadRuntimeDesign.md downloads query contract section
9. current downloads contracts DTOs
10. current downloads facade and SQLite job repository surfaces

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow `list_jobs(...)` boundary that uses a downloads-owned job repository page as the first read source, maps records to conservative list items, and defers live runtime-list joining and policy surfaces to later slices.

## Cheap Check

1. Add a README_IMPL section that chooses the first `list_jobs(...)` read source and projection rules.
2. Record current Rust reality, first Rust slice, adapter impact, validation, and explicit exclusions.
3. Update PWF records so AT-204 is published as `2ccc436` and AT-205 is active.
4. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL names the first `list_jobs(...)` code slice and leaves policy surfaces for later.
2. The boundary requires a future RED test before Rust code.
3. The boundary explicitly states whether `DownloadJobRepository` or `JobRuntime` is the first read source.
4. The boundary defines conservative `DownloadJobListItemDto` projection rules.
5. The boundary forbids runtime list APIs, live snapshot joins, policy storage, host transport, frontend, concrete IO, retry/backoff, and terminal runtime completion.
6. Scoped `git diff --check` passes.
7. Commit only AT-205 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.23 now names `DownloadsFacade::list_jobs(...)` as the next code slice.
2. The first read source is explicitly the downloads-owned job repository page, not `JobRuntime`.
3. The boundary defines conservative `DownloadJobListItemDto` projection rules from `DownloadJobRecord`.
4. The boundary leaves `get_policy(...)` and `update_policy(...)` for later.
5. The boundary forbids runtime list APIs, live snapshot joins, policy storage, host transport, frontend, concrete IO, retry/backoff, and terminal runtime completion.
6. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-204 final local commit is `2ccc436` and was pushed to `origin/main`.
- AT-2026-05-17-205 initial local commit is `c66d3bb` before PWF hash backfill amend.
