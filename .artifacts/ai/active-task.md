# Active Atomic Task

## Identity

- task id: AT-2026-05-17-212
- title: Seed runtime policy from persisted downloads policy at startup
- status: completed

## Goal

Implement the documented startup-seeding slice so composition-root loads the persisted downloads policy before constructing the shared runtime and uses the loaded concurrency slots as the initial `RuntimeQueuePolicy` budget.

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
  - runtime queue-policy mutation
  - live runtime policy update API
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
6. README_IMPL 7.26 first Rust slice
7. current `RuntimeQueuePolicy`, `SharedJobRuntimeHost`, composition bootstrap, and `DownloadPolicyStore` surfaces
8. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: composition-root can construct/load one `SqliteDownloadPolicyStore` before runtime construction, use the loaded policy's concurrency slots for initial `RuntimeQueuePolicy`, and pass that same policy store into `DownloadFacade` without adding live runtime mutation.

## Cheap Check

1. Add focused composition-root RED tests for persisted-policy startup seeding and empty-store fallback.
2. Refactor composition-root startup order minimally to load policy before `build_job_runtime(...)`.
3. Run focused composition-root tests, affected policy tests, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. Focused composition-root tests fail before implementation and pass after implementation.
2. Persisted policy slots seed `SharedJobRuntimeHost::policy().max_concurrent_jobs`.
3. Empty policy store falls back to `DesktopBootstrapConfig.default_download_slots`.
4. `DownloadsFacade::update_policy(...)` remains persistence-only and does not add live runtime mutation.
5. Scoped `git diff --check` passes.
6. Commit only AT-212 files locally, then push `main` to `origin`.

## Validation Result

1. RED confirmed focused composition-root tests failed before implementation because `build_job_runtime(...)` did not accept a policy store.
2. `build_job_runtime(...)` now loads `DownloadPolicyStore` and seeds `RuntimeQueuePolicy::new(...)` from the loaded downloads policy.
3. `build_desktop_services(...)` now constructs one `SqliteDownloadPolicyStore` before runtime construction and passes it into the downloads facade.
4. Focused composition-root startup-seeding tests passed: 2 passed / 0 failed.
5. Existing shared downloads scheduler wiring test passed after moving its sqlite path under project-local `.artifacts/tmp`.
6. `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
7. `cargo test -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_policy_store` passed: 2 passed / 0 failed.
8. `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml policy` passed: 2 passed / 0 failed.
9. `cargo fmt -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 is ready to commit and push.
