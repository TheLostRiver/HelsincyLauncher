# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-211
- title: Define downloads runtime policy application boundary
- status: completed; final local commit `1d31f56`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-213
- title: Define downloads live runtime policy update boundary
- status: completed; docs-only boundary validated, commit/push pending

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit and push AT-213 only.
2. If continuing, implement the documented `kernel-jobs` live runtime policy control surface with focused RED tests.

## Validation

- Required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.26, download runtime concurrency docs, kernel-jobs queue policy docs, composition-root ownership docs, current runtime/bootstrap/policy-store surfaces, and current PWF tails.
- Required context read in focused chunks: README_IMPL runtime-policy sections, `TauriIPCAndStateContractsDesign.md`, `TauriDownloadRuntimeDesign.md`, host transport downloads handlers, and current `SharedJobRuntimeHost` / downloads policy surfaces.
- README_IMPL 7.27 defines the live runtime policy update boundary and first Rust test expectations.
- Scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not write Rust code during AT-213.
- Do not add downloads facade wiring, composition-root wiring, global settings, transport, frontend, concrete IO, retry/backoff, pending resume work mutation, active runtime job mutation, runtime lease mutation, or runtime snapshot migration in AT-213.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-213:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
