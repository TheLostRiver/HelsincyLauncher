# Active Atomic Task

## Identity

- task id: AT-2026-05-17-227
- title: Define accepted execution state projection boundary
- status: completed

## Goal

Define the next durable backend boundary after one-shot dispatch: when a registered driver returns `JobRunDisposition::Accepted`, the shared runtime may project the queued snapshot to non-terminal `Running` state, while `Deferred` remains non-mutating and terminal completion/failure stays out of scope.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust code changes
  - downloads concrete execution or composition-root execution-port wiring
  - durable leases
  - scheduler loops/background tasks
  - terminal completion/failure projection
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

1. README.md and CONTRIBUTING.md routing/validation rules.
2. docs/modules/downloads/README_IMPL.md sections 7.29-7.31.
3. docs/TauriKernelJobsRuntimeDesign.md lifecycle, driver, context, and snapshot rules.
4. docs/TauriCompositionRootWiringDesign.md runtime/driver wiring boundaries.
5. docs/TauriTestingStrategyAndQualityGateDesign.md backend test placement rules.
6. current `SharedJobRuntimeHost::run_one_execution_turn(...)` and `DownloadJobDriver::run(...)`.

## Hypothesis

- falsifiable documentation hypothesis: the next safe Rust slice is a `kernel-jobs` state projection step where `Accepted` maps to non-terminal `Running`, `Deferred` stays non-mutating, and terminal state/retry/lease/download IO remain later boundaries.

## Cheap Check

1. Add a concise README_IMPL section defining the accepted dispatch projection boundary.
2. Run scoped `git diff --check` for README_IMPL and PWF files.
3. Commit and push AT-227 before any Rust implementation.

## Validation Gate

1. The document states current Rust reality after AT-226.
2. The document defines the first Rust slice and explicit non-goals.
3. The document avoids task-log detail and keeps `.artifacts/ai` as the task record surface.
4. Scoped `git diff --check` passes before commit.

## Validation Result

1. `docs/modules/downloads/README_IMPL.md` now includes section 7.32 for accepted execution state projection.
2. The section defines `Accepted -> Running`, keeps `Deferred` non-mutating, and leaves terminal state, leases, scheduler loops, downloads IO, host transport, frontend, and SQLite schema out of scope.
3. Scoped `git diff --check` passed with CRLF normalization warnings only.
