# Active Atomic Task

## Identity

- task id: AT-2026-05-17-227
- task id: AT-2026-05-17-228
- title: Project accepted execution dispatch to running state
- status: completed

## Goal

Implement README_IMPL 7.32 in `kernel-jobs`: when `SharedJobRuntimeHost::run_one_execution_turn(...)` receives `JobRunDisposition::Accepted`, update the stored snapshot to non-terminal `JobState::Running` and `JobUiState::Running`; keep deferred dispatch non-mutating.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - docs updates beyond PWF records
  - downloads driver or concrete execution changes
  - composition-root execution-port wiring
  - durable leases
  - scheduler loops/background tasks
  - snapshot-writer/cancellation context
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

1. README_IMPL 7.32.
2. docs/TauriKernelJobsRuntimeDesign.md lifecycle, driver, context, and snapshot rules.
3. docs/TauriTestingStrategyAndQualityGateDesign.md kernel-jobs test guidance.
4. current `SharedJobRuntimeHost::run_one_execution_turn(...)` dispatch tests.
5. current `JobRunDisposition` and `JobSnapshotStore` contracts.

## Hypothesis

- falsifiable RED/GREEN hypothesis: changing focused dispatch tests to expect accepted execution to project `Running` should fail first because dispatch currently leaves snapshots `Queued`; a minimal runtime update can make accepted dispatch mutate to `Running` while deferred paths stay unchanged.

## Cheap Check

1. Update focused `kernel-jobs` dispatch tests for accepted and deferred projection behavior.
2. Run `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml dispatch` and observe the RED failure.
3. Implement the smallest runtime update.
4. Run focused dispatch tests, full `launcher-kernel-jobs` lib tests, composition-root check, scoped rustfmt, and scoped diff check.

## Validation Gate

1. Accepted dispatch returns `Accepted` and updates the stored snapshot to `Running` / UI `Running`.
2. Missing driver deferred dispatch keeps the queued snapshot unchanged.
3. Missing snapshot deferred dispatch still returns deferred without mutation.
4. Existing runtime policy/enqueue/default-run tests keep passing.

## Validation Result

1. RED: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml dispatch` failed because accepted dispatch left the snapshot `Queued`.
2. GREEN: the same dispatch filter passed, 3 passed / 0 failed.
3. `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed, 7 passed / 0 failed.
4. `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
5. `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed.
6. Scoped `git diff --check` passed with CRLF normalization warnings only.
