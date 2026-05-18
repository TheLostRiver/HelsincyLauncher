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
- status: completed; final commit `256f89b`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-235
- title: Define host runtime execution command boundary
- status: completed; final commit `18ea7d7`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-236
- title: Add host runtime execution command
- status: completed; final commit `f720d9c`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-237
- title: Cover host runtime command downloads deferred path
- status: completed; final commit `a8e3492`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-238
- title: Define downloads concrete segment execution boundary
- status: completed; final commit `d5af454`, pushed to `origin/main`

## Active Atomic Task

- task id: AT-2026-05-17-239
- title: Add downloads segment executor adapter shell
- status: completed; final commit `1375a06`, pushed to `origin/main`

## Current Slice

- `crates/module-downloads/src/driver.rs`
- `crates/module-downloads/src/lib.rs`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Next Resume Point

1. Phase 114 follow-up subtasks through AT-243 are complete and pushed.
2. The next backend slice should be opened as a new documented phase/task before coding.

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
- AT-235 required context read in focused chunks: README/docs routing, composition helper docs and Tauri integration rules, IPC command/query envelopes and implementation guidance, startup ownership rules, downloads runtime execution sections, current host command modules, bootstrap/state wrappers, and transport smoke patterns.
- AT-235 documented `jobs_run_next_execution_turn` as a command returning a stable `RuntimeExecutionTurnDto`; `Deferred` and `Failed` dispositions remain successful command envelopes unless the composition helper returns `AppError`.
- AT-235 scoped docs/PWF `git diff --check` passed with CRLF normalization warnings only.
- AT-235 final commit `18ea7d7` was pushed to `origin/main`.
- AT-236 required context read in focused chunks: IPC section 7.4, composition/startup docs, downloads runtime execution sections, current host command registry/jobs handler, desktop bootstrap/state wrappers, and transport smoke patterns.
- AT-236 RED/GREEN validation passed for the documented host command. The final implementation adds `RuntimeExecutionTurnDto`, `RuntimeExecutionTurnDispositionDto`, `map_runtime_execution_turn_result(...)`, the `jobs_run_next_execution_turn` registry entry, and a thin jobs command handler.
- AT-236 validation passed: focused transport smoke, focused DTO mapper test, full desktop package tests, desktop compile gate, scoped rustfmt with `skip_children=true`, and scoped diff-check all passed.
- AT-236 final commit `f720d9c` was pushed to `origin/main`.
- AT-237 required context read in focused chunks: IPC command section, downloads deferred run docs, current downloads driver run reason, shared runtime deferred non-mutation, and current transport smoke helper.
- AT-237 validation passed: host transport smoke now proves a queued production downloads job returns `Deferred` through `jobs_run_next_execution_turn`, reason contains `execution port not wired`, and the snapshot remains `Queued` / `Queued`.
- Full desktop package tests, desktop compile gate, scoped rustfmt, and scoped diff-check passed.
- AT-237 final commit `a8e3492` was pushed to `origin/main`.
- AT-238 required context read in focused chunks: downloads module docs, README_IMPL runtime sections, Tauri download runtime fetch/write/verify/staging references, kernel-jobs runtime context references, and current segment execution request/result/port shape.
- AT-238 README_IMPL 7.35 defines the next safe Rust slice: a module-owned executor adapter shell behind the existing `DownloadSegmentExecutionPort`, starting with fake/in-memory sub-ports or adapter-shell tests and no real IO.
- AT-238 scoped docs/PWF diff-check passed with CRLF normalization warnings only.
- AT-238 final commit `d5af454` was pushed to `origin/main`.
- AT-239 required context read in focused chunks: downloads module docs, README_IMPL 7.35, Tauri download runtime fetch/write/verify/staging references, kernel-jobs driver/runtime references, and current driver request/result/port tests.
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

## Latest Handoff - AT-2026-05-17-239

