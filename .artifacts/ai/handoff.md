# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-219
- title: Prove downloads policy host transport runtime application
- status: completed; final local commit `f618718`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-220
- title: Document shared runtime execution-turn boundary
- status: completed locally; validation passed, commit and push pending

## Current Slice

- `docs/modules/downloads/README_IMPL.md`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Commit only AT-220 files and push `main` to `origin`.
2. Stop, per user request.
3. Future resume point: code the minimal `kernel-jobs` execution-turn contract described in README_IMPL 7.29, after re-reading the relevant docs.

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
- AT-218 changes are docs/rules only; no Rust behavior changes are in scope.
- AT-218 scoped `git diff --check` passed for the allowed file set with CRLF normalization warnings only.
- AT-219 required context read in focused chunks: README/docs routing, downloads module ARCH/API/FLOW, README_IMPL 7.28, IPC downloads contract section, composition-root transport/wiring rule, testing strategy transport smoke gate, task protocol lifecycle, current downloads command handler, current transport smoke test, and composition-root runtime policy wiring.
- AT-219 host transport smoke validation passed: `downloads_update_policy` returned success, `downloads_get_policy` read back the persisted policy, and the shared runtime policy snapshot reflected the updated concurrency slot count.
- `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` passed with 1 test passed / 0 failed.
- `rustfmt --check src-tauri\tests\transport_wiring_smoke.rs` passed after formatting only the touched smoke test.
- AT-220 required context read in focused chunks: README_IMPL 7.13 and 7.28, kernel-jobs runtime design driver/lease/recovery/runtime-context sections, download scheduler/budget sections, and current `kernel-jobs` runtime/lib/model surfaces.
- AT-220 docs-only validation passed: README_IMPL 7.29 now defines current Rust reality, first Rust slice, and non-goals for the shared runtime execution-turn boundary; scoped `git diff --check` passed with CRLF normalization warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not change Rust code, frontend, host transport, scheduler execution, concrete IO, retry/backoff, terminal completion, SQLite schema, hooks, `.codex`, Cargo.lock, or unrelated dirty files in AT-220.
- Push is authorized by the user-provided GitHub remote after each completed task commit.
- Stop after completing and pushing AT-220 because the user said quota is low.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-218:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
