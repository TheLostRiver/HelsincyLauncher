# Active Atomic Task

## Identity

- task id: AT-2026-05-15-149
- title: Wire downloads pause and cancel runtime controls
- status: completed

## Goal

继续 Phase 28 Backend Development Scope Recovery，在不触碰前端和不改变 resume 语义的前提下，把 downloads facade 的 pause/cancel 命令接到共享 `JobRuntime` 控制端口。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` pause/cancel control slice

## Scope

- in scope:
  - add failing tests proving `pause_download` delegates to `JobRuntime::pause`
  - add failing tests proving `cancel_download` delegates to `JobRuntime::cancel`
  - implement the minimal facade delegation after RED is observed
  - leave `resume_download` out of scope because its `AcceptedJob` return requires a separate design slice
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change `resume_download`, transport handlers, shared runtime semantics, frontend files, or unrelated dirty worktree files
  - touch unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, or `src/` changes already present in the worktree

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/TauriCodeCommentStandard.md
5. docs/TauriDownloadRuntimeDesign.md
6. docs/TauriKernelJobsRuntimeDesign.md
7. docs/TauriCurrentRepoArchitectureOverview.md
8. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If `pause_download` and `cancel_download` delegate to `JobRuntime::pause/cancel`, then module-downloads can stop returning `DOWNLOADS_NOT_WIRED` for those two backend control commands while keeping resume and transport behavior unchanged.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_delegates_to_runtime_control`

## Validation Gate

1. RED: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_delegates_to_runtime_control` fails because pause/cancel still return `DOWNLOADS_NOT_WIRED`
2. GREEN: same command passes after minimal facade delegation
3. `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- RED observed: focused pause/cancel tests failed with `DOWNLOADS_NOT_WIRED`.
- GREEN passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_delegates_to_runtime_control`.
- Module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 5 tests.
- `git diff --check` passed for the scoped file set; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-148 completed and was committed locally as `a13a2e6`.
- Push remains blocked for direct `origin/main` mutation; per user rule, continue without bypassing push review.
- Existing English comments must be preserved; this task changes behavior only after the RED tests demonstrate the missing delegation.

## 安全恢复点

- AT-2026-05-15-149 is validated and ready for publication. If work resumes before publishing, inspect only `crates/module-downloads/src/facade/mod.rs` plus the touched `.artifacts/ai` records, then commit those files only.

## Completion Summary

- Added focused RED tests for downloads pause/cancel delegation, observed the expected `DOWNLOADS_NOT_WIRED` failure, then wired `pause_download` and `cancel_download` to `JobRuntime::pause/cancel`.
- `resume_download` remains unchanged and out of scope for a later resume-acceptance slice.