- Status: completed; final commit `1375a06`, pushed to `origin/main`.
- Scope completed:
  - Added `DownloadSegmentExecutor` behind the existing `DownloadSegmentExecutionPort`.
  - Added module-owned fetch/write/verify sub-port traits plus fake/in-memory result DTOs.
  - Re-exported the new adapter shell and contracts from `launcher-module-downloads`.
  - Added focused TDD coverage proving request facts pass through fake sub-ports and successful output becomes the existing `Completed` result shape.
- Preserved boundaries:
  - No production composition-root wiring.
  - No real HTTP range requests, provider object fetches, disk/staging writes, artifact moves, or hash verification.
  - No retry/backoff, terminal runtime projection, leases, host transport, frontend, or SQLite schema work.
- Validation passed:
  - focused adapter test
  - `driver_run` regression tests
  - full `launcher-module-downloads --lib`
  - `launcher-composition-root` check
  - scoped rustfmt check

## Latest Handoff - AT-2026-05-17-240

- Status: completed; final commit `62c6bb8`, pushed to `origin/main`.
- Scope completed:
  - Added README_IMPL 7.36 `Segment Executor Failure Mapping Boundary`.
  - Defined handled fetch/write/verify segment failures as in-band `DownloadSegmentExecutionResult::Failed`.
  - Kept infrastructure/configuration errors that prevent a segment decision on the propagated `AppError` path.
- Next likely code task:
  - fake write or verify failure test against `DownloadSegmentExecutor`;
  - adapter maps handled failure to `Failed`;
  - separate test preserves true `AppError` propagation;
  - rerun focused adapter tests and existing failed-result checkpoint tests.

## Latest Handoff - AT-2026-05-17-241

- Status: completed; final commit `cab15c2`, pushed to `origin/main`.
- Scope completed:
  - Added module-local handled-failure/outcome types for fetch/write/verify sub-ports.
  - Updated `DownloadSegmentExecutor` so handled sub-port failures become existing `DownloadSegmentExecutionResult::Failed`.
  - Preserved `AppError` propagation for infrastructure/configuration failures.
  - Re-exported the new outcome/failure types from `launcher-module-downloads`.
- Validation passed:
  - focused adapter tests
  - failed-result checkpoint mutation test
  - full `launcher-module-downloads --lib`
  - `launcher-composition-root` check
  - scoped rustfmt check

## Latest Handoff - AT-2026-05-17-242

- Status: completed; final commit `7527476`, pushed to `origin/main`.
- Scope completed:
  - Added README_IMPL 7.37 `Segment Staging Target Guard Boundary`.
  - Defined safe staging-relative `write_target` acceptance and unsafe target rejection cases.
  - Defined unsafe target rejection as a local handled segment failure, not public `DL_WRITE_FAILED` projection yet.
- Next likely code task:
  - add focused tests for safe/unsafe staging target guard behavior;
  - implement pure module-owned target guard with no file system side effects;
  - optionally prove guard rejection can feed `DownloadSegmentHandledFailure` for future write sub-ports.

## Latest Handoff - AT-2026-05-17-243

- Status: completed; final commit `69ea5e7`, pushed to `origin/main`.
- Scope completed:
  - Added `DownloadSegmentStagingTarget::parse(...)` as a pure staging-relative target guard.
  - Rejected unsafe targets as `DownloadSegmentHandledFailure` with zero bytes and `retryable = false`.
  - Re-exported `DownloadSegmentStagingTarget` for future writer sub-ports.
- Validation passed:
  - focused staging target guard tests
  - focused adapter tests
  - full `launcher-module-downloads --lib`
  - `launcher-composition-root` check
  - scoped rustfmt check

## Latest Handoff - AT-2026-05-18-244

- Status: completed; final commit `e447444`, pushed to `origin/main`.
- Scope:
  - Define README_IMPL 7.38 for a guarded downloads writer sub-port boundary.
  - Keep this slice documentation/PWF-only.
- Validation passed:
  - scoped docs/PWF `git diff --check` returned no whitespace errors;
  - only expected CRLF normalization warnings were emitted.
