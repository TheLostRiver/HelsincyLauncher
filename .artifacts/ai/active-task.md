# Active Atomic Task

## Identity

- task id: AT-2026-05-17-232
- title: Add one-shot queue policy slot gate
- status: completed

## Goal

Implement the documented `kernel-jobs` one-shot selector policy gate: `run_next_execution_turn(...)` should count current `Running` snapshots and defer queued dispatch when `RuntimeQueuePolicy::max_concurrent_jobs` has no remaining capacity, while scheduler loops, durable leases, precise active-slot accounting, fairness, terminal projection, and downloads IO remain out of scope.

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
  - precise active-slot accounting or queue fairness beyond the snapshot-observed running-count gate
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
2. README_IMPL 7.34.
3. docs/TauriKernelJobsRuntimeDesign.md queue policy and eligible-job selection notes.
4. docs/TauriTestingStrategyAndQualityGateDesign.md kernel-jobs test guidance.
5. docs/TauriDownloadRuntimeDesign.md concurrency budget notes.
6. current `SharedJobRuntimeHost::run_next_execution_turn(...)` and selector tests.

## Hypothesis

- falsifiable implementation hypothesis: `run_next_execution_turn(...)` can reuse the same `list_resumable()` read, count `Running` snapshots, defer when `running_count >= policy.max_concurrent_jobs`, and otherwise preserve current deterministic queued selection.

## Cheap Check

1. Add RED tests for capacity-exhausted deferral and remaining-capacity dispatch.
2. Implement the minimal running-count gate.
3. Run focused tests, full `launcher-kernel-jobs` lib tests, composition-root check, scoped rustfmt, and scoped diff check.

## Validation Gate

1. Focused capacity tests fail before implementation and pass after implementation.
2. Existing deterministic/no-queued selector tests keep passing.
3. Existing accepted/deferred dispatch behavior remains unchanged.
4. Verification commands pass before commit/push.

## Validation Result

1. RED: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml policy_capacity` failed as expected because capacity-exhausted and zero-capacity cases still returned `Accepted`.
2. GREEN focused: the same command passed with 3 tests passed / 0 failed.
3. Full package: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 12 tests passed / 0 failed.
4. Integration compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
5. Scoped format/checks: `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed; scoped `git diff --check` passed with CRLF normalization warnings only.
