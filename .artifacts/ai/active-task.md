# Active Atomic Task

## Identity

- task id: AT-2026-05-14-112
- title: Annotate kernel-jobs restore disposition contract
- status: completed

## Goal

继续 Phase 23 Backend Comment Rollout，在不改变共享作业模型行为或恢复语义的前提下，为 `crates/kernel-jobs/src/model.rs` 中下一组最小恢复结果契约补齐中文声明注释。

本轮只覆盖：

- `RestoreDisposition`

## Scope

- in scope:
  - add Chinese declaration comments in `crates/kernel-jobs/src/model.rs`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change enum variants, variant payloads, or restore behavior
  - document `JobSnapshot` or `JobSnapshotDto` in this slice
  - modify runtime, adapter, frontend, database, or transport files
  - touch unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, or `src/` changes already present in the worktree

## Allowed Files

1. crates/kernel-jobs/src/model.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

1. docs/TauriCodeCommentStandard.md
2. docs/TauriKernelJobsRuntimeDesign.md
3. docs/TauriFirstCrateApiDrafts.md
4. .artifacts/ai/handoff.md

## Hypothesis

- falsifiable local hypothesis: If this slice adds only high-signal Chinese declaration comments to `RestoreDisposition` and its variants, then the restore result contract will comply better with the repository comment standard while preserving the compiled public API shape.

## Cheap Check

- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`

## Validation Gate

1. `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`
2. `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/kernel-jobs/src/model.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- `git diff --check` passed for the scoped file set.

## Notes

- AT-2026-05-14-111 completed the immediately preceding accepted/enqueue contract comments and was pushed as part of commit `7ff33fb`.
- This slice stops before snapshot contracts so Phase 23 keeps moving in small, reviewable units.

## 安全恢复点

- AT-2026-05-14-112 is validated and ready for publication. If work resumes before publishing, rerun the scoped `cargo check` and `git diff --check`, then publish only `crates/kernel-jobs/src/model.rs` plus the touched `.artifacts/ai` records.

## Completion Summary

- `RestoreDisposition` and its variants now have Chinese declaration comments.
- The slice preserves enum variants, variant payloads, and restore behavior.
- `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
