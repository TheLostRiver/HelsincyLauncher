# Active Atomic Task

## Identity

- task id: AT-2026-05-17-211
- title: Define downloads runtime policy application boundary
- status: completed

## Goal

Document the next runtime-policy integration boundary so persisted downloads policy can safely seed the initial shared runtime queue budget without adding live runtime mutation, transport/frontend settings, concrete scheduler work, or unrelated settings behavior.

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
6. current `RuntimeQueuePolicy`, `SharedJobRuntimeHost`, composition bootstrap, and `DownloadPolicyStore` surfaces
7. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: README_IMPL can define a narrow startup-seeding boundary where persisted downloads policy influences the initial `RuntimeQueuePolicy`, while live runtime mutation and all scheduler/transport/frontend surfaces remain explicitly deferred.

## Cheap Check

1. Add README_IMPL section 7.26 documenting the runtime policy application boundary.
2. Update PWF records and note AT-210 final pushed commit `2f9e828`.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. README_IMPL defines startup seeding from persisted downloads policy into initial `RuntimeQueuePolicy`.
2. README_IMPL explicitly defers live runtime mutation, transport/frontend settings, scheduler loop, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
3. README_IMPL defines the first Rust slice and focused test expectations.
4. Scoped `git diff --check` passes.
5. Commit only AT-211 files locally, then push `main` to `origin`.

## Validation Result

1. README_IMPL section 7.26 now defines startup seeding from persisted downloads policy into initial `RuntimeQueuePolicy`.
2. README_IMPL explicitly defers live runtime mutation, runtime policy update APIs, scheduler loop, active jobs/leases/snapshots, pending work, transport/frontend settings, concrete IO, retry/backoff, and terminal completion.
3. README_IMPL defines the first Rust slice and focused composition-root test expectations.
4. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 is ready to commit and push.
