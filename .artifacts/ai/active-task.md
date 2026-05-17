# Active Atomic Task

## Identity

- task id: AT-2026-05-17-190
- title: Add downloads driver local execution-turn classification
- status: completed

## Goal

在不修改 `kernel-jobs::JobDriver`、不引入真实 `run()`、不执行 HTTP fetch / staging write / verifier 的前提下，为 `DownloadJobDriver` 增加一个 downloads 模块本地 execution-turn 分类方法。该方法只负责在未来 runtime turn 调用前证明边界顺序：

1. 先重新读取 durable checkpoint；
2. checkpoint 缺失时不 drain pending work；
3. checkpoint 存在时再 drain 指定 job 的 pending resume work；
4. 返回模块自有的显式分类，而不是把空 pending work 当成成功完成。

## Scope

- in scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs` only if the new public type needs crate-level re-export
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `kernel-jobs::JobDriver` trait changes
  - runtime `run()` implementation
  - composition-root wiring changes
  - host transport or frontend changes
  - SQLite schema or persistence changes
  - concrete HTTP fetch, staging writes, hash/length verifier
  - runtime snapshot mutation, job completion APIs, public DTO changes
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

Read in scoped snippets before editing code:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md sections 7.12 and 7.13
8. docs/TauriKernelJobsRuntimeDesign.md driver/runtime sections
9. docs/TauriDownloadRuntimeDesign.md scheduler/fetcher/writer/verifier/checkpoint sections
10. docs/TauriTestingStrategyAndQualityGateDesign.md backend test matrix
11. docs/TauriAIDevelopmentTransactionProtocolDesign.md atomic task protocol
12. docs/TauriCodeCommentStandard.md comment rules plus user request for bilingual comments
13. crates/module-downloads/src/driver.rs
14. crates/module-downloads/src/facade/mod.rs relevant pending-work source sections
15. crates/module-downloads/src/lib.rs
16. crates/composition-root/src/bootstrap.rs relevant shared scheduler/driver construction sections

## Hypothesis

- falsifiable local hypothesis: a focused driver test can prove the new local execution-turn method checks checkpoint presence before draining pending work and returns explicit module-owned classifications without changing shared runtime execution semantics.

## Cheap Check

1. Add a focused RED driver test for checkpoint-missing execution turn preserving pending work.
2. Add a focused RED/GREEN driver test for checkpoint-present pending work classification.
3. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml <new-test-filter>`.
4. Run full `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
5. Run `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check`.
6. Run scoped `git diff --check` and path-limited status for allowed files.

## Validation Gate

1. RED test fails for the intended missing API/behavior.
2. GREEN implementation adds only local driver classification behavior.
3. New public comments are bilingual and existing English comments are preserved.
4. Focused and full module tests pass.
5. Formatting and scoped diff checks pass.
6. Commit only AT-190 files locally.

## Validation Result

- RED: focused `download_job_driver_execution_turn` filter failed on missing `DownloadDriverExecutionTurn` and `prepare_resume_execution_turn`.
- GREEN: focused `download_job_driver_execution_turn` filter passed with 3 tests.
- Full module: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 29 tests.
- Format: `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed after rustfmt adjusted export wrapping.
- Diff: scoped `git diff --check` passed with CRLF warnings only.
- Local commit: completed with the AT-190 code/PWF file set; verify the final amended hash with `git log --oneline -1`.

## Notes

- AT-2026-05-17-189 committed locally as `3117914`.
- Push to `origin/main` was rejected earlier by safety review; per user rule, skip push if it cannot be done and do not work around it.
