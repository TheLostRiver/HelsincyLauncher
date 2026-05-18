# Active Atomic Task

## Identity

- task id: AT-2026-05-19-247
- title: Define downloads completion roadmap
- status: completed

## Goal

Record the remaining downloads backend implementation order in README_IMPL so subsequent coding slices can proceed without mixing filesystem writer, verifier, fetcher, composition wiring, runtime projection, transport, or frontend work.

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
2. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` current state.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter/staging/fetch/verify sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership notes.
5. `docs/TauriErrorHandlingAndProjectionDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, and composition-root wiring rules.

## Hypothesis

- falsifiable documentation hypothesis: README_IMPL can define a clear completion roadmap that lets the next coding slice start with filesystem staging writer work without prematurely adding production wiring, retry/backoff, public error projection, host transport, or frontend changes.

## Cheap Check

1. Update the README_IMPL port-status table for current segment execution sub-ports.
2. Add a concise completion roadmap with stable implementation order and boundaries.
3. Update PWF records.
4. Run scoped docs/PWF diff-check and PWF completeness check.

## Validation Gate

1. README_IMPL identifies current segment sub-port status and next implementation order.
2. PWF records identify AT-247 and leave all phases complete.
3. Scoped diff-check passes before commit/push.
