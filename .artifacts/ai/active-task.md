# Active Atomic Task

## Identity

- task id: AT-2026-05-19-256
- title: Implement downloads composition-root static executor wiring proof
- status: completed

## Goal

Implement README_IMPL 7.42 with a focused composition-root TDD proof that an explicitly supplied static source map can wire `DownloadSegmentExecutor` into a downloads driver while default desktop production still defers when no execution port is wired.

## Scope

- in scope:
  - `crates/composition-root/src/bootstrap.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `README.md` and durable docs unless implementation reveals a documented boundary error
  - `module-downloads` behavior changes
  - default desktop production execution-port wiring
  - real HTTP range requests, provider auth, CDN policy, retry/backoff, streaming workers
  - public `DL_NETWORK_*` / `DL_PROVIDER_*` / `DL_WRITE_FAILED` / `DL_VERIFY_FAILED` projection
  - host transport, frontend, SQLite schema, or unrelated dirty files

## Allowed Files

1. crates/composition-root/src/bootstrap.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `docs/modules/downloads/README_IMPL.md` 7.42.
2. `docs/TauriCompositionRootWiringDesign.md` composition-root assembly owner rules.
3. `docs/TauriDownloadRuntimeDesign.md` staging/fetch/write/verify boundaries.
4. `docs/TauriTestingStrategyAndQualityGateDesign.md` focused backend test guidance.
5. `docs/TauriCodeCommentStandard.md` Chinese-first and boundary-comment rules.
6. `crates/composition-root/src/bootstrap.rs` downloads driver registry and tests.
7. `crates/module-downloads/src/driver.rs` `DownloadJobDriver::run(...)`, executor, writer, verifier, and checkpoint mutation behavior.

## Hypothesis

- falsifiable implementation hypothesis: a private composition-root helper can build a downloads driver with `DownloadSegmentExecutor` from explicit static sources and `app_data_dir/.downloads/staging`, allowing a focused test to run one queued segment to `Completed`, while the default `build_desktop_services(...)` path still returns the existing deferred disposition.

## Cheap Check

1. Add a RED composition-root test that calls the missing private static executor wiring helper.
2. Verify the test fails because the helper does not exist.
3. Implement the smallest helper using `DownloadSegmentStaticFetchPort`, `DownloadSegmentFilesystemWritePort`, `DownloadSegmentLengthVerifyPort`, and `DownloadJobDriver::with_pending_resume_work_source_and_execution_port(...)`.
4. Re-run the focused test and the existing default-deferred smoke path.
5. Run composition-root check/tests, scoped rustfmt, scoped diff-check, then commit and push.

## Validation Gate

1. RED failure observed before production/helper implementation.
2. Focused composition-root static executor wiring test passes.
3. Default desktop production no-execution-port/deferred behavior remains covered.
4. `cargo check -p launcher-composition-root` passes.
5. Scoped rustfmt and diff-check pass.

## Completion Evidence

- RED: `cargo test -p launcher-composition-root static_download_executor_wiring_records_completed_checkpoint_and_writes_staging_file --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` failed with `cannot find function build_download_job_driver_with_static_segment_executor`.
- GREEN focused: the same focused test passed after adding the private static executor helper.
- Focused regression: `cargo test -p launcher-composition-root download --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 5 bootstrap tests and 3 smoke tests.
- Composition-root regression: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 14 unit tests and 7 integration tests.
- Compile gate: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Module regression: `cargo test -p launcher-module-downloads --lib --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 71/71 tests.
- Format gate: `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --package launcher-composition-root -- --check` passed after scoped formatting.
