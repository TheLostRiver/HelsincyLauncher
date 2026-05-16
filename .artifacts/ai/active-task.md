# Active Atomic Task

## Identity

- task id: AT-2026-05-16-188
- title: Persist downloads segment checkpoint facts in SQLite
- status: completed

## Goal

按 README_IMPL 7.12，为 `SqliteDownloadCheckpointRepository` 增加 segment checkpoint facts 的最小持久化 round-trip，先 RED 测试，再实现，只覆盖 repository/adapter persistence boundary。

本轮只覆盖：

- `DownloadCheckpointRecord.segments` 在 SQLite adapter 的 save/load round-trip；
- focused adapter test；
- README_IMPL/PWF 当前 Rust slice 更新；
- 不进入 driver execution、fetch/write/verify 或 runtime completion。

## Scope

- in scope:
  - `crates/adapter-storage-sqlite/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - driver execution and pending-work consumption changes
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - `kernel-jobs` runtime execution loop or job completion APIs
  - host transport, frontend IPC, UI projection
  - composition-root public API changes
  - broad storage abstractions or global unit-of-work
  - unrelated dirty worktree files

## Allowed Files

1. crates/adapter-storage-sqlite/src/lib.rs
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## 控制性文档

Required context read in scoped snippets before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md section 7.12
8. docs/TauriDownloadRuntimeDesign.md checkpoint/resume sections
9. docs/TauriStorageAndDatabaseDesign.md storage placement sections
10. docs/TauriRepositoryPortsAndAdapterDesign.md repository/checkpoint transaction sections
11. crates/adapter-storage-sqlite/src/lib.rs checkpoint implementation
12. superpowers:test-driven-development skill

## Hypothesis

- falsifiable local hypothesis: `SqliteDownloadCheckpointRepository` can persist and load current `DownloadSegmentCheckpointRecord` facts for one job using adapter-local SQL/schema changes while preserving existing job-level restore behavior and without changing driver execution, runtime, transport, frontend, or concrete IO.

## Cheap Check

- RED/GREEN focused test:
  - `cargo test -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml sqlite_download_checkpoint_round_trips_segment_facts`
- then full adapter test:
  - `cargo test -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. Add focused RED adapter test before production code.
2. Observe expected RED failure caused by segment facts not round-tripping.
3. Implement minimal job/segment checkpoint table persistence.
4. Update README_IMPL and PWF records.
5. Run focused test, full adapter tests, format check, scoped diff check, and path-limited status.
6. Commit only AT-188 files locally.

## Validation Result

- passed
- RED focused adapter test failed as expected: saved checkpoint loaded back with `segments: []`.
- GREEN implementation added `download_segment_checkpoints`, transactional job-scoped segment replacement, segment row mapping, status encoding/decoding, and u64 text encoding.
- Focused test passed: `sqlite_download_checkpoint_round_trips_segment_facts`.
- Full `launcher-adapter-storage-sqlite` test suite passed: 1 unit test passed; doc tests 0.
- Initial `cargo fmt --check` failed on crate formatting; after `cargo fmt`, `cargo fmt -p launcher-adapter-storage-sqlite --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
- README_IMPL section 7.12 now records the current Rust slice.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` showed only AT-188 files.
- Local commit completed after validation.

## Notes

- AT-2026-05-16-187 committed as `95cf6fa`.
- Push to `origin/main` was rejected earlier by safety review; do not work around it.
