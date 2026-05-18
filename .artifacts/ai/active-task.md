# Active Atomic Task

## Identity

- task id: AT-2026-05-19-265
- title: Define backoff policy and terminal failure eligibility boundary
- status: completed

## Goal

Document the next downloads backend boundary after durable retry facts: define the pure backoff policy, retry exhaustion, user-attention, and terminal failure eligibility rules before any Rust retry policy implementation.

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

1. `README.md` current status and near-term roadmap after AT-264.
2. `docs/modules/downloads/README_IMPL.md` 7.46 implementation status and next boundary.
3. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics and stable public code rules.
4. `docs/TauriDownloadRuntimeDesign.md` checkpoint save rules and failure classification principles.
5. `docs/modules/downloads/README_API.md`, `README_ARCH.md`, and `README_FLOW.md` module boundary notes.

## Hypothesis

- falsifiable documentation hypothesis: the next safe code slice is a pure retry/backoff policy calculator that can classify automatic retry eligibility, retry exhaustion, and user-attention needs without scheduling work or returning `TerminalFailed`.

## Cheap Check

1. Update README current status to point the next Rust slice at pure backoff policy calculation.
2. Add README_IMPL 7.47 defining automatic retry gates, max attempts, delay schedule, user-attention classes, terminal-candidate constraints, and first Rust slice.
3. Update PWF records and handoff.
4. Run scoped `git diff --check` for touched docs/task files.
5. Commit locally; do not reattempt push unless the user explicitly approves after the prior safety block.

## Validation Gate

1. README names pure retry/backoff policy calculation as the next Rust slice.
2. README_IMPL 7.47 defines deterministic policy rules without starting retry execution.
3. Public `DL_*`, `TerminalFailed`, host/frontend, provider, scheduler, lease, and snapshot error payload work remain explicitly out of scope.
4. Scoped diff-check passes or CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current roadmap to say the next downloads Rust slice is a pure backoff policy calculator before scheduler, `TerminalFailed`, or public `DL_*`.
- Added `docs/modules/downloads/README_IMPL.md` 7.47 defining:
  - automatic retry gates by `failure_retryable` and `DownloadSegmentFailureClass`;
  - initial max automatic segment retry attempts of three observed failures;
  - deterministic delay schedule of attempt 1 -> 30s and attempt 2 -> 120s;
  - exhausted/user-attention/terminal-candidate boundaries;
  - first Rust slice as a pure policy helper with fixed `IsoDateTime now` input.
- Preserved Rust code, SQLite schema, retry scheduler loops, automatic dispatch, `TerminalFailed`, public `DL_*`, host/frontend/provider/lease/snapshot payload changes out of scope.
- Validation: scoped `git diff --check` over README, README_IMPL, and PWF files passed with CRLF normalization warnings only.
