# Active Atomic Task

## Identity

- task id: AT-2026-05-14-129
- title: Localize shared jobs transport query comments
- status: completed

## Goal

继续 Phase 23 Backend Comment Rollout，在不改变 shared jobs transport query 行为或跨模块 snapshot 查询路径的前提下，把 `src-tauri/src/commands/jobs.rs` 中的英文注释改为高信号中文注释。

本轮只覆盖：

- `src-tauri/src/commands/jobs.rs`

## Scope

- in scope:
  - replace English `jobs_list_active` comments with Chinese comments
  - keep the comment aligned with the current `JobSnapshotStore::list_resumable` query semantics
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
- out of scope:
  - change handler signature, snapshot store query behavior, DTO mapping, command registration, or Tauri wiring
  - modify `src-tauri/src/commands/mod.rs`, composition-root, kernel-jobs, storage adapters, or frontend files
  - touch unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, or `src/` changes already present in the worktree

## Allowed Files

1. src-tauri/src/commands/jobs.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/TauriCodeCommentStandard.md
5. docs/TauriIPCAndStateContractsDesign.md
6. docs/TauriCurrentRepoArchitectureOverview.md
7. docs/TauriBackendSkeletonImplementationDesign.md
8. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If this slice only localizes `src-tauri/src/commands/jobs.rs` comments and leaves the handler implementation untouched, then the shared jobs transport query documentation will match the repository comment policy while preserving compiled behavior and current resumable snapshot semantics.

## Cheap Check

- `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`
2. `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- src-tauri/src/commands/jobs.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md`

## Validation Result

- passed
- `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- `git diff --check` passed for the scoped file set.

## Notes

- AT-2026-05-14-128 completed and was committed locally as `206e603`.
- Push remains blocked for direct `origin/main` mutation; per user rule, continue without bypassing push review.
- The current handler reads `JobSnapshotStore::list_resumable`, so comments must describe the resumable snapshot query rather than overclaiming all active/non-terminal jobs.

## 安全恢复点

- AT-2026-05-14-129 is validated and ready for publication. If work resumes before publishing, rerun the scoped `cargo check` and `git diff --check`, then publish only `src-tauri/src/commands/jobs.rs` plus the touched `.artifacts/ai` records.

## Completion Summary

- `src-tauri/src/commands/jobs.rs` now uses Chinese comments for the shared jobs transport query.
- The localized comments describe the current `JobSnapshotStore::list_resumable` semantics without changing handler behavior.
