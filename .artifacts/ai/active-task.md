# Active Atomic Task

## Identity

- task id: AT-2026-05-19-260
- title: Return completed disposition from downloads driver all-completed checkpoint
- status: completed

## Goal

Implement README_IMPL 7.44's first Rust slice so `DownloadJobDriver::run(...)` returns `JobRunDisposition::Completed` only after local execution saves a non-empty checkpoint whose known segment records are all completed, while failed segment mutation stays non-terminal.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - returning `TerminalFailed` from downloads driver
  - retry/backoff or stable public `DL_*` execution errors
  - host transport, frontend, SQLite schema, provider HTTP, production wiring, scheduler loops, or leases
  - unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. README.md
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `docs/modules/downloads/README_IMPL.md` 7.44.
2. `docs/TauriKernelJobsRuntimeDesign.md` terminal snapshot projection ownership.
3. `docs/TauriDownloadRuntimeDesign.md` checkpoint/runtime ownership.
4. `docs/TauriTestingStrategyAndQualityGateDesign.md` focused module test guidance.
5. `docs/TauriCodeCommentStandard.md` Chinese-first and bilingual companion comment rules.
6. `crates/module-downloads/src/driver.rs` checkpoint statuses, checkpoint mutation helpers, `execute_local_resume_turn(...)`, `run(...)`, and existing driver tests.
7. `README.md` current status for post-green refresh.

## Hypothesis

- falsifiable implementation hypothesis: a saved checkpoint can be classified locally as terminal completed when it has at least one segment and all segment statuses are `Completed`; otherwise `DownloadJobDriver::run(...)` should preserve the existing non-terminal `Accepted`/`Deferred` behavior.

## Cheap Check

1. Add RED driver tests for all-completed checkpoint returning `Completed` and failed checkpoint mutation staying non-terminal.
2. Verify RED fails because completed checkpoint mutation still returns `Accepted`.
3. Add the smallest downloads-owned helper/decision after `execute_local_resume_turn(...)`.
4. Re-run focused driver tests, full downloads module lib tests, composition-root check, scoped rustfmt, and scoped diff-check.
5. Update README/README_IMPL implementation status, commit, and attempt push.

## Validation Gate

1. RED failure observed before production changes.
2. All-completed checkpoint run returns `JobRunDisposition::Completed`.
3. Failed checkpoint mutation remains non-terminal.
4. Existing missing-port/missing-checkpoint/no-work/accepted-only deferred tests keep passing.
5. Full `launcher-module-downloads --lib` passes.
6. `cargo check -p launcher-composition-root` passes.
7. Scoped rustfmt and diff-check pass.

## Completion Evidence

- RED: `cargo test -p launcher-module-downloads driver_run_with_execution_port_records_completed_checkpoint --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` failed because the completed checkpoint run returned `Accepted` instead of `Completed`.
- GREEN focused completed: the same focused test passed after adding the completion-first checkpoint decision.
- GREEN focused failed-nonterminal: `cargo test -p launcher-module-downloads driver_run_with_failed_checkpoint_mutation_stays_non_terminal --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 72/72 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `rustfmt --edition 2021 --check crates\module-downloads\src\driver.rs` passed.
- Scoped diff-check passed for AT-260 files with CRLF normalization warnings only.
- Published as commit `55ab2da` and pushed to `origin/main`.
