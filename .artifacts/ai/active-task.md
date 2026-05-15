# Active Atomic Task

## Identity

- task id: AT-2026-05-15-165
- title: Add downloads resume queue remaining coverage
- status: complete

## Goal

补上 downloads resume segment decision 的 `queue_remaining` 覆盖：当 manifest segment 没有可用 segment checkpoint 时，必须推导为 `queue_remaining`，并且是 runtime enqueue candidate。

本轮只覆盖：

- focused module test for queueing a segment with no checkpoint
- 验证现有 `build_resume_segment_decisions` fallback branch
- 保持 completed sealing、partial resume、reject mismatch、runtime enqueue 行为不扩大

## Scope

- in scope:
  - update `crates/module-downloads/src/facade/mod.rs`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change frontend files
  - change host transport or composition-root wiring
  - change driver or SQLite adapter persistence
  - enqueue resumed runtime jobs
  - implement runtime resume execution
  - change sqlite database files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

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
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
10. docs/TauriFirstCrateApiDrafts.md
11. docs/TauriKernelJobsRuntimeDesign.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md
14. docs/TauriCodeCommentStandard.md

## Hypothesis

- falsifiable local hypothesis: Given a manifest segment with no matching segment checkpoint, a focused module test can prove the derived resume decision is `queue_remaining` and remains a runtime enqueue candidate, without touching runtime enqueue or persistence.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_queue_remaining_without_checkpoint`

## Validation Gate

1. Read required module docs and related backend/runtime/testing/collaboration docs before code.
2. Add the focused queue-remaining coverage test first.
3. Change production code only if the focused test exposes a real gap.
4. Run focused test, full `launcher-module-downloads` tests, scoped rustfmt check, and scoped diff checks.

## Validation Result

- passed
- Focused coverage test passed immediately: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_queue_remaining_without_checkpoint` reported 1 passed, 0 failed.
- Because the focused test passed without production edits, AT-2026-05-15-165 is confirmed as safety coverage for existing `QueueRemaining` fallback behavior rather than a new behavior branch.
- Full downloads module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 14 passed, 0 failed, plus 0 doctests.
- `crates/module-downloads/src/facade/mod.rs` passed `rustfmt --check`.
- Scoped whitespace validation passed for the AT-2026-05-15-165 slice; Git only reported Windows LF/CRLF conversion warnings.

## Notes

- AT-2026-05-15-164 completed and was committed locally as current HEAD `ba06e7c`.
- Current code already falls back to `QueueRemaining`; this slice documents and verifies that behavior before runtime enqueue work.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
