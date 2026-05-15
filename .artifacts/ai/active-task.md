# Active Atomic Task

## Identity

- task id: AT-2026-05-15-153
- title: Prepare checkpoint-aware resume_download design boundary
- status: completed

## Goal

在进入任何 `resume_download` 行为修改前，按用户要求重新读取 README、协作/架构/测试/AI 事务协议、downloads/jobs 专题文档、downloads 模块文档和当前 Rust 实现，明确下一步可接受的后端切片边界。

本轮只覆盖：

- `.artifacts/ai` 设计发现与恢复点记录

## Scope

- in scope:
  - confirm docs-first context before coding
  - identify the next backend-only `resume_download` slice
  - record why `resume_download` must be checkpoint-aware
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change Rust production code or tests
  - start `resume_download` implementation before design approval
  - touch frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/TauriArchitecturePrinciplesDesign.md
5. docs/TauriTestingStrategyAndQualityGateDesign.md
6. docs/TauriAIDevelopmentTransactionProtocolDesign.md
7. docs/TauriCurrentRepoArchitectureOverview.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriKernelJobsRuntimeDesign.md
10. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
11. docs/TauriFirstCrateApiDrafts.md
12. docs/modules/downloads/README_ARCH.md
13. docs/modules/downloads/README_API.md
14. docs/modules/downloads/README_FLOW.md

## Hypothesis

- falsifiable local hypothesis: If the docs are followed strictly, the first acceptable `resume_download` backend task should start with a RED test proving explicit checkpoint loading, not a direct `JobRuntime::resume` passthrough.

## Cheap Check

- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Run scoped `git diff --check` on the touched `.artifacts/ai` records.
2. Confirm no Rust or frontend files are staged for this record-only task.

## Validation Result

- passed
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- `resume_download` docs require explicit checkpoint loading.
- User explicitly re-emphasized that backend coding must not begin before reading the relevant docs and constraints.
- The recommended next implementation slice is a TDD RED test in `crates/module-downloads/src/facade/mod.rs` proving `resume_download` consults `DownloadCheckpointRepository`.
