# Active Atomic Task

## Identity

- task id: AT-2026-05-16-173
- title: Add downloads resume work plan derivation
- status: completed

## Goal

在 `crates/module-downloads` 内引入最小的 downloads-owned resume work plan/payload 派生：把已经存在的 `resume_partial` / `queue_remaining` 决策转成模块内 scheduler/driver 未来可消费的 work item，同时证明 `seal_completed` 和 `reject_mismatch` 不产生 work item。

本轮只覆盖模块内纯派生和文档/PWF：

- add focused RED test first
- add minimal module-local `DownloadResumeWorkPlan` / `DownloadResumeWorkItem` shape
- derive work items from manifest/checkpoint/decision facts without executing downloads
- keep scheduler execution, SQLite schema, frontend, host transport, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/lib.rs`
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
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

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

- falsifiable local hypothesis: a pure module-local work-plan derivation can turn `resume_partial` and `queue_remaining` decisions into deterministic work items without changing runtime enqueue, `kernel-jobs`, host transport, frontend, SQLite schema, or concrete scheduler execution.

## Cheap Check

- write one focused failing Rust unit test for mixed resume decisions, then add the smallest module-local derivation and run the focused test.

## Validation Gate

1. Read required root, docs index, downloads module, implementation, download runtime, kernel-jobs runtime, crate layout/API, testing, and collaboration docs before editing Rust.
2. Add RED test proving `resume_partial` / `queue_remaining` produce work items and `seal_completed` / `reject_mismatch` do not.
3. Record findings and recovery state in `.artifacts/ai`.
4. Add minimal production derivation and bilingual comments for new declarations.
5. Run focused cargo tests and scoped `git diff --check`.
6. Commit the Rust/docs/PWF slice locally without staging unrelated dirty files.

## Validation Result

- passed
- Required root, docs index, downloads module, implementation, runtime, kernel-jobs, crate layout/API, testing, AI transaction, and comment-standard snippets were read before Rust edits.
- RED test `resume_work_plan_derives_only_partial_and_remaining_items` first failed on missing `build_resume_work_plan` / `DownloadResumeWorkMode`.
- Minimal GREEN implementation added `DownloadResumeWorkPlan`, `DownloadResumeWorkItem`, `DownloadResumeWorkMode`, and `build_resume_work_plan()` with bilingual declaration comments.
- `crates/module-downloads/src/lib.rs` re-exports the new work-plan function and types through the crate entry.
- Focused test passed: `resume_work_plan_derives_only_partial_and_remaining_items` returned 1 passed, 0 failed.
- Full module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` returned 18 passed, 0 failed.
- Scoped `git diff --check` passed for AT-173 files with CRLF warnings only.
- Runtime enqueue, concrete scheduler execution, SQLite schema, host transport, frontend, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-172 documented the scheduler/driver payload boundary in README_IMPL section 7.6.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Resume point: next slice should define or introduce the downloads-owned scheduler/driver boundary that will consume `DownloadResumeWorkPlan`, before any concrete fetch/write/verify execution or persistence schema work.