- Next likely code task after this docs commit:
  - add RED tests for a guarded `DownloadSegmentWritePort` wrapper;
  - prove unsafe `write_target` skips the wrapped writer and returns `DownloadSegmentWriteOutcome::Failed`;
  - prove safe targets delegate exactly once and preserve wrapped writer results.
- Preserved boundaries:
  - No Rust code in AT-244.
  - No real staging writes, directory creation, temp naming, artifact moves, hash checks, production wiring, retry/backoff, public `DL_*` projection, host transport, frontend, or schema work.

## Latest Handoff - AT-2026-05-18-245

- Status: completed; final commit `97c4870`, pushed to `origin/main`.
- Scope:
  - Added focused TDD for a guarded downloads writer wrapper.
  - Implemented only the module-owned wrapper behind `DownloadSegmentWritePort`.
- Validation passed:
  - RED failed on missing `DownloadSegmentGuardedWritePort`;
  - focused guarded writer tests passed;
  - full `launcher-module-downloads --lib` passed;
  - `launcher-composition-root` check passed;
  - scoped rustfmt and diff-check passed.
- Preserved boundaries:
  - No real disk IO, no composition-root wiring, no retry/backoff, no public `DL_*` projection, no host transport/frontend/schema work.

## Latest Handoff - AT-2026-05-18-246

- Status: completed; final commit `e6ce619`, pushed to `origin/main`.
- Scope:
  - Added focused executor coverage for guarded writer unsafe-target failures.
  - Kept the slice in `crates/module-downloads/src/driver.rs` plus PWF records.
- Validation passed:
  - mutation RED failed when the guard was temporarily bypassed;
  - focused executor adapter tests passed;
  - full downloads module tests passed;
  - composition-root check passed;
  - scoped rustfmt and diff-check passed.
- Preserved boundaries:
  - No new docs boundary, no real disk IO, no composition-root wiring, no retry/backoff, no public `DL_*` projection, no host transport/frontend/schema work.

## Latest Handoff - AT-2026-05-19-247

- Status: completed; final commit `99e8160`, pushed to `origin/main`.
- Scope:
  - Updated README_IMPL current port status and remaining downloads completion roadmap.
  - Kept the slice documentation/PWF-only.
- Validation passed:
  - README_IMPL roadmap re-read;
  - scoped docs/PWF diff-check passed with CRLF warnings only.
- Next likely coding task:
  - define and implement the first concrete filesystem staging writer boundary behind `DownloadSegmentWritePort`.
- Preserved boundaries:
  - No Rust code in AT-247.
  - No production wiring, retry/backoff, public execution errors, transport, frontend, or schema work.

## Latest Handoff - AT-2026-05-19-248

- Status: completed; final commit `cd6fa4b`, pushed to `origin/main`.
- Scope:
  - Added README_IMPL 7.39 for the filesystem staging writer boundary.
  - Kept the slice documentation/PWF-only.
- Validation passed:
  - README_IMPL 7.39 re-read;
  - scoped docs/PWF diff-check passed with CRLF warnings only.
- Next likely coding task:
  - implement `DownloadSegmentFilesystemWritePort` with TDD for job-scoped writes and partial offset writes.
- Preserved boundaries:
  - No Rust code in AT-248.
  - No provider fetch, verifier, production wiring, retry/backoff, public errors, transport, frontend, or schema work.

## Latest Handoff - AT-2026-05-19-249

- Status: completed; final commit `e267db9`, pushed to `origin/main`.
- Scope:
  - Add `DownloadSegmentFilesystemWritePort` and focused TDD in `crates/module-downloads/src/driver.rs`.
  - Re-export the writer from `crates/module-downloads/src/lib.rs`.
  - Update `docs/modules/downloads/README_IMPL.md` implementation status and next-slice note.
- Validation passed:
  - RED failed on missing `DownloadSegmentFilesystemWritePort`.
  - Focused filesystem writer tests passed.
  - Focused executor adapter tests passed.
  - Full `launcher-module-downloads --lib` passed with 63/63 tests.
  - `launcher-composition-root` check passed.
  - Scoped rustfmt check passed after formatting.
