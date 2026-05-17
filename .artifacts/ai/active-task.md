# Active Atomic Task

## Identity

- task id: AT-2026-05-17-233
- title: Define composition one-shot runtime execution helper boundary
- status: completed

## Goal

Define the composition-root boundary for explicitly composing the shared runtime host and driver registry into a one-shot execution helper, while avoiding automatic scheduler loops, startup hidden side effects, durable leases, terminal projection, downloads IO, transport, frontend, and SQLite schema changes.

## Scope

- in scope:
  - `docs/TauriCompositionRootWiringDesign.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust code changes
  - downloads driver/concrete execution changes
  - composition-root wiring
  - durable leases
  - scheduler loops/background tasks
  - snapshot-writer/cancellation context
  - precise active-slot accounting or queue fairness beyond existing runtime policy gate
  - terminal completion/failure projection
  - host transport, frontend, SQLite schema, and unrelated dirty files

## Allowed Files

1. docs/TauriCompositionRootWiringDesign.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md routing.
2. docs/TauriCompositionRootWiringDesign.md startup and public API sections.
3. docs/TauriStartupPipelineDesign.md restore/warmup rules.
4. README_IMPL 7.34 current runtime state.
5. current composition-root runtime/driver registry wiring.

## Hypothesis

- falsifiable documentation hypothesis: the next safe composition-root slice is a one-shot helper that explicitly calls `SharedJobRuntimeHost::run_next_execution_turn(&registry)` without being invoked automatically by `build_desktop_services()` or startup stages.

## Cheap Check

1. Update composition-root wiring docs with current state and first Rust slice.
2. Keep task-log detail in `.artifacts/ai`.
3. Run scoped `git diff --check` for docs and PWF files.

## Validation Gate

1. The doc states current AT-232 runtime state and current composition-root wiring gap.
2. The doc defines the helper as explicit one-shot behavior, not an automatic loop or startup side effect.
3. Out-of-scope scheduler, lease, terminal, IO, transport, frontend, and schema changes remain clear.
4. Scoped `git diff --check` passes before commit.

## Validation Result

1. `docs/TauriCompositionRootWiringDesign.md` now includes section 9.4 for the one-shot runtime execution helper boundary.
2. The section states current AT-232 runtime state, the composition-root runtime/registry wiring gap, and the first Rust slice for `StartupPipelineFacade::run_one_runtime_execution_turn(...)`.
3. The section keeps automatic scheduler loops, startup hidden side effects, durable leases, precise active-slot accounting, terminal projection, downloads IO, transport, frontend, and SQLite schema changes out of scope.
4. Scoped `git diff --check` passed for the composition doc and PWF files with CRLF normalization warnings only.
