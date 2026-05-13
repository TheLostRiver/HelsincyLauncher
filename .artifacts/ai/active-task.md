# Active Atomic Task

## Identity

- task id: AT-2026-05-14-116
- title: Annotate kernel-jobs driver contract
- status: completed

## Goal

继续 Phase 23 Backend Comment Rollout，在不改变共享作业驱动 trait API 或恢复行为的前提下，为 `crates/kernel-jobs/src/runtime.rs` 中的 `JobDriver<E>` 契约补齐中文声明注释。

本轮只覆盖：

- `JobDriver<E>`
- `JobDriver<E>::module`
- `JobDriver<E>::kind`
- `JobDriver<E>::restore`

## Scope

- in scope:
  - add Chinese declaration comments to `JobDriver<E>`
  - add Chinese method comments for the trait methods
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
- out of scope:
  - change trait bounds, method signatures, restore semantics, driver registry behavior, or runtime host behavior
  - document `JobDriverRegistry`, `JobSnapshotStore`, `SharedJobRuntimeHost`, or `JobRuntime` in this slice
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

- falsifiable local hypothesis: If this slice only adds high-signal Chinese declaration comments to `JobDriver<E>` and its methods, then the module-driver routing and restore boundary will match the repository comment standard while preserving the compiled public API and runtime behavior.

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

- AT-2026-05-14-115 completed and was pushed as commit `390fc9b`.
- `JobDriver<E>` is the next smallest runtime contract surface after the queue policy and maps to the shared runtime design's module driver registry boundary.

## 安全恢复点

- AT-2026-05-14-116 is validated and ready for publication. If work resumes before publishing, rerun the scoped `cargo check` and `git diff --check`, then publish only `crates/kernel-jobs/src/runtime.rs` plus the touched `.artifacts/ai` records.

## Completion Summary

- `JobDriver<E>` and its `module`, `kind`, and `restore` methods now have Chinese declaration comments.
- The slice preserves trait bounds, method signatures, driver routing semantics, and restore behavior.
- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
