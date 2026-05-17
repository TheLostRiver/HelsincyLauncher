# Active Atomic Task

## Identity

- task id: AT-2026-05-17-226
- title: Cover downloads driver run deferred branches
- status: completed

## Goal

Add focused module-downloads coverage for the remaining guarded `DownloadJobDriver::run(...)` deferred branches from README_IMPL 7.31: missing checkpoint, no pending work, and no checkpoint mutation.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - production behavior changes unless a test exposes a gap
  - docs updates beyond PWF records
  - composition-root wiring
  - concrete HTTP/file/hash execution
  - retry/backoff
  - terminal snapshot completion, host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. docs/modules/downloads/README_IMPL.md section 7.31.
2. current `DownloadJobDriver::run(...)`, `execute_local_resume_turn(...)`, and fake execution port tests.
3. current module-downloads validation expectations.

## Hypothesis

- falsifiable local hypothesis: branch tests for missing checkpoint, no pending work, and Accepted-only/no-mutation fake port should pass against the guarded run implementation; if any fail, fix only the smallest behavior gap in `DownloadJobDriver::run(...)`.

## Cheap Check

1. Add focused branch tests in `crates/module-downloads/src/driver.rs`.
2. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml driver_run`.
3. Run module-downloads lib tests, rustfmt check, and scoped `git diff --check`.

## Validation Gate

1. Missing checkpoint with an execution port returns deferred and does not drain pending work.
2. Existing checkpoint with no pending work returns deferred.
3. Accepted-only/no checkpoint mutation fake port returns deferred.
4. Existing driver_run tests still pass.

## Validation Result

1. `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml driver_run` passed, 5 passed / 0 failed.
2. `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed, 52 passed / 0 failed.
3. `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
4. `rustfmt --edition 2021 --check crates\module-downloads\src\driver.rs` passed.
5. Scoped `git diff --check` passed with CRLF normalization warnings only.
