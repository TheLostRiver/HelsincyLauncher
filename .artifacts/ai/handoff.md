# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-16-176
- title: Guard runtime enqueue on scheduler failure
- status: completed and ready for the AT-176 local commit

## Current In-progress Atomic Task

- none

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `crates/module-downloads/src/facade/mod.rs`
- `crates/module-downloads/src/facade/mod.rs`
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
- Passed for AT-2026-05-16-174:
  - Required docs were read in scoped snippets before editing README_IMPL.
  - README_IMPL now defines `DownloadResumeWorkScheduler`, call order, failure behavior, and next Rust slice.
  - Scoped `git diff --check` passed for AT-174 files with CRLF warnings only.
- Passed for AT-2026-05-16-175:
  - RED failed on missing `DownloadResumeWorkScheduler`.
  - Focused scheduler-before-enqueue test passed after implementation.
  - Full `launcher-module-downloads` suite passed with 19 passed, 0 failed.
  - Composition `bootstrap_wiring_smoke` passed after placeholder scheduler wiring and stale checkpoint initializer repair.
  - README_IMPL records implemented scheduler port and next scheduler-failure guard slice.
- Passed for AT-2026-05-16-176:
  - RED failed on missing failing scheduler fake behavior.
  - Focused scheduler-failure guard passed after fake behavior was added.
  - Full `launcher-module-downloads` suite passed with 20 passed, 0 failed.
  - README_IMPL records scheduler failures returning before runtime enqueue.

## Current Git State To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-2026-05-16-173:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`

## Next Resume Point

1. Commit AT-2026-05-16-176, then start task 4/4.
2. Next slice: add a focused all-sealed/no-scheduler guard so already-complete resumes do not touch the scheduler boundary.
3. Keep concrete execution, persistence, host transport, frontend, and `kernel-jobs` payload changes out of scope.
3. Preserve unrelated dirty frontend, sqlite, Cargo.lock, `.codex`, `src/`, and pen files.
4. Do not retry direct `origin/main` push without explicit approval; previous direct push attempts were blocked by safety review.
