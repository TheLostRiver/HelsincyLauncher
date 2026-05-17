# Handoff

## Latest Published Atomic Task

- task id: AT-2026-05-17-192
- title: Add downloads segment execution request handoff shell
- status: completed and committed locally; verify final amended hash with `git log --oneline -1`

## Current In-progress Atomic Task

- none

## Current Slice

- `crates/module-downloads/src/driver.rs`
- `crates/module-downloads/src/lib.rs`
- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

## Context Read

- README, CONTRIBUTING, docs map, downloads module docs, README_IMPL 7.13 and 7.14, kernel-jobs/download runtime design snippets, testing strategy, code comment standard.
- Current `crates/module-downloads/src/driver.rs` and `crates/module-downloads/src/lib.rs`.

## Next Resume Point

1. Reassess README_IMPL 7.14 for the next safe slice. Do not jump directly to concrete IO.
2. Keep concrete HTTP fetch, staging writes, hash verification, checkpoint mutation, runtime completion, transport, frontend, SQLite schema, composition-root changes, and `kernel-jobs` changes out of scope unless the next task explicitly documents them.

## Validation

- RED focused `segment_execution_request` filter failed on missing request/helper API.
- GREEN focused `segment_execution_request` filter passed with 2 tests.
- Full `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` passed with 31 tests.
- `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed after rustfmt.
- Scoped `git diff --check` passed with CRLF warnings only.

## Boundaries

- Do not modify files outside `D:\DEV\MyEpicLauncher`.
- Do not run destructive commands.
- Do not implement concrete HTTP fetch, staging writes, hash verification, checkpoint mutation, runtime completion, transport, frontend, SQLite schema, composition-root changes, or `kernel-jobs` changes.
- Skip push unless a safe push path is explicitly authorized later.

## Dirty Worktree To Preserve

- Unrelated unstaged/unknown work remains present and must not be committed with AT-192:
  - `Cargo.lock`
  - `MyEpicLauncher.pen`
  - frontend files under `app/` and `components/`
  - sqlite files under `crates/composition-root/` and `src-tauri/`
  - `.codex` files
  - `src/`
  - `crates/composition-root/src/startup.rs`
