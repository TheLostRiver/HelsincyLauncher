# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-204
- title: Implement downloads get-job snapshot query
- status: completed; final local commit `2ccc436`, pushed to `origin/main`

## Current In-progress Atomic Task

- task id: AT-2026-05-17-205
- title: Define downloads list-jobs query boundary
- status: completed; initial local commit `c66d3bb` before PWF backfill amend

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Amend AT-205 with the PWF hash backfill and push `main` to `origin`.
2. Start the next code slice with RED tests for `DownloadsFacade::list_jobs(...)`.
3. Next code slice should start with RED tests for `DownloadsFacade::list_jobs(...)`.
4. Do not start policy persistence, runtime list APIs, live snapshot joins, transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- AT-204 pushed: `2ccc436`.
- AT-205 required context read: root docs, README_IMPL, backend use-case table, repository ports docs, kernel runtime list gap, download runtime query contracts, downloads DTOs, facade surface, and SQLite job repository surface.
- AT-205 scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, SQLite schema or adapter changes, runtime completion, transport, frontend, composition-root changes, or `kernel-jobs` changes.
- Do not implement Rust code during AT-205.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-205:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
