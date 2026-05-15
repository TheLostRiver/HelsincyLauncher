# Active Atomic Task

## Identity

- task id: AT-2026-05-16-169
- title: Document downloads resume all-sealed completion boundary
- status: completed

## Goal

在 `docs/modules/downloads/README_IMPL.md` 中补清 `resume_download` 的 all-sealed outcome 边界：当 manifest 中的所有 segment 都能由 checkpoint 安全 `seal_completed`，且没有 `resume_partial`、`queue_remaining` 或 `reject_mismatch` 时，当前实现不能再靠 `DOWNLOADS_NOT_WIRED` 长期占位，也不能伪造一个已入队的 `AcceptedJob` 来表达“无需入队且已完成”。

本轮只覆盖后端实现文档：

- record why all-sealed is a distinct outcome from runtime enqueue
- document the current contract gap between `AcceptedJob` and already-complete resume outcomes
- define the next code slice boundary before any Rust behavior change
- keep frontend, IPC shape, SQLite schema, scheduler execution, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

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
12. docs/TauriIPCAndStateContractsDesign.md
13. docs/TauriErrorHandlingAndProjectionDesign.md
14. docs/TauriTestingStrategyAndQualityGateDesign.md
15. docs/TauriAIDevelopmentTransactionProtocolDesign.md
16. current `crates/kernel-jobs/src/model.rs` and `runtime.rs` snippets for `AcceptedJob` semantics

## Hypothesis

- falsifiable local hypothesis: a docs-only implementation guide slice can make the all-sealed outcome safe for the next TDD code task by explicitly forbidding fake runtime enqueue/`AcceptedJob` completion projection and by naming the minimal Rust boundary that must be introduced next.

## Cheap Check

- update README_IMPL, then run scoped doc/Git validation and read back the all-sealed wording.

## Validation Gate

1. Read required root, docs index, module, runtime, error, IPC, testing, collaboration, and current runtime model snippets before editing README_IMPL.
2. Update README_IMPL with the all-sealed completion boundary and current contract gap.
3. Record findings and recovery state in `.artifacts/ai`.
4. Run scoped `git diff --check` for AT-169 files.
5. Commit the docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required root, docs index, downloads module docs, README_IMPL, download runtime, crate layout/API drafts, kernel-jobs runtime, IPC/error/testing/collaboration docs, and current runtime model snippets were read before editing README_IMPL.
- README_IMPL now documents the all-sealed no-enqueue boundary and explicitly forbids faking it as a queued `AcceptedJob`.
- Scoped `git diff --check` passed for README_IMPL and PWF files with CRLF conversion warnings only.
- AT-169 is committed locally as a docs/PWF slice.

## Notes

- AT-2026-05-15-168 committed the mismatch error projection branch.
- Resume point: start the next Rust boundary slice by introducing a narrow module-owned resume outcome before changing all-sealed behavior.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