- Preserved boundaries:
  - No provider fetch, verifier, final artifact moves, production wiring, retry/backoff, public errors, transport, frontend, or schema work.
- Next likely task:
  - define the verifier shell boundary behind `DownloadSegmentVerifyPort`, starting with byte-length checks before hash algorithms.

## Latest Handoff - AT-2026-05-19-250

- Status: completed; final commit `10d8c58`, pushed to `origin/main`.
- Scope:
  - Update `docs/modules/downloads/README_IMPL.md` port status and add the concrete segment length verifier boundary.
  - Update PWF records for Phase 121.
- Validation passed:
  - README_IMPL port status and 7.40 were re-read.
  - Scoped docs/PWF diff-check passed with CRLF warnings only.
- Preserved boundaries:
  - No Rust code, no hash algorithms, no file/job-level integrity sealing, no retry/backoff, no public `DL_VERIFY_FAILED` projection, no production wiring, no host transport/frontend/schema work.
- Next likely code task after this commit:
  - add focused RED tests for `DownloadSegmentLengthVerifyPort` success and handled mismatch failure, then implement the smallest verifier behind `DownloadSegmentVerifyPort`.

## Latest Handoff - AT-2026-05-19-251

- Status: completed; final commit `c8d1e5c`, pushed to `origin/main`.
- Scope:
  - Add `DownloadSegmentLengthVerifyPort` with focused TDD in `crates/module-downloads/src/driver.rs`.
  - Re-export the verifier from `crates/module-downloads/src/lib.rs`.
  - Update PWF records for Phase 122.
- Validation passed:
  - RED failed on missing `DownloadSegmentLengthVerifyPort`.
  - Focused verifier tests passed.
  - Focused executor adapter tests passed.
  - Full `launcher-module-downloads --lib` passed with 65/65 tests.
  - `launcher-composition-root` check passed.
  - Scoped rustfmt check passed.
- Preserved boundaries:
  - No hash algorithms, no disk readback, no file/job-level integrity sealing, no retry/backoff, no public `DL_VERIFY_FAILED` projection, no production wiring, no host transport/frontend/schema work.

## Latest Handoff - AT-2026-05-19-252

- Status: completed; final commit `6684205`, pushed to `origin/main`.
- Scope:
  - Correct `DownloadSegmentLengthVerifyPort` partial resume completion semantics.
  - Update README_IMPL 7.40 to document from-start versus partial length checks.
  - Update PWF records for Phase 123.
- Validation passed:
  - RED failed for partial completion before the fix.
  - Focused verifier tests passed with 3/3 tests.
  - Full `launcher-module-downloads --lib` passed with 66/66 tests.
  - `launcher-composition-root` check passed.
  - Scoped rustfmt check passed.
- Preserved boundaries:
  - No fetcher boundary or HTTP range implementation, no hash algorithms, no file/job-level integrity sealing, no retry/backoff, no public `DL_VERIFY_FAILED` projection, no production wiring, no host transport/frontend/schema work.

## Latest Handoff - AT-2026-05-19-253

- Status: completed; final commit `dcf62cf`, pushed to `origin/main`.
- Scope:
  - Update README_IMPL port status, roadmap, verifier implementation status, and add static fetcher boundary 7.41.
  - Update PWF records for Phase 124.
- Validation passed:
  - README_IMPL port status/roadmap and 7.41 were re-read.
  - Scoped docs/PWF diff-check passed with CRLF warnings only.
- Preserved boundaries:
  - No Rust code, no real HTTP range requests, no provider auth/CDN/retry behavior, no public network/provider error projection, no production wiring, no host transport/frontend/schema work.
- Next likely code task:
  - implement `DownloadSegmentStaticFetchPort` with focused TDD for from-start bytes+etag, partial remaining bytes, and handled missing/invalid source failures.

