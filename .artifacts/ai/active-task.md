# Active Atomic Task

## Identity

- task id: AT-2026-05-19-259
- title: Define downloads driver completion-first terminal decision boundary
- status: completed

## Goal

Document the downloads-owned rule for when `DownloadJobDriver::run(...)` may return `JobRunDisposition::Completed`, while reserving terminal failure for later retry/backoff classification.

## Scope

- in scope:
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust code changes
  - returning `TerminalFailed` from downloads driver
  - retry/backoff or stable public `DL_*` execution errors
  - host transport, frontend, SQLite schema, provider HTTP, production wiring, scheduler loops, or leases
  - unrelated dirty files

## Allowed Files

1. README.md
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status and near-term roadmap.
2. `docs/modules/downloads/README_IMPL.md` 6.1 and 7.43.
3. `docs/TauriKernelJobsRuntimeDesign.md` terminal snapshot projection ownership.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint/runtime ownership.
5. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable and public error projection rules.
6. `crates/module-downloads/src/driver.rs` checkpoint statuses, checkpoint mutation helpers, `execute_local_resume_turn(...)`, and `run(...)`.
7. `crates/module-downloads/src/facade/mod.rs` all-sealed resume and completion-related precedent.

## Hypothesis

- falsifiable documentation hypothesis: the safe first downloads terminal decision is `Completed` only for a non-empty all-completed known checkpoint after local execution; `TerminalFailed` must stay reserved because current failed checkpoint persistence does not preserve retryability or a stable public error classification.

## Cheap Check

1. Update root README status/roadmap to name completion-first downloads driver implementation as next.
2. Update README_IMPL 6.1 and add 7.44 with current reality, boundary rules, and first Rust slice.
3. Update PWF task records and handoff.
4. Run scoped `git diff --check` for touched docs/task files.
5. Commit and attempt push.

## Validation Gate

1. README points to completion-first downloads driver implementation, not already-completed kernel projection.
2. README_IMPL defines `Completed` conditions and explicitly keeps terminal failure out of the first downloads driver slice.
3. Scope keeps Rust/transport/frontend/provider/retry/schema work out of AT-259.
4. Scoped diff-check passes or CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current status and near-term roadmap to name downloads driver completion-first implementation as the next backend slice.
- Updated `docs/modules/downloads/README_IMPL.md` 6.1 and added 7.44 with the completion-first terminal decision boundary.
- Validation: scoped `git diff --check` over README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- No Rust, transport, frontend, provider, retry/backoff, public `DL_*`, schema, scheduler, or lease files were edited.
- Commit/push pending.
