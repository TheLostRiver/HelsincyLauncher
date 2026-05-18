# Active Atomic Task

## Identity

- task id: AT-2026-05-19-269
- title: Implement due retry-ready segment selector
- status: completed

## Goal

Implement the pure downloads helper that selects failed checkpoint segment facts whose policy-computed `next_retry_after` is due, without deriving executable retry work yet.

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - SQLite schema or adapter changes
  - manifest reconstruction or manifest-bound retry work derivation
  - scheduler loops, background workers, runtime dispatch, durable leases, or automatic retry execution
  - `TerminalFailed` decisions, job-level retry aggregation, public `DL_*` projection, host transport, frontend state, provider HTTP, production wiring, or snapshot error payload changes

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/lib.rs
3. README.md
4. docs/modules/downloads/README_IMPL.md
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md
9. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and Key Docs / roadmap notes.
2. `CONTRIBUTING.md` collaboration rules and minimal validation matrix.
3. `docs/README.md` docs map.
4. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` 7.47-7.48.
5. `docs/TauriDownloadRuntimeDesign.md` checkpoint/retry/segment ownership notes.
6. `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` and `docs/TauriFirstCrateApiDrafts.md` module crate/port boundaries.
7. `docs/TauriKernelJobsRuntimeDesign.md` runtime vs module checkpoint boundary.
8. `docs/TauriTestingStrategyAndQualityGateDesign.md` and `docs/TauriAIDevelopmentTransactionProtocolDesign.md`.
9. `docs/TauriCodeCommentStandard.md` comment-language and high-signal comment rules.

## Hypothesis

- falsifiable code hypothesis: a pure selector can return only due failed segment checkpoint facts in persisted checkpoint order when `status = Failed` and `next_retry_after <= now`, while excluding delayed, missing-time, and non-failed facts.

## Cheap Check

1. Add RED tests for due, delayed, missing-time, non-failed, and order-preservation branches.
2. Implement the smallest pure selector and export it from the crate entry if it becomes part of the module public surface.
3. Update README / README_IMPL status and next boundary after green.
4. Run focused downloads module tests, `cargo check -p launcher-composition-root`, rustfmt check for touched Rust files, and scoped `git diff --check`.
5. Commit and attempt push to `origin/main`.

## Validation Gate

1. Focused RED test fails before implementation for missing selector.
2. Focused selector tests pass after implementation.
3. Full `launcher-module-downloads --lib` tests pass.
4. Composition root check still passes.
5. No out-of-scope frontend, host transport, SQLite schema, scheduler/runtime dispatch, public `DL_*`, or terminal failure changes are introduced.

## Completion Evidence

- RED: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_retry_ready_selector --lib` failed first with unresolved `select_retry_ready_failed_segments`.
- GREEN: the same focused selector tests passed with 3 passed / 0 failed.
- Full module validation: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 82 passed / 0 failed.
- Assembly validation: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Formatting validation: `rustfmt --check crates\module-downloads\src\driver.rs crates\module-downloads\src\lib.rs` passed after formatting touched Rust files only.
- Scoped diff validation passed for README, README_IMPL, touched Rust files, and PWF files with CRLF normalization warnings only.
- No SQLite schema, manifest binding, scheduler/runtime dispatch, host transport, frontend, public `DL_*`, or terminal projection changes were introduced.
