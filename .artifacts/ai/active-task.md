# Active Atomic Task

## Identity

- task id: AT-2026-05-19-266
- title: Implement pure download segment retry policy
- status: completed

## Goal

Implement the first Rust slice from downloads README_IMPL 7.47: a pure module-local retry/backoff policy helper that classifies automatic retry scheduling, exhaustion, user attention, and no-automatic-retry decisions without wiring scheduler or terminal projection.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `crates/kernel-foundation/src/time.rs` for the small shared `IsoDateTime` offset helper required by pure policy time arithmetic
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
2. crates/module-downloads/src/lib.rs
3. crates/kernel-foundation/src/time.rs
4. README.md
5. docs/modules/downloads/README_IMPL.md
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md
10. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and near-term roadmap after AT-265.
2. `docs/modules/downloads/README_IMPL.md` 7.47 policy defaults and first Rust slice.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable public code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.
5. `docs/modules/downloads/README_API.md`, `README_ARCH.md`, and `README_FLOW.md` module boundary notes.

## Hypothesis

- falsifiable implementation hypothesis: a pure retry policy helper can produce deterministic retry scheduling/exhaustion/user-attention decisions from persisted failed segment facts and explicit `IsoDateTime now` without touching scheduler, driver terminal projection, storage, host, or frontend code.

## Cheap Check

1. Add RED tests for first-attempt retry scheduling, second-attempt retry scheduling, automatic retry exhaustion, user-attention classes, and no-automatic-retry classes.
2. Implement the smallest `DownloadSegmentRetryPolicy` / decision type that passes those tests.
3. Update README and README_IMPL implementation status after green.
4. Run focused/full downloads tests, composition check, scoped rustfmt, and scoped diff-check.
5. Commit locally; do not reattempt push unless the user explicitly approves after the prior safety block.

## Validation Gate

1. RED tests fail for missing retry policy/decision types before implementation.
2. `cargo test -p launcher-module-downloads --lib` passes after implementation.
3. `cargo test -p launcher-kernel-foundation --lib` passes for the shared time helper.
4. `cargo check -p launcher-composition-root` passes.
5. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
6. README and README_IMPL reflect the completed pure policy slice.

## Completion Evidence

- RED evidence:
  - `cargo test -p launcher-module-downloads download_segment_retry_policy --lib` failed before implementation because `DownloadSegmentRetryPolicy` and `DownloadSegmentRetryDecision` did not exist.
- Implemented `DownloadSegmentRetryPolicy` and `DownloadSegmentRetryDecision` as a pure module-local policy helper.
- Added focused policy tests proving:
  - attempt `1` schedules `now + 30s`;
  - attempt `2` schedules `now + 120s`;
  - attempt `3` returns `RetryExhausted`;
  - `DiskNoSpace` and `PolicyBlocked` return `UserAttentionRequired`;
  - fatal, non-retryable, or incomplete failed facts return `NoAutomaticRetry`.
- Added `IsoDateTime::add_seconds(...)` in `kernel-foundation` so policy time arithmetic reuses the shared time wrapper without adding a new module dependency.
- README and downloads README_IMPL were updated to mark this pure policy slice complete and route the next boundary to wiring the policy into failed checkpoint mutation.
- Validation:
  - `cargo test -p launcher-module-downloads download_segment_retry_policy --lib` -> 5 passed.
  - `cargo test -p launcher-kernel-foundation --lib` -> 0 tests, exit 0.
  - `cargo test -p launcher-module-downloads --lib` -> 78 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `rustfmt --check crates/kernel-foundation/src/time.rs crates/module-downloads/src/driver.rs crates/module-downloads/src/lib.rs` -> passed.
  - scoped `git diff --check` -> passed with CRLF normalization warnings only.
- `Cargo.lock` was inspected and only contains a pre-existing unrelated `launcher-module-engines` hunk; it must remain uncommitted for this AT.
- Unintended package-wide rustfmt noise in unrelated foundation files was removed; only `time.rs` remains in scope.
- Commit `6910108` created locally.
- Push was not reattempted because the previous direct `origin/main` push was blocked by the safety reviewer and explicit approval is required before retrying.
