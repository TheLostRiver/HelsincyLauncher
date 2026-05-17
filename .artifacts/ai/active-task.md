# Active Atomic Task

## Identity

- task id: AT-2026-05-17-201
- title: Define downloads fake local mixed-result checkpoint orchestration boundary
- status: completed

## Goal

After AT-200 added local failed-result checkpoint mutation, define the next orchestration boundary before coding: `execute_local_resume_turn(...)` should eventually reconcile both completed and failed fake segment results through existing checkpoint mutation helpers.

This is a documentation/planning slice. It must not add Rust code, retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema work, composition-root, transport, or frontend behavior.

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
  - terminal runtime job state
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
4. docs/modules/downloads/README_IMPL.md sections 7.18 through 7.20
5. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
6. current `crates/module-downloads/src/driver.rs` orchestration and checkpoint helper snippets

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can define a narrow mixed-result orchestration slice where `execute_local_resume_turn(...)` records both completed and failed fake results by chaining existing helpers, while avoiding retry/backoff, public error projection, terminal runtime state, concrete IO, and runtime integration.

## Cheap Check

1. Update README_IMPL with a new fake local mixed-result checkpoint orchestration boundary.
2. Mark the current-state table so the next Rust slice is explicit and constrained.
3. Run scoped `git diff --check` on touched files.
4. Commit and push only AT-201 files.

## Validation Gate

1. README_IMPL explains that the next Rust slice may have `execute_local_resume_turn(...)` call both completed and failed checkpoint mutation helpers.
2. README_IMPL explicitly defers retry/backoff, public `DL_*` projection, terminal job state, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend changes.
3. Scoped `git diff --check` passes.
4. Commit only AT-201 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.21 defines the fake local mixed-result checkpoint orchestration boundary and first Rust slice.
2. The boundary keeps retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend work out of AT-201.
3. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-200 final local commit is `c973da9` and was pushed to `origin/main`.
- AT-2026-05-17-201 initial local commit created as `9f6402a`; PWF backfill is amended into the same task commit.
