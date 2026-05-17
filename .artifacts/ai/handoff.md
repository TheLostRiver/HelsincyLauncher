# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-210
- title: Implement downloads SQLite policy store
- status: completed; final local commit `2f9e828`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-211
- title: Define downloads runtime policy application boundary
- status: completed; docs-only boundary validated, commit/push pending

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit and push AT-211 only.
2. If continuing, implement the documented startup-seeding Rust slice with focused composition-root RED tests.

## Validation

- Required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL policy sections, download runtime concurrency docs, kernel-jobs queue policy docs, composition-root ownership docs, current runtime/bootstrap/policy-store surfaces, and current PWF tails.
- README_IMPL 7.26 defines persisted-policy startup seeding, empty-store fallback, deferred live runtime mutation, and the first Rust test expectations.
- Scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not write Rust code during AT-211.
- Do not mutate `RuntimeQueuePolicy`, global settings, transport, frontend, concrete IO, retry/backoff, pending resume work, active runtime jobs, runtime leases, or runtime snapshots in AT-211.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-211:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
