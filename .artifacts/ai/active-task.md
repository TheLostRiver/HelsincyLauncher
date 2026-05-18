# Active Atomic Task

## Identity

- task id: AT-2026-05-19-249
- title: Add downloads filesystem staging writer
- status: completed

## Goal

Implement README_IMPL 7.39 as a concrete module-local filesystem staging writer behind `DownloadSegmentWritePort`.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - real HTTP range requests or provider object fetches
  - final artifact moves or hash verification
  - host file-system canonicalization
  - composition-root production execution-port wiring
  - retry/backoff policy
  - terminal runtime completion/failure projection
  - host transport, frontend, or SQLite schema changes
  - public `DL_*` execution error DTOs
  - unrelated dirty files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README/docs routing and documentation-budget rules.
2. `docs/modules/downloads/README_IMPL.md` 7.39.
3. `docs/TauriDownloadRuntimeDesign.md` SegmentWriter, sparse/partial writes, staging-first, and cleanup sections.
4. `docs/TauriStorageAndDatabaseDesign.md` staging file ownership and SQLite boundary notes.
5. Existing `DownloadSegmentStagingTarget`, `DownloadSegmentWritePort`, and `DownloadResumeWorkMode` code.

## Hypothesis

- falsifiable implementation hypothesis: `DownloadSegmentFilesystemWritePort` can create job-scoped staging parents, write from-start bytes, preserve existing prefixes for partial writes at `start_offset`, and return existing write facts without production wiring.

## Cheap Check

1. Add RED tests for job-scoped from-start writes and partial writes preserving an existing prefix.
2. Implement the smallest filesystem writer behind `DownloadSegmentWritePort`.
3. Re-export the writer from `launcher-module-downloads`.
4. Update README_IMPL to mark the filesystem writer implemented and point the next slice at verifier work.
5. Run focused writer tests, executor adapter tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED tests fail before production code because the filesystem writer type is missing.
2. GREEN focused filesystem writer tests pass.
3. Existing executor adapter tests pass.
4. Full downloads module tests and composition-root compile gate pass.
5. Scoped rustfmt and diff-check pass before commit/push.

## Completion Evidence

- RED: focused writer tests failed before production code because `DownloadSegmentFilesystemWritePort` was missing.
- GREEN: `cargo test -p launcher-module-downloads --lib download_segment_filesystem_write_port --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 2/2 tests.
- Regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 63/63 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --package launcher-module-downloads -- --check` passed after scoped formatting.
- README_IMPL now records the implemented writer status and the next verifier-shell slice.
