# Active Atomic Task

## Identity

- task id: AT-2026-05-15-166
- title: Document downloads resume runtime enqueue boundary
- status: completed

## Goal

在 downloads 模块实现文档中固化 `resume_download` 下一步 runtime enqueue 的最小边界：先明确 job-level runtime request、segment decision 到 enqueue 的映射、以及仍然留在后续切片的 scheduler/persistence/error projection 内容，然后再进入 Rust 代码切片。

本轮只覆盖：

- update `docs/modules/downloads/README_IMPL.md`
- update PWF records
- no Rust production/test code changes

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
  - change Rust tests
  - change frontend files
  - change host transport or composition-root wiring
  - change driver or SQLite adapter persistence
  - implement runtime resume execution
  - change sqlite database files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

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
14. current `crates/module-downloads/src/facade/mod.rs` and `crates/kernel-jobs/src/runtime.rs` snippets for existing code shape

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define the next runtime-enqueue slice narrowly enough that AT-167 can add a RED module test for job-level resume enqueue without inventing segment persistence, scheduler execution, host transport, or frontend projection.

## Cheap Check

- read back the updated README_IMPL runtime-enqueue section and run scoped `git diff --check`.

## Validation Gate

1. Read required module docs and related backend/runtime/testing/collaboration docs before editing.
2. Update README_IMPL only.
3. Verify the new section states the job-level enqueue request, decision mapping, and out-of-scope boundaries.
4. Run scoped doc readback, scoped `git diff --check`, and scoped `git status --short`.

## Validation Result

- passed
- README_IMPL runtime-enqueue section was updated and read back with `rg`; the readback confirmed the new current-state rows, `Runtime Enqueue Boundary` section, minimum job-level runtime request table, decision mapping table, and out-of-scope list.
- Scoped `git diff --check` passed for README_IMPL and PWF files with CRLF conversion warnings only.
- Scoped diff stat and scoped status confirmed only the AT-166 allowed files are in scope.
- Committed locally.

## Notes

- AT-2026-05-15-165 completed and was committed locally as current HEAD `491add7`.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: start AT-167 as a Rust RED-test slice for the documented runtime-enqueue boundary.
