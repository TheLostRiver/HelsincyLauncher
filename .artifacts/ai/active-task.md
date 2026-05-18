# Active Atomic Task

## Identity

- task id: AT-2026-05-19-257
- title: Define runtime terminal completion/failure projection boundary
- status: completed

## Goal

Update the durable downloads implementation docs and root README after the AT-256 static wiring proof, defining the next shared runtime terminal projection boundary before changing `kernel-jobs` state behavior.

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
  - downloads driver terminal decision implementation
  - retry/backoff or stable public `DL_*` execution errors
  - host transport, frontend, SQLite schema, provider HTTP, or real production execution-port wiring
  - moving `.artifacts/ai` records to the repo root; current project docs keep `.artifacts/ai` authoritative

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
2. `docs/README.md` README/docs update routing rules.
3. `docs/ModuleDocumentationStandard.md` documentation-budget rules.
4. `docs/modules/downloads/README_IMPL.md` 6.1 and 7.42.
5. `docs/TauriKernelJobsRuntimeDesign.md` job lifecycle and snapshot projection rules.
6. `docs/TauriDownloadRuntimeDesign.md` downloads checkpoint/runtime ownership.
7. `docs/TauriErrorHandlingAndProjectionDesign.md` long-job failure and public error projection rules.
8. `docs/TauriCompositionRootWiringDesign.md` assembly-owner rules.
9. Current `kernel-jobs` runtime and downloads driver snippets for `JobRunDisposition`, checkpoint mutation, and default deferred wiring.

## Hypothesis

- falsifiable documentation hypothesis: after AT-256, the correct next durable boundary is an explicit `kernel-jobs` terminal disposition/projection contract, not immediate downloads production wiring or public `DL_*` execution errors; README and README_IMPL can describe this without changing Rust behavior.

## Cheap Check

1. Update root README current status and roadmap.
2. Update README_IMPL 6.1 and add a concise 7.43 terminal projection boundary.
3. Update PWF task records and handoff.
4. Run scoped `git diff --check` for the touched docs/task files.
5. Commit and attempt push.

## Validation Gate

1. README no longer says composition-root wiring proof is still next.
2. README_IMPL records AT-256 implementation status and defines terminal projection before code.
3. Scope keeps Rust/transport/frontend/provider/retry work out of this docs-only task.
4. Scoped diff-check passes or any CRLF-only warnings are recorded.

## Completion Evidence

- Updated `README.md` current status and near-term roadmap to point at runtime terminal completion/failure projection.
- Updated `docs/modules/downloads/README_IMPL.md` 6.1, 7.42 implementation status, and new 7.43 terminal projection boundary.
- Validation: `git diff --check -- README.md docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/findings.md .artifacts/ai/progress.md .artifacts/ai/handoff.md` passed with CRLF normalization warnings only.
- No Rust, transport, frontend, provider, retry/backoff, public `DL_*`, or schema files were edited.
- Commit/push pending.
