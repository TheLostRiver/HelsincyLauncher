# Active Atomic Task

## Identity

- task id: AT-2026-05-17-235
- title: Define host runtime execution command boundary
- status: completed

## Goal

Document the smallest durable host transport boundary for invoking exactly one shared runtime execution turn through the composition-root helper, without adding frontend UI, scheduler loops, background tasks, durable leases, terminal projection, downloads concrete IO, retry/backoff, or SQLite schema changes.

## Scope

- in scope:
  - `docs/TauriIPCAndStateContractsDesign.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production or test code
  - frontend code
  - downloads module behavior
  - composition-root helper implementation changes
  - scheduler loops/background tasks
  - durable leases or precise active-slot accounting
  - terminal completion/failure projection
  - concrete HTTP/file/hash execution
  - retry/backoff policy
  - SQLite schema or adapter changes
  - unrelated dirty files

## Allowed Files

1. docs/TauriIPCAndStateContractsDesign.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md routing.
2. docs/TauriCompositionRootWiringDesign.md 9.4 and Tauri integration boundary.
3. docs/TauriIPCAndStateContractsDesign.md command/query envelope and implementation guidance.
4. docs/TauriStartupPipelineDesign.md restore/warmup ownership rules.
5. docs/modules/downloads/README_IMPL.md runtime execution sections 7.29-7.34.
6. Current `src-tauri/src/commands/*`, `src-tauri/src/bootstrap.rs`, `src-tauri/src/state.rs`, and transport smoke patterns.

## Hypothesis

- falsifiable design hypothesis: the next safe host boundary is a command, not a query, tentatively `jobs_run_next_execution_turn`, that calls only `DesktopAppServices.startup.run_one_runtime_execution_turn()` and returns a stable disposition DTO without exposing segment-level downloads details.

## Cheap Check

1. Add a focused IPC design section for the command name, DTO shape, mapping rules, validation gate, and non-goals.
2. Update PWF records with AT-234 final commit and AT-235 scope.
3. Run scoped docs/PWF `git diff --check`.

## Validation Gate

1. IPC design clearly distinguishes command side effects from read-model queries.
2. DTO mapping covers `Accepted`, `Deferred`, and `Failed` without converting non-terminal deferred/failed dispositions into `AppErrorDto`.
3. Design keeps automatic startup invocation, scheduler loops, durable leases, terminal projection, downloads segment internals, frontend, and schema changes out of scope.
4. Scoped diff check passes before commit/push.

## Validation Result

1. Added the IPC boundary section for `jobs_run_next_execution_turn`, including command-vs-query rationale, DTO shape, mapping rules, non-goals, and host-boundary validation.
2. Updated PWF records with AT-234 commit `256f89b` and AT-235 scope.
3. Scoped `git diff --check -- docs/TauriIPCAndStateContractsDesign.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/findings.md .artifacts/ai/handoff.md .artifacts/ai/progress.md` passed with CRLF normalization warnings only.
