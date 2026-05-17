# Active Atomic Task

## Identity

- task id: AT-2026-05-17-237
- title: Cover host runtime command downloads deferred path
- status: completed

## Goal

Add focused host transport smoke coverage proving that `jobs_run_next_execution_turn` can see a queued production downloads job, returns a successful deferred DTO while the production downloads driver has no execution port, and leaves the job snapshot queued.

## Scope

- in scope:
  - `src-tauri/tests/transport_wiring_smoke.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - production Rust changes
  - frontend code
  - downloads module behavior changes
  - composition-root helper changes
  - kernel-jobs runtime behavior changes
  - scheduler loops/background tasks/timers
  - durable leases or precise active-slot accounting
  - terminal completed/failed snapshot projection
  - concrete HTTP/file/hash execution
  - retry/backoff policy
  - SQLite schema or adapter changes
  - unrelated dirty files

## Allowed Files

1. src-tauri/tests/transport_wiring_smoke.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. docs/TauriIPCAndStateContractsDesign.md 7.4.
2. docs/modules/downloads/README_IMPL.md 7.31-7.34.
3. Current `DownloadJobDriver::run(...)` deferred reason.
4. Current `SharedJobRuntimeHost::run_next_execution_turn(...)` deferred non-mutation behavior.
5. Current `src-tauri/tests/transport_wiring_smoke.rs` isolated host helper.

## Hypothesis

- falsifiable coverage hypothesis: with an isolated host graph, `downloads_start` queues a production downloads job, then `jobs_run_next_execution_turn` returns a successful `Deferred` DTO whose reason mentions the missing downloads execution port, and the stored snapshot remains `Queued`.

## Cheap Check

1. Add a transport smoke assertion after queueing one isolated downloads job.
2. Reuse the project-local sqlite helper and clean up after dropping the service handle.
3. Run focused transport smoke, scoped rustfmt, desktop package tests, desktop check, and scoped diff-check.

## Validation Gate

1. The new smoke assertion passes without production code changes.
2. The deferred reason proves production downloads execution is not wired yet.
3. Snapshot state and UI state remain queued after deferred dispatch.
4. Focused and package-level desktop validation pass before commit/push.

## Validation Result

1. Added isolated transport smoke coverage for a queued production downloads job dispatched through `jobs_run_next_execution_turn`.
2. The command returned a successful `Deferred` DTO whose reason contains `execution port not wired`.
3. The stored job snapshot remained `JobState::Queued` and `JobUiState::Queued`.
4. Focused smoke passed: `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` passed with 1 integration test.
5. Full desktop package tests passed with 3 unit tests and 1 integration smoke test.
6. Compile gate passed: `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
7. Scoped `rustfmt --edition 2021 --check src-tauri\tests\transport_wiring_smoke.rs` passed; scoped `git diff --check` passed with CRLF normalization warnings only.
