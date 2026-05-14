# Active Atomic Task

## Identity

- task id: AT-2026-05-15-151
- title: Refresh downloads facade wiring comments
- status: completed

## Goal

继续 Phase 28 Backend Development Scope Recovery，在 AT-149/AT-150 已经完成 downloads pause/cancel 后端接线与 host smoke 覆盖后，修正 `crates/module-downloads/src/facade/mod.rs` 文件头里已经过期的 C2 stub 描述。

本轮只覆盖：

- `crates/module-downloads/src/facade/mod.rs` facade 文件头注释

## Scope

- in scope:
  - update the facade module comment so it says start/pause/cancel are wired
  - keep resume, queries, and policy entries documented as still `DOWNLOADS_NOT_WIRED`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change production behavior
  - change `resume_download` semantics
  - touch frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

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
3. docs/TauriCodeCommentStandard.md
4. docs/TauriDownloadRuntimeDesign.md
5. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If the facade header comment is refreshed to match the current wiring state, the module remains behaviorally unchanged while documentation no longer claims pause/cancel are C2 stubs.

## Cheap Check

- `cargo check -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`

## Validation Gate

1. Run `cargo check -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`
2. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- `cargo check -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-150 completed and was committed locally as `958a0e6`.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
- This task is comment-only and does not reopen `resume_download`.

## Completion Summary

- Refreshed the downloads facade file header so it reflects that start/pause/cancel are wired while resume, queries, and policy entries remain stubs.
- Production behavior was not changed in this slice.
