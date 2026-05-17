# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-202
- title: Record failed results during downloads fake local resume execution
- status: completed; final local commit `043f3f7`, pushed to `origin/main`

## Current In-progress Atomic Task

- task id: AT-2026-05-17-203
- title: Define downloads get-job snapshot query boundary
- status: completed; initial local commit `98c491b` before PWF backfill amend

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Amend AT-203 with the PWF hash backfill and push `main` to `origin`.
2. Start the next code slice with RED tests for `DownloadsFacade::get_job_snapshot(...)` only.
3. Next code slice should start with RED tests for `DownloadsFacade::get_job_snapshot(...)` only.
4. Do not start `list_jobs`, policy persistence, runtime list APIs, adapters, transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- AT-202 pushed: `043f3f7`.
- AT-203 required context read: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/IMPL, download runtime query contracts, IPC query catalog/envelope, AI transaction protocol, downloads contracts DTOs, facade stubs, and kernel-jobs runtime snapshot surface.
- AT-203 scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, SQLite schema or adapter changes, runtime completion, transport, frontend, composition-root changes, or `kernel-jobs` changes.
- Do not implement `list_jobs`, `get_policy`, or `update_policy` during AT-203.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-203:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
