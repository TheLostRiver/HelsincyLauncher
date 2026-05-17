# Active Atomic Task

## Identity

- task id: AT-2026-05-17-202
- title: Record failed results during downloads fake local resume execution
- status: completed

## Goal

After AT-201 documented the mixed-result orchestration boundary, update `DownloadJobDriver::execute_local_resume_turn(...)` so a fake local resume turn records both completed and failed segment results by delegating to the existing checkpoint mutation helpers.

This stays module-local. It must not add retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema work, composition-root, transport, or frontend behavior.

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
  - retry/backoff policy
  - public `DL_*` execution error projection
  - terminal runtime job state
  - concrete HTTP range requests or provider object fetch
  - staging file writes, artifact moves, or hash/length verification
  - SQLite schema or adapter changes
  - runtime `JobDriver::run()` / runtime snapshot / job completion
  - composition-root, host transport, frontend changes
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
4. docs/modules/downloads/README_IMPL.md section 7.21 and validation section
5. docs/TauriDownloadRuntimeDesign.md failure/retry/checkpoint snippets
6. current `crates/module-downloads/src/driver.rs`
7. superpowers TDD skill

## Hypothesis

- falsifiable local hypothesis: a focused driver test can prove `execute_local_resume_turn(...)` persists a fake failed segment result by chaining existing completed and failed checkpoint mutation helpers without duplicating mutation logic or widening scope.

## Cheap Check

1. Add a focused RED test in `crates/module-downloads/src/driver.rs` for fake local resume execution with a failed result.
2. Run focused `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml fake_local_resume_execution_records_failed`.
3. Update only `execute_local_resume_turn(...)` to call both existing checkpoint mutation helpers.
4. Update README_IMPL to mark mixed-result orchestration implemented.
5. Run focused test, full module test, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. RED fails because fake local resume execution does not persist failed result facts.
2. GREEN records completed and failed results through existing helpers and returns the final checkpoint option.
3. No retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work is added.
4. Public comments are bilingual and existing English comments are preserved.
5. Focused and full module tests pass.
6. Formatting and scoped diff checks pass.
7. Commit only AT-202 files locally, then push `main` to `origin`.

## Validation Result

1. RED command `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml fake_local_resume_execution_records_failed` failed because `execute_local_resume_turn(...)` returned an empty checkpoint instead of persisted failed segment facts.
2. GREEN updated `execute_local_resume_turn(...)` to call both `record_completed_segment_checkpoints(...)` and `record_failed_segment_checkpoints(...)`.
3. Focused validation passed: 1 passed, 0 failed.
4. Full downloads module validation passed: 38 passed, 0 failed.
5. `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
6. Scoped `git diff --check` passed with CRLF normalization warnings only.
7. No retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, or frontend work was added.

## Notes

- AT-2026-05-17-201 final local commit is `8790f8d` and was pushed to `origin/main`.
- AT-2026-05-17-202 initial local commit is `eae3c4f` before PWF hash backfill amend.
