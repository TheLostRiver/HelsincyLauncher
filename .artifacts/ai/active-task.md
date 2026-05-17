# Active Atomic Task

## Identity

- task id: AT-2026-05-17-241
- title: Add downloads segment executor handled failure mapping
- status: completed

## Goal

Implement README_IMPL 7.36: let `DownloadSegmentExecutor` map fake/in-memory handled fetch/write/verify sub-port failures into the existing module-local `DownloadSegmentExecutionResult::Failed`, while preserving propagated `AppError` for infrastructure/configuration failures.

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
  - real staging writes, artifact moves, or hash verification
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

1. README.md and docs/README.md routing.
2. docs/ModuleDocumentationStandard.md documentation-budget rules.
3. docs/modules/downloads/README_ARCH.md/API/FLOW relevant boundaries.
4. docs/modules/downloads/README_IMPL.md 7.35 and 7.36.
5. docs/TauriErrorHandlingAndProjectionDesign.md `AppError` versus public projection rules.
6. Current `DownloadSegmentExecutor` and adapter tests.

## Hypothesis

- falsifiable implementation hypothesis: a small module-local handled-failure outcome can let fake write/verify sub-ports return `Failed` through `DownloadSegmentExecutor` without changing the driver-facing `DownloadSegmentExecutionResult` shape or production composition wiring.

## Cheap Check

1. Add RED tests for handled write/verify failure mapping and infrastructure `AppError` propagation.
2. Add minimal handled-failure/outcome structs or enums behind the sub-port traits.
3. Keep the successful adapter test green.
4. Run focused adapter tests, failed-result checkpoint tests, full module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test fails before production code because handled-failure outcome types/mapping are missing.
2. GREEN focused tests pass after implementation.
3. Existing failed-result checkpoint tests still pass.
4. Full `launcher-module-downloads` lib tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.
