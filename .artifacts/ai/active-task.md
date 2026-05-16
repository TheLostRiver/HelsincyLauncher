# Active Atomic Task

## Identity

- task id: AT-2026-05-16-184
- title: Add DownloadJobDriver local pending-work consumer
- status: completed

## Goal

按 AT-183 的实现文档，在 `DownloadJobDriver` 上增加一个本地 pending resume work consumer 方法，为后续 runtime execution turn 预留明确入口；保持 `restore()` 语义、`kernel-jobs::JobDriver` trait、composition-root wiring、真实下载 IO 和 checkpoint mutation 全部不变。

本轮只覆盖：

- no-op `DownloadPendingResumeWorkSource` for `()`;
- `DownloadJobDriver` 持有 pending-work source；
- 保持 `DownloadJobDriver::new(checkpoint_repo)` 兼容；
- 新增 `with_pending_resume_work_source(...)` 和 `drain_pending_resume_work(&JobId)`;
- focused driver tests。

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/driver.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `kernel-jobs::JobDriver` API changes or runtime execution loop
  - composition-root wiring changes
  - host transport, frontend IPC, UI projection
  - SQLite schema or work-item persistence
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation or new checkpoint repository methods
  - snapshot mutation, completion APIs, or event publication
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/driver.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## 控制性文档

Required context read in scoped snippets before coding:

1. docs/modules/downloads/README_IMPL.md section 7.10
2. crates/module-downloads/src/driver.rs
3. crates/module-downloads/src/facade/mod.rs
4. crates/kernel-jobs/src/runtime.rs
5. superpowers:test-driven-development skill

Prior required architecture/collaboration docs from AT-181/AT-183 remain controlling.

## Hypothesis

- falsifiable local hypothesis: `DownloadJobDriver` can expose a local pending-work consumer method by delegating to `DownloadPendingResumeWorkSource`, while existing restore tests and module tests remain green and no shared runtime/composition/IO/persistence behavior changes.

## Cheap Check

- RED/GREEN focused test:
  - `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml download_job_driver_pending_resume_work`
- then full module test:
  - `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. Write focused RED driver tests before production code.
2. Observe RED failure caused by missing local driver consumer API.
3. Implement no-op source for `()`, driver source field, compatibility constructor, injected constructor, and local drain method.
4. Update README_IMPL and PWF records.
5. Run focused test, full module test, format check, scoped diff check, and path-limited status.
6. Commit only AT-184 files locally.

## Validation Result

- passed
- RED focused driver test failed because `DownloadJobDriver` had no `with_pending_resume_work_source(...)` and no `drain_pending_resume_work(...)`.
- GREEN implementation added no-op `DownloadPendingResumeWorkSource` for `()`, a pending source field on `DownloadJobDriver`, a compatibility-preserving `new(checkpoint_repo)`, an injected constructor, and a local drain method.
- Focused `download_job_driver_pending_resume_work` tests passed: 2 passed, 0 failed.
- Full `launcher-module-downloads` suite passed: 26 unit tests passed, 0 failed; doc tests 0.
- Initial `cargo fmt --check` failed on driver test formatting; after `cargo fmt`, `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` was rerun after approval recovered and showed only AT-184 files.
- Local commit is ready.

## Notes

- AT-2026-05-16-183 committed as `17402bc`.
- New public declaration comments must preserve English and add Chinese companion comments.
- Next slice should reassess composition wiring that shares the same `InMemoryDownloadResumeWorkScheduler` between facade and driver, or document that wiring first.
- Previous commit blocker cleared after approval recovered; AT-184 is ready to commit.
