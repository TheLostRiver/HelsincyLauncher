# Active Atomic Task

## Identity

- task id: AT-2026-05-17-225
- title: Add guarded downloads driver run override
- status: completed

## Goal

Implement the README_IMPL 7.31 downloads driver `run(...)` boundary: default drivers must defer without draining pending work, while an opt-in fake segment execution port path may execute the existing local fake turn, mutate checkpoint facts, and return `JobRunDisposition::Accepted`.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - composition-root wiring
  - concrete HTTP/file/hash execution
  - retry/backoff
  - durable lease persistence
  - terminal snapshot completion or runtime loop scheduling
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and collaboration guidance refreshed this session.
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md section 7.31.
3. docs/TauriKernelJobsRuntimeDesign.md driver/runtime-context sections.
4. docs/TauriDownloadRuntimeDesign.md ownership, scheduler, and checkpoint sections.
5. current `DownloadJobDriver` helpers: pending-work source, `prepare_resume_execution_turn(...)`, `execute_local_resume_turn(...)`, checkpoint mutation helpers, and tests.

## Hypothesis

- falsifiable local hypothesis: focused module-downloads RED tests can prove the guarded override boundary by showing that `with_pending_resume_work_source(...)` defers without draining, while an opt-in constructor with a fake completed segment execution port returns `Accepted` and persists checkpoint mutation.

## Cheap Check

1. Add focused RED tests in `crates/module-downloads/src/driver.rs`.
2. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml driver_run`.
3. Implement the minimal optional execution-port dependency and `JobDriver::run(...)` override.
4. Run focused and full module-downloads tests, affected composition check, scoped rustfmt, and scoped `git diff --check`.

## Validation Gate

1. Default/injected-source-only driver run defers without draining pending work.
2. Opt-in fake completed execution port path consumes pending work, persists checkpoint mutation, and returns `JobRunDisposition::Accepted`.
3. Missing checkpoint/no pending/no mutation paths defer rather than pretending completion.
4. Existing restore/local helper tests keep passing.
5. No composition-root, concrete IO, retry/backoff, terminal runtime completion, transport, frontend, or SQLite schema changes.

## Validation Result

- RED validation failed first on missing `with_pending_resume_work_source_and_execution_port(...)` after fixing a test fixture import.
- Added optional `DownloadSegmentExecutionPort` dependency to `DownloadJobDriver`.
- Added `DownloadJobDriver::with_pending_resume_work_source_and_execution_port(...)`.
- Added guarded `JobDriver::run(...)` override that defers without an execution port and accepts checkpoint-mutating fake/local execution.
- Updated README_IMPL 7.31 current Rust state.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml driver_run` passed with 2 tests passed / 0 failed.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 49 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\module-downloads\src\driver.rs` passed.
- Scoped `git diff --check` passed with CRLF normalization warnings only.
