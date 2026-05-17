# Active Atomic Task

## Identity

- task id: AT-2026-05-17-230
- title: Add one-shot queued execution selector
- status: completed

## Goal

Implement the documented `kernel-jobs` one-shot selector that chooses exactly one queued snapshot deterministically and delegates to existing execution dispatch, while full scheduler loops, leases, active-slot accounting, terminal projection, and downloads IO remain out of scope.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - durable docs updates beyond PWF records
  - downloads driver/concrete execution changes
  - composition-root wiring
  - durable leases
  - scheduler loops/background tasks
  - snapshot-writer/cancellation context
  - active-slot accounting or queue fairness beyond deterministic first queued selection
  - terminal completion/failure projection
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md routing.
2. Module documentation budget and README_IMPL 7.33.
3. docs/TauriKernelJobsRuntimeDesign.md queue policy and eligible-job selection notes.
4. docs/TauriTestingStrategyAndQualityGateDesign.md kernel-jobs test guidance.
5. current `SharedJobRuntimeHost::run_one_execution_turn(...)`.
6. current `JobSnapshotStore::list_resumable(...)` implementations.

## Hypothesis

- falsifiable implementation hypothesis: `run_next_execution_turn(...)` can use `list_resumable()`, filter `JobState::Queued`, sort by `(updated_at, job_id)`, and call `run_one_execution_turn(...)` without changing existing dispatch semantics.

## Cheap Check

1. Add RED tests in `launcher-kernel-jobs` proving deterministic queued selection and no-queued deferral.
2. Implement the smallest selector method inside `SharedJobRuntimeHost`.
3. Run focused tests, full `launcher-kernel-jobs` lib tests, composition-root check, scoped rustfmt, and scoped diff check.

## Validation Gate

1. Focused queued-selector tests fail before implementation and pass after implementation.
2. Existing accepted/deferred dispatch behavior remains unchanged.
3. No store ordering contract, scheduler loop, lease, active-slot accounting, downloads IO, transport, frontend, or SQLite schema changes are introduced.
4. Verification commands pass before commit/push.

## Validation Result

1. RED: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml next_execution_turn` failed with `E0599` because `run_next_execution_turn(...)` did not exist yet.
2. GREEN focused: the same command passed with 2 tests passed / 0 failed.
3. Full package: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 9 tests passed / 0 failed.
4. Integration compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
5. Scoped format/checks: `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed; scoped `git diff --check` passed with CRLF normalization warnings only.
