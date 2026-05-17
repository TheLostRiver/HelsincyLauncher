# Active Atomic Task

## Identity

- task id: AT-2026-05-17-216
- title: Add downloads runtime policy applier port
- status: completed

## Goal

Add the downloads-owned runtime policy applier port in `module-downloads` so `DownloadsFacade::update_policy(...)` can pass the normalized persisted `DownloadPolicyDto` to a dedicated applier, while keeping composition-root concrete wiring, direct `SharedJobRuntimeHost` calls from downloads code, transport/frontend behavior, scheduler execution, active jobs, leases, snapshots, pending work, concrete IO, retry/backoff, and terminal completion out of scope.

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - composition-root policy-applier wiring
  - direct `SharedJobRuntimeHost` calls from downloads code
  - global settings/config-system implementation
  - host transport or frontend changes
  - scheduler loop, per-module caps, per-host caps, writer backpressure, or fairness implementation
  - concrete IO, retry/backoff, or terminal runtime completion
  - active jobs, runtime leases, runtime snapshots, pending resume work, or segment scheduling behavior

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. README.md and docs/README.md backend/documentation routing
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md policy sections
3. docs/TauriDownloadRuntimeDesign.md concurrency/policy sections
4. docs/TauriKernelJobsRuntimeDesign.md queue-policy sections
5. docs/TauriCompositionRootWiringDesign.md composition-root ownership sections
6. README_IMPL 7.28 first Rust slice
7. current `DownloadsFacade::update_policy(...)`, `DownloadPolicyStore`, module facade tests, `SharedJobRuntimeHost::update_policy(...)`, and composition-root wiring surfaces
8. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: `DownloadsFacade` can add a default no-op runtime policy applier and an opt-in constructor for tests/later composition wiring without changing existing `DownloadModuleDeps` call sites or applying concrete runtime policy inside downloads code.

## Cheap Check

1. Add focused RED tests proving update_policy passes the normalized policy to a dedicated runtime policy applier.
2. Implement the minimal applier port, no-op default, facade storage, and public re-export.
3. Run focused downloads module policy tests, affected composition check if public construction changes, rustfmt check scoped to touched Rust files, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. Focused downloads module applier tests fail before implementation and pass after implementation.
2. `DownloadsFacade::update_policy(...)` stores the normalized policy and passes the same normalized `DownloadPolicyDto` to the applier.
3. The default/no-op applier path keeps existing policy-store behavior intact.
4. Composition-root concrete wiring, direct `SharedJobRuntimeHost` calls from downloads code, host transport/frontend behavior, scheduler execution, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion remain unchanged.
5. Scoped `git diff --check` passes.
6. Commit only AT-216 files locally, then push `main` to `origin`.

## Validation Result

Passed before commit/push.

1. RED was observed before implementation: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy_applies_normalized_snapshot_to_runtime_applier` failed because `DownloadRuntimePolicyApplier` did not exist.
2. GREEN after implementation: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy_applies_normalized_snapshot_to_runtime_applier` passed, 1 passed / 0 failed.
3. Default no-op coverage passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy_default_applier_keeps_policy_store_behavior`, 1 passed / 0 failed.
4. Existing policy-store behavior passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy_clamps_and_stores_snapshot_for_later_reads`, 1 passed / 0 failed.
5. Full downloads module tests passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`, 47 passed / 0 failed.
6. Affected composition check passed: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
7. `rustfmt --check crates\module-downloads\src\facade\mod.rs crates\module-downloads\src\lib.rs` passed.
8. Final scoped `git diff --check` and commit/push are pending.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 final commit is `38c32b2` and is already pushed to `origin/main`.
- AT-2026-05-17-214 final commit is `c92be25` and is already pushed to `origin/main`.
- AT-2026-05-17-215 final commit is `4ef3f10` and is already pushed to `origin/main`.
