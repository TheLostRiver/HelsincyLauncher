# Active Atomic Task

## Identity

- task id: AT-2026-05-17-208
- title: Implement downloads policy store facade semantics
- status: completed

## Goal

Implement the first backend-only downloads policy slice: `DownloadsFacade::get_policy(...)` reads the current downloads-owned policy snapshot, and `DownloadsFacade::update_policy(...)` stores a clamped user-facing policy snapshot for later reads.

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/composition-root/src/bootstrap.rs` only if facade dependencies change
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - runtime queue-policy mutation
  - shared runtime lease or active-job mutation
  - SQLite schema or SQLite adapter policy persistence
  - host transport or frontend changes
  - concrete HTTP/file/hash IO
  - retry/backoff
  - terminal runtime completion
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/composition-root/src/bootstrap.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing Rust implementation:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md section 7.24
8. docs/TauriDownloadRuntimeDesign.md concurrency/policy sections
9. docs/TauriKernelJobsRuntimeDesign.md queue policy sections
10. docs/TauriStorageAndDatabaseDesign.md downloads policy storage note
11. docs/TauriRepositoryPortsAndAdapterDesign.md port/adapter ownership rules
12. docs/TauriCompositionRootWiringDesign.md composition ownership rules
13. docs/TauriTestingStrategyAndQualityGateDesign.md module facade test rules
14. docs/TauriAIDevelopmentTransactionProtocolDesign.md atomic task rules
15. docs/TauriCodeCommentStandard.md comment language and declaration rules
16. current downloads policy DTO/request/query contracts
17. current downloads facade policy stubs and dependency construction

## Hypothesis

- falsifiable local hypothesis: a downloads-owned in-memory policy store/port can satisfy current `get_policy(...)` and `update_policy(...)` facade semantics, clamp `concurrency_slots` to `1..=128`, and compile through composition without mutating `RuntimeQueuePolicy` or adding SQLite/transport/frontend work.

## Cheap Check

1. Add focused RED tests for policy read and clamped update/readback.
2. Implement only the minimal policy store/port and facade calls.
3. Run focused policy tests, full downloads module tests, affected composition check if dependencies change, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. `get_policy(...)` returns the current downloads policy snapshot from the policy store.
2. `update_policy(...)` stores a clamped policy snapshot and a later `get_policy(...)` reads it back.
3. `concurrency_slots` clamps to `1..=128`.
4. `bandwidth_limit_bytes_per_sec` and `auto_resume` are stored and returned without driving runtime behavior.
5. Runtime queue-policy mutation, SQLite schema/adapter work, host transport, frontend, concrete IO, retry/backoff, and terminal completion remain unchanged.
6. Commit only AT-208 files, then push `main` to `origin`.

## Validation Result

1. RED: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml policy` failed because `InMemoryDownloadPolicyStore` and `policy_store` were missing.
2. GREEN: focused policy tests passed, 2 passed / 0 failed.
3. Full module: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed, 45 passed / 0 failed.
4. Composition: `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
5. Formatting: `cargo fmt -p launcher-module-downloads -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
6. Scoped diff check passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-207 final commit is `1d9a04c` and is already pushed to `origin/main`.
- AT-2026-05-17-208 implemented the module-owned policy store/get/update slice; local commit hash will be recorded after commit.
