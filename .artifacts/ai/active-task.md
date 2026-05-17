# Active Atomic Task

## Identity

- task id: AT-2026-05-17-192
- title: Add downloads segment execution request handoff shell
- status: completed

## Goal

按 README_IMPL 7.14 的 first Rust slice，新增 downloads 模块本地的 segment execution request/result/port shell，并增加本地 driver helper，把 `DownloadDriverExecutionTurn::PendingWorkAccepted` 中的 pending work 转换为稳定、有序、带 `JobId` 的 segment execution requests。

本轮只证明 request handoff 边界，不执行真实下载。

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - concrete HTTP fetch / provider object fetch
  - staging file writes or artifact moves
  - hash/length verification
  - checkpoint mutation or SQLite schema changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/lib.rs
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Already read in scoped snippets this session:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md sections 7.13 and 7.14
8. docs/TauriKernelJobsRuntimeDesign.md driver/runtime sections
9. docs/TauriDownloadRuntimeDesign.md scheduler/fetcher/writer/verifier/checkpoint sections
10. docs/TauriTestingStrategyAndQualityGateDesign.md backend test matrix
11. docs/TauriCodeCommentStandard.md comment rules plus user request for bilingual comments
12. crates/module-downloads/src/driver.rs
13. crates/module-downloads/src/lib.rs

## Hypothesis

- falsifiable local hypothesis: focused driver tests can prove `PendingWorkAccepted` maps to stable job-scoped segment execution requests in pending-work/item order without adding concrete IO or runtime execution.

## Cheap Check

1. Add focused RED tests in `crates/module-downloads/src/driver.rs`.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml segment_execution_request`.
3. Implement the minimal request/result/port shell and local helper.
4. Run focused test, full `launcher-module-downloads` tests, `cargo fmt --check`, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails for missing request/helper API.
2. GREEN keeps only local request handoff behavior.
3. Public comments are bilingual and existing English comments are preserved.
4. Focused and full module tests pass.
5. Formatting and scoped diff checks pass.
6. Commit only AT-192 files locally.

## Validation Result

- RED: focused `segment_execution_request` filter failed on missing `DownloadSegmentExecutionRequest` and `prepare_segment_execution_requests`.
- GREEN: focused `segment_execution_request` filter passed with 2 tests.
- Full module: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 31 tests.
- Format: `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed after rustfmt adjusted one test helper signature.
- Diff: scoped `git diff --check` passed with CRLF warnings only.
- Local commit: completed with the AT-192 code/PWF file set; verify the final amended hash with `git log --oneline -1`.

## Notes

- AT-2026-05-17-191 committed locally as `3d7f246`.
- Push remains skipped unless a safe push path is explicitly authorized later.
