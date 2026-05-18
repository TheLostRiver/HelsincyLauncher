# Active Atomic Task

## Identity

- task id: AT-2026-05-18-246
- title: Cover guarded writer through executor failure mapping
- status: completed

## Goal

Add focused TDD coverage proving `DownloadSegmentExecutor` maps a guarded writer unsafe-target failure into the existing `DownloadSegmentExecutionResult::Failed` path without delegating to the wrapped writer.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - real HTTP range requests or provider object fetches
  - real staging writes, directory creation, file opening, artifact moves, or hash verification
  - host file-system canonicalization
  - composition-root production execution-port wiring
  - retry/backoff policy
  - terminal runtime completion/failure projection
  - host transport, frontend, or SQLite schema changes
  - public `DL_*` execution error DTOs
  - unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and documentation-budget rules.
2. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` around 7.36-7.38.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter/staging/fetch/verify sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership notes.
5. Current `DownloadSegmentExecutor`, `DownloadSegmentGuardedWritePort`, and writer sub-port tests.

## Hypothesis

- falsifiable implementation hypothesis: when `DownloadSegmentExecutor` uses `DownloadSegmentGuardedWritePort`, an unsafe request target returns `DownloadSegmentExecutionResult::Failed`, the wrapped writer is not called, and the verifier is not reached.

## Cheap Check

1. Add a RED test using `DownloadSegmentExecutor` with `DownloadSegmentGuardedWritePort`.
2. Confirm the test fails before any production adjustment because the combined behavior is not covered or not wired as expected.
3. Add only the minimal production adjustment if the RED test exposes a behavior gap.
4. Run focused executor/guarded writer tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test is observed before production adjustment.
2. GREEN focused executor/guarded writer tests pass.
3. Full `launcher-module-downloads` lib tests and composition-root compile gate pass.
4. Scoped rustfmt and diff-check pass before commit/push.