## Current Handoff - AT-2026-05-19-254

- Status: completed; final commit `43a44e1`, pushed to `origin/main`.
- Scope:
  - Add `DownloadSegmentStaticFetchPort` and static source record with focused TDD.
  - Re-export the fetcher/source from `crates/module-downloads/src/lib.rs`.
  - Update README_IMPL implementation status and PWF records.
- Validation passed:
  - RED failed on missing static fetcher types.
  - Focused static fetcher tests passed with 5/5 tests.
  - Focused executor adapter tests passed.
  - Full `launcher-module-downloads --lib` passed with 71/71 tests.
  - `launcher-composition-root` check passed.
  - Scoped rustfmt check passed after formatting.
- Preserved boundaries:
  - No real HTTP range requests, provider auth/CDN/retry behavior, public network/provider error projection, streaming worker pools, production wiring, host transport/frontend/schema work.

## Current Handoff - AT-2026-05-19-255

- Status: completed; final commit `1baf9a7`, pushed to `origin/main`.
- Scope:
  - Update `docs/modules/downloads/README_IMPL.md` with 7.42 composition-root segment executor wiring boundary.
  - Update root `README.md` so the first entry point reflects downloads concrete segment execution progress and the module implementation doc.
  - Update PWF records and correct AT-254 pushed status.
- Boundary selected:
  - composition-root may prove executor wiring through an explicit private static-source helper/test path.
  - default desktop production remains no-execution-port/deferred until a real provider fetcher or explicit non-empty source config is designed.
  - staging root should derive from `DesktopBootstrapConfig.app_data_dir/.downloads/staging`.
- Preserved boundaries:
  - No Rust code in AT-255.
  - No real HTTP/provider behavior, retry/backoff, public `DL_*` execution projection, host transport, frontend, or SQLite schema changes.
- Validation passed:
  - Scoped `git diff --check` passed for README, README_IMPL, and PWF task files with CRLF warnings only.
- Next likely code task:
  - implement a focused composition-root private static executor wiring proof with TDD, explicit static source input, app-data staging root, and default desktop deferred behavior preserved.

## Current Handoff - AT-2026-05-19-256

- Status: completed; final commit `38940ff`, pushed to `origin/main`.
- Scope:
  - `crates/composition-root/src/bootstrap.rs`
  - PWF records under `.artifacts/ai/`
- Planned TDD path:
  - Add a focused composition-root test that calls a missing private static executor wiring helper.
  - Watch it fail for the missing helper.
  - Implement the smallest helper using explicit static sources, app-data staging, filesystem writer, length verifier, and `DownloadJobDriver::with_pending_resume_work_source_and_execution_port(...)`.
  - Verify focused test, default deferred behavior, composition-root check, rustfmt, and diff-check.
- Preserved boundaries:
  - No default desktop production execution-port wiring, no real HTTP/provider behavior, no retry/backoff/public execution error projection, no host transport/frontend/schema changes.
- Validation passed:
  - RED failed on missing private static executor wiring helper.
  - Focused static wiring test passed after implementation.
  - Focused composition-root download tests passed.
  - Full composition-root tests passed.
  - `cargo check -p launcher-composition-root` passed.
  - Full downloads module lib tests passed with 71/71.
  - Scoped rustfmt check passed after formatting.
- Next likely task:
  - reassess README_IMPL after the deterministic wiring proof; likely define the runtime terminal completion/failure projection boundary before changing shared runtime state.

## Current Handoff - AT-2026-05-19-257

- Status: completed; final commit `95c3e76`, pushed to `origin/main`.
- Scope:
  - Update `README.md` current status/roadmap.
  - Update `docs/modules/downloads/README_IMPL.md` port/roadmap status, 7.42 implementation status, and new 7.43 runtime terminal projection boundary.
  - Update PWF records under `.artifacts/ai`.
