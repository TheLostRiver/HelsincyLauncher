# Active Atomic Task

## Identity

- task id: AT-2026-05-19-263
- title: Define retry/backoff and failure-class persistence boundary
- status: completed

## Goal

Document the next downloads backend boundary after failed reason/retryable persistence: define durable retry count, backoff scheduling facts, and module-owned failure classes before adding more Rust policy fields or enabling `TerminalFailed`.

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
  - SQLite schema changes
  - retry scheduler implementation or execution loops
  - returning `TerminalFailed` from downloads driver
  - stable public `DL_*` error projection
  - host transport, frontend state, provider HTTP, production wiring, leases, scheduler loops, or job snapshot error payloads

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

1. `README.md` current status and near-term roadmap after AT-262.
2. `docs/modules/downloads/README_IMPL.md` 7.45 implementation status and next boundary.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.

## Hypothesis

- falsifiable documentation hypothesis: the next safe slice is not retry execution, but a durable internal classification contract that separates retry hints, retry counters, next retry eligibility, and future public `DL_*` projection.

## Cheap Check

1. Update README current status and roadmap to say the retry/backoff/failure-class boundary is being defined before Rust policy fields.
2. Add README_IMPL 7.46 with concrete field semantics, failure class names, non-goals, and first Rust slice.
3. Update PWF task records and handoff.
4. Run scoped `git diff --check` for touched docs/task files.
5. Commit and attempt push.

## Validation Gate

1. README no longer implies immediate retry engine implementation.
2. README_IMPL defines retry count, next retry eligibility, and failure class without overloading `retryable`.
3. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
4. Scoped diff-check passes or CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current status and roadmap to route the next implementation to retry count, next retry eligibility, and internal failure class Rust persistence.
- Added `docs/modules/downloads/README_IMPL.md` 7.46 defining retry/backoff and failure-class persistence boundaries.
- 7.46 defines internal failure class candidates, retry field semantics, first Rust slice, and non-goals.
- Preserved Rust code, SQLite schema, retry execution, public `DL_*`, `TerminalFailed`, host transport, frontend, provider HTTP, production wiring, leases, scheduler loops, and snapshot error payloads out of scope.
- Validation: scoped `git diff --check` over README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- Commit/push pending.
