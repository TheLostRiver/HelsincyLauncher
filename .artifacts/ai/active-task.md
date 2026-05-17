# Active Atomic Task

## Identity

- task id: AT-2026-05-17-214
- title: Add kernel-jobs runtime policy control surface
- status: completed

## Goal

Add the first live runtime policy-control surface in `kernel-jobs` so cloned `SharedJobRuntimeHost` handles can update and read a shared `RuntimeQueuePolicy` snapshot, without wiring downloads facade, composition-root, host transport, frontend, scheduler execution, active jobs, leases, snapshots, or pending work.

## Scope

- in scope:
  - `crates/kernel-jobs/src/runtime.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - downloads facade policy-applier wiring
  - composition-root policy-applier wiring
  - global settings/config-system implementation
  - host transport or frontend changes
  - scheduler loop, per-module caps, per-host caps, writer backpressure, or fairness implementation
  - concrete IO, retry/backoff, or terminal runtime completion
  - active jobs, runtime leases, runtime snapshots, pending resume work, or segment scheduling behavior

## Allowed Files

1. crates/kernel-jobs/src/runtime.rs
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
6. README_IMPL 7.27 first Rust slice
7. docs/TauriKernelJobsRuntimeDesign.md queue-policy sections
8. current `RuntimeQueuePolicy`, `SharedJobRuntimeHost`, and kernel-jobs unit tests
9. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: `SharedJobRuntimeHost` can keep `policy()` as a by-value snapshot while storing policy behind shared synchronization so cloned host handles observe updates through an explicit update method.

## Cheap Check

1. Add focused kernel-jobs RED tests for policy update/readback through cloned hosts.
2. Implement the minimal shared policy storage and update method.
3. Run focused kernel-jobs tests, affected composition check, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. Focused kernel-jobs tests fail before implementation and pass after implementation.
2. `SharedJobRuntimeHost::policy()` returns the updated by-value policy snapshot after update.
3. A cloned `SharedJobRuntimeHost` observes the updated policy snapshot.
4. Downloads facade, composition-root, host transport, frontend, scheduler execution, active jobs, leases, snapshots, pending work, concrete IO, retry/backoff, and terminal completion remain unchanged.
5. Scoped `git diff --check` passes.
6. Commit only AT-214 files locally, then push `main` to `origin`.

## Validation Result

Passed before commit/push.

1. RED was observed before implementation: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml shared_job_runtime_host_updates_policy_for_cloned_handles` failed with no `update_policy` method on `SharedJobRuntimeHost`.
2. GREEN after implementation: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml shared_job_runtime_host_updates_policy_for_cloned_handles` passed, 1 passed / 0 failed.
3. Existing runtime snapshot behavior stayed green: `cargo test -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml shared_job_runtime_host_records_enqueued_snapshot` passed, 1 passed / 0 failed.
4. Affected composition check passed: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
5. Startup policy regression tests passed: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib build_job_runtime_`, 2 passed / 0 failed.
6. `rustfmt --check crates\kernel-jobs\src\runtime.rs` passed after formatting only the AT-214 Rust file.
7. Package-level `cargo fmt -p launcher-kernel-jobs --check` still reports pre-existing out-of-scope formatting diffs in `crates/kernel-jobs/src/lib.rs` and `crates/kernel-jobs/src/model.rs`; those files were not modified for AT-214.
8. Final scoped `git diff --check` and commit/push are pending.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 final commit is `38c32b2` and is already pushed to `origin/main`.
