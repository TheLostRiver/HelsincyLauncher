# Active Atomic Task

## Identity

- task id: AT-2026-05-16-170
- title: Add downloads module-owned resume outcome boundary
- status: completed

## Goal

在 `module-downloads` 内引入窄的模块自有 `resume_download` outcome 边界，让 all-sealed 分支能够在模块层表达为“无需 runtime enqueue 且已经完成”，但不在本轮改变 host transport、IPC DTO、SQLite schema、scheduler execution 或 `kernel-jobs` payload。

本轮只覆盖后端模块边界：

- add a focused RED test first for an all-sealed resume outcome
- introduce a module-owned `DownloadResumeOutcome`
- add `resume_download_outcome` as the narrow outcome-returning module method
- keep the existing `resume_download -> AppResult<AcceptedJob>` entry compatible for current host transport wiring
- document the newly wired module-owned outcome boundary in README_IMPL

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `crates/module-downloads/src/facade/mod.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - frontend files
  - host transport IPC shape
  - shared `AcceptedJobDto` shape
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. crates/module-downloads/src/facade/mod.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

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
16. current `crates/module-downloads/src/facade/mod.rs` snippets for existing code/test shape

## Hypothesis

- falsifiable local hypothesis: a focused module facade test can force an all-sealed resume plan to return a module-owned `AlreadyComplete` outcome and avoid runtime enqueue, while preserving the existing public `resume_download -> AcceptedJob` compatibility method for current host transport wiring.

## Cheap Check

- add the RED test for `resume_download_outcome` and confirm it fails because the module-owned outcome boundary does not exist yet.

## Validation Gate

1. Read required root, docs index, module, runtime, error, IPC, testing, collaboration, README_IMPL, and current facade snippets before editing Rust code.
2. Add RED test before production code.
3. Confirm RED fails for the expected missing outcome boundary.
4. Add only the minimal outcome enum/method and compatible wrapper needed to pass.
5. Update README_IMPL current-state wording.
6. Run focused test, full `launcher-module-downloads` tests, formatter/checks, and scoped `git diff --check`.

## Validation Result

- passed
- RED confirmed with focused compile failure: `resume_download_outcome_returns_already_complete_when_all_segments_are_sealed` failed because `resume_download_outcome` and `DownloadResumeOutcome` did not exist.
- GREEN confirmed with focused test pass after adding `DownloadResumeOutcome` and `resume_download_outcome`.
- Full module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 17 passed, 0 failed after formatting.
- `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml -p launcher-module-downloads` completed successfully.
- Scoped `git diff --check` passed for facade, README_IMPL, and PWF files with CRLF conversion warnings only.
- AT-170 is committed locally as a code/docs/PWF slice.

## Notes

- AT-2026-05-16-169 documented why all-sealed must not be faked as queued `AcceptedJob`.
- Resume point: decide whether to adapt host transport/shared DTO for `AlreadyComplete`, or continue the downloads scheduler/driver payload design.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
