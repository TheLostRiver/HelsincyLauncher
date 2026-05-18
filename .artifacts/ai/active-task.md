# Active Atomic Task

## Identity

- task id: AT-2026-05-19-258
- title: Project explicit terminal run dispositions in kernel-jobs
- status: completed

## Goal

Implement README_IMPL 7.43's first Rust slice by adding explicit terminal run dispositions to `launcher-kernel-jobs` and projecting them to completed/failed runtime snapshots with focused TDD before changing downloads driver terminal decisions.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `module-downloads` driver terminal decision logic
  - retry/backoff or stable public `DL_*` execution errors
  - snapshot error payload fields
  - host transport, frontend, SQLite schema, provider HTTP, real production execution-port wiring, leases, or scheduler loops
  - unrelated dirty files

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
2. README.md
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and near-term roadmap.
2. `docs/modules/downloads/README_IMPL.md` 7.43.
3. `docs/TauriKernelJobsRuntimeDesign.md` lifecycle/snapshot projection rules.
4. `docs/TauriTestingStrategyAndQualityGateDesign.md` focused Rust test guidance.
5. `docs/TauriCodeCommentStandard.md` Chinese-first and bilingual companion comment rules.
6. `crates/kernel-jobs/src/runtime.rs` `JobRunDisposition`, `run_one_execution_turn(...)`, and existing fake-driver tests.
7. `crates/kernel-jobs/src/model.rs` `JobState`, `JobUiState`, and current snapshot shape.

## Hypothesis

- falsifiable implementation hypothesis: adding explicit `Completed` and terminal failed dispositions to `JobRunDisposition`, while leaving `Accepted`, `Deferred`, and existing non-terminal `Failed { reason }` behavior unchanged, is sufficient for `SharedJobRuntimeHost::run_one_execution_turn(...)` to project stored snapshots to terminal states.

## Cheap Check

1. Add RED `launcher-kernel-jobs` tests using fake drivers that return missing terminal dispositions.
2. Verify RED fails because the terminal disposition variants do not exist.
3. Add the smallest enum variants and projection branches in `run_one_execution_turn(...)`.
4. Re-run focused tests, full `launcher-kernel-jobs --lib`, `cargo check -p launcher-composition-root`, scoped rustfmt, scoped diff-check.
5. Update README/README_IMPL implementation status, commit, and attempt push.

## Validation Gate

1. RED failure observed before production enum/projection changes.
2. Focused terminal projection tests pass.
3. Existing accepted/deferred behavior tests keep passing.
4. Full `launcher-kernel-jobs --lib` passes.
5. `cargo check -p launcher-composition-root` passes.
6. Scoped rustfmt and diff-check pass.

## Completion Evidence

- RED: `cargo test -p launcher-kernel-jobs terminal --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` failed with missing `JobRunDisposition::Completed` and `JobRunDisposition::TerminalFailed`.
- GREEN focused terminal failed: `cargo test -p launcher-kernel-jobs terminal --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed for `execution_dispatch_projects_terminal_failed_driver_to_failed_snapshot`.
- GREEN focused completed: `cargo test -p launcher-kernel-jobs completed_driver --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed for `execution_dispatch_projects_completed_driver_to_completed_snapshot`.
- Regression: `cargo test -p launcher-kernel-jobs --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 14/14 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed.
- Scoped diff-check passed for AT-258 files with CRLF normalization warnings only.
- Commit/push pending.
