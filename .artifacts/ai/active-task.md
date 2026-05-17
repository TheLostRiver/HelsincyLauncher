# Active Atomic Task

## Identity

- task id: AT-2026-05-17-239
- title: Add downloads segment executor adapter shell
- status: completed

## Goal

Implement the README_IMPL 7.35 first Rust slice: a module-owned segment executor adapter shell behind `DownloadSegmentExecutionPort`, validated with fake/in-memory fetch/write/verify sub-ports, without real HTTP/disk/hash IO, composition-root production wiring, retry/backoff, terminal projection, host transport, frontend, or schema changes.

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
  - real staging file writes, artifact moves, or hash verification
  - composition-root production execution-port wiring
  - retry/backoff policy
  - terminal runtime completion/failure projection
  - scheduler loops/background tasks/timers
  - durable leases or precise active-slot accounting
  - host transport or frontend changes
  - SQLite schema or adapter changes
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

1. docs/modules/downloads/README_ARCH.md/API/FLOW relevant boundary snippets.
2. docs/modules/downloads/README_IMPL.md 7.35.
3. docs/TauriDownloadRuntimeDesign.md fetcher/writer/verifier/staging references.
4. docs/TauriKernelJobsRuntimeDesign.md driver/runtime context references.
5. Current `DownloadSegmentExecutionRequest`, `DownloadSegmentExecutionResult`, `DownloadSegmentExecutionPort`, and driver tests.

## Hypothesis

- falsifiable implementation hypothesis: a `DownloadSegmentExecutor` adapter can call fake fetch/write/verify sub-ports with the existing request facts and return an existing `DownloadSegmentExecutionResult::Completed` without changing driver request/result shape or production wiring.

## Cheap Check

1. Add RED driver test for adapter pass-through and completed result projection.
2. Implement minimal result structs, sub-port traits, and adapter.
3. Re-export the public adapter shell from `module-downloads`.
4. Run focused module tests, full module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test fails before production code because the executor adapter/sub-ports are missing.
2. GREEN focused test passes after implementation.
3. Existing driver run/deferred tests still pass.
4. Full `launcher-module-downloads` lib tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.
