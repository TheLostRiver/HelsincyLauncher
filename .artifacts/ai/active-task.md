# Active Atomic Task

## Identity

- task id: AT-2026-05-15-156
- title: Add missing-checkpoint resume_download error semantics
- status: complete

## Goal

继续 `resume_download` 的第二个后端实现切片：在 AT-155 已证明 facade 会读取 checkpoint 的基础上，先定义缺失 checkpoint 的稳定 downloads-domain 错误语义。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` missing-checkpoint branch and module-level facade test

## Scope

- in scope:
  - add a failing module facade test proving `resume_download` returns a stable downloads error when checkpoint is missing
  - observe RED failure before production code changes
  - implement the minimal missing-checkpoint error branch after `checkpoint_repo.load()`
  - verify `code`, `retryable`, and `severity` for the missing-checkpoint branch
  - keep existing post-checkpoint `DOWNLOADS_NOT_WIRED` placeholder until a later full resume orchestration slice
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - implement full resume orchestration
  - add job lookup, staging validation, manifest reconstruction, or runtime enqueue-resume behavior
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
6. docs/TauriAIDevelopmentTransactionProtocolDesign.md
7. docs/TauriDownloadRuntimeDesign.md
8. docs/TauriKernelJobsRuntimeDesign.md
9. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
10. docs/TauriFirstCrateApiDrafts.md
11. docs/TauriIPCAndStateContractsDesign.md
12. docs/TauriErrorHandlingAndProjectionDesign.md
13. docs/modules/downloads/README_ARCH.md
14. docs/modules/downloads/README_API.md
15. docs/modules/downloads/README_FLOW.md
16. .artifacts/ai/findings.md

## Hypothesis

- falsifiable local hypothesis: If `resume_download` only loads checkpoint and then returns `DOWNLOADS_NOT_WIRED`, a new facade test expecting a stable `DL_CHECKPOINT_MISSING` error for `None` checkpoint will fail. After minimal implementation, that missing-checkpoint test should pass while the checkpoint-present branch still returns `DOWNLOADS_NOT_WIRED`.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download`

## Validation Gate

1. RED: focused missing-checkpoint `resume_download` test fails because current behavior still returns `DOWNLOADS_NOT_WIRED`.
2. GREEN: focused `resume_download` tests pass after adding the missing-checkpoint branch.
3. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`
4. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- RED observed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download` failed because the missing-checkpoint path returned `DOWNLOADS_NOT_WIRED` instead of `DL_CHECKPOINT_MISSING`.
- GREEN passed: same focused command passed after adding the minimal missing-checkpoint branch.
- Module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 7 tests.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-155 completed and was committed locally as `645dd93`.
- User approved option 1: missing-checkpoint error semantics before full resume orchestration.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
