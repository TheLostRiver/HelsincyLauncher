# Active Atomic Task

## Identity

- task id: AT-2026-05-14-131
- title: Localize downloads transport command comments
- status: completed

## Goal

继续 Phase 23 Backend Comment Rollout，在不改变 downloads transport handler 行为、`DOWNLOADS_NOT_WIRED` fallback 或 accepted-job 投影路径的前提下，把 `src-tauri/src/commands/downloads.rs` 中的英文注释改为高信号中文注释。

本轮只覆盖：

- `src-tauri/src/commands/downloads.rs`

## Scope

- in scope:
  - replace English module and function comments with Chinese comments
  - keep comments aligned with downloads facade, backend-owned runtime/checkpoint boundary, query fallback, and accepted-job projection semantics
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
- out of scope:
  - change handler signatures, DTO mapping, fallback payloads, facade calls, policy defaults, command registration, or Tauri wiring
  - modify downloads module contracts/facade, composition-root, storage adapters, runtime logic, or frontend files
  - touch unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, or `src/` changes already present in the worktree

## Allowed Files

1. src-tauri/src/commands/downloads.rs
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
8. docs/TauriDownloadRuntimeDesign.md
9. docs/modules/downloads/README_ARCH.md
10. docs/modules/downloads/README_API.md
11. docs/modules/downloads/README_FLOW.md
12. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If this slice only localizes `src-tauri/src/commands/downloads.rs` comments and leaves the handler implementation untouched, then downloads transport documentation will match the repository comment policy while preserving compiled behavior, query fallback behavior, policy stub defaults, and accepted-job projection behavior.

## Cheap Check

- `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib`
2. `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- src-tauri/src/commands/downloads.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md`

## Validation Result

- passed
- `cargo check -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- `git diff --check` passed for the scoped file set.

## Notes

- AT-2026-05-14-130 completed and was committed locally as `4d0e1f2`.
- Push remains blocked for direct `origin/main` mutation; per user rule, continue without bypassing push review.
- Current downloads facade has a real `start_download` accepted-job path; pause/resume/cancel/query/policy paths still preserve `DOWNLOADS_NOT_WIRED` semantics.
- Comments must keep frontend/transport as intent and projection surfaces, not imply ownership of scheduler, checkpoint, writer, verifier, or provider logic.

## 安全恢复点

- AT-2026-05-14-131 is validated and ready for publication. If work resumes before publishing, rerun the scoped `cargo check` and `git diff --check`, then publish only `src-tauri/src/commands/downloads.rs` plus the touched `.artifacts/ai` records.

## Completion Summary

- `src-tauri/src/commands/downloads.rs` module and handler comments are now Chinese.
- The localized comments preserve the `DOWNLOADS_NOT_WIRED` query fallback boundary, policy stub defaults, and backend accepted-job projection semantics.
