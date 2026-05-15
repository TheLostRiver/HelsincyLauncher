# Active Atomic Task

## Identity

- task id: AT-2026-05-15-161
- title: Document downloads resume segment checkpoint shape
- status: complete

## Goal

在继续实现 completed-segment sealing 之前，把 downloads resume 需要的 manifest segment、segment checkpoint、resume decision 三层数据形状和不变量写入模块实现文档。

本轮只覆盖：

- 更新 `docs/modules/downloads/README_IMPL.md`
- 记录 AT-160 后的 manifest provider 现状
- 明确下一轮代码的最小数据契约和验证入口

## Scope

- in scope:
  - update `docs/modules/downloads/README_IMPL.md`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change Rust production code
  - change frontend files
  - change host transport or composition-root wiring
  - implement concrete SQLite schema changes
  - enqueue resumed runtime jobs
  - change sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
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

## Hypothesis

- falsifiable local hypothesis: If README_IMPL names the minimal segment/checkpoint/resume-decision shape and invariants, the next code slice can write a focused RED test for completed-segment sealing without inventing segment fields inside implementation.

## Cheap Check

- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Read README, collaboration docs, downloads module docs, README_IMPL, download runtime, first crate API, kernel jobs runtime, testing, and AI transaction docs before editing.
2. Update README_IMPL only as the module-local implementation guide.
3. Keep the update aligned with existing `TauriDownloadRuntimeDesign.md` segment/checkpoint fields.
4. Run scoped `git diff --check`.

## Validation Result

- passed
- Updated `docs/modules/downloads/README_IMPL.md` with manifest segment, segment checkpoint, and resume decision data-shape guidance.
- Confirmed key anchors exist with `rg -n "Resume Segment Data Shape|seal_completed|resume_partial|queue_remaining|reject_mismatch|DownloadManifestProviderPort" docs\modules\downloads\README_IMPL.md`.
- Scoped whitespace validation passed: `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`.
- No cargo test was required because this slice changed documentation and task records only.

## Notes

- AT-2026-05-15-160 completed and was committed locally as `0d9689a`.
- The current code has only `DownloadCheckpointRecord { job_id }`; completed-segment sealing needs explicit shape before code.
