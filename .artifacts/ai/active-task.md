# Active Atomic Task

## Identity

- task id: AT-2026-05-17-223
- title: Add one-shot kernel-jobs execution dispatch
- status: completed

## Goal

Add the smallest host-owned `kernel-jobs` execution dispatch surface so `SharedJobRuntimeHost` can load one job snapshot, resolve its driver, and call one `JobDriver::run(...)` turn through `JobExecutionContext`, without scheduler loops, leases, terminal state, downloads IO, transport, frontend, or SQLite schema changes.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - downloads driver override/integration
  - concrete HTTP/file/hash execution
  - retry/backoff
  - durable lease persistence
  - snapshot writer/cancellation context
  - terminal snapshot completion or runtime loop scheduling
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md, CONTRIBUTING.md, and docs/README.md routing guidance.
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md sections 7.29-7.30.
3. docs/TauriKernelJobsRuntimeDesign.md driver, runtime-host, runtime-context, lease, and first-slice sections.
4. docs/TauriDownloadRuntimeDesign.md scheduler/budget/ownership notes.
5. docs/TauriTestingStrategyAndQualityGateDesign.md job runtime test guidance.
6. current `crates/kernel-jobs/src/runtime.rs`, `lib.rs`, `model.rs`, and composition-root driver-registry wiring.

## Hypothesis

- falsifiable local hypothesis: a focused `launcher-kernel-jobs` RED test can prove the missing host-owned dispatch method by enqueuing a snapshot, registering a fake driver, and expecting exactly one `run(...)` call; missing snapshot and missing driver branches should return explicit deferred dispositions.

## Cheap Check

1. Add focused RED tests in `crates/kernel-jobs/src/runtime.rs`.
2. Run `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib execution_dispatch`.
3. Implement the minimal one-shot dispatch method.
4. Run focused and full `launcher-kernel-jobs` lib tests, affected composition check, scoped rustfmt, and scoped `git diff --check`.

## Validation Gate

1. A fake driver registered in `JobDriverRegistry` is called exactly once for the queued snapshot.
2. Missing job id and missing driver branches return `JobRunDisposition::Deferred` with diagnostic reasons.
3. Existing enqueue/snapshot/policy tests keep passing.
4. No downloads driver override, concrete IO, retry/backoff, leases, snapshot writer, terminal completion, transport, frontend, or SQLite schema changes.
5. Commit and push only AT-223 files.

## Validation Result

- RED validation failed as expected because `SharedJobRuntimeHost::run_one_execution_turn(...)` did not exist.
- Added `SharedJobRuntimeHost::run_one_execution_turn(...)` in `crates/kernel-jobs/src/runtime.rs`.
- Added focused tests proving the registered driver is called once and missing snapshot/driver branches defer explicitly.
- Updated README_IMPL 7.30 current Rust state.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib execution_dispatch` passed with 3 tests passed / 0 failed.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 7 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed.
- Scoped `git diff --check` passed with CRLF normalization warnings only.
