# Active Atomic Task

## Identity

- task id: AT-2026-05-17-238
- title: Define downloads concrete segment execution boundary
- status: completed

## Goal

Document the next durable downloads implementation boundary for moving from fake/local segment execution results toward real fetch/write/verify execution ports, without writing concrete IO, scheduler loops, retry/backoff, terminal projection, host transport, frontend, or schema changes.

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
  - frontend code
  - host transport changes
  - concrete HTTP/file/hash execution
  - retry/backoff policy
  - terminal runtime completion/failure projection
  - scheduler loops/background tasks/timers
  - durable leases or precise active-slot accounting
  - SQLite schema or adapter changes
  - unrelated dirty files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. docs/modules/downloads/README_ARCH.md/API/FLOW routing snippets.
2. docs/modules/downloads/README_IMPL.md current state and sections 7.31-7.34.
3. docs/TauriDownloadRuntimeDesign.md fetcher/writer/verifier/staging rules.
4. docs/TauriKernelJobsRuntimeDesign.md runtime context and driver execution rules.
5. Current `DownloadSegmentExecutionRequest`, `DownloadSegmentExecutionResult`, and `DownloadSegmentExecutionPort`.

## Hypothesis

- falsifiable design hypothesis: the next safe implementation boundary is not real IO yet; it should first define module-local concrete execution sub-port responsibilities and a future executor adapter shape while keeping current `DownloadSegmentExecutionPort` and runtime dispatch contracts stable.

## Cheap Check

1. Add a concise README_IMPL section after the current runtime execution sections.
2. Keep it durable-boundary only, not a per-task log.
3. Run scoped docs/PWF `git diff --check`.

## Validation Gate

1. The new section identifies the next Rust slice and its non-goals.
2. It does not duplicate full earlier history or widen into task logs.
3. It keeps concrete IO, retry/backoff, terminal runtime state, transport, frontend, and schema out of scope.
4. Scoped docs/PWF diff-check passes before commit/push.

## Validation Result

1. Added `docs/modules/downloads/README_IMPL.md` section 7.35 for the concrete segment execution port boundary.
2. The section defines the next Rust slice as a module-owned executor adapter shell behind the existing `DownloadSegmentExecutionPort`.
3. The section keeps real HTTP/disk/hash IO, retry/backoff, terminal projection, scheduler loops, host transport, frontend, schema, and public `DL_*` execution errors out of scope.
4. Scoped `git diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/findings.md .artifacts/ai/handoff.md .artifacts/ai/progress.md` passed with CRLF normalization warnings only.
