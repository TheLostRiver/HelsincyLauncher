# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-16-173
- title: Add downloads resume work plan derivation
- status: completed and ready for the AT-173 local commit

## Current In-progress Atomic Task

- none

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `crates/module-downloads/src/facade/mod.rs`
- `crates/module-downloads/src/lib.rs`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Validation

- Passed for AT-2026-05-16-172:
  - Required root, docs index, downloads module, runtime, kernel-jobs, crate layout/API, testing, and collaboration docs were read in scoped snippets.
  - README_IMPL section 7.6 now documents that shared `JobRuntime` receives only job-level enqueue, while resume segment work plans remain downloads-owned.
  - `resume_partial` and `queue_remaining` are future scheduler work candidates; `seal_completed` and `reject_mismatch` produce no work item.
  - Scoped `git diff --check` passed for AT-172 files with CRLF warnings only.
- Passed for AT-2026-05-16-173:
  - RED test failed on missing work-plan API before production code was added.
  - Focused work-plan test passed after adding the minimal module-local derivation.
  - Full `launcher-module-downloads` test suite passed with 18 passed, 0 failed.
  - README_IMPL records the implemented work-plan derivation and unchanged out-of-scope surfaces.
  - Scoped `git diff --check` passed for AT-173 files with CRLF warnings only.

## Current Git State To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-2026-05-16-173:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`

## Next Resume Point

1. Start the next slice by reading the required docs again and deciding the downloads-owned scheduler/driver boundary that consumes `DownloadResumeWorkPlan`.
2. Keep concrete fetch/write/verify execution, persistence schema, host transport, frontend, and `kernel-jobs` payload changes out of scope until that boundary is explicit.
3. Preserve unrelated dirty frontend, sqlite, Cargo.lock, `.codex`, `src/`, and pen files.
4. Do not retry direct `origin/main` push without explicit approval; previous direct push attempts were blocked by safety review.