- Boundary selected:
  - `JobRunDisposition::Accepted` remains non-terminal and projects only Running.
  - Terminal Completed/Failed snapshot projection must be explicit, owned by `kernel-jobs`, and proven first with fake runtime drivers.
  - Downloads driver terminal decisions, retry/backoff, public `DL_*` execution errors, provider HTTP, host transport, frontend, and schema work stay later.
- Validation passed:
  - Scoped `git diff --check` for README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- Next likely code task after this docs commit:
  - add focused RED `launcher-kernel-jobs` tests for explicit terminal run dispositions projecting stored snapshots to Completed/Failed.

## Current Handoff - AT-2026-05-19-258

- Status: completed; final commit `ac12ec0`, pushed to `origin/main`.
- Scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - root README and downloads README_IMPL implementation status after green
  - PWF records under `.artifacts/ai`
- Completed TDD path:
  - RED failed on missing terminal disposition variants.
  - Added minimal `JobRunDisposition::Completed` and `JobRunDisposition::TerminalFailed { reason }` variants.
  - Added projection branches in `SharedJobRuntimeHost::run_one_execution_turn(...)`.
  - Verified focused tests, full `launcher-kernel-jobs --lib`, `cargo check -p launcher-composition-root`, scoped rustfmt, and diff-check.
- Preserved boundaries:
  - No downloads driver terminal decision logic, no retry/backoff/public `DL_*` errors, no snapshot error payload fields, no host transport/frontend/schema/provider/scheduler/lease changes.
- Next likely task:
  - define the downloads driver terminal decision boundary before allowing `DownloadJobDriver::run(...)` to return `Completed` or `TerminalFailed`.

## Current Handoff - AT-2026-05-19-259

- Status: completed; final commit `bf6af24`, pushed to `origin/main`.
- Scope:
  - root README
  - `docs/modules/downloads/README_IMPL.md`
  - PWF records under `.artifacts/ai`
- Boundary selected:
  - first downloads terminal decision should be completion-first only;
  - `DownloadJobDriver::run(...)` may return `Completed` after a saved non-empty checkpoint has all known segment records `Completed`;
  - `TerminalFailed` remains reserved because current failed checkpoint records do not persist retryable/reason facts.
- Validation passed:
  - scoped `git diff --check` for README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- Next likely code task:
  - add focused downloads driver RED tests for all-completed checkpoint returning `JobRunDisposition::Completed` and failed checkpoint mutation remaining non-terminal.

## Current Handoff - AT-2026-05-19-260

- Status: completed; final commit `55ab2da`, pushed to `origin/main`.
- Scope:
  - `crates/module-downloads/src/driver.rs`
  - root README and downloads README_IMPL implementation status after green
  - PWF records under `.artifacts/ai`
- Completed TDD path:
  - RED failed on all-completed checkpoint returning `Accepted`.
  - Added the smallest downloads-owned checkpoint terminal-completion decision after local execution.
  - Verified all-completed returns `Completed` and failed checkpoint mutation remains non-terminal.
  - Verified focused driver tests, full downloads module lib tests, composition-root check, scoped rustfmt, and diff-check.
- Preserved boundaries:
  - No `TerminalFailed` return from downloads, no retry/backoff/public `DL_*`, no transport/frontend/schema/provider/scheduler/lease changes.
- Next likely task:
  - define retry/backoff and terminal-failed classification before any downloads driver path returns `TerminalFailed`.

## Current Handoff - AT-2026-05-19-261

- Status: completed; final commit `124dbb3`, pushed to `origin/main`.
- Scope:
  - root README
  - `docs/modules/downloads/README_IMPL.md`
  - PWF records under `.artifacts/ai`
- Boundary selected:
  - failed segment metadata must be durable before retry/backoff or terminal failed decisions;
  - preserve local diagnostic reason and retryable hint in checkpoint facts first;
  - do not expose public `DL_*` execution errors or return `TerminalFailed` yet.
- Validation passed:
  - scoped `git diff --check` for README, README_IMPL, and PWF task files passed with CRLF normalization warnings only.
