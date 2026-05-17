# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-221
- title: Add minimal kernel-jobs execution-turn contract
- status: completed; final commit `89d3a19`, pushed to `origin/main`

## Current Atomic Task

- task id: AT-2026-05-17-223
- title: Add one-shot kernel-jobs execution dispatch
- status: completed; final commit `f87df03`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-224
- title: Document downloads driver runtime-run override boundary
- status: completed; final commit `597c0e5`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-225
- title: Add guarded downloads driver run override
- status: completed; final commit `c5b0695`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-226
- title: Cover downloads driver run deferred branches
- status: completed; final commit `d2d5405`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-227
- title: Define accepted execution state projection boundary
- status: completed; final commit `fc615db`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-228
- title: Project accepted execution dispatch to running state
- status: completed; final commit `fb9fb57`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-229
- title: Define one-shot queued execution selection boundary
- status: completed; final commit `d339db7`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-230
- title: Add one-shot queued execution selector
- status: completed; final commit `8db4900`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-231
- title: Define one-shot queue policy slot gate boundary
- status: completed; final commit `6f5bd32`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-232
- title: Add one-shot queue policy slot gate
- status: completed; final commit `d2fa1d9`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-233
- title: Define composition one-shot runtime execution helper boundary
- status: completed; final commit `01c206d`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-234
- title: Add composition one-shot runtime execution helper
- status: completed; validation passed; publication handled in Git history

## Current Slice

