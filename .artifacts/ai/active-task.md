# Active Atomic Task

## Identity

- task id: AT-2026-05-17-217
- title: Wire downloads runtime policy applier in composition-root
- status: completed

## Goal

Wire the concrete downloads runtime policy applier inside composition-root so `downloads.update_policy(...)` maps the normalized persisted `DownloadPolicyDto.concurrency_slots` to `RuntimeQueuePolicy` and updates the shared `SharedJobRuntimeHost`, while keeping host transport/frontend behavior, scheduler execution, active jobs, leases, snapshots, pending work, concrete IO, retry/backoff, and terminal completion out of scope.

## Scope

- in scope:
  - `crates/composition-root/src/bootstrap.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - global settings/config-system implementation
  - host transport or frontend changes
  - scheduler loop, per-module caps, per-host caps, writer backpressure, or fairness implementation
  - concrete IO, retry/backoff, or terminal runtime completion
  - active jobs, runtime leases, runtime snapshots, pending resume work, or segment scheduling behavior

## Allowed Files

1. crates/composition-root/src/bootstrap.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. README.md and docs/README.md backend/documentation routing
2. docs/modules/downloads/README_ARCH.md, README_API.md, README_FLOW.md, and README_IMPL.md policy sections
3. docs/TauriDownloadRuntimeDesign.md concurrency/policy sections
4. docs/TauriKernelJobsRuntimeDesign.md queue-policy sections
5. docs/TauriCompositionRootWiringDesign.md composition-root ownership sections
6. README_IMPL 7.28 completed state and composition-root later-slice note
7. current `DownloadRuntimePolicyApplier`, `DownloadsFacade::update_policy(...)`, `SharedJobRuntimeHost::update_policy(...)`, and composition-root downloads/runtime wiring surfaces
8. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: composition-root can provide a private `DownloadRuntimePolicyApplier` implementation backed by a cloned `SharedJobRuntimeHost` and wire it through `build_downloads_module(...)` without changing host transport, frontend, scheduler, or module-downloads code.

## Cheap Check

1. Add a focused composition-root RED test proving `downloads.update_policy(...)` updates a cloned `SharedJobRuntimeHost` policy through composition wiring.
2. Implement the minimal private composition-root applier and wire it through `build_downloads_module(...)`.
3. Run focused composition-root test, affected downloads module policy tests if needed, composition-root check, rustfmt check scoped to touched Rust files, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. Focused composition-root test fails before implementation and passes after implementation.
2. `downloads.update_policy(...)` updates the shared runtime policy snapshot through composition-root wiring.
3. `DownloadPolicyDto.concurrency_slots` maps to `RuntimeQueuePolicy::new(...)`.
4. Host transport/frontend behavior, scheduler execution, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion remain unchanged.
5. Scoped `git diff --check` passes.
6. Commit only AT-216 files locally, then push `main` to `origin`.

## Validation Result

Passed before commit/push.

1. RED was observed before implementation: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib downloads_policy_update_applies_runtime_policy_through_composition_wiring` failed with `left: 2` / `right: 17`.
2. GREEN after implementation: the same focused composition-root test passed, 1 passed / 0 failed.
3. Existing runtime startup policy tests passed: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib build_job_runtime_`, 2 passed / 0 failed.
4. Downloads module policy tests passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml update_policy`, 3 passed / 0 failed.
5. Affected composition check passed: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
6. `rustfmt --check crates\composition-root\src\bootstrap.rs` passed after formatting the touched Rust file.
7. Final scoped `git diff --check` and commit/push are pending.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 final commit is `38c32b2` and is already pushed to `origin/main`.
- AT-2026-05-17-214 final commit is `c92be25` and is already pushed to `origin/main`.
- AT-2026-05-17-215 final commit is `4ef3f10` and is already pushed to `origin/main`.
- AT-2026-05-17-216 final commit is `1094c10` and is already pushed to `origin/main`.
