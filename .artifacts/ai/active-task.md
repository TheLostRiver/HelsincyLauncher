# Active Atomic Task

## Identity

- task id: AT-2026-05-19-268
- title: Define due retry-ready segment selection boundary
- status: completed

## Goal

Document the next downloads retry boundary after policy-computed `next_retry_after`: define how to select due retry-ready failed segment checkpoint facts and how that later combines with manifest facts before any scheduler or execution wiring.

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

1. `README.md` current status and near-term roadmap after AT-267.
2. `docs/modules/downloads/README_IMPL.md` 7.47 implementation status and next implementation target.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable public code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.
5. `docs/modules/downloads/README_API.md`, `README_ARCH.md`, and `README_FLOW.md` module boundary notes.

## Hypothesis

- falsifiable documentation hypothesis: due retry-ready selection should first return eligible failed checkpoint facts, and only a later manifest-bound slice should turn those facts into executable segment work.

## Cheap Check

1. Update README current status to point the next Rust slice at due retry-ready checkpoint selection.
2. Add README_IMPL 7.48 defining due eligibility, selector input/output, manifest binding, first Rust slice, and non-goals.
3. Update PWF records and handoff.
4. Run scoped `git diff --check` for touched docs/task files.
5. Commit locally; do not reattempt push unless the user explicitly approves after the prior safety block.

## Validation Gate

1. README names due retry-ready checkpoint selection as the next Rust slice.
2. README_IMPL 7.48 defines selection before executable work derivation.
3. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
4. Scoped diff-check passes or CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current roadmap to route the next Rust slice to due retry-ready checkpoint selection before manifest-bound retry work derivation.
- Added `docs/modules/downloads/README_IMPL.md` 7.48 defining:
  - due-selection rules for failed checkpoint facts with `next_retry_after <= now`;
  - delayed/missing-time/non-failed exclusion rules;
  - order-preservation requirements;
  - first Rust selector slice;
  - explicit separation from manifest reconstruction and executable retry work.
- Preserved Rust code, SQLite schema, scheduler loops, automatic dispatch, `TerminalFailed`, public `DL_*`, host/frontend/provider/lease/snapshot payload changes out of scope.
- Validation: scoped `git diff --check` over README, README_IMPL, and PWF files passed with CRLF normalization warnings only.
