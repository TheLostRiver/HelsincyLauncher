# Active Atomic Task

## Identity

- task id: AT-2026-05-17-198
- title: Add downloads fake segment failure result contract
- status: completed

## Goal

After AT-197 documented the fake segment failure result boundary, add the narrow Rust contract that lets a fake or future segment executor return an in-band failed segment result through `DownloadSegmentExecutionResult`.

This is a module-local result shape only. It must not introduce public `DL_*` execution errors, checkpoint mutation, retry/backoff, runtime completion, concrete HTTP/write/verify behavior, SQLite adapter/schema work, composition-root, transport, or frontend changes.

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
  - checkpoint mutation for failed results
  - retry/backoff policy
  - concrete HTTP range requests or provider object fetch
  - staging file writes, artifact moves, or hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - stable public `DL_*` execution error projection
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
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md section 7.19 and validation section
8. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
9. docs/TauriKernelJobsRuntimeDesign.md runtime ownership snippets
10. docs/TauriErrorHandlingAndProjectionDesign.md retryable/severity/public-error snippets
11. current `crates/module-downloads/src/driver.rs`
12. superpowers TDD skill

## Hypothesis

- falsifiable local hypothesis: a focused driver test can prove `DownloadSegmentExecutionPort` can return a local failed segment result value through the existing collection helper without changing port signatures, checkpoint mutation, runtime behavior, concrete IO, transport, or frontend behavior.

## Cheap Check

1. Add a focused RED test in `crates/module-downloads/src/driver.rs` for `segment_failure_result`.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml segment_failure_result`.
3. Add the minimal `DownloadSegmentExecutionResult::Failed` variant and fields required by the test.
4. Update README_IMPL to mark the failure result contract implemented.
5. Run focused test, full module test, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails because the failed result variant does not exist.
2. GREEN preserves failed result payload through the existing execution port helper.
3. No checkpoint mutation, retry/backoff, runtime completion, public error projection, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work is added.
4. Public comments are bilingual and existing English comments are preserved.
5. Focused and full module tests pass.
6. Formatting and scoped diff checks pass.
7. Commit only AT-198 files locally, then push `main` to `origin`.

## Validation Result

1. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml segment_failure_result` failed because `DownloadSegmentExecutionResult::Failed` did not exist.
2. GREEN added `DownloadSegmentExecutionResult::Failed` with request facts, downloaded bytes, local reason, and retryable hint.
3. Focused validation passed: 1 passed, 0 failed.
4. Full downloads module validation passed: 36 passed, 0 failed.
5. `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
6. Scoped `git diff --check` passed with CRLF normalization warnings only.
7. No checkpoint mutation, retry/backoff, public error projection, runtime completion, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work was added.

## Notes

- AT-2026-05-17-197 final local commit is `af6ca27` and was pushed to `origin/main`.
- AT-2026-05-17-198 initial local commit created as `c4156bb`; PWF backfill is amended into the same task commit.
