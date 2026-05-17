# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-207
- title: Define downloads policy source boundary
- status: completed; final local commit `1d9a04c`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-208
- title: Implement downloads policy store facade semantics
- status: completed; local commit hash will be recorded after commit/push

## Current Slice

- `crates/module-downloads/src/facade/mod.rs`
- `crates/composition-root/src/bootstrap.rs` only if facade dependencies change
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit and push AT-208 only.
2. Re-read README_IMPL and task plan to select the next downloads backend slice.
3. Do not mutate `RuntimeQueuePolicy`, add SQLite schema/adapter policy persistence, host transport, frontend, concrete IO, retry/backoff, or runtime completion without a separate boundary.

## Validation

- Required docs were read in focused chunks: root README, CONTRIBUTING, docs map, downloads module docs, README_IMPL 7.24, download runtime policy/concurrency docs, kernel queue policy docs, storage policy note, repository/adapters rules, composition wiring rules, testing gates, AI transaction protocol, and comment standard.
- RED confirmed missing `InMemoryDownloadPolicyStore` / `policy_store` boundary before implementation.
- Focused policy tests passed: 2 passed / 0 failed.
- Full downloads module tests passed: 45 passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- rustfmt check passed.
- Scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement runtime queue-policy mutation, active job mutation, SQLite schema or adapter policy persistence, transport, frontend, concrete IO, retry/backoff, or terminal completion.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-208:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
