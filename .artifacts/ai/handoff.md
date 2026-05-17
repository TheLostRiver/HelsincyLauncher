# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-205
- title: Define downloads list-jobs query boundary
- status: completed; final local commit `17e0bb4`, pushed to `origin/main`

## Current In-progress Atomic Task

- task id: AT-2026-05-17-206
- title: Implement downloads list-jobs query
- status: completed; initial local commit `87b09ab` before PWF backfill amend

## Current Slice

- `crates/module-downloads/src/facade/mod.rs`
- `crates/adapter-storage-sqlite/src/lib.rs`
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Amend AT-206 with the PWF hash backfill and push `main` to `origin`.
2. Reassess README_IMPL before selecting the next downloads backend slice.
3. Reassess README_IMPL before selecting the next downloads backend slice.
4. Do not start policy persistence, runtime list APIs, live snapshot joins, transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- AT-205 pushed: `17e0bb4`.
- AT-206 required context read: root docs, README_IMPL 7.23, backend use-case table, repository ports docs, kernel runtime list gap, download runtime query contracts, downloads DTOs, facade/test helpers, SQLite job repository surface, and TDD skill.
- AT-206 RED failed on `DOWNLOADS_NOT_WIRED`; focused GREEN passed with 2 tests; full downloads module passed with 43 tests; SQLite adapter `cargo check` passed; rustfmt check passed; scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, SQLite schema or adapter changes, runtime completion, transport, frontend, composition-root changes, or `kernel-jobs` changes.
- Do not implement `get_policy` or `update_policy` during AT-206.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-206:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
