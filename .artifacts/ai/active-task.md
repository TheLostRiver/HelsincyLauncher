# Active Atomic Task

## Identity

- task id: AT-2026-05-16-178
- title: Document downloads scheduler execution boundary
- status: completed

## Goal

补上 downloads resume scheduler execution 的实现边界文档：在写任何真实 fetch/write/verify/scheduler execution 代码前，先把 module-owned scheduler 如何消费 `DownloadResumeWorkPlan`、如何继续保持 `kernel-jobs` job-level 边界、以及下一步最小 Rust slice 写清楚。

本轮只覆盖 docs-first boundary：

- add a dedicated README_IMPL section for concrete scheduler execution boundaries
- define ownership split between module facade, downloads scheduler/driver, shared `JobRuntime`, repositories, and future adapters
- define the next minimal Rust slice after documentation
- keep actual fetch/write/verify execution, SQLite schema, host transport, frontend, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production behavior changes
  - concrete scheduler/fetch/write/verify execution
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
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
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md

Related architecture/collaboration docs read in scoped snippets:

1. docs/modules/downloads/README_IMPL.md
2. docs/TauriDownloadRuntimeDesign.md
3. docs/TauriKernelJobsRuntimeDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md
6. docs/TauriRewriteArchitectureBlueprint.md
7. docs/TauriArchitecturePrinciplesDesign.md
8. docs/TauriCurrentRepoArchitectureOverview.md
9. docs/TauriCompositionRootWiringDesign.md

## Hypothesis

- falsifiable local hypothesis: the next safe backend slice is documentation-only because the current README_IMPL explicitly keeps concrete scheduler execution and persistence unchanged until a dedicated scheduler implementation task exists.

## Cheap Check

- docs-only validation: path/reference readback, scoped `git diff --check`, and path-limited `git status --short`.

## Validation Gate

1. Read required docs in scoped snippets before editing README_IMPL.
2. Update README_IMPL with the dedicated scheduler execution implementation boundary.
3. Update PWF records for AT-178.
4. Run scoped `git diff --check` over touched files.
5. Run path-limited `git status --short`.
6. Commit the docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required README, collaboration, docs index, downloads module docs, implementation guide, download runtime, kernel-jobs runtime, testing strategy, AI transaction protocol, crate API draft, architecture, and composition snippets were read before editing README_IMPL.
- Added README_IMPL section `7.8 Concrete Scheduler Execution Boundary`.
- The new boundary keeps command-path scheduler preparation separate from future downloads driver/scheduler execution.
- The documented next Rust slice is a module-local pending resume work queue/scheduler shell, not real fetch/write/verify behavior.
- Readback found the new implementation-state row, section heading, pending-work boundary, failure layering, and next Rust slice anchors.
- Scoped `git diff --check` passed for AT-178 files with CRLF warnings only.
- Path-limited `git status --short` shows only AT-178 files plus the pre-existing unrelated `crates/composition-root/src/startup.rs` formatting side effect.

## Notes

- AT-2026-05-16-177 committed all-sealed scheduler guard as `31942bd`.
- User approved starting AT-178 after selecting this docs-first next step.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Shell note: PowerShell `Select-Object -Index (a..b),(c..d)` failed because the comma expression did not bind as the required `Int32[]`; use `-Skip/-First` or an explicit array next time.
