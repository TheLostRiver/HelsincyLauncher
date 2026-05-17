# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-203
- title: Define downloads get-job snapshot query boundary
- status: completed; final local commit `fb5a94e`, pushed to `origin/main`

## Current In-progress Atomic Task

- task id: AT-2026-05-17-204
- title: Implement downloads get-job snapshot query
- status: completed; initial local commit `d1de743` before PWF backfill amend

## Current Slice

- `crates/module-downloads/src/facade/mod.rs`
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Amend AT-204 with the PWF hash backfill and push `main` to `origin`.
2. Reassess README_IMPL before selecting the next downloads backend slice.
3. Reassess README_IMPL before selecting the next downloads backend slice.
4. Do not start `list_jobs`, policy persistence, runtime list APIs, adapters, transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- AT-203 pushed: `fb5a94e`.
- AT-204 required context read: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/IMPL section 7.22, download runtime query contracts, IPC query catalog/envelope, AI transaction protocol, downloads contracts DTOs, facade stubs/test helpers, kernel-jobs runtime snapshot surface, and TDD skill.
- AT-204 RED failed on the current `DOWNLOADS_NOT_WIRED` stub, focused GREEN passed with 3 tests, full downloads module passed with 41 tests, rustfmt check passed after formatting, and scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, SQLite schema or adapter changes, runtime completion, transport, frontend, composition-root changes, or `kernel-jobs` changes.
- Do not implement `list_jobs`, `get_policy`, or `update_policy` during AT-204.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-204:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
