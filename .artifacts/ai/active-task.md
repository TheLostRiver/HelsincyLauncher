# Active Atomic Task

## Identity

- task id: AT-2026-05-15-163
- title: Add downloads resume partial segment decision
- status: complete

## Goal

按 README_IMPL 的 partial resume 规则，让 matching partial segment checkpoint 导出 `resume_partial` decision，并保持它是 runtime enqueue candidate；仍不做实际 runtime enqueue。

本轮只覆盖：

- focused RED test for partial segment checkpoint
- `build_resume_segment_decisions` 的 partial branch
- 保持 completed sealing、queue remaining、reject mismatch 行为不扩大

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
  - implement mismatch error projection
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

- falsifiable local hypothesis: Given a manifest segment and a matching in-progress segment checkpoint with `0 < downloaded_bytes < length`, a focused module test can prove the derived resume decision is `resume_partial` and remains a runtime enqueue candidate, without touching runtime enqueue or persistence.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_resume_partial_checkpoint_segments`

## Validation Gate

1. Read required module docs and related backend/runtime/testing/collaboration docs before code.
2. Write the RED test first.
3. Implement only the partial branch.
4. Run focused test, full `launcher-module-downloads` tests, scoped rustfmt check, and scoped diff checks.

## Validation Result

- passed
- RED failure was observed first: focused test returned `QueueRemaining` where `ResumePartial` was expected.
- Focused GREEN test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_resume_partial_checkpoint_segments` reported 1 passed, 0 failed.
- Full downloads module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 12 passed, 0 failed.
- `crates/module-downloads/src/facade/mod.rs` passed `rustfmt --check`.
- Scoped whitespace validation passed for the AT-2026-05-15-163 slice.

## Notes

- AT-2026-05-15-162 completed and was committed locally as `f7afcd2`.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
