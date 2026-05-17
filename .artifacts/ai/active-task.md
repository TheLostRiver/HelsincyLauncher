# Active Atomic Task

## Identity

- task id: AT-2026-05-17-199
- title: Define downloads fake failed-result checkpoint mutation boundary
- status: completed

## Goal

After AT-198 added a module-local failed segment result contract, define the next checkpoint mutation boundary before coding: how `DownloadJobDriver` should eventually persist failed segment checkpoint facts without widening into retry policy, public error projection, runtime terminal state, concrete IO, or adapter/schema changes.

This is a planning/documentation slice. It prepares the next Rust task and keeps implementation scope narrow.

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
  - retry/backoff policy
  - public `DL_*` execution error projection
  - concrete HTTP range requests or provider object fetch
  - staging file writes, artifact moves, or hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before editing:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_IMPL.md sections 7.17 through 7.19 and section 8
5. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
6. docs/TauriErrorHandlingAndProjectionDesign.md retry/public-error snippets
7. docs/TauriKernelJobsRuntimeDesign.md runtime ownership snippets

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can define a failed-result checkpoint mutation slice that records durable segment status/progress while keeping failure reason/retryable policy local and non-public until a later error projection design.

## Cheap Check

1. Update README_IMPL with a new failed-result checkpoint mutation boundary section.
2. Mark the current-state table so the next Rust slice is explicit and constrained.
3. Run scoped `git diff --check` on touched files.
4. Commit and push only AT-199 files.

## Validation Gate

1. README_IMPL explains that the next Rust slice may persist `Failed` segment status and downloaded bytes through the existing checkpoint repository.
2. README_IMPL explicitly defers public `DL_*` projection, retry/backoff, terminal job state, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend changes.
3. Scoped `git diff --check` passes.
4. Commit only AT-199 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.20 defines the fake failed-result checkpoint mutation boundary and first Rust slice.
2. The boundary keeps retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend work out of AT-199.
3. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-198 final local commit is `89f5a06` and was pushed to `origin/main`.
- AT-2026-05-17-199 initial local commit created as `fa71553`; PWF backfill is amended into the same task commit.
