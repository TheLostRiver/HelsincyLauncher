# Active Atomic Task

## Identity

- task id: AT-2026-05-15-164
- title: Add downloads resume mismatch rejection coverage
- status: complete

## Goal

补上 downloads resume segment decision 的 stale manifest/checkpoint boundary 覆盖：当 `segment_id` 匹配但 `file_id`、`offset` 或 `length` 不匹配时，必须推导为 `reject_mismatch`，并且不能成为 runtime enqueue candidate。

本轮只覆盖：

- focused module test for mismatched segment checkpoint facts
- 验证现有 `build_resume_segment_decisions` safety branch
- 保持 completed sealing、partial resume、queue remaining、runtime enqueue 行为不扩大

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
  - implement mismatch public error projection
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

- falsifiable local hypothesis: Given a manifest segment and a checkpoint with the same `segment_id` but stale boundary facts, a focused module test can prove the derived resume decision is `reject_mismatch` and is not a runtime enqueue candidate, without touching runtime enqueue or persistence.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_reject_mismatched_checkpoint_segments`

## Validation Gate

1. Read required module docs and related backend/runtime/testing/collaboration docs before code.
2. Add the focused mismatch coverage test first.
3. Change production code only if the focused test exposes a real gap.
4. Run focused test, full `launcher-module-downloads` tests, scoped rustfmt check, and scoped diff checks.

## Validation Result

- passed
- Focused coverage test passed immediately: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_reject_mismatched_checkpoint_segments` reported 1 passed, 0 failed.
- Because the focused test passed without production edits, AT-2026-05-15-164 is confirmed as safety coverage for existing `RejectMismatch` behavior rather than a new behavior branch.
- Full downloads module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 13 passed, 0 failed, plus 0 doctests.
- `crates/module-downloads/src/facade/mod.rs` passed `rustfmt --check`.
- Scoped whitespace validation passed for the AT-2026-05-15-164 slice; Git only reported Windows LF/CRLF conversion warnings.

## Notes

- AT-2026-05-15-163 completed and was committed locally as `07ed4aa`.
- Current code already contains a `RejectMismatch` branch; this slice documents and verifies that safety behavior rather than widening resume orchestration.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
