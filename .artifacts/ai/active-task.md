# Active Atomic Task

## Identity

- task id: AT-2026-05-17-210
- title: Implement downloads SQLite policy store
- status: completed

## Goal

Add a SQLite-backed implementation of the existing downloads-owned `DownloadPolicyStore`, persist the current policy snapshot in a small `download_policy_snapshot` table, and wire composition-root to use that adapter without widening into runtime queue-policy application, global settings, host transport, frontend, or concrete download IO.

## Scope

- in scope:
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `crates/composition-root/src/bootstrap.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
  - `docs/modules/downloads/README_IMPL.md`
- out of scope:
  - runtime queue-policy mutation
  - global settings/config-system implementation
  - host transport or frontend changes
  - concrete IO, retry/backoff, or terminal runtime completion
  - active jobs, runtime leases, runtime snapshots, pending resume work, or segment scheduling behavior

## Allowed Files

1. crates/adapter-storage-sqlite/src/lib.rs
2. crates/composition-root/src/bootstrap.rs
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
2. docs/TauriStorageAndDatabaseDesign.md downloads policy and storage-placement sections
3. docs/TauriRepositoryPortsAndAdapterDesign.md port/adapter ownership sections
4. current `adapter-storage-sqlite` repository shapes
5. current `DownloadPolicyStore` / `InMemoryDownloadPolicyStore` surface
6. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: a narrow `SqliteDownloadPolicyStore` can round-trip normalized `DownloadPolicyDto` facts through `download_policy_snapshot` using project-local test database paths, while composition-root can replace the in-memory policy store without changing runtime queue policy or public transport/frontend behavior.

## Cheap Check

1. Add focused adapter RED tests for default load and save/load round-trip.
2. Implement only table creation, row mapping, adapter type, and composition-root wiring.
3. Run focused adapter policy tests, affected composition check, rustfmt check, scoped `git diff --check`, and path-limited status.

## Validation Gate

1. Focused policy store tests fail before production implementation and pass after implementation.
2. `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passes after wiring.
3. Runtime queue-policy mutation, global settings, transport, frontend, concrete IO, retry/backoff, pending work, and terminal completion remain out of scope.
4. Scoped `git diff --check` passes.
5. Commit only AT-210 files locally, then push `main` to `origin`.

## Validation Result

1. RED confirmed the focused adapter tests failed before production implementation because `SqliteDownloadPolicyStore` did not exist.
2. `SqliteDownloadPolicyStore` now creates `download_policy_snapshot`, loads the default normalized policy when the table is empty, and upserts normalized policy snapshots.
3. Composition-root now wires `SqliteDownloadPolicyStore` into the desktop downloads facade.
4. Focused adapter policy tests passed: 2 passed / 0 failed.
5. `cargo check -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed.
6. `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml policy` passed: 2 passed / 0 failed.
7. `cargo fmt -p launcher-adapter-storage-sqlite -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
8. Scoped `git diff --check` passed with CRLF normalization warnings only.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 is ready to commit and push.
