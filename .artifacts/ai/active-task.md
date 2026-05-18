# Active Atomic Task

## Identity

- task id: AT-2026-05-19-261
- title: Define failed segment metadata and retry classification boundary
- status: completed

## Goal

Document the next downloads backend boundary: preserve failed segment metadata durably before implementing retry/backoff, terminal-failed driver decisions, or public `DL_*` execution errors.

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
  - returning `TerminalFailed` from downloads driver
  - retry scheduler/backoff engine or stable public `DL_*` execution errors
  - host transport, frontend, provider HTTP, production wiring, scheduler loops, leases, or snapshot error payloads

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

1. `README.md` current status and near-term roadmap.
2. `docs/modules/downloads/README_IMPL.md` 7.44 and current roadmap.
3. `docs/TauriDownloadRuntimeDesign.md` failure classification and retry principles.
4. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable semantics.
5. `crates/module-downloads/src/driver.rs` failed result and handled failure data shape.
6. `crates/adapter-storage-sqlite/src/lib.rs` checkpoint table/round-trip surface.

## Hypothesis

- falsifiable documentation hypothesis: the next safe backend slice is durable failed-segment metadata, not terminal failure projection, because current checkpoint persistence drops retryability and local diagnostic reason.

## Cheap Check

1. Update root README current status/roadmap.
2. Update README_IMPL 6.1 and add 7.45 with current reality, rules, and first Rust slice.
3. Update PWF task records and handoff.
4. Run scoped `git diff --check` for touched docs/task files.
5. Commit and attempt push.

## Validation Gate

1. README points to failed metadata/retry classification, not immediate terminal failure.
2. README_IMPL explains why `TerminalFailed` remains blocked.
3. First Rust slice is concrete and avoids host/frontend/provider/scheduler work.
4. Scoped diff-check passes or CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current status and near-term roadmap to name failed metadata persistence and retry/backoff classification as the next backend boundary.
- Updated `docs/modules/downloads/README_IMPL.md` 6.1 and added 7.45 with failed segment metadata and retry classification rules.
- Validation: scoped `git diff --check` over README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- No Rust, SQLite schema, transport, frontend, provider, scheduler, lease, or snapshot error payload files were edited.
- Commit `124dbb3` pushed to `origin/main`.
