# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-209
- title: Define downloads policy SQLite persistence boundary
- status: completed; final local commit `41f0b8c`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-210
- title: Implement downloads SQLite policy store
- status: completed; validation passed, commit/push pending

## Current Slice

- `crates/adapter-storage-sqlite/src/lib.rs`
- `crates/composition-root/src/bootstrap.rs`
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit and push AT-210 only.
2. If continuing, read README_IMPL and relevant docs again before selecting the next backend slice.
3. Likely next candidate: document the runtime policy-application boundary before any code that translates persisted downloads policy into shared `RuntimeQueuePolicy`.

## Validation

- Required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL policy sections, storage policy placement, port/adapter ownership rules, current SQLite adapter shapes, current policy store surface, and current PWF tails.
- RED: focused adapter tests failed before implementation because `SqliteDownloadPolicyStore` did not exist.
- GREEN: focused adapter policy tests passed: 2 passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml policy` passed: 2 passed / 0 failed.
- rustfmt check passed for `launcher-adapter-storage-sqlite` and `launcher-composition-root`.
- Scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not mutate `RuntimeQueuePolicy`, global settings, transport, frontend, concrete IO, retry/backoff, pending resume work, active runtime jobs, runtime leases, or runtime snapshots in AT-210.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-210:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
