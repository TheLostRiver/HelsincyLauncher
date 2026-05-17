# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-211
- title: Define downloads runtime policy application boundary
- status: completed; final local commit `1d31f56`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-212
- title: Seed runtime policy from persisted downloads policy at startup
- status: completed; validation passed, commit/push pending

## Current Slice

- `crates/composition-root/src/bootstrap.rs`
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit and push AT-212 only.
2. If continuing, read README_IMPL and relevant transport/runtime docs before selecting the next backend slice.
3. Likely next candidate: document the live runtime policy update boundary or host transport policy surface before any code changes.

## Validation

- Required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.26, download runtime concurrency docs, kernel-jobs queue policy docs, composition-root ownership docs, current runtime/bootstrap/policy-store surfaces, and current PWF tails.
- RED: focused composition-root tests failed before implementation because `build_job_runtime(...)` did not accept a policy store.
- GREEN: focused composition-root startup-seeding tests passed: 2 passed / 0 failed.
- Existing shared downloads scheduler wiring lib test passed after moving its sqlite path under project-local `.artifacts/tmp`.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- Adapter policy store and downloads module policy tests passed.
- rustfmt check passed for composition-root.
- Full composition-root integration tests were not run because existing integration tests still create/delete sqlite files under system temp/default package paths.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not add live runtime policy mutation, global settings, transport, frontend, concrete IO, retry/backoff, pending resume work mutation, active runtime job mutation, runtime lease mutation, or runtime snapshot migration in AT-212.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-212:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
