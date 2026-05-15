# Active Atomic Task

## Identity

- task id: AT-2026-05-15-158
- title: Add resume_download staging validation boundary
- status: complete

## Goal

继续 `resume_download` 的第四个后端实现切片：在 job 记录和 checkpoint 都存在后，证明 facade 会进入 staging 校验边界，再把完整 manifest/runtime 编排留到后续任务。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` staging object-store port and resume staging call

## Scope

- in scope:
  - add a failing module facade test proving `resume_download` calls staging validation after job and checkpoint are present
  - observe RED failure before production code changes
  - define the minimal `DownloadStagingObjectStore` boundary needed by current facade
  - implement the minimal staging call while preserving post-staging `DOWNLOADS_NOT_WIRED`
  - preserve existing `DL_JOB_NOT_FOUND` and `DL_CHECKPOINT_MISSING` behavior
  - add or preserve bilingual English/Chinese source comments for new production code
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - implement real filesystem staging validation
  - add manifest reconstruction or runtime enqueue-resume behavior
  - change composition-root wiring beyond keeping current `()` placeholder compatible
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
3. docs/TauriDownloadRuntimeDesign.md
4. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
5. docs/TauriFirstCrateApiDrafts.md
6. docs/modules/downloads/README_ARCH.md
7. .artifacts/ai/findings.md

## Hypothesis

- falsifiable local hypothesis: If `resume_download` still stops after checkpoint without a staging boundary, a new facade test expecting `DownloadStagingObjectStore::ensure_staging_root()` to be called will fail. After minimal implementation, the staging call should be observed while the result still returns `DOWNLOADS_NOT_WIRED`.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download`

## Validation Gate

1. RED: focused `resume_download` test fails because staging validation is not called or the staging port is not defined.
2. GREEN: focused `resume_download` tests pass after adding the minimal staging boundary.
3. Run `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`
4. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- RED observed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download` failed to compile because `DownloadStagingObjectStore` and `DownloadStagingRoot` were not defined yet.
- GREEN passed: same focused command passed after adding the minimal staging boundary and `resume_download` staging call.
- Module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 9 tests.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-157 completed and was committed locally as `2dc46c4`.
- User requested the next task.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
