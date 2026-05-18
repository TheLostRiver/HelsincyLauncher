# Active Atomic Task

## Identity

- task id: AT-2026-05-19-264
- title: Implement retry/backoff failure facts persistence
- status: completed

## Goal

Implement the first Rust slice from downloads README_IMPL 7.46: persist retry-ready failed segment facts and an internal module-owned failure class while keeping failed checkpoint mutation non-terminal.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/facade/mod.rs` fixture updates for widened checkpoint records only
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `crates/adapter-storage-sqlite/Cargo.toml` only if timestamp parsing needs an explicit dependency
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - retry scheduler implementation or execution loops
  - returning `TerminalFailed` from downloads driver
  - stable public `DL_*` error projection
  - host transport, frontend state, provider HTTP, production wiring, leases, scheduler loops, or job snapshot error payloads

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/facade/mod.rs fixture updates for widened checkpoint records only
3. crates/adapter-storage-sqlite/src/lib.rs
4. crates/adapter-storage-sqlite/Cargo.toml if required by timestamp parsing
5. README.md
6. docs/modules/downloads/README_IMPL.md
7. .artifacts/ai/active-task.md
8. .artifacts/ai/task-plan.md
9. .artifacts/ai/progress.md
10. .artifacts/ai/findings.md
11. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and near-term roadmap after AT-263.
2. `docs/modules/downloads/README_IMPL.md` 7.46 first Rust slice.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable public code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.
5. `docs/modules/downloads/README_API.md`, `README_ARCH.md`, and `README_FLOW.md` module boundary notes.

## Hypothesis

- falsifiable implementation hypothesis: failed segment retry facts can be persisted through the driver and SQLite checkpoint adapter without introducing retry execution, public `DL_*` projection, or `TerminalFailed`.

## Cheap Check

1. Add RED driver coverage proving failed checkpoint mutation stores `failure_class`, starts/increments `retry_attempt_count`, leaves `next_retry_after` unset, and remains non-terminal.
2. Add RED SQLite round-trip coverage for `failure_class`, `retry_attempt_count`, and `next_retry_after`.
3. Implement the smallest fields/schema/mapping needed to pass.
4. Update README and downloads README_IMPL status after green.
5. Run focused and package validation, then commit and attempt push.

## Validation Gate

1. RED tests fail for missing retry/failure-class fields before implementation.
2. `cargo test -p launcher-module-downloads --lib` passes after implementation.
3. `cargo test -p launcher-adapter-storage-sqlite --lib` passes after implementation.
4. `cargo check -p launcher-composition-root` passes.
5. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
6. README and README_IMPL reflect the new completed persistence slice.

## Completion Evidence

- RED evidence:
  - `cargo test -p launcher-module-downloads download_job_driver_failed_result_checkpoint_mutation_tracks_retry_facts --lib` failed before implementation because `DownloadSegmentFailureClass`, `failure_class`, `retry_attempt_count`, and `next_retry_after` did not exist.
  - `cargo test -p launcher-adapter-storage-sqlite sqlite_download_checkpoint_round_trips_segment_facts --lib` failed before implementation for the same missing enum/fields.
- Implemented `DownloadSegmentFailureClass`, carried it through `DownloadSegmentHandledFailure` and `DownloadSegmentExecutionResult::Failed`, and persisted optional `failure_class`, `retry_attempt_count`, and `next_retry_after` on `DownloadSegmentCheckpointRecord`.
- Failed checkpoint mutation now starts new failed facts at `retry_attempt_count = 1`, increments existing failed segment counts, preserves internal failure class, and leaves `next_retry_after` unset.
- SQLite checkpoint schema/backfill/save/load/round-trip mapping now preserves `failure_class`, `retry_attempt_count`, and `next_retry_after`.
- README and downloads README_IMPL were updated to mark this persistence slice complete and route the next boundary to backoff policy, retry exhaustion, and terminal failure eligibility.
- Validation:
  - `cargo test -p launcher-module-downloads --lib` -> 73 passed.
  - `cargo test -p launcher-adapter-storage-sqlite --lib` -> 3 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `cargo fmt -p launcher-module-downloads -p launcher-adapter-storage-sqlite -- --check` -> passed.
  - scoped `git diff --check` -> passed with CRLF normalization warnings only.
- Commit `ad6cef2` created locally.
- Push to `origin/main` was attempted and blocked by the safety reviewer because direct shared-branch mutation/export needs explicit approval; no workaround was attempted.
