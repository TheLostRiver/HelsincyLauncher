# Active Atomic Task

## Identity

- task id: AT-2026-05-17-224
- title: Document downloads driver runtime-run override boundary
- status: completed

## Goal

Define the safe downloads-owned boundary for overriding `JobDriver::run(...)` now that `kernel-jobs` has one-shot dispatch, and repair stale README_IMPL current-state wording that still described `JobDriver::run(...)` and runtime dispatch as absent.

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
  - concrete HTTP/file/hash execution
  - retry/backoff
  - durable lease persistence
  - terminal snapshot completion or runtime loop scheduling
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and collaboration guidance already refreshed this session.
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md sections 7.9-7.13 and 7.29-7.30.
3. docs/TauriKernelJobsRuntimeDesign.md driver/runtime-host/runtime-context sections.
4. docs/TauriDownloadRuntimeDesign.md ownership, scheduler, and checkpoint sections.
5. current `DownloadJobDriver` local execution helpers and current `SharedJobRuntimeHost::run_one_execution_turn(...)`.

## Hypothesis

- falsifiable local hypothesis: downloads should not override `run(...)` by directly calling `prepare_resume_execution_turn(...)` alone, because that can drain pending work without an execution port. The next code slice needs a documented optional segment execution port boundary or equivalent guard before any downloads `run(...)` override.

## Cheap Check

1. Update stale README_IMPL current-state wording around the earlier execution-turn sections.
2. Add a concise README_IMPL boundary section for the downloads driver `run(...)` override.
3. Run scoped `git diff --check` for the allowed files.

## Validation Gate

1. README_IMPL no longer leaves the reader believing `kernel-jobs::JobDriver::run(...)` or host dispatch are absent in current Rust.
2. The new boundary explicitly forbids draining pending work from `run(...)` unless an execution port path is present.
3. The next Rust slice is named tightly enough for TDD.

## Validation Result

- Updated stale README_IMPL execution-turn sections to distinguish pre-slice reality from current Rust state.
- Added README_IMPL 7.31 defining the downloads driver `run(...)` override boundary.
- Scoped `git diff --check` passed for the allowed file set with CRLF normalization warnings only.
