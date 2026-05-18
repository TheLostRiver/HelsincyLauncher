# Active Atomic Task

## Identity

- task id: AT-2026-05-19-267
- title: Wire retry policy into failed checkpoint mutation
- status: completed

## Goal

Implement the next downloads retry slice after the pure policy helper: use `DownloadSegmentRetryPolicy` during failed checkpoint mutation to persist `next_retry_after` only for `ScheduleRetry` decisions while keeping failed mutation non-terminal.

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
  - SQLite schema changes
  - retry scheduler implementation or execution loops
  - returning `TerminalFailed` from downloads driver
  - stable public `DL_*` error projection
  - host transport, frontend state, provider HTTP, production wiring, leases, scheduler loops, or job snapshot error payloads

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

1. `README.md` current status and near-term roadmap after AT-266.
2. `docs/modules/downloads/README_IMPL.md` 7.47 implementation status and next implementation target.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable public code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.
5. `docs/modules/downloads/README_API.md`, `README_ARCH.md`, and `README_FLOW.md` module boundary notes.

## Hypothesis

- falsifiable implementation hypothesis: failed checkpoint mutation can persist a policy-computed `next_retry_after` for automatic retry decisions without starting retry work, changing storage schema, or returning `TerminalFailed`.

## Cheap Check

1. Add RED test using an explicit `IsoDateTime now` to prove failed mutation persists `next_retry_after = now + 30s` for an automatic retry decision.
2. Implement the smallest failed checkpoint mutation wiring, preferably through an internal helper that accepts explicit `now` for deterministic tests.
3. Update affected existing failed-mutation tests so automatic retry cases expect a scheduled retry fact while exhausted/non-automatic cases remain unset.
4. Update README and README_IMPL implementation status after green.
5. Run focused/full downloads tests, composition check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED test fails because failed checkpoint mutation still leaves `next_retry_after` unset.
2. `cargo test -p launcher-module-downloads --lib` passes after implementation.
3. `cargo check -p launcher-composition-root` passes.
4. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
5. README and README_IMPL reflect completed policy wiring.

## Completion Evidence

- RED evidence:
  - `cargo test -p launcher-module-downloads download_job_driver_failed_result_checkpoint_mutation_schedules_next_retry_after --lib` failed before implementation because `record_failed_segment_checkpoints_at(...)` did not exist.
- Implemented failed checkpoint mutation policy wiring:
  - public `record_failed_segment_checkpoints(...)` now uses current `IsoDateTime::now()`;
  - internal `record_failed_segment_checkpoints_at(...)` accepts explicit `now` for deterministic tests;
  - `next_retry_after` is persisted only when `DownloadSegmentRetryPolicy` returns `ScheduleRetry`.
- Updated existing failed-mutation tests so automatic retry cases expect a scheduled retry fact while failed mutation remains non-terminal.
- README and downloads README_IMPL were updated to mark policy wiring complete and route the next boundary to selecting due retry-ready segment checkpoints.
- Validation:
  - `cargo test -p launcher-module-downloads download_job_driver_failed_result_checkpoint_mutation_schedules_next_retry_after --lib` -> 1 passed.
  - `cargo test -p launcher-module-downloads --lib` -> 79 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `rustfmt --check crates/module-downloads/src/driver.rs` -> passed.
  - scoped `git diff --check` -> passed with CRLF normalization warnings only.
- Commit `d99470d` created locally.
- Push was not reattempted because the previous direct `origin/main` push was blocked by the safety reviewer and explicit approval is required before retrying.
