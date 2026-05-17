# Active Atomic Task

## Identity

- task id: AT-2026-05-17-243
- title: Add downloads staging target guard
- status: completed

## Goal

Implement README_IMPL 7.37 as a pure Rust guard for staging-relative `write_target` values, rejecting unsafe targets as module-local handled segment failures without touching the file system or adding real writer IO.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
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
2. crates/module-downloads/src/lib.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and documentation-budget rules from this session.
2. docs/modules/downloads/README_IMPL.md 7.37.
3. docs/TauriDownloadRuntimeDesign.md SegmentWriter/staging sections.
4. docs/TauriStorageAndDatabaseDesign.md staging file ownership notes.
5. Current `DownloadSegmentHandledFailure` and writer sub-port contracts.

## Hypothesis

- falsifiable implementation hypothesis: a pure `DownloadSegmentStagingTarget` guard can accept normal relative target components and reject unsafe target strings as `DownloadSegmentHandledFailure { downloaded_bytes: 0, retryable: false, ... }` without file-system side effects.

## Cheap Check

1. Add RED tests for accepted relative target and rejected unsafe targets.
2. Implement the smallest guard using path component inspection only.
3. Re-export the guard type if it is a module-owned extension point for future writer sub-ports.
4. Run focused guard tests, focused adapter tests, full module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test fails before production code because the guard type is missing.
2. GREEN focused guard tests pass after implementation.
3. Existing adapter tests still pass.
4. Full `launcher-module-downloads` lib tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.
