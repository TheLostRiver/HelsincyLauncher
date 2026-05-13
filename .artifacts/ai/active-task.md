# Active Atomic Task

## Identity

- task id: AT-2026-05-14-115
- title: Annotate kernel-jobs runtime queue policy
- status: completed

## Goal

继续 Phase 23 Backend Comment Rollout，在不改变共享作业运行时队列策略结构或构造行为的前提下，为 `crates/kernel-jobs/src/runtime.rs` 中的 `RuntimeQueuePolicy` 契约补齐中文声明注释。

本轮只覆盖：

- `RuntimeQueuePolicy`
- `RuntimeQueuePolicy::new`

## Scope

- in scope:
  - add Chinese declaration comments to `RuntimeQueuePolicy`
  - add a Chinese field comment for `max_concurrent_jobs`
  - add a Chinese method comment for `RuntimeQueuePolicy::new`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
- out of scope:
  - change queue scheduling behavior, validation rules, struct fields, trait APIs, or runtime host behavior
  - document `JobDriver`, `JobDriverRegistry`, `JobSnapshotStore`, `SharedJobRuntimeHost`, or `JobRuntime` in this slice
  - modify model, adapter, frontend, database, transport, or composition files
  - touch unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, or `src/` changes already present in the worktree

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/TauriCodeCommentStandard.md
5. docs/TauriKernelJobsRuntimeDesign.md
6. docs/TauriCurrentRepoArchitectureOverview.md
7. docs/TauriAIDevelopmentTransactionProtocolDesign.md
8. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If this slice only adds high-signal Chinese declaration comments to `RuntimeQueuePolicy`, then the queue-policy contract will match the repository comment standard while preserving the compiled public API and runtime behavior.

## Cheap Check

- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`

## Validation Gate

1. `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`
2. `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/kernel-jobs/src/runtime.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md`

## Validation Result

- passed
- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- `git diff --check` passed for the scoped file set.

## Notes

- AT-2026-05-14-114 completed and was pushed as commit `76132f4`.
- `RuntimeQueuePolicy` is the smallest remaining `kernel-jobs` runtime surface and maps directly to the shared runtime design's queue-policy responsibility.

## 安全恢复点

- AT-2026-05-14-115 is validated and ready for publication. If work resumes before publishing, rerun the scoped `cargo check` and `git diff --check`, then publish only `crates/kernel-jobs/src/runtime.rs` plus the touched `.artifacts/ai` records.

## Completion Summary

- `RuntimeQueuePolicy`, `max_concurrent_jobs`, and `RuntimeQueuePolicy::new` now have Chinese declaration comments.
- The slice preserves queue policy fields, derives, constructor behavior, and runtime scheduling behavior.
- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
