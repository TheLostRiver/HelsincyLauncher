# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-206
- title: Implement downloads list-jobs query
- status: completed; final local commit `d0ad61a`, pushed to `origin/main`

## Current In-progress Atomic Task

- task id: AT-2026-05-17-207
- title: Define downloads policy source boundary
- status: completed; initial local commit `07bdcfb` before PWF backfill amend

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Amend AT-207 with the PWF hash backfill and push `main` to `origin`.
2. Start the next code slice with RED tests for downloads policy store/get/update semantics.
3. Next code slice should start with RED tests for downloads policy store/get/update semantics.
4. Do not mutate `RuntimeQueuePolicy`, add SQLite schema, transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- AT-206 pushed: `d0ad61a`.
- AT-207 required context read: root docs, README_IMPL policy/query state, download runtime policy/concurrency docs, kernel runtime queue policy docs, storage policy note, current policy DTOs, and facade policy stubs.
- AT-207 scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, SQLite schema or adapter changes, runtime completion, transport, frontend, composition-root changes, or `kernel-jobs` changes.
- Do not implement Rust code during AT-207.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-207:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
