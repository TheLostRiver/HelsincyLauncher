# Active Atomic Task

## Identity

- task id: AT-2026-05-18-244
- title: Define guarded downloads writer boundary
- status: completed

## Goal

Define the next durable downloads implementation boundary for applying `DownloadSegmentStagingTarget` inside a guarded writer sub-port before real disk IO or production wiring exists.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production or test code
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

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and documentation-budget rules.
2. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` around 7.35-7.37.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter/staging/fetch/verify sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership notes.
5. Current `DownloadSegmentHandledFailure`, `DownloadSegmentStagingTarget`, and writer sub-port contracts.

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can make the next code slice concrete enough to TDD a guarded `DownloadSegmentWritePort` wrapper that rejects unsafe targets before delegation, while keeping real disk IO and production wiring out of scope.

## Cheap Check

1. Add README_IMPL 7.38 describing the guarded writer boundary.
2. Record the next RED tests and non-goals clearly enough for AT-245.
3. Update PWF records and run scoped docs/PWF diff-check.

## Validation Gate

1. README_IMPL 7.38 exists and does not duplicate task-log detail.
2. Active task, task plan, findings, progress, and handoff identify AT-244 and the next Rust slice.
3. Scoped `git diff --check` passes before commit/push.
