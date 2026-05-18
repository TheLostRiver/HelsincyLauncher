# Active Atomic Task

## Identity

- task id: AT-2026-05-19-253
- title: Define downloads static segment fetcher boundary
- status: completed

## Goal

Document the next downloads fetcher slice before coding, starting with a deterministic static/local byte-source fetcher behind `DownloadSegmentFetchPort` and leaving real HTTP range/provider behavior for later.

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
  - real HTTP range requests
  - provider authentication, CDN policy, or retry/backoff
  - public `DL_NETWORK_*` / `DL_PROVIDER_*` projection
  - streaming fetch, backpressure, or Tokio worker pools
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

1. downloads module docs for provider/fetch/source locator boundaries.
2. `docs/TauriDownloadRuntimeDesign.md` fetcher/provider/range/resume notes.
3. `docs/TauriRepositoryPortsAndAdapterDesign.md` provider adapter ownership rules.
4. `docs/TauriErrorHandlingAndProjectionDesign.md` retryable and public projection rules.
5. `DownloadResumeWorkItem` and `DownloadSegmentExecutionRequest` start-offset semantics from code.

## Hypothesis

- falsifiable planning hypothesis: the next safe fetcher slice can be documented as a deterministic static byte-source fetcher that looks up `request.source_locator`, returns full segment bytes for `FromStart`, returns remaining bytes for `Partial`, preserves an optional etag, and maps missing/invalid static source facts to handled fetch failures without introducing real HTTP/provider behavior.

## Cheap Check

1. Update README_IMPL port status and roadmap now that the verifier is implemented.
2. Add a focused fetcher boundary section defining static byte-source behavior, non-goals, and next RED tests.
3. Re-read changed README_IMPL snippets.
4. Run PWF check and scoped docs/PWF `git diff --check`.

## Validation Gate

1. README_IMPL states writer and length verifier are implemented and fetcher is next.
2. README_IMPL defines from-start and partial static fetch behavior.
3. README_IMPL keeps real HTTP/provider/retry/public projection out of scope.
4. Scoped diff-check passes before commit/push.

## Completion Evidence

- README_IMPL port status and roadmap now mark writer and length verifier as implemented, with fetcher as the next boundary.
- README_IMPL 7.41 defines static source lookup, from-start behavior, partial remaining-byte behavior, handled missing/invalid source failures, and non-goals.
- PWF check reported 123/124 phases complete before final status update, as expected while Phase 124 was still in progress.
- Scoped docs/PWF `git diff --check` passed with CRLF warnings only.
