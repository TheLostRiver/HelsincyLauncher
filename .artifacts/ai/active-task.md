# Active Atomic Task

## Identity

- task id: AT-2026-05-17-236
- title: Add host runtime execution command
- status: completed

## Goal

Implement the documented `jobs_run_next_execution_turn` host command as a thin transport boundary over `DesktopAppServices.startup.run_one_runtime_execution_turn()`, returning a stable runtime execution-turn DTO while avoiding frontend UI, scheduler loops, automatic startup invocation, terminal projection, downloads concrete IO, retry/backoff, and schema changes.

## Scope

- in scope:
  - `src-tauri/src/commands/mod.rs`
  - `src-tauri/src/commands/jobs.rs`
  - `src-tauri/tests/transport_wiring_smoke.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - docs updates beyond PWF records
  - frontend code
  - downloads module behavior
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

1. src-tauri/src/commands/mod.rs
2. src-tauri/src/commands/jobs.rs
3. src-tauri/tests/transport_wiring_smoke.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md routing.
2. docs/TauriIPCAndStateContractsDesign.md 7.4.
3. docs/TauriCompositionRootWiringDesign.md 9.4 and Tauri integration boundary.
4. docs/TauriStartupPipelineDesign.md restore/warmup ownership rules.
5. docs/modules/downloads/README_IMPL.md runtime execution sections 7.29-7.34.
6. Current `src-tauri/src/commands/mod.rs`, `src-tauri/src/commands/jobs.rs`, `src-tauri/tests/transport_wiring_smoke.rs`, `src-tauri/src/bootstrap.rs`, and `src-tauri/src/state.rs`.

## Hypothesis

- falsifiable implementation hypothesis: adding a DTO mapper plus `jobs_run_next_execution_turn(...)` in the host command layer will make a fresh bootstrap return a successful deferred DTO with a no-queued reason, without touching runtime/composition/downloads internals.

## Cheap Check

1. Add RED transport smoke assertion expecting the new registered command and fresh no-queued deferred DTO.
2. Add the minimal DTO, mapper, command registration, and command handler.
3. Run focused desktop transport smoke, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test fails before production code because the command/DTO is missing.
2. GREEN test passes after implementation.
3. `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` passes.
4. `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passes.
5. Scoped `rustfmt --edition 2021 --check` and scoped `git diff --check` pass before commit/push.

## Validation Result

1. RED: `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` failed as expected because `RuntimeExecutionTurnDispositionDto` and `jobs_run_next_execution_turn(...)` did not exist yet.
2. GREEN focused: the same transport smoke command passed with 1 test passed / 0 failed.
3. DTO mapper focused test passed: `maps_runtime_execution_turn_dispositions_without_error_envelope` passed with 1 test passed / 0 failed.
4. Full desktop package tests passed: `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 3 unit tests and 1 integration smoke test.
5. Compile gate passed: `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
6. Scoped format/diff checks passed: `rustfmt --edition 2021 --check --config skip_children=true src-tauri\src\commands\mod.rs src-tauri\src\commands\jobs.rs src-tauri\tests\transport_wiring_smoke.rs`; scoped `git diff --check` passed with CRLF normalization warnings only.
