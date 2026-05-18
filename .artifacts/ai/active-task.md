# Active Atomic Task

## Identity

- task id: AT-2026-05-18-245
- title: Add downloads guarded writer port
- status: completed

## Goal

Implement README_IMPL 7.38 as a minimal module-owned guarded writer wrapper behind `DownloadSegmentWritePort`, rejecting unsafe `write_target` values before delegation and preserving safe delegation behavior.

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

1. README/docs routing and documentation-budget rules.
2. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` around 7.35-7.38.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter/staging/fetch/verify sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership notes.
5. Current `DownloadSegmentHandledFailure`, `DownloadSegmentStagingTarget`, and writer sub-port contracts.

## Hypothesis

- falsifiable implementation hypothesis: a small guarded writer wrapper can parse `request.write_target` before calling the inner writer, return `DownloadSegmentWriteOutcome::Failed` for unsafe targets, delegate safe targets exactly once, and propagate inner `AppError` values unchanged.

## Cheap Check

1. Add RED tests for unsafe target rejection without delegation, safe target delegation, and inner `AppError` propagation.
2. Implement the smallest guarded writer wrapper around `DownloadSegmentWritePort`.
3. Re-export the wrapper if it is a module-owned extension point.
4. Run focused guarded writer tests, focused adapter tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED tests fail before production code because the guarded writer type is missing.
2. GREEN focused guarded writer tests pass after implementation.
3. Existing executor adapter tests still pass.
4. Full `launcher-module-downloads` lib tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.
