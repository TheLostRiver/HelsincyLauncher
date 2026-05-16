# Active Atomic Task

## Identity

- task id: AT-2026-05-16-186
- title: Wire downloads shared scheduler/source in composition-root
- status: completed

## Goal

按 README_IMPL 7.11，把 composition-root downloads wiring 改成同一个 `InMemoryDownloadResumeWorkScheduler` 同时供 facade `DownloadResumeWorkScheduler` 和 `DownloadJobDriver` 的 `DownloadPendingResumeWorkSource` 使用。先用 RED composition-root 单测证明当前 driver 会从空/no-op source drain，不会看到 facade scheduler 登记的 pending work，再做最小实现。

本轮只覆盖：

- composition-root 私有 builder/helper 层的共享 scheduler/source wiring；
- focused composition-root 单测；
- 必要 README_IMPL/PWF 记录更新；
- 不扩大 composition-root public API。

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
  - `kernel-jobs::JobDriver` trait changes or runtime `run()` semantics
  - host transport, frontend IPC, UI projection
  - SQLite work-item persistence or schema changes
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation, snapshot mutation, completion APIs, or event publication
  - startup stage-2 restore behavior changes
  - unrelated dirty worktree files, including existing `crates/composition-root/src/startup.rs`

## Allowed Files

1. crates/composition-root/src/bootstrap.rs
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
7. docs/modules/downloads/README_IMPL.md section 7.11
8. docs/TauriCompositionRootWiringDesign.md
9. docs/TauriKernelJobsRuntimeDesign.md
10. docs/TauriDownloadRuntimeDesign.md
11. superpowers:test-driven-development skill

## Hypothesis

- falsifiable local hypothesis: composition-root can create one in-memory downloads resume scheduler and pass shared clones to both facade deps and `DownloadJobDriver::with_pending_resume_work_source(...)`, while preserving facade-only public services, startup restore semantics, current smoke behavior, and all out-of-scope runtime/transport/IO/persistence boundaries.

## Cheap Check

- RED/GREEN focused test:
  - `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_driver_drains_work_scheduled_through_shared_facade_scheduler`
- then full composition-root tests:
  - `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. Add the focused RED test first.
2. Observe expected RED failure caused by missing shared scheduler/source wiring.
3. Implement minimal private builder/helper wiring.
4. Update README_IMPL current Rust slice and PWF records.
5. Run focused test, full composition-root test, format check, scoped diff check, and path-limited status.
6. Commit only AT-186 files locally.

## Validation Result

- passed
- RED focused test failed for the expected reasons: `build_downloads_module(...)` did not accept a shared scheduler argument and `build_download_job_driver(...)` did not exist.
- GREEN implementation creates one composition-root owned `InMemoryDownloadResumeWorkScheduler`, passes it to the downloads facade deps, and injects it into `DownloadJobDriver::with_pending_resume_work_source(...)`.
- Focused composition-root test passed: `download_driver_drains_work_scheduled_through_shared_facade_scheduler`.
- Full `launcher-composition-root` suite passed: 6 unit tests and 7 integration tests passed; doc tests 0.
- Initial `cargo fmt --check` failed on one test line wrap; after `cargo fmt`, `cargo fmt -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
- README_IMPL section 7.11 now records the current Rust slice.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` showed only AT-186 files.
- Local commit completed after validation.

## Notes

- AT-2026-05-16-185 committed as `cb991f3`.
- Push to `origin/main` was rejected by safety review; do not work around it.
