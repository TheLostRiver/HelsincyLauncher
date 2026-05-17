# Active Atomic Task

## Identity

- task id: AT-2026-05-17-215
- title: Define downloads runtime policy applier boundary
- status: completed

## Goal

Document the downloads runtime policy applier boundary before Rust coding, keeping the first module slice focused on a downloads-owned applier port and deferring composition-root wiring, concrete `SharedJobRuntimeHost` calls, transport/frontend behavior, scheduler execution, active jobs, leases, snapshots, pending work, concrete IO, retry/backoff, and terminal completion.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code
  - Rust tests
  - downloads facade policy-applier wiring
  - composition-root policy-applier wiring
  - direct `SharedJobRuntimeHost` calls from downloads code
  - global settings/config-system implementation
  - host transport or frontend changes
  - scheduler loop, per-module caps, per-host caps, writer backpressure, or fairness implementation
  - concrete IO, retry/backoff, or terminal runtime completion
  - active jobs, runtime leases, runtime snapshots, pending resume work, or segment scheduling behavior

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. README.md and docs/README.md backend/documentation routing
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md policy sections
3. docs/TauriDownloadRuntimeDesign.md concurrency/policy sections
4. docs/TauriKernelJobsRuntimeDesign.md queue-policy sections
5. docs/TauriCompositionRootWiringDesign.md composition-root ownership sections
6. README_IMPL 7.27 completed state and 7.28 boundary target
7. current `DownloadsFacade::update_policy(...)`, `DownloadPolicyStore`, `SharedJobRuntimeHost::update_policy(...)`, and composition-root wiring surfaces
8. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow runtime policy applier boundary whose first Rust slice changes only the downloads module facade dependency surface, leaving composition-root concrete runtime wiring and transport/frontend behavior for later.

## Cheap Check

1. Add README_IMPL section 7.28 defining the downloads runtime policy applier boundary.
2. Update PWF records and note AT-214 final pushed commit `c92be25`.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL defines the first downloads runtime policy applier slice in `module-downloads`.
2. README_IMPL requires a normalized `DownloadPolicyDto` to be passed to the applier after persistence.
3. README_IMPL explicitly defers composition-root concrete wiring, direct `SharedJobRuntimeHost` calls from downloads code, host transport/frontend behavior, scheduler execution, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
4. Scoped `git diff --check` passes.
5. Commit only AT-215 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.28 now defines the downloads runtime policy applier boundary.
2. README_IMPL selects a first Rust slice in `module-downloads` that introduces a downloads-owned applier port and passes the normalized/clamped `DownloadPolicyDto` after persistence.
3. README_IMPL explicitly defers composition-root concrete wiring, direct `SharedJobRuntimeHost` calls from downloads code, host transport/frontend behavior, scheduler execution, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
4. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 final commit is `38c32b2` and is already pushed to `origin/main`.
- AT-2026-05-17-214 final commit is `c92be25` and is already pushed to `origin/main`.
