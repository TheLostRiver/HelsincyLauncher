# Active Atomic Task

## Identity

- task id: AT-2026-05-19-251
- title: Add downloads segment length verifier
- status: completed

## Goal

Implement README_IMPL 7.40 as a concrete module-local byte-length verifier behind `DownloadSegmentVerifyPort`.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - hash algorithm verification
  - reading bytes back from disk
  - file-level or job-level manifest sealing
  - retry/backoff policy
  - public `DL_VERIFY_FAILED` DTO projection
  - composition-root production execution-port wiring
  - host transport, frontend, or SQLite schema changes
  - unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/lib.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and collaboration constraints from AT-250.
2. `docs/modules/downloads/README_IMPL.md` 7.40.
3. `docs/TauriDownloadRuntimeDesign.md` verifier and verification stage notes.
4. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable/public projection rules.
5. Existing `DownloadSegmentVerifyPort`, `DownloadSegmentVerifyOutcome`, `DownloadSegmentExecutor`, and verifier test helpers.

## Hypothesis

- falsifiable implementation hypothesis: `DownloadSegmentLengthVerifyPort` can return `Verified` when `written.downloaded_bytes == request.length`, return a retryable handled verify failure with `downloaded_bytes = written.downloaded_bytes` on mismatch, and feed the existing executor failure path without changing request/result/checkpoint shapes.

## Cheap Check

1. Add RED tests for direct length-match verification and executor-routed length mismatch failure.
2. Implement the smallest verifier behind `DownloadSegmentVerifyPort`.
3. Re-export the verifier from `launcher-module-downloads`.
4. Run focused verifier tests, focused executor adapter tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED tests fail before production code because `DownloadSegmentLengthVerifyPort` is missing.
2. GREEN focused verifier tests pass.
3. Existing executor adapter tests pass.
4. Full downloads module tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.

## Completion Evidence

- RED: focused verifier tests failed before production code because `DownloadSegmentLengthVerifyPort` was missing.
- GREEN: `cargo test -p launcher-module-downloads --lib download_segment_length_verify_port --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 2/2 tests.
- Regression: `cargo test -p launcher-module-downloads --lib download_segment_executor_adapter --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 4/4 focused adapter tests.
- Regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 65/65 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --package launcher-module-downloads -- --check` passed.
