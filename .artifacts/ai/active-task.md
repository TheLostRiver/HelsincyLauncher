# Active Atomic Task

## Identity

- task id: AT-2026-05-17-197
- title: Define downloads fake segment failure result boundary
- status: completed

## Goal

After AT-196 proved one fake/local resume turn can flow end to end through the downloads driver helpers, define the next implementation boundary before coding: a module-local fake segment failure result contract.

This is a planning/documentation slice. It prepares the next Rust task without adding a public `DL_*` execution error surface, runtime `run()`, concrete HTTP, staging writes, verification, SQLite adapter work, transport, composition-root, or frontend behavior.

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
  - concrete HTTP range requests or provider object fetch
  - staging file writes, artifact moves, or hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - stable public `DL_*` execution error projection
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
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md sections 7.18 through 9
8. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
9. docs/TauriKernelJobsRuntimeDesign.md runtime ownership snippets
10. docs/TauriErrorHandlingAndProjectionDesign.md retryable/severity/public-error snippets
11. docs/TauriTestingStrategyAndQualityGateDesign.md module test snippets

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can define a narrow fake/local failed segment result contract that is useful for the next TDD slice while preserving the boundary that public `DL_*` execution errors and runtime failure projection remain future work.

## Cheap Check

1. Update README_IMPL with a new fake segment failure result boundary section.
2. Mark current implementation table so the next Rust slice is explicit and constrained.
3. Run scoped `git diff --check` on touched files.
4. Commit and push only AT-197 files.

## Validation Gate

1. README_IMPL explains why the next code slice should add a local failed result contract before checkpoint mutation or concrete IO.
2. README_IMPL explicitly forbids public `DL_*` execution errors, runtime completion, concrete IO, SQLite adapter/schema, transport, and frontend work for this slice.
3. Scoped `git diff --check` passes.
4. Commit only AT-197 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.19 now defines a module-local fake failed segment result boundary and first Rust slice.
2. The boundary keeps public `DL_*` execution codes, `AppErrorDto`, retry/backoff, checkpoint mutation, runtime snapshots, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend behavior out of AT-197.
3. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-196 final local commit is `9294f9d` and was pushed to `origin/main`.
- AT-2026-05-17-197 initial local commit created as `83315bf`; PWF backfill is amended into the same task commit.
