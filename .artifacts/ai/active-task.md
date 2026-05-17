# Active Atomic Task

## Identity

- task id: AT-2026-05-17-207
- title: Define downloads policy source boundary
- status: completed

## Goal

After `get_job_snapshot(...)` and `list_jobs(...)` landed, document the remaining downloads policy surface before coding.

The boundary must decide where `DownloadPolicyDto` comes from, whether `get_policy(...)` and `update_policy(...)` should land together or separately, and how this stays separate from runtime queue-policy mutation.

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
  - runtime queue policy mutation
  - SQLite schema or adapter changes
  - host transport or frontend changes
  - concrete IO
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
4. docs/modules/downloads/README_IMPL.md current query/policy sections
5. docs/TauriDownloadRuntimeDesign.md concurrency and policy sections
6. docs/TauriKernelJobsRuntimeDesign.md runtime queue policy section
7. docs/TauriStorageAndDatabaseDesign.md downloads policy storage note
8. current downloads policy DTO/request/query contracts
9. current downloads facade policy stubs

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow policy source boundary that introduces a downloads-owned policy store before coding, treats `DownloadPolicyDto` as user-facing policy snapshot, and defers runtime queue-policy mutation to a later runtime integration slice.

## Cheap Check

1. Add a README_IMPL section that defines the policy source of truth and first Rust slice.
2. Update PWF records so AT-206 is published as `d0ad61a` and AT-207 is active.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL defines a downloads-owned policy source of truth.
2. The first Rust slice is explicit and TDD-gated.
3. Runtime queue-policy mutation is out of scope.
4. SQLite schema/adapter changes, transport, frontend, concrete IO, retry/backoff, and terminal completion are out of scope.
5. Scoped `git diff --check` passes.
6. Commit only AT-207 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.24 now defines a downloads-owned policy store/port as the source of truth before coding policy methods.
2. The first Rust slice is explicitly TDD-gated around `get_policy(...)` and `update_policy(...)` store/readback semantics.
3. Runtime queue-policy mutation is out of scope.
4. SQLite schema/adapter changes, transport, frontend, concrete IO, retry/backoff, and terminal completion are out of scope.
5. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-206 final local commit is `d0ad61a` and was pushed to `origin/main`.
- AT-2026-05-17-207 initial local commit is `07bdcfb` before PWF hash backfill amend.
