# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-216
- title: Add downloads runtime policy applier port
- status: completed; final local commit `1094c10`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-217
- title: Wire downloads runtime policy applier in composition-root
- status: completed locally; validation passed, final scoped diff check and commit/push pending

## Current Slice

- `crates/composition-root/src/bootstrap.rs`
- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Run final scoped `git diff --check` for AT-217 files.
2. Commit only AT-217 files.
3. Push `main` to `origin`, then choose the next backend slice if continuing.

## Validation

- Required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.26, download runtime concurrency docs, kernel-jobs queue policy docs, composition-root ownership docs, current runtime/bootstrap/policy-store surfaces, and current PWF tails.
- Required context read in focused chunks: README_IMPL runtime-policy sections, `TauriIPCAndStateContractsDesign.md`, `TauriDownloadRuntimeDesign.md`, host transport downloads handlers, and current `SharedJobRuntimeHost` / downloads policy surfaces.
- Required context read in focused chunks: README_IMPL 7.27, kernel-jobs queue policy design, current `RuntimeQueuePolicy` / `SharedJobRuntimeHost` implementation, and PWF tails.
- AT-214 RED/GREEN validation passed for cloned runtime policy update/readback and existing runtime enqueue snapshots.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed after the runtime host shape change.
- `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib build_job_runtime_` passed, 2 passed / 0 failed.
- `rustfmt --check crates\kernel-jobs\src\runtime.rs` passed after formatting only the AT-214 Rust file.
- Package-level `cargo fmt -p launcher-kernel-jobs --check` still reports pre-existing out-of-scope formatting diffs in `crates/kernel-jobs/src/lib.rs` and `crates/kernel-jobs/src/model.rs`; do not stage those files for AT-214.
- AT-215 required docs were read in focused chunks: README/docs routing, downloads module docs, `TauriDownloadRuntimeDesign.md`, `TauriKernelJobsRuntimeDesign.md`, `TauriCompositionRootWiringDesign.md`, current downloads facade policy code, current composition-root wiring, and PWF tails.
- AT-215 README_IMPL 7.28 defines the downloads runtime policy applier boundary and first Rust slice.
- AT-215 scoped `git diff --check` passed with CRLF normalization warnings only.
- AT-216 required docs/code were read in focused chunks: README_IMPL 7.28, downloads facade policy methods/tests, current module exports, current composition-root runtime/downloads wiring, and PWF tails.
- AT-216 RED/GREEN validation passed for normalized policy handoff to `DownloadRuntimePolicyApplier`.
- Full downloads module tests passed, 47 passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --check crates\module-downloads\src\facade\mod.rs crates\module-downloads\src\lib.rs` passed.
- AT-217 required docs/code were read in focused chunks: README_IMPL 7.28, `TauriCompositionRootWiringDesign.md`, current composition-root runtime/downloads builders, current composition-root project-local SQLite tests, and PWF tails.
- AT-217 RED/GREEN validation passed for composition-root runtime policy applier wiring.
- `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib build_job_runtime_` passed, 2 passed / 0 failed.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy` passed, 3 passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --check crates\composition-root\src\bootstrap.rs` passed after formatting.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not add host transport, frontend, global settings, concrete IO, retry/backoff, pending resume work mutation, active runtime job mutation, runtime lease mutation, runtime snapshot migration, or scheduler execution in AT-217.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-217:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
