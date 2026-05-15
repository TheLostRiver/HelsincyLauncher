# Active Atomic Task

## Identity

- task id: AT-2026-05-15-168
- title: Add downloads resume mismatch error projection
- status: completed

## Goal

在 `module-downloads` 内为 `resume_download` 的 `reject_mismatch` 决策补上稳定 downloads-domain 错误投影：当 manifest segment 与 checkpoint segment 的边界事实不匹配时，恢复流程必须停止、不得 runtime enqueue，并返回稳定错误码。

本轮只覆盖后端：

- document the mismatch error boundary in README_IMPL before Rust changes
- write a failing module facade test first
- add the minimal production branch needed to pass it
- keep segment details inside downloads
- do not touch frontend, IPC, SQLite schema, scheduler execution, or kernel-jobs payloads

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
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload extensions
  - all-sealed completion handling
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
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md
14. current `crates/module-downloads/src/facade/mod.rs` snippets for existing code/test shape

## Hypothesis

- falsifiable local hypothesis: a focused module facade test can force `resume_download` to return `DL_RESUME_SEGMENT_MISMATCH` and avoid runtime enqueue when any derived resume decision is `RejectMismatch`, without changing segment persistence, IPC, scheduler execution, or shared job runtime contracts.

## Cheap Check

- update README_IMPL boundary, then run the focused RED test and confirm it fails because `resume_download` still falls through to `DOWNLOADS_NOT_WIRED`.

## Validation Gate

1. Read required root, docs index, module, runtime, testing, collaboration, and implementation docs before editing Rust code.
2. Update README_IMPL with the stable mismatch error boundary before Rust changes.
3. Add the RED test before production edits.
4. Confirm RED fails for the expected missing mismatch error projection.
5. Add only the minimal production branch to return the stable error and skip runtime enqueue.
6. Run focused test, full `launcher-module-downloads` tests, formatter/checks, and scoped `git diff --check`.

## Validation Result

- passed
- RED confirmed with focused test failure: `resume_download_returns_stable_error_when_segment_checkpoint_mismatches_manifest` failed on `DOWNLOADS_NOT_WIRED` instead of `DL_RESUME_SEGMENT_MISMATCH`.
- GREEN confirmed with focused test pass after the minimal mismatch error branch.
- Full module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 16 passed, 0 failed after formatting.
- `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml -p launcher-module-downloads` completed successfully.
- Scoped `git diff --check` passed for facade, README_IMPL, and PWF files with CRLF conversion warnings only.
- Code/docs and final PWF records are included in the same local AT-168 commit.

## Notes

- AT-2026-05-15-167 committed the job-level runtime enqueue branch.
- Resume point: choose the next backend slice after mismatch projection, likely all-sealed completion handling or a documented scheduler/driver payload boundary.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
