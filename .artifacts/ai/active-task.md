# Active Atomic Task

## Identity

- task id: AT-2026-05-19-254
- title: Add downloads static segment fetcher
- status: completed

## Goal

Implement README_IMPL 7.41 as a deterministic static byte-source fetcher behind `DownloadSegmentFetchPort`.

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
  - real HTTP range requests
  - provider authentication, CDN policy, or retry/backoff
  - public `DL_NETWORK_*` / `DL_PROVIDER_*` projection
  - streaming fetch, backpressure, or Tokio worker pools
  - composition-root production execution-port wiring
  - host transport, frontend, or SQLite schema changes
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

1. `docs/modules/downloads/README_IMPL.md` 7.41.
2. `DownloadSegmentFetchPort`, `DownloadSegmentFetchOutcome`, and `DownloadSegmentFetchResult` code.
3. `DownloadResumeWorkMode` / `DownloadSegmentExecutionRequest` offset semantics.
4. Existing executor/fetch test helpers and crate re-export surface.

## Hypothesis

- falsifiable implementation hypothesis: `DownloadSegmentStaticFetchPort` can return configured bytes and etag for from-start requests, return only remaining bytes for partial requests, map missing locators and impossible partial offsets to handled fetch failures, and feed the existing executor completion path with the static fetcher plus existing writer/verifier ports.

## Cheap Check

1. Add RED tests for from-start bytes+etag, partial remaining bytes, missing locator handled failure, and executor completion with the static fetcher.
2. Implement the smallest static fetcher and source record.
3. Re-export the fetcher/source from `launcher-module-downloads`.
4. Update README_IMPL implementation status.
5. Run focused fetcher tests, focused executor adapter tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Validation Gate

1. RED tests fail before production code because static fetcher types are missing.
2. GREEN focused fetcher tests pass.
3. Existing executor adapter tests pass.
4. Full downloads module tests and composition-root compile gate pass.
5. README_IMPL records the implemented static fetcher status.
6. Scoped rustfmt and diff-check pass before commit/push.

## Completion Evidence

- RED: focused static fetcher tests failed before production code because `DownloadSegmentStaticFetchPort` and `DownloadSegmentStaticFetchSource` were missing.
- GREEN: `cargo test -p launcher-module-downloads --lib download_segment_static_fetch_port --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 5/5 tests after implementation and formatting.
- Regression: `cargo test -p launcher-module-downloads --lib download_segment_executor_adapter --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 4/4 focused adapter tests.
- Regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 71/71 tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Format gate: `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --package launcher-module-downloads -- --check` passed after scoped formatting.
- README_IMPL now records the static fetcher implementation status and the next composition-root wiring boundary.
