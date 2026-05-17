# Active Atomic Task

## Identity

- task id: AT-2026-05-17-221
- title: Add minimal kernel-jobs execution-turn contract
- status: completed

## Goal

Introduce the smallest module-neutral execution-turn contract in `kernel-jobs` so drivers can later be asked to run a queued job through a runtime-owned context, without wiring downloads execution, concrete IO, retry, lease persistence, or terminal completion.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `crates/kernel-jobs/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - downloads driver integration
  - concrete HTTP/file/hash execution
  - retry/backoff
  - durable lease persistence
  - terminal snapshot completion or runtime loop scheduling
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
2. crates/kernel-jobs/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. docs/modules/downloads/README_IMPL.md section 7.29.
2. docs/TauriKernelJobsRuntimeDesign.md driver, queue policy, lease, recovery, and runtime-context sections.
3. docs/TauriDownloadRuntimeDesign.md scheduler and budget sections.
4. current `crates/kernel-jobs/src/runtime.rs`, `lib.rs`, and `model.rs` surfaces.
5. current module driver implementations only to confirm compatibility boundaries.

## Hypothesis

- falsifiable local hypothesis: a focused kernel-jobs RED test around a fake driver can prove the missing execution-turn contract; the minimal implementation can add a read-only `JobExecutionContext` plus explicit `JobRunDisposition` without touching concrete module execution.

## Cheap Check

1. Add focused RED tests in `crates/kernel-jobs/src/runtime.rs`.
2. Run `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib execution_turn`.
3. Implement the minimal contract and re-run the focused test.
4. Run affected package check and scoped rustfmt checks.
5. Run scoped `git diff --check`.

## Validation Gate

1. Fake driver tests prove `JobDriver::run(...)` can observe a read-only execution context.
2. Default driver `run(...)` returns an explicit deferred disposition rather than pretending execution exists.
3. `kernel-jobs` exports the new contract types.
4. Existing module driver impls compile without concrete execution wiring.
5. No downloads IO, retry/backoff, lease persistence, terminal completion, host transport, frontend, or SQLite schema changes.
6. Commit only AT-221 files locally, then push `main` to `origin`.

## Validation Result

- Added `JobRunDisposition`, `JobExecutionContext`, and default `JobDriver::run(...)` in `crates/kernel-jobs/src/runtime.rs`.
- Exported `JobExecutionContext` and `JobRunDisposition` from `crates/kernel-jobs/src/lib.rs`.
- Added focused fake-driver tests proving default run is explicitly deferred and a registry-resolved fake driver can accept an execution turn through a read-only context.
- Updated README_IMPL 7.29 current Rust state without adding a long per-task log.
- RED validation failed before implementation with missing `JobExecutionContext`, `JobRunDisposition`, and `JobDriver::run(...)`.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 4 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed; broader rustfmt still sees a pre-existing out-of-scope `model.rs` formatting diff.
- Commit and push are pending for the AT-221 file set.

## Notes

- AT-2026-05-17-220 final commit is `aa8d6e3` and is already pushed to `origin/main`.
