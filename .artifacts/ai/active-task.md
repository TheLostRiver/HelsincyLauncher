# Active Atomic Task

## Identity

- task id: AT-2026-05-17-209
- title: Define downloads policy SQLite persistence boundary
- status: validating

## Goal

Document the next downloads policy persistence slice before Rust coding, so `DownloadPolicyStore` can later gain a SQLite implementation without accidentally widening into runtime queue policy, global settings, host transport, frontend, or concrete download IO.

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
  - SQLite schema or adapter implementation
  - runtime queue-policy mutation
  - global settings/config-system implementation
  - host transport or frontend changes
  - concrete IO, retry/backoff, or terminal runtime completion

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. docs/modules/downloads/README_IMPL.md policy sections
2. docs/TauriStorageAndDatabaseDesign.md downloads policy and storage-placement sections
3. docs/TauriRepositoryPortsAndAdapterDesign.md port/adapter ownership sections
4. current `adapter-storage-sqlite` repository shapes
5. current `DownloadPolicyStore` / `InMemoryDownloadPolicyStore` surface
6. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow SQLite policy persistence boundary that maps the existing `DownloadPolicyStore` port to a future `SqliteDownloadPolicyStore`, uses project-local test database paths, and defers runtime policy application plus settings/transport/frontend work.

## Cheap Check

1. Add README_IMPL section 7.25 describing the policy persistence boundary.
2. Update PWF records and note AT-208 final pushed commit `6d8c022`.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL defines the SQLite policy store adapter boundary.
2. README_IMPL defines first Rust slice tests and says project-local test DB paths must be used.
3. Runtime queue-policy mutation, global settings, transport, frontend, concrete IO, retry/backoff, and terminal completion remain out of scope.
4. Scoped `git diff --check` passes.
5. Commit only AT-209 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.25 now defines `SqliteDownloadPolicyStore`, `download_policy_snapshot`, first tests, and deferred surfaces.
2. Scoped `git diff --check` passed with CRLF normalization warnings only.
3. Path-limited status confirms only AT-209 files are changed.

## Notes

- AT-2026-05-17-208 final commit is `6d8c022` and is already pushed to `origin/main`.
