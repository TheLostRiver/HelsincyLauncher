# Active Atomic Task

## Identity

- task id: AT-2026-05-17-242
- title: Define downloads segment staging target guard boundary
- status: completed

## Goal

Document the next safe downloads Rust slice after executor success/failure mapping: a pure staging-relative write-target guard that can reject unsafe `write_target` values before any real file writes, without adding HTTP, disk IO, hash verification, production wiring, host transport, frontend, or public `DL_*` execution projection.

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

1. README/docs routing and documentation-budget rules already refreshed in this session.
2. docs/modules/downloads/README_IMPL.md 7.35 and 7.36.
3. docs/TauriDownloadRuntimeDesign.md SegmentWriter/staging/checkpoint/failure sections.
4. docs/TauriStorageAndDatabaseDesign.md storage placement for staging files.
5. Current `DownloadSegmentExecutionRequest.write_target` and writer sub-port shape.

## Hypothesis

- falsifiable documentation hypothesis: the first safe step toward real writer IO is a pure staging target guard that validates `write_target` as a relative, normalized, non-empty, non-escaping path before any file system side effect exists.

## Cheap Check

1. Add a compact README_IMPL subsection after 7.36.
2. Define the next Rust slice as pure write-target guard logic.
3. Keep real IO, production wiring, public error projection, and retry/terminal behavior out of scope.
4. Run scoped docs/PWF diff-check.

## Validation Gate

1. README_IMPL explicitly names the next code test target.
2. README_IMPL defines accepted/rejected staging target cases.
3. README_IMPL states whether rejection maps to local handled failure versus `AppError`.
4. Scoped docs/PWF diff-check passes before commit/push.
