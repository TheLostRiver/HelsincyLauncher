# Active Atomic Task

## Identity

- task id: AT-2026-05-17-227
- task id: AT-2026-05-17-228
- task id: AT-2026-05-17-229
- title: Define one-shot queued execution selection boundary
- status: completed

## Goal

Define the next `kernel-jobs` boundary after accepted dispatch projection: a deterministic one-shot method can select the next queued snapshot and dispatch it, while full scheduler loops, leases, active-slot accounting, terminal projection, and downloads IO remain out of scope.

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
  - docs updates beyond PWF records
  - downloads driver/concrete execution changes
  - composition-root wiring
  - durable leases
  - scheduler loops/background tasks
  - snapshot-writer/cancellation context
  - active-slot accounting or queue fairness beyond deterministic first queued selection
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

1. README_IMPL 7.32 and current Rust state after AT-228.
2. docs/TauriKernelJobsRuntimeDesign.md queue policy and eligible-job selection notes.
3. docs/TauriTestingStrategyAndQualityGateDesign.md kernel-jobs test guidance.
4. current `SharedJobRuntimeHost::run_one_execution_turn(...)`.
5. current `JobSnapshotStore::list_resumable(...)` implementations.

## Hypothesis

- falsifiable documentation hypothesis: the next safe Rust slice is a one-shot queued selector, likely `run_next_execution_turn(...)`, which filters `Queued` snapshots from `list_resumable()`, applies deterministic ordering, dispatches one selected job, and leaves loops/leases/fairness/terminal/download IO for later.

## Cheap Check

1. Update README_IMPL with current AT-228 state and a concise queued selector boundary.
2. Run scoped `git diff --check` for README_IMPL and PWF files.
3. Commit and push before Rust changes.

## Validation Gate

1. The document states current Rust state after AT-228.
2. The document defines deterministic queued selection and explicit non-goals.
3. The document keeps task-log detail out of README_IMPL.
4. Scoped `git diff --check` passes before commit.

## Validation Result

1. `docs/modules/downloads/README_IMPL.md` now includes section 7.33 for one-shot queued execution selection.
2. The section requires deterministic `(updated_at, job_id)` selection from queued snapshots and keeps loops, leases, active-slot accounting, fairness, terminal projection, downloads IO, transport, frontend, and SQLite schema out of scope.
3. Scoped `git diff --check` passed with CRLF normalization warnings only.
