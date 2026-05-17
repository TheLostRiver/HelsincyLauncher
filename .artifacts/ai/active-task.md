# Active Atomic Task

## Identity

- task id: AT-2026-05-17-234
- title: Add composition one-shot runtime execution helper
- status: completed

## Goal

Implement the documented composition-root helper that explicitly composes `SharedJobRuntimeHost` and `JobDriverRegistry<()>` for exactly one runtime execution turn, while avoiding automatic scheduler loops, startup hidden side effects, durable leases, terminal projection, downloads IO, transport, frontend, and SQLite schema changes.

## Scope

- in scope:
  - `crates/composition-root/src/startup.rs`
  - `crates/composition-root/src/bootstrap.rs`
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
  - precise active-slot accounting or queue fairness beyond existing runtime policy gate
  - terminal completion/failure projection
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/composition-root/src/startup.rs
2. crates/composition-root/src/bootstrap.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md routing.
2. docs/TauriCompositionRootWiringDesign.md 9.4.
3. docs/TauriStartupPipelineDesign.md restore/warmup rules.
4. current `StartupPipelineFacade` and bootstrap runtime/driver registry wiring.
5. current composition-root startup/bootstrap tests.

## Hypothesis

- falsifiable implementation hypothesis: `StartupPipelineFacade` can expose an explicit one-shot helper that returns deferred when runtime or registry wiring is absent and delegates once to `SharedJobRuntimeHost::run_next_execution_turn(...)` when both are wired.

## Cheap Check

1. Add RED tests for absent wiring and bootstrap-wired no-queued behavior.
2. Implement the helper and build-time wiring.
3. Run focused composition tests, composition-root check, scoped rustfmt, and scoped diff check.

## Validation Gate

1. Focused helper tests fail before implementation and pass after implementation.
2. Existing startup restore/prewarm behavior remains unchanged.
3. No helper is invoked automatically by construction or startup stages.
4. Verification commands pass before commit/push.

## Validation Result

1. RED: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml runtime_execution` failed as expected because `run_one_runtime_execution_turn(...)` and `with_runtime_execution(...)` did not exist yet.
2. GREEN focused: the same command passed with 3 tests passed / 0 failed.
3. Full package lib: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 12 tests passed / 0 failed.
4. Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
5. Scoped format/checks: `rustfmt --edition 2021 --check crates\composition-root\src\startup.rs crates\composition-root\src\bootstrap.rs` passed; scoped `git diff --check` passed with CRLF normalization warnings only.
