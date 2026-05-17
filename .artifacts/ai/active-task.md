# Active Atomic Task

## Identity

- task id: AT-2026-05-17-200
- title: Add downloads fake failed-result checkpoint mutation
- status: completed

## Goal

After AT-199 documented the failed-result checkpoint mutation boundary, add the narrow Rust helper that records same-job fake failed segment results into `DownloadSegmentCheckpointRecord` facts through the existing `DownloadCheckpointRepository`.

This stays module-local. It must not add retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema work, composition-root, transport, or frontend behavior.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `crates/module-downloads/src/driver.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - retry/backoff policy
  - public `DL_*` execution error projection
  - terminal runtime job state
  - concrete HTTP range requests or provider object fetch
  - staging file writes, artifact moves, or hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. crates/module-downloads/src/driver.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_IMPL.md section 7.20 and validation section
5. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
6. docs/TauriErrorHandlingAndProjectionDesign.md retry/public-error snippets
7. docs/TauriKernelJobsRuntimeDesign.md runtime ownership snippets
8. current `crates/module-downloads/src/driver.rs`
9. superpowers TDD skill

## Hypothesis

- falsifiable local hypothesis: a focused driver test can prove failed execution results replace or append segment checkpoint facts as `Failed` and save through `DownloadCheckpointRepository` without adding retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend behavior.

## Cheap Check

1. Add a focused RED test in `crates/module-downloads/src/driver.rs` for failed-result checkpoint mutation.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml failed_result_checkpoint`.
3. Implement the minimal local driver helper.
4. Update README_IMPL to mark failed-result checkpoint mutation implemented.
5. Run focused test, full module test, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails because the failed-result checkpoint mutation helper does not exist.
2. GREEN reloads checkpoint, applies only same-job failed results, preserves existing partial metadata on replacement, saves through the existing repository port, and returns the saved checkpoint.
3. No retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work is added.
4. Public comments are bilingual and existing English comments are preserved.
5. Focused and full module tests pass.
6. Formatting and scoped diff checks pass.
7. Commit only AT-200 files locally, then push `main` to `origin`.

## Validation Result

1. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml failed_result_checkpoint` failed because `record_failed_segment_checkpoints` did not exist.
2. GREEN added `DownloadJobDriver::record_failed_segment_checkpoints(...)`, which reloads checkpoint facts, applies same-job failed results, preserves existing partial metadata on replacement, saves through `DownloadCheckpointRepository`, and returns the saved checkpoint.
3. Focused validation passed: 1 passed, 0 failed.
4. Full downloads module validation passed: 37 passed, 0 failed.
5. `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
6. Scoped `git diff --check` passed with CRLF normalization warnings only.
7. No retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work was added.

## Notes

- AT-2026-05-17-199 final local commit is `59db102` and was pushed to `origin/main`.
- AT-2026-05-17-200 initial local commit created as `94573e3`; PWF backfill is amended into the same task commit.
