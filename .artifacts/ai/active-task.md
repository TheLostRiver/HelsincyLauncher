# Active Atomic Task

## Identity

- task id: AT-2026-05-17-231
- title: Define one-shot queue policy slot gate boundary
- status: completed

## Goal

Define the next `kernel-jobs` boundary after one-shot queued selection: `run_next_execution_turn(...)` should start respecting `RuntimeQueuePolicy::max_concurrent_jobs` through a minimal snapshot-observed running-count gate, while scheduler loops, durable leases, precise active-slot accounting, fairness, terminal projection, and downloads IO remain out of scope.

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
  - downloads driver/concrete execution changes
  - composition-root wiring
  - durable leases
  - scheduler loops/background tasks
  - snapshot-writer/cancellation context
  - precise active-slot accounting or queue fairness beyond the snapshot-observed running-count gate
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

1. README.md and docs/README.md routing.
2. Module documentation budget and README_IMPL 7.33/current Rust state after AT-230.
3. docs/TauriKernelJobsRuntimeDesign.md queue policy and eligible-job selection notes.
4. docs/TauriTestingStrategyAndQualityGateDesign.md kernel-jobs test guidance.
5. docs/TauriDownloadRuntimeDesign.md concurrency budget notes.
6. current `JobSnapshotStore::list_resumable(...)` implementations.

## Hypothesis

- falsifiable documentation hypothesis: the next safe Rust slice is a minimal policy gate inside `run_next_execution_turn(...)` that counts current `Running` snapshots from the same resumable read and returns `Deferred` when that count is greater than or equal to `RuntimeQueuePolicy::max_concurrent_jobs`.

## Cheap Check

1. Update README_IMPL with a concise one-shot policy slot-gate boundary.
2. Keep task-log detail in `.artifacts/ai`.
3. Run scoped `git diff --check` for README_IMPL and PWF files.

## Validation Gate

1. README_IMPL states current AT-230 Rust reality.
2. README_IMPL defines the minimal running-count gate and no-capacity deferral semantics.
3. README_IMPL keeps scheduler loops, durable leases, precise active-slot accounting, fairness, terminal projection, downloads IO, transport, frontend, and SQLite schema out of scope.
4. Scoped `git diff --check` passes before commit.

## Validation Result

1. README_IMPL 7.34 now documents the one-shot queue policy slot gate boundary.
2. The section states current AT-230 Rust reality, defines the minimal `Running` snapshot count vs `max_concurrent_jobs` gate, and keeps scheduler loops, durable leases, precise active-slot accounting, fairness, terminal projection, downloads IO, transport, frontend, and SQLite schema out of scope.
3. Scoped `git diff --check` passed for README_IMPL and PWF files with CRLF normalization warnings only.
