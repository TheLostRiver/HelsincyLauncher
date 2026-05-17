# Active Atomic Task

## Identity

- task id: AT-2026-05-17-196
- title: Add downloads fake local resume execution orchestration
- status: completed

## Goal

After AT-195 added completed-result checkpoint mutation, implement the next narrow downloads-owned orchestration step: `DownloadJobDriver` may locally coordinate one fake resume execution turn by preparing the execution turn, building segment requests, delegating them to a supplied `DownloadSegmentExecutionPort`, and recording completed segment checkpoints.

This remains a module-local helper. It is not `kernel-jobs::JobDriver::run()`, not a shared runtime loop, and not concrete download execution.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `crates/module-downloads/src/driver.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - concrete HTTP range requests or provider object fetch
  - staging file writes or artifact moves
  - hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - public `DL_*` execution error design
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. crates/module-downloads/src/driver.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_IMPL.md sections 7.13 through 7.17
5. docs/TauriDownloadRuntimeDesign.md scheduler/fetch/write/verify/checkpoint snippets
6. docs/TauriKernelJobsRuntimeDesign.md current runtime boundary snippets
7. current `crates/module-downloads/src/driver.rs`
8. superpowers TDD skill

Previously read in this session and still governing scope:

1. docs/modules/downloads/README_ARCH.md
2. docs/modules/downloads/README_API.md
3. docs/modules/downloads/README_FLOW.md
4. docs/TauriRepositoryPortsAndAdapterDesign.md
5. docs/TauriStorageAndDatabaseDesign.md
6. docs/TauriTestingStrategyAndQualityGateDesign.md
7. docs/TauriCodeCommentStandard.md
8. docs/TauriAIDevelopmentTransactionProtocolDesign.md
9. crates/module-downloads/src/facade/mod.rs
10. crates/module-downloads/src/lib.rs

## Hypothesis

- falsifiable local hypothesis: a focused driver test can prove a prepared pending resume work item flows through a fake local execution port and produces a saved completed segment checkpoint without adding runtime `run()`, concrete IO, SQLite adapter changes, host transport, or frontend behavior.

## Cheap Check

1. Update README_IMPL with the AT-196 fake/local orchestration boundary before coding.
2. Add a focused RED test in `crates/module-downloads/src/driver.rs` for local fake resume execution orchestration.
3. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml fake_local_resume_execution`.
4. Implement the minimal local driver helper that calls the already-tested helper sequence.
5. Run focused test, full module test, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. README_IMPL documents that this is fake/local orchestration, not shared runtime execution.
2. RED fails because the orchestration helper does not exist.
3. GREEN only chains existing helpers and returns the updated checkpoint option.
4. Public comments are bilingual and existing English comments are preserved.
5. Focused and full module tests pass.
6. Formatting and scoped diff checks pass.
7. Commit only AT-196 files locally.

## Validation Result

1. README_IMPL section 7.18 documents and closes the AT-196 fake/local orchestration boundary.
2. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml fake_local_resume_execution` failed because `execute_local_resume_turn` did not exist.
3. GREEN added `DownloadJobDriver::execute_local_resume_turn(...)`, which chains the existing execution-turn, request, port, and checkpoint helpers.
4. Focused validation passed: 1 passed, 0 failed.
5. Full downloads module validation passed: 35 passed, 0 failed.
6. `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed after a mechanical `cargo fmt`.
7. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-195 committed locally as `227458a`.
- User has now provided the GitHub repository link and asked to push after completion; `origin` already matches `https://github.com/TheLostRiver/HelsincyLauncher.git`.
- AT-2026-05-17-196 initial local commit created as `3d6f4f7`; PWF backfill is amended into the same task commit.
