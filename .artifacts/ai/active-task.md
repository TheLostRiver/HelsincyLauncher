# Active Atomic Task

## Identity

- task id: AT-2026-05-17-220
- title: Document shared runtime execution-turn boundary
- status: completed

## Goal

Define the next durable backend implementation boundary before coding a `kernel-jobs` execution turn, so later Rust work has a clear first slice and does not pretend downloads concrete IO, retry, lease, or terminal completion exists.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust code changes
  - frontend changes
  - host transport changes
  - concrete downloads IO, retry/backoff, lease implementation, terminal completion
  - rewriting earlier long README_IMPL history
  - editing unrelated dirty files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. docs/modules/downloads/README_IMPL.md section 7.13 and 7.28.
2. docs/TauriKernelJobsRuntimeDesign.md driver, queue policy, lease, recovery, and runtime-context sections.
3. docs/TauriDownloadRuntimeDesign.md scheduler and budget sections.
4. current `crates/kernel-jobs/src/runtime.rs`, `lib.rs`, and `model.rs` surfaces.
5. Current PWF task plan, findings, progress, and handoff tails.

## Hypothesis

- falsifiable local hypothesis: a short README_IMPL section can make the next code slice unambiguous: introduce a module-neutral execution-turn contract in `kernel-jobs` first, while keeping concrete downloads execution and terminal runtime completion out of scope.

## Cheap Check

1. Add one concise durable section to `docs/modules/downloads/README_IMPL.md`.
2. Confirm the section names current Rust reality, first Rust slice, and explicit non-goals.
3. Update PWF records, including AT-219 final commit `f618718`.
4. Run scoped `git diff --check` for the AT-220 file set.

## Validation Gate

1. README_IMPL defines the minimal shared runtime execution-turn boundary before code.
2. The first Rust slice is limited to `kernel-jobs` execution-turn contracts/tests.
3. Downloads concrete IO, retry/backoff, lease persistence, snapshot completion, host transport, frontend, and SQLite schema changes remain out of scope.
4. No code files are changed.
5. Commit only AT-220 files locally, then push `main` to `origin`.

## Validation Result

- Added `docs/modules/downloads/README_IMPL.md` section 7.29, "Shared Runtime Execution-Turn Boundary".
- The section records current Rust reality, the first later Rust slice, and explicit non-goals for downloads concrete IO, retry/backoff, leases, terminal completion, host transport, frontend, and SQLite schema changes.
- No code files were changed.
- Scoped `git diff --check` passed for the AT-220 file set with CRLF normalization warnings only.
- Commit and push are pending for the AT-220 file set.

## Notes

- AT-2026-05-17-219 final commit is `f618718` and is already pushed to `origin/main`.
- User requested stopping after completing one more task.