- Next likely code task:
  - add failed metadata fields to `DownloadSegmentCheckpointRecord`, record them from failed execution results, and update SQLite checkpoint round-trip tests.

## Current Handoff - AT-2026-05-19-262

- Status: completed; final commit `74cdf2c`, pushed to `origin/main`.
- Scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/facade/mod.rs` test fixture updates for the widened checkpoint record
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - root README and `docs/modules/downloads/README_IMPL.md` after green
  - PWF records under `.artifacts/ai`
- TDD target:
  - RED driver test for failed checkpoint mutation preserving local reason and retryable hint failed on missing fields, then passed after implementation.
  - RED SQLite checkpoint round-trip assertion for the same metadata failed on missing fields, then passed after implementation.
- Completed implementation:
  - `DownloadSegmentCheckpointRecord` now carries optional `failure_reason` and `failure_retryable`.
  - failed checkpoint mutation persists failed execution reason/retryable into checkpoint facts.
  - SQLite checkpoint table creation/backfill and save/load mapping preserve failed metadata.
  - README and downloads implementation doc now mark failed metadata persistence complete.
- Validation passed:
  - `cargo test -p launcher-module-downloads --lib` -> 72 passed.
  - `cargo test -p launcher-adapter-storage-sqlite --lib` -> 3 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `cargo fmt -p launcher-module-downloads -p launcher-adapter-storage-sqlite -- --check` -> passed.
  - scoped `git diff --check` -> CRLF warnings only.
- Boundaries:
  - keep failed mutation non-terminal;
  - no retry/backoff scheduling, public `DL_*`, host transport, frontend, provider HTTP, production wiring, leases, scheduler loop, or snapshot error payload changes.
- Next likely task:
  - define retry count, backoff scheduling facts, and module-owned failure class before enabling `TerminalFailed` or public `DL_*` projection.

## Current Handoff - AT-2026-05-19-263

- Status: completed; final commit `4642653`, pushed to `origin/main`.
- Scope:
  - root README
  - `docs/modules/downloads/README_IMPL.md`
  - PWF records under `.artifacts/ai`
- Boundary to define:
  - durable retry attempt count;
  - next retry eligibility/backoff fact;
  - module-owned internal failure class;
  - explicit non-goals for retry execution, public `DL_*`, and `TerminalFailed`.
- Completed:
  - README points the next implementation at retry count, next retry eligibility, and internal failure class persistence.
  - README_IMPL 7.46 defines failure-class candidates, retry field semantics, first Rust slice, and non-goals.
  - scoped doc diff-check passed with CRLF warnings only.
- Next likely code task:
  - implement `DownloadSegmentFailureClass`, `retry_attempt_count`, and `next_retry_after` persistence with TDD while keeping failed mutation non-terminal.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-249:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
## Current Handoff - AT-2026-05-19-264

- Status: completed; local commit `ad6cef2`, push blocked by safety review.
- Scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/facade/mod.rs` fixture updates only if required by widened checkpoint records
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `crates/adapter-storage-sqlite/Cargo.toml` only if timestamp parsing requires it
  - root README and `docs/modules/downloads/README_IMPL.md` after green
  - PWF records under `.artifacts/ai`
- TDD target:
  - RED driver coverage for failed checkpoint mutation preserving an internal failure class, starting/incrementing `retry_attempt_count`, leaving `next_retry_after` unset, and staying non-terminal failed before implementation.
  - RED SQLite checkpoint round-trip coverage for `failure_class`, `retry_attempt_count`, and `next_retry_after` failed before implementation.
- Completed:
  - `DownloadSegmentFailureClass` is carried through handled failure and failed execution result paths.
  - `DownloadSegmentCheckpointRecord` persists optional `failure_class`, `retry_attempt_count`, and `next_retry_after`.
  - SQLite checkpoint schema/backfill/save/load mapping preserves the new fields.
  - README and downloads README_IMPL now route the next boundary to backoff policy, retry exhaustion, and terminal failure eligibility.
