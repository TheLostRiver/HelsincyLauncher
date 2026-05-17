# Active Atomic Task

## Identity

- task id: AT-2026-05-17-240
- title: Define downloads segment executor failure mapping boundary
- status: completed

## Goal

Document the next downloads-owned boundary after `DownloadSegmentExecutor`: how fake/concrete fetch/write/verify sub-port failures should map into module-local `DownloadSegmentExecutionResult::Failed` versus propagated infrastructure `AppError`, without introducing public `DL_*` execution errors or production wiring yet.

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
  - real HTTP range requests or provider object fetches
  - real staging writes, artifact moves, or hash verification
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

1. README.md and docs/README.md routing.
2. docs/ModuleDocumentationStandard.md documentation-budget rules.
3. docs/modules/downloads/README_ARCH.md/API/FLOW relevant boundaries.
4. docs/modules/downloads/README_IMPL.md 7.35 and error semantics.
5. docs/TauriErrorHandlingAndProjectionDesign.md `AppError`, `retryable`, and public projection rules.
6. docs/TauriDownloadRuntimeDesign.md download error taxonomy notes.

## Hypothesis

- falsifiable documentation hypothesis: the next safe code slice can be made precise by separating handled segment execution failures (`Failed` result with local reason/retryable/downloaded bytes) from infrastructure/configuration errors that still propagate as `AppError`.

## Cheap Check

1. Add a compact README_IMPL subsection after 7.35.
2. Define the next Rust slice as fake sub-port failure mapping only.
3. Keep public `DL_*` execution projection, retry/backoff, terminal runtime state, and production wiring out of scope.
4. Run scoped docs/PWF diff-check.

## Validation Gate

1. README_IMPL explicitly names the next code test target.
2. README_IMPL distinguishes in-band `Failed` from propagated `AppError`.
3. README_IMPL keeps concrete IO, production wiring, retry/backoff, terminal runtime projection, public DTOs, and frontend out of scope.
4. Scoped docs/PWF diff-check passes before commit/push.
