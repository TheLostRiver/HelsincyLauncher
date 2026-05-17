# Active Atomic Task

## Identity

- task id: AT-2026-05-17-213
- title: Define downloads live runtime policy update boundary
- status: completed

## Goal

Document the live runtime policy update boundary before Rust coding, keeping the first mutation surface in `kernel-jobs` and deferring downloads facade wiring, transport/frontend work, scheduler behavior, and active runtime mutation semantics.

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
  - runtime queue-policy mutation
  - live runtime policy update API
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
6. README_IMPL 7.26 completed state and 7.27 boundary target
7. current `RuntimeQueuePolicy`, `SharedJobRuntimeHost`, composition bootstrap, and `DownloadPolicyStore` surfaces
8. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow live policy update boundary whose first Rust slice changes only `kernel-jobs::SharedJobRuntimeHost` policy-control behavior, leaving downloads facade, composition wiring, transport, frontend, scheduler, active jobs, leases, snapshots, and pending work unchanged.

## Cheap Check

1. Add README_IMPL section 7.27 defining the live runtime policy update boundary.
2. Update PWF records and note AT-212 final pushed commit `ed27996`.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL defines the first live policy update slice in `kernel-jobs`.
2. README_IMPL explicitly defers downloads facade wiring, composition-root wiring, host transport, frontend, scheduler loop, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
3. README_IMPL defines first Rust test expectations for `SharedJobRuntimeHost` policy update.
4. Scoped `git diff --check` passes.
5. Commit only AT-213 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.27 now defines the live runtime policy update boundary.
2. README_IMPL selects a first Rust slice in `kernel-jobs` that updates `SharedJobRuntimeHost` policy-control behavior only.
3. README_IMPL explicitly defers downloads facade wiring, composition-root wiring, host transport, frontend, scheduler loop, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
4. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 is ready to commit and push.
