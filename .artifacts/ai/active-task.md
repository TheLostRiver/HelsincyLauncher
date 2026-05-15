# Active Atomic Task

## Identity

- task id: AT-2026-05-15-155
- title: Add checkpoint-aware resume_download RED test
- status: complete

## Goal

开始 `resume_download` 的第一个后端实现切片：先用 RED test 证明 `resume_download` 必须显式读取 `DownloadCheckpointRepository`，避免把恢复语义误做成简单 `JobRuntime::resume` passthrough。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` resume checkpoint-read test and minimal behavior

## Scope

- in scope:
  - add a failing module facade test proving `resume_download` consults checkpoint repository
  - observe RED failure before production code changes
  - implement the minimal checkpoint read while leaving full manifest/staging enqueue-resume out of scope
  - keep `DOWNLOADS_NOT_WIRED` as the post-checkpoint placeholder until the full resume orchestration slice
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - implement full resume orchestration
  - add manifest provider or staging store behavior
  - change host transport, frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/TauriArchitecturePrinciplesDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md
6. docs/TauriDownloadRuntimeDesign.md
7. docs/TauriKernelJobsRuntimeDesign.md
8. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
9. docs/TauriFirstCrateApiDrafts.md
10. docs/modules/downloads/README_ARCH.md
11. docs/modules/downloads/README_API.md
12. docs/modules/downloads/README_FLOW.md
13. .artifacts/ai/findings.md

## Hypothesis

- falsifiable local hypothesis: If `resume_download` is still a direct stub, a new facade test expecting checkpoint repository access will fail because no checkpoint load occurs. After minimal implementation, the same test should pass while full resume remains explicitly not wired.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download`

## Validation Gate

1. RED: focused `resume_download` test fails because current stub does not load checkpoint.
2. GREEN: focused `resume_download` test passes after minimal checkpoint read.
3. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`
4. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- RED observed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download` failed because `resume_download` did not load the checkpoint repository; `loaded_job_ids` was empty.
- GREEN passed: same focused command passed after adding the minimal checkpoint load.
- Module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 6 tests.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-154 completed and was committed locally as `71b0ee1`.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
- User approved starting this checkpoint-aware first implementation slice.
