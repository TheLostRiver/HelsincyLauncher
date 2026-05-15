# Active Atomic Task

## Identity

- task id: AT-2026-05-15-159
- title: Add downloads module implementation documentation
- status: complete

## Goal

补齐 `docs/modules/downloads/README_IMPL.md`，把 downloads 后端实现前必须读取的模块文档、设计文档、协作文档和当前 Rust 切片顺序写成稳定入口。

本轮只覆盖：

- downloads 模块实现文档
- 模块文档标准与 docs 导航中的 README_IMPL 入口
- README_IMPL 模板

## Scope

- in scope:
  - add `docs/modules/downloads/README_IMPL.md`
  - add `docs/modules/_template/README_IMPL.md`
  - update `docs/ModuleDocumentationStandard.md`
  - update `docs/README.md`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change Rust/Tauri production code
  - change frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files
  - define implementation docs for every module in this slice

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. docs/modules/_template/README_IMPL.md
3. docs/ModuleDocumentationStandard.md
4. docs/README.md
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md
9. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/ModuleDocumentationStandard.md
5. docs/modules/downloads/README_ARCH.md
6. docs/modules/downloads/README_API.md
7. docs/modules/downloads/README_FLOW.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
10. docs/TauriFirstCrateApiDrafts.md
11. docs/TauriAIDevelopmentTransactionProtocolDesign.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md

## Hypothesis

- falsifiable local hypothesis: If module implementation docs are missing, future module code slices lack a single file that states the required pre-code reading order, Rust landing zones, current implementation state, and validation gates. Adding `README_IMPL.md` plus docs navigation should make that rule explicit without changing code behavior.

## Cheap Check

- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- docs/modules/downloads/README_IMPL.md docs/modules/_template/README_IMPL.md docs/ModuleDocumentationStandard.md docs/README.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Read the required module docs and related backend/design/collaboration docs before writing.
2. Create the downloads implementation doc and template.
3. Update docs navigation/standard so README_IMPL is discoverable.
4. Run scoped `git diff --check`.

## Validation Result

- passed
- Read required module docs and related backend/design/collaboration docs before writing.
- Added `docs/modules/downloads/README_IMPL.md`.
- Added `docs/modules/_template/README_IMPL.md`.
- Updated `docs/ModuleDocumentationStandard.md` and `docs/README.md` so README_IMPL is discoverable and required for backend-connected modules.
- Scoped `git diff --check` passed for tracked documentation/PWF changes; staged diff check must still be run before commit so new files are included.

## Notes

- AT-2026-05-15-158 completed and was committed locally as `cd5e848`.
- User explicitly required module docs under `docs/modules` to be read before module code, in addition to module design docs and collaboration docs.
