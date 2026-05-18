# Active Atomic Task

## Identity

- task id: AT-2026-05-19-250
- title: Define downloads segment length verifier boundary
- status: completed

## Goal

Document the next downloads verifier shell slice before coding, so `DownloadSegmentVerifyPort` can get a concrete byte-length verifier without guessing hash, retry, public error, or production wiring semantics.

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
  - hash algorithm verification
  - file-level or job-level manifest sealing
  - retry/backoff policy
  - public `DL_VERIFY_FAILED` DTO projection
  - composition-root production execution-port wiring
  - host transport, frontend, or SQLite schema changes
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

1. `README.md`, `docs/README.md`, and `CONTRIBUTING.md` entry/collaboration constraints.
2. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` verifier/segment sections.
3. `docs/TauriDownloadRuntimeDesign.md` verifier, staging-first, and verification stages.
4. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable/public projection rules.
5. `docs/TauriTestingStrategyAndQualityGateDesign.md` quality-gate guidance.
6. `docs/TauriCodeCommentStandard.md` Chinese-first comment rule for the later code slice.

## Hypothesis

- falsifiable planning hypothesis: the next safe verifier slice can be documented as a module-local byte-length verifier that compares `written.downloaded_bytes` to `request.length`, returns `Verified` on match, returns an in-band handled failure on mismatch, and leaves hash/public projection/retry/wiring for later tasks.

## Cheap Check

1. Update README_IMPL port status and add a focused verifier shell boundary section.
2. Record AT-249 as committed/pushed and AT-250 as the active docs boundary task.
3. Re-read changed README_IMPL snippets.
4. Run PWF check and scoped docs/PWF `git diff --check`.

## Validation Gate

1. README_IMPL states the concrete filesystem writer is implemented and the verifier shell is next.
2. README_IMPL defines verifier success/failure behavior, non-goals, and next Rust RED tests.
3. PWF records identify AT-250 as docs-only and preserve unrelated dirty work.
4. Scoped diff-check passes before commit/push.

## Completion Evidence

- README_IMPL port status now records `DownloadSegmentFilesystemWritePort` as implemented and `DownloadSegmentVerifyPort` as the next byte-length verifier boundary.
- README_IMPL 7.40 defines byte-length verifier success, handled mismatch failure, non-goals, and next Rust RED tests.
- PWF check reported 120/121 phases complete before final status update, as expected while Phase 121 was still in progress.
- Scoped docs/PWF `git diff --check` passed with CRLF warnings only.
