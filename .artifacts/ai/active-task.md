# Active Atomic Task

## Identity

- task id: AT-2026-05-19-270
- title: Define retry-ready manifest binding boundary
- status: completed

## Goal

Document how due retry-ready checkpoint facts must be rebound to current manifest segments before any executable retry work is derived.

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
  - Rust implementation
  - SQLite schema or adapter changes
  - retry work item derivation, scheduler loops, runtime dispatch, leases, or automatic retry execution
  - `TerminalFailed`, public `DL_*`, host transport, frontend state, provider HTTP, production wiring, or snapshot error payload changes

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

1. `README.md` current status and roadmap after AT-269.
2. `docs/modules/downloads/README_IMPL.md` 7.1-7.4 and 7.48.
3. `crates/module-downloads/src/facade/mod.rs` existing `build_resume_segment_decisions(...)`, `build_resume_work_plan(...)`, and mismatch tests.
4. `docs/TauriDownloadRuntimeDesign.md` manifest/checkpoint/retry ownership notes.
5. `CONTRIBUTING.md` and `docs/TauriAIDevelopmentTransactionProtocolDesign.md` task-splitting and validation rules.

## Hypothesis

- falsifiable documentation hypothesis: retry-ready checkpoint facts must bind to current manifest segments by `segment_id` and reject missing or stale manifest boundaries before any retry work item can be created.

## Cheap Check

1. Add README_IMPL 7.49 defining binding inputs, mismatch rules, output shape, first Rust slice, and non-goals.
2. Refresh README roadmap to say the binding boundary is documented and the next Rust slice is the pure binding helper.
3. Update PWF records.
4. Run scoped `git diff --check`.
5. Commit and attempt push.

## Validation Gate

1. README_IMPL 7.49 defines manifest binding before executable retry work.
2. Missing manifest segment and `file_id` / `offset` / `length` mismatch are explicitly rejected with existing mismatch semantics.
3. Scheduler/runtime dispatch, SQLite schema, host/frontend, public `DL_*`, and terminal projection remain out of scope.

## Completion Evidence

- Added README_IMPL 7.49 defining retry-ready checkpoint-to-manifest binding before executable retry work derivation.
- README roadmap now points the next Rust slice at a pure manifest binding helper.
- Reused existing resume mismatch semantics: missing manifest segment and `file_id` / `offset` / `length` mismatches stop before scheduler/runtime work.
- Validation: scoped `git diff --check` passed for README, README_IMPL, and PWF files with CRLF normalization warnings only.
- No Rust implementation, SQLite schema, scheduler/runtime dispatch, host/frontend, public `DL_*`, provider HTTP, production wiring, or terminal projection changes were introduced.
