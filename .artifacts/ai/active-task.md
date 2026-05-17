# Active Atomic Task

## Identity

- task id: AT-2026-05-17-222
- title: Document shared runtime execution dispatch boundary
- status: completed

## Goal

Define the next durable backend implementation boundary after the minimal `JobDriver::run(...)` contract: a one-shot `SharedJobRuntimeHost` dispatch method that composes snapshot lookup, driver registry resolution, and one driver execution turn without starting a scheduler loop or concrete downloads IO.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production or test code
  - downloads driver integration
  - concrete HTTP/file/hash execution
  - retry/backoff
  - durable lease persistence
  - terminal snapshot completion or runtime loop scheduling
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md, CONTRIBUTING.md, and docs/README.md routing guidance.
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md section 7.29.
3. docs/TauriKernelJobsRuntimeDesign.md driver, runtime-host, runtime-context, lease, and first-slice sections.
4. docs/TauriDownloadRuntimeDesign.md scheduler/budget/ownership notes.
5. current `crates/kernel-jobs/src/runtime.rs`, `lib.rs`, and existing composition-root driver-registry wiring.

## Hypothesis

- falsifiable local hypothesis: the next Rust slice is clear only after README_IMPL records whether dispatch belongs in `SharedJobRuntimeHost`, `JobDriverRegistry`, or downloads. If the doc can pin a host-owned one-shot dispatch method with explicit non-goals, coding can continue in the following task without guessing.

## Cheap Check

1. Add README_IMPL section 7.30.
2. Update PWF records with AT-221 final commit/push and AT-222 scope.
3. Run scoped `git diff --check` for the allowed files.

## Validation Gate

1. README_IMPL 7.30 names the next Rust slice and required `launcher-kernel-jobs` tests.
2. The doc explicitly keeps scheduler loop, leases, snapshot writer, terminal state, downloads execution, transport, frontend, and SQLite schema out of scope.
3. PWF records show AT-221 was committed and pushed as `89d3a19`.

## Validation Result

- Added README_IMPL 7.30 defining the one-shot shared runtime dispatch boundary and validation expectations.
- Updated PWF records to publish AT-221 as commit `89d3a19` and scope AT-222.
- Scoped `git diff --check` passed for the allowed file set with CRLF normalization warnings only.
