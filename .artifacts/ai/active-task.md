# Active Atomic Task

## Identity

- task id: AT-2026-05-15-157
- title: Add resume_download job lookup error semantics
- status: complete

## Goal

继续 `resume_download` 的第三个后端实现切片：在 checkpoint 缺失分支之后，补上文档要求的模块 job lookup 前置条件。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` job lookup order and missing-job error branch

## Scope

- in scope:
  - add a failing module facade test proving `resume_download` checks `DownloadJobRepository` before checkpoint
  - add a failing module facade test/branch for missing job record returning stable downloads-domain error
  - observe RED failure before production code changes
  - implement the minimal job lookup branch before checkpoint loading
  - verify `code`, `retryable`, and `severity` for missing job record
  - preserve existing `DL_CHECKPOINT_MISSING` and post-checkpoint `DOWNLOADS_NOT_WIRED` behavior
  - add or preserve bilingual English/Chinese source comments for new production code
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - implement full resume orchestration
  - add staging validation, manifest reconstruction, or runtime enqueue-resume behavior
  - change host transport, frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/TauriArchitecturePrinciplesDesign.md
5. docs/TauriTestingStrategyAndQualityGateDesign.md
6. docs/TauriDownloadRuntimeDesign.md
7. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
8. docs/TauriFirstCrateApiDrafts.md
9. docs/TauriIPCAndStateContractsDesign.md
10. docs/TauriErrorHandlingAndProjectionDesign.md
11. docs/modules/downloads/README_ARCH.md
12. docs/modules/downloads/README_API.md
13. .artifacts/ai/findings.md

## Hypothesis

- falsifiable local hypothesis: If `resume_download` still reads checkpoint before `DownloadJobRepository`, a new facade test expecting `DL_JOB_NOT_FOUND` without checkpoint access will fail. After minimal implementation, missing job should stop before checkpoint, while existing checkpoint branches still pass.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download`

## Validation Gate

1. RED: focused `resume_download` test fails because current behavior does not check the job repository first.
2. GREEN: focused `resume_download` tests pass after adding the missing-job branch.
3. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`
4. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- RED observed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download` failed because `resume_download` did not call `DownloadJobRepository::get_job()` before checkpoint.
- GREEN passed: same focused command passed after adding the minimal job lookup branch.
- Module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 8 tests.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-156 completed and was committed locally as `b3bfb1f`.
- User asked to continue to the next backend slice.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