- Validation:
  - `cargo test -p launcher-module-downloads --lib` -> 73 passed.
  - `cargo test -p launcher-adapter-storage-sqlite --lib` -> 3 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `cargo fmt -p launcher-module-downloads -p launcher-adapter-storage-sqlite -- --check` -> passed.
  - scoped `git diff --check` -> passed with CRLF normalization warnings only.
- Publish:
  - Local commit `ad6cef2 feat: persist download retry failure facts` exists.
  - Push to `origin/main` was attempted and blocked by the safety reviewer because direct shared-branch mutation/export needs explicit approval.
- Boundaries:
  - no retry scheduler loop, background worker, automatic retry dispatch, public `DL_*`, `TerminalFailed`, host/frontend/provider HTTP, production wiring, leases, or snapshot error payload change.
## Current Handoff - AT-2026-05-19-265

- Status: completed; local commit `d25ef93`, push not reattempted after safety block.
- Scope:
  - root README
  - `docs/modules/downloads/README_IMPL.md`
  - PWF records under `.artifacts/ai`
- Boundary to define:
  - pure retry/backoff policy calculation;
  - retry exhaustion and user-attention rules;
  - terminal failure eligibility constraints before any `TerminalFailed` code.
- Boundaries:
  - no Rust code, SQLite schema, retry scheduler loop, automatic retry dispatch, public `DL_*`, host/frontend/provider HTTP, production wiring, leases, or snapshot error payload change.
- Completed:
  - README now points the next Rust slice at pure backoff policy calculation.
  - README_IMPL 7.47 defines automatic retry classes, non-automatic classes, three-attempt budget, 30s/120s delay schedule, user-attention rules, terminal-candidate constraints, first Rust slice, and non-goals.
- Validation:
  - scoped `git diff --check` over README, README_IMPL, and PWF files passed with CRLF normalization warnings only.
- Publish:
  - Local commit `d25ef93 docs: define download backoff policy boundary` exists.
  - Push was not reattempted because the previous direct `origin/main` push was blocked by the safety reviewer and explicit approval is required before retrying.
## Current Handoff - AT-2026-05-19-266

- Status: completed; local commit pending.
- Scope:
  - `crates/module-downloads/src/driver.rs`
  - `crates/module-downloads/src/lib.rs`
  - `crates/kernel-foundation/src/time.rs` for the shared `IsoDateTime` offset helper required by policy time arithmetic
  - root README and downloads README_IMPL after green
  - PWF records under `.artifacts/ai`
- TDD target:
  - pure retry/backoff policy tests for attempt 1 -> 30s, attempt 2 -> 120s, attempt 3+ exhausted, user-attention classes, and no-automatic-retry classes.
- Completed:
  - `DownloadSegmentRetryPolicy`
  - `DownloadSegmentRetryDecision`
  - `IsoDateTime::add_seconds(...)`
  - README/README_IMPL status and next-boundary updates.
- Validation:
  - `cargo test -p launcher-module-downloads download_segment_retry_policy --lib` -> 5 passed after RED failed on missing types.
  - `cargo test -p launcher-kernel-foundation --lib` -> 0 tests, exit 0.
  - `cargo test -p launcher-module-downloads --lib` -> 78 passed.
  - `cargo check -p launcher-composition-root` -> passed.
  - `rustfmt --check crates/kernel-foundation/src/time.rs crates/module-downloads/src/driver.rs crates/module-downloads/src/lib.rs` -> passed.
  - scoped `git diff --check` -> passed with CRLF normalization warnings only.
- Note:
  - `Cargo.lock` has a pre-existing unrelated `launcher-module-engines` hunk and must not be committed for this AT.
  - Unintended package-wide rustfmt noise in unrelated foundation files was removed; only `time.rs` is part of this AT.
- Boundaries:
  - no scheduler loop, automatic retry dispatch, SQLite schema, `TerminalFailed`, public `DL_*`, host/frontend/provider HTTP, production wiring, leases, or snapshot error payload change.