- `crates/composition-root/src/startup.rs`
- `crates/composition-root/src/bootstrap.rs`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. If continuing, choose the next backend slice from the documented runtime/downloads execution path before coding.

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
- AT-221 required context read in focused chunks: README_IMPL 7.29, kernel-jobs runtime design driver/queue/lease/recovery/runtime-context sections, downloads scheduler/budget notes, current `kernel-jobs` runtime/lib/model code, and current module driver implementations.
- AT-221 RED/GREEN validation passed: missing execution-turn contract failed first, then `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 4 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed; broader rustfmt remains blocked by pre-existing out-of-scope `crates/kernel-jobs/src/model.rs` formatting.
- AT-222 required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.29, kernel-jobs runtime driver/runtime-host/runtime-context sections, download runtime scheduler/budget notes, current runtime/lib/model code, and composition-root driver-registry wiring.
- README_IMPL 7.30 now defines the next Rust slice as a one-shot `SharedJobRuntimeHost` dispatch method over snapshot lookup, driver registry resolution, and `driver.run(...)`.
- AT-222 scoped `git diff --check` passed for README_IMPL and PWF files with CRLF normalization warnings only.
- AT-223 required context read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.29-7.30, kernel-jobs runtime design sections, download runtime scheduler/budget notes, testing strategy, current runtime/lib/model code, and composition-root driver-registry wiring.
- AT-223 RED/GREEN validation passed: focused dispatch tests first failed on missing `run_one_execution_turn(...)`, then passed after implementation.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 7 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed.
- Scoped `git diff --check` passed for AT-228 files with CRLF normalization warnings only.
- AT-228 final commit `fb9fb57` was pushed to `origin/main`.
- AT-229 required context read in focused chunks: README_IMPL 7.32/current state, kernel-jobs queue policy and eligible-job selection notes, testing strategy kernel-jobs guidance, current `run_one_execution_turn(...)`, and current `JobSnapshotStore::list_resumable(...)` implementations.
- AT-229 README_IMPL 7.33 defines the next Rust slice: a one-shot queued selector filters `JobState::Queued`, orders candidates deterministically by `(updated_at, job_id)`, and delegates to `run_one_execution_turn(...)`.
- AT-229 scoped docs diff check passed with CRLF normalization warnings only.
- AT-229 final commit `d339db7` is present on `origin/main`.
- AT-230 required context read in focused chunks: README.md, docs/README.md, ModuleDocumentationStandard, README_IMPL 7.33, kernel-jobs queue policy notes, testing strategy kernel-jobs guidance, current runtime dispatch code, ID/time contracts, and memory/SQLite list-resumable behavior.
- Git ownership protection requires temporary `git -c safe.directory=D:/DEV/MyEpicLauncher ...`; do not write global git config.
- AT-230 RED command failed as expected with missing `run_next_execution_turn(...)`; GREEN focused command passed with 2 tests passed / 0 failed.
- AT-230 full validation passed: `launcher-kernel-jobs --lib` 9 passed / 0 failed, `launcher-composition-root` check passed, scoped rustfmt passed, and scoped diff-check passed with CRLF warnings only.
- AT-230 was committed with one-shot Git author config because this shell has no default author identity and global config must not be changed.
- AT-230 final commit `8db4900` was pushed to `origin/main`.
- AT-231 required context read in focused chunks: README_IMPL 7.33, kernel-jobs queue-policy design, downloads concurrency/budget notes, and current PWF state.
- AT-231 next boundary: count current `Running` snapshots from `list_resumable()`, defer when `running_count >= max_concurrent_jobs`, then use existing deterministic queued selection only when capacity remains.
- AT-231 README_IMPL 7.34 defines the one-shot policy slot gate boundary; scoped docs/PWF diff-check passed with CRLF normalization warnings only.
- AT-231 final commit `6f5bd32` was pushed to `origin/main`.
- AT-232 required context read in focused chunks: README_IMPL 7.34, kernel-jobs queue-policy notes, downloads concurrency/budget notes, current runtime selector code/tests, and current PWF state.
- AT-232 RED/GREEN validation passed for capacity-exhausted, zero-capacity, and remaining-capacity selector behavior.
- AT-232 full validation passed: `launcher-kernel-jobs --lib` 12 passed / 0 failed, `launcher-composition-root` check passed, scoped rustfmt passed, and scoped diff-check passed with CRLF warnings only.
- AT-232 final commit `d2fa1d9` was pushed to `origin/main`.
- AT-233 required context read in focused chunks: composition-root wiring docs, startup pipeline restore/warmup rules, README_IMPL 7.34, current composition runtime/registry wiring, and startup facade registry usage.
- AT-233 `docs/TauriCompositionRootWiringDesign.md` 9.4 defines `StartupPipelineFacade::run_one_runtime_execution_turn(...)` as the next one-shot helper boundary; scoped docs/PWF diff-check passed with CRLF normalization warnings only.
- AT-233 final commit `01c206d` was pushed to `origin/main`.
- AT-234 required context read in focused chunks: composition docs 9.4, startup pipeline docs, current startup/bootstrap code, and existing composition tests.
- AT-234 RED/GREEN validation passed for absent wiring, wired fake-driver execution, and build-time helper wiring.
- AT-234 full validation passed: composition-root lib 12 passed / 0 failed, composition-root check passed, scoped rustfmt passed, and scoped diff-check passed with CRLF warnings only.
- Scoped `git diff --check` passed with CRLF normalization warnings only.
- AT-224 found that downloads should not call `prepare_resume_execution_turn(...)` from `run(...)` unless an execution-port path is present, because that helper drains pending work after checkpoint reload.
- README_IMPL 7.31 defines the next Rust slice: add an optional downloads-owned segment execution port or equivalent explicit strategy, keep the default constructor deferred/non-draining, and test fake completed execution through `run(...)`.
- AT-224 scoped `git diff --check` passed with CRLF normalization warnings only.
- AT-225 required context read in focused chunks: README_IMPL 7.31, current downloads driver helpers/tests, kernel-jobs runtime driver/context sections, and download runtime ownership/checkpoint notes.
- AT-225 RED/GREEN validation passed: focused driver_run tests first failed on missing opt-in execution-port constructor, then passed after implementation.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 49 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\module-downloads\src\driver.rs` passed.
- Scoped `git diff --check` passed for AT-226 files with CRLF normalization warnings only.
- AT-226 final commit `d2d5405` was pushed to `origin/main`.
- AT-227 required context read in focused chunks: README/CONTRIBUTING routing, README_IMPL 7.29-7.31, kernel-jobs lifecycle/driver/context/snapshot sections, composition-root runtime/driver wiring, testing strategy backend test placement, current runtime dispatch code, current downloads driver run behavior, and composition-root driver registry wiring.
- AT-227 README_IMPL 7.32 defines the next Rust slice: `Accepted` dispatch projects queued snapshots to non-terminal `Running`; `Deferred` remains non-mutating; `Failed`, terminal state, leases, scheduler loops, downloads IO, host transport, frontend, and SQLite schema remain out of scope.
- AT-227 scoped docs diff check passed with CRLF normalization warnings only.
- AT-227 final commit `fc615db` was pushed to `origin/main`.
- AT-228 required context read in focused chunks: README_IMPL 7.32, kernel-jobs lifecycle/driver/context/snapshot rules, testing strategy kernel-jobs guidance, current runtime dispatch tests, and current `JobRunDisposition` / `JobSnapshotStore` contracts.
- AT-228 RED/GREEN validation passed: accepted dispatch initially left snapshots `Queued`, then `SharedJobRuntimeHost::run_one_execution_turn(...)` projected accepted dispatch to `Running` / UI `Running`.
- Missing-driver deferred dispatch keeps the queued snapshot unchanged.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml dispatch` passed with 3 tests passed / 0 failed.
- `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 7 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\kernel-jobs\src\runtime.rs` passed.
- Scoped `git diff --check` passed with CRLF normalization warnings only.
- AT-226 required context read in focused chunks: README_IMPL 7.31, current run implementation, and driver fake execution port tests.
- AT-226 RED/GREEN validation passed: missing checkpoint deferred and preserved pending work; no-pending-work and Accepted-only/no-mutation initially misclassified as `Accepted` until checkpoint helpers returned `None` for non-mutating turns.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml driver_run` passed with 5 tests passed / 0 failed.
- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed with 52 tests passed / 0 failed.
- `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
- `rustfmt --edition 2021 --check crates\module-downloads\src\driver.rs` passed.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not change Rust code, downloads code, composition-root wiring, frontend, host transport, scheduler loop, concrete IO, retry/backoff, durable lease persistence, terminal completion, SQLite schema, hooks, `.codex`, Cargo.lock, or unrelated dirty files in AT-229.
- Push is authorized by the user-provided GitHub remote after each completed task commit.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-218:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
