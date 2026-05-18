# Active Atomic Task

## Identity

- task id: AT-2026-05-19-262
- title: Persist failed segment metadata through checkpoint records
- status: completed

## Goal

Implement the first downloads failed-metadata persistence slice: preserve module-local failure reason and retryable hint from failed execution results through `DownloadSegmentCheckpointRecord` and the SQLite checkpoint adapter.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - returning `TerminalFailed` from downloads driver
  - retry counts, backoff scheduling, retry queues, or retry execution
  - stable public `DL_*` execution error projection
  - host transport, frontend state, provider HTTP, production wiring, scheduler loops, leases, or job snapshot error payloads
  - broad SQLite migration framework beyond the existing table-creation surface needed by current tests

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/facade/mod.rs
3. crates/adapter-storage-sqlite/src/lib.rs
4. README.md
5. docs/modules/downloads/README_IMPL.md
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md
10. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and near-term roadmap.
2. `docs/README.md` docs layering and update routing.
3. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` 7.45.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint, resume, and failure classification rules.
5. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable and diagnostics semantics.
6. `docs/TauriTestingStrategyAndQualityGateDesign.md` module/adapter verification gates.
7. `docs/TauriAIDevelopmentTransactionProtocolDesign.md` atomic task and commit requirements.
8. `docs/TauriCodeCommentStandard.md` Chinese-first comment requirements.
9. `docs/TauriCurrentRepoArchitectureOverview.md`, `docs/TauriBackendSkeletonImplementationDesign.md`, and `docs/TauriRepositoryPortsAndAdapterDesign.md` module/adapter boundaries.
10. `crates/module-downloads/src/driver.rs` checkpoint record, failed result, mutation helper, and focused tests.
11. `crates/adapter-storage-sqlite/src/lib.rs` checkpoint schema, row mapping, and round-trip tests.

## Hypothesis

- falsifiable code hypothesis: adding optional failed reason and retryable fields to segment checkpoint facts, then mapping them through failed-result mutation and SQLite round-trip, is enough to make failure metadata durable while preserving the current non-terminal driver behavior.

## Cheap Check

1. Add a focused RED driver test proving failed checkpoint mutation persists `failure_reason` and `failure_retryable`.
2. Add a focused RED SQLite round-trip assertion proving failed metadata survives save/load.
3. Implement only the record fields, failed mutation mapping, and SQLite schema/load/save mapping needed to pass.
4. Verify failed checkpoint mutation still produces non-terminal `Accepted` from `DownloadJobDriver::run(...)`.
5. Update README and downloads implementation doc status after green.
6. Run focused tests, affected crate tests, `cargo check -p launcher-composition-root`, scoped rustfmt, and scoped diff-check.
7. Commit and attempt push.

## Validation Gate

1. RED tests fail for missing failed metadata fields before production code.
2. Failed execution result metadata is preserved in in-memory checkpoint mutation.
3. SQLite checkpoint round-trip preserves failed metadata without exposing public `DL_*` errors.
4. Existing completed/pending/in-progress checkpoint records can still be constructed and round-trip with no failed metadata.
5. Downloads driver failed mutation remains non-terminal.
6. No host, frontend, provider HTTP, production wiring, scheduler, lease, or snapshot error payload behavior changes.

## Completion Evidence

- RED verification:
  - `cargo test -p launcher-module-downloads download_job_driver_failed_result_checkpoint_mutation_replaces_and_saves_segment` failed on missing `failure_reason` / `failure_retryable` fields.
  - `cargo test -p launcher-adapter-storage-sqlite sqlite_download_checkpoint_round_trips_segment_facts` failed on missing `failure_reason` / `failure_retryable` fields.
- Implemented optional `failure_reason` and `failure_retryable` fields on `DownloadSegmentCheckpointRecord`.
- Updated failed checkpoint mutation to preserve local failed execution reason/retryable metadata.
- Updated SQLite checkpoint schema creation/backfill, save mapping, load mapping, and round-trip coverage for failed metadata.
- Verified failed checkpoint mutation still stays non-terminal through `driver_run_with_failed_checkpoint_mutation_stays_non_terminal`.
- Updated `README.md` and `docs/modules/downloads/README_IMPL.md` to mark failed metadata persistence complete and keep retry/backoff/failure classification as the next boundary.
- Validation:
  - `cargo test -p launcher-module-downloads --lib` passed: 72/72.
  - `cargo test -p launcher-adapter-storage-sqlite --lib` passed: 3/3.
  - `cargo check -p launcher-composition-root` passed.
  - `cargo fmt -p launcher-module-downloads -p launcher-adapter-storage-sqlite -- --check` passed.
  - scoped `git diff --check` passed with CRLF normalization warnings only.
- Commit `74cdf2c` pushed to `origin/main`.
