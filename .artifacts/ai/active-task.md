# Active Atomic Task

## Identity

- task id: AT-2026-05-19-248
- title: Define filesystem staging writer boundary
- status: completed

## Goal

Define the first concrete filesystem staging writer boundary behind `DownloadSegmentWritePort` before adding disk-writing Rust code.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
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

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and documentation-budget rules.
2. `docs/modules/downloads/README_IMPL.md` roadmap and 7.35-7.38.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter, sparse/partial writes, staging-first, and cleanup sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership and SQLite boundary notes.
5. Existing `DownloadSegmentStagingTarget`, `DownloadSegmentWritePort`, and `DownloadResumeWorkMode` code.

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can make the first concrete filesystem writer slice specific enough to TDD path scoping, parent directory creation, from-start writes, and partial offset writes without adding production wiring or public error projection.

## Cheap Check

1. Add README_IMPL 7.39 for the filesystem staging writer boundary.
2. Record accepted behavior, non-goals, and next RED tests.
3. Update PWF records.
4. Run scoped docs/PWF diff-check and PWF completeness check.

## Validation Gate

1. README_IMPL 7.39 identifies path scoping, write facts, partial-write behavior, and non-goals.
2. PWF records identify AT-247 and leave all phases complete.
3. Scoped diff-check passes before commit/push.
