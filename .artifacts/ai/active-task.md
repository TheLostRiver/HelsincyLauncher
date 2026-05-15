# Active Atomic Task

## Identity

- task id: AT-2026-05-15-162
- title: Add downloads resume sealed segment decision
- status: complete

## Goal

按 AT-161 的 README_IMPL 数据形状，新增最小 completed-segment sealing 代码契约：completed segment checkpoint 应导出 `seal_completed` resume decision，且不是 runtime enqueue candidate。

本轮只覆盖：

- manifest segment 最小数据结构
- segment checkpoint 最小数据结构与状态
- resume segment decision/action 最小数据结构
- 纯函数决策：completed checkpoint -> sealed decision

## Scope

- in scope:
  - update `crates/module-downloads/src/driver.rs`
  - update `crates/module-downloads/src/facade/mod.rs`
  - update `crates/module-downloads/src/lib.rs`
  - update `crates/adapter-storage-sqlite/src/lib.rs` only if needed to preserve `DownloadCheckpointRecord` compatibility without schema changes
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change frontend files
  - change host transport or composition-root wiring
  - change SQLite schema
  - persist segment checkpoints in SQLite
  - enqueue resumed runtime jobs
  - implement partial resume or mismatch error semantics
  - change sqlite database files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/facade/mod.rs
3. crates/module-downloads/src/lib.rs
4. crates/adapter-storage-sqlite/src/lib.rs
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md
9. .artifacts/ai/handoff.md

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

- falsifiable local hypothesis: Given a manifest segment and a matching completed segment checkpoint with `downloaded_bytes == length`, a focused module test can prove the derived resume decision is `seal_completed` and is not a runtime enqueue candidate, without touching SQLite schema or actual runtime enqueue.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_seal_completed_checkpoint_segments`

## Validation Gate

1. Read required module docs and related backend/runtime/testing/collaboration docs before code.
2. Write the RED facade/module test first.
3. Implement minimal data structures and pure decision function.
4. Preserve current adapter compatibility without schema changes.
5. Run focused test, full `launcher-module-downloads` tests, and scoped diff checks.

## Validation Result

- passed
- RED compile failure was observed first: the focused test failed because segment manifest/checkpoint/decision types and `build_resume_segment_decisions` did not exist.
- Focused GREEN test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_segment_decisions_seal_completed_checkpoint_segments` reported 1 passed, 0 failed.
- Full downloads module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 11 passed, 0 failed.
- Adapter compatibility test passed: `cargo test -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
- Touched module-downloads files passed scoped rustfmt checks: driver/facade directly, crate entry with `skip_children=true` to avoid unrelated contracts drift.
- Scoped whitespace validation passed for the AT-2026-05-15-162 slice.

## Notes

- AT-2026-05-15-161 completed and was committed locally as `5e08cd2`.
- New comments should be bilingual for declaration-level additions, preserving existing English comments.
