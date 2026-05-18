# Active Atomic Task

## Identity

- task id: AT-2026-05-19-252
- title: Correct downloads length verifier partial resume semantics
- status: completed

## Goal

Fix the concrete length verifier so partial resume verification treats `request.start_offset + written.downloaded_bytes == request.length` as a completed segment, while keeping from-start behavior unchanged.

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
  - fetcher boundary or HTTP range implementation
  - hash algorithm verification
  - file-level or job-level manifest sealing
  - retry/backoff policy
  - public `DL_VERIFY_FAILED` DTO projection
  - composition-root production execution-port wiring
  - host transport, frontend, or SQLite schema changes
  - unrelated dirty files

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

1. `docs/modules/downloads/README_IMPL.md` 7.40 verifier boundary.
2. `DownloadSegmentExecutionRequest` field comments for `start_offset`, `length`, and `resume_mode`.
3. `DownloadResumeWorkItem` derivation showing partial items use `start_offset = checkpoint.downloaded_bytes` and `length = segment.length`.
4. Current `DownloadSegmentLengthVerifyPort` implementation and focused verifier tests.

## Root Cause

- AT-251 implemented the documented direct comparison `written.downloaded_bytes == request.length`.
- While preparing the fetcher boundary, code inspection showed partial resume work stores the already completed byte count in `request.start_offset` while `length` remains the total segment length.
- The writer reports bytes written in the current operation, not total segment completion, so partial verification must add the starting offset before comparing against total length.

## Hypothesis

- falsifiable implementation hypothesis: a RED test with `resume_mode = Partial`, `start_offset = 6`, `length = 10`, and `written.downloaded_bytes = 4` should fail on the current verifier and pass after the verifier computes completed bytes as `start_offset + written.downloaded_bytes` for partial requests.

## Cheap Check

1. Add RED test for partial resume completion.
2. Update README_IMPL 7.40 to document from-start versus partial length semantics.
3. Implement the smallest verifier fix.
4. Run focused verifier tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED partial verifier test fails before production fix.
2. Focused verifier tests pass after the fix.
3. Full downloads module tests and composition-root compile gate pass.
4. Scoped rustfmt and diff-check pass before commit/push.

## Completion Evidence

- RED: `cargo test -p launcher-module-downloads --lib download_segment_length_verify_port_accepts_partial_completion_from_start_offset --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` failed because partial completion returned a handled mismatch failure.
- GREEN: `cargo test -p launcher-module-downloads --lib download_segment_length_verify_port --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 3/3 focused verifier tests.
- Regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 66/66 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --package launcher-module-downloads -- --check` passed.
