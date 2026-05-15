# Active Atomic Task

## Identity

- task id: AT-2026-05-15-167
- title: Add downloads resume runtime enqueue boundary
- status: completed

## Goal

在 `module-downloads` 内用 TDD 证明 `resume_download` 的第一个 runtime enqueue 边界：当作业、checkpoint、staging、manifest 都可用，segment decision 中至少存在 runtime enqueue candidate 且没有 `reject_mismatch` 时，facade 使用现有 job id 和持久化 priority 调用 job-level `JobRuntime::enqueue()`，并返回 runtime `AcceptedJob`。

本轮只覆盖后端：

- first write a failing module facade test
- then add the minimal production code needed to pass it
- preserve segment decision ownership inside downloads
- do not expose segment details through host transport or frontend

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/contracts/commands.rs`
  - `crates/module-downloads/src/contracts/dto.rs`
  - `crates/module-downloads/src/contracts/events.rs`
  - `crates/module-downloads/src/contracts/mod.rs`
  - `docs/modules/downloads/README_IMPL.md`
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
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/contracts/commands.rs
3. crates/module-downloads/src/contracts/dto.rs
4. crates/module-downloads/src/contracts/events.rs
5. crates/module-downloads/src/contracts/mod.rs
6. docs/modules/downloads/README_IMPL.md
7. .artifacts/ai/active-task.md
8. .artifacts/ai/task-plan.md
9. .artifacts/ai/progress.md
10. .artifacts/ai/findings.md
11. .artifacts/ai/handoff.md

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

- falsifiable local hypothesis: a focused module facade test can force `resume_download` to call `JobRuntime::enqueue()` with the existing download job id, module `downloads`, kind `download`, persisted priority, recoverable `true`, and unit extension, without introducing segment payloads or scheduler execution.

## Cheap Check

- run the focused `module-downloads` facade test first and confirm it fails for the expected missing runtime enqueue behavior.

## Validation Gate

1. Read required root, docs index, module, runtime, testing, collaboration, and implementation docs before editing Rust code.
2. Add the RED test before production edits.
3. Confirm RED fails for the expected enqueue/accepted-job boundary.
4. Add only the minimal production code to make the test pass.
5. Run focused test and then `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
6. Update README_IMPL current-state notes if the runtime enqueue boundary becomes wired.

## Validation Result

- passed
- RED confirmed with focused test failure: `resume_download_enqueues_existing_job_when_decisions_have_runtime_candidates` failed on `DOWNLOADS_NOT_WIRED`.
- GREEN confirmed with focused test pass after the minimal runtime enqueue branch.
- Full module validation passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 15 passed, 0 failed.
- `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml -p launcher-module-downloads` completed successfully.
- Scoped `git diff --check` passed for facade, README_IMPL, and PWF files with CRLF conversion warnings only.
- The package formatter also normalized module contracts EOF/newline wrapping; those formatting-only files are included in the AT-167 allowed set.
- Committed locally.

## Notes

- AT-2026-05-15-166 completed the docs boundary and was current HEAD before this task.
- Resume point: choose the next backend slice after job-level enqueue, likely either mismatch/error projection, all-sealed completion handling, or a documented scheduler/driver payload boundary.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
