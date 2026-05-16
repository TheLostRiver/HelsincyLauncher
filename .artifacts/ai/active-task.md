# Active Atomic Task

## Identity

- task id: AT-2026-05-16-183
- title: Define DownloadJobDriver local pending-work consumer boundary
- status: completed

## Goal

在 AT-182 已经提供 `DownloadPendingResumeWorkSource` 之后，继续按文档优先原则明确 `DownloadJobDriver` 的下一步落点：当前 `kernel-jobs::JobDriver` 仍然只有 `restore()`，所以本轮只补充实现文档，定义一个 future-ready 的本地 consumer 方法边界，而不是改 shared runtime、composition wiring 或真实下载执行。

本轮要回答：

- `DownloadJobDriver` 是否可以在没有 `JobDriver::run()` 的情况下先拥有一个本地 pending-work consumer 方法；
- 该方法怎样依赖 `DownloadPendingResumeWorkSource`，并如何保持 `DownloadJobDriver::new(checkpoint_repo)` 兼容；
- 空 drain、source failure、stage-2 restore、future execution turn 之间的行为边界；
- 下一片 Rust slice 是否足够明确。

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code for AT-183
  - changing `DownloadJobDriver`
  - changing `kernel-jobs::JobDriver`
  - composition-root wiring
  - host transport, frontend IPC, UI projection
  - SQLite schema or work-item persistence
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation or new checkpoint repository methods
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

Required context read in scoped snippets before editing:

1. docs/modules/downloads/README_IMPL.md
2. crates/module-downloads/src/driver.rs
3. crates/kernel-jobs/src/runtime.rs
4. Prior required docs read for AT-181/AT-182 remain controlling: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW, download runtime, kernel-jobs runtime, composition-root wiring, first crate API draft, testing strategy, and AI transaction protocol.

## Hypothesis

- falsifiable local hypothesis: the next safe code slice can add a `DownloadJobDriver` local consumer method backed by `DownloadPendingResumeWorkSource`, while keeping `restore()` unchanged and avoiding any shared runtime execution API change.

## Cheap Check

- docs-only validation:
  - read back the new README_IMPL section;
  - run scoped `git diff --check`;
  - run path-limited `git status --short` for AT-183 files.

## Validation Gate

1. Confirm AT-182 is committed.
2. Read README_IMPL 7.9 plus current `DownloadJobDriver` and `JobDriver` trait snippets.
3. Add README_IMPL section defining the local driver consumer boundary.
4. Update PWF records.
5. Run docs-only validation.
6. Commit only AT-183 files locally.

## Validation Result

- passed
- README_IMPL section `7.10 DownloadJobDriver Local Consumer Boundary` was added and read back.
- The section preserves `DownloadJobDriver::new(checkpoint_repo)` compatibility, defines `with_pending_resume_work_source(...)`, and defines a local `drain_pending_resume_work(&JobId)` method for a later code slice.
- The section explicitly keeps `restore()` unchanged and says restore must not drain in-memory pending work.
- Scoped `git diff --check` passed for AT-183 files with CRLF warnings only.
- Path-limited `git status --short` showed only AT-183 files.

## Notes

- AT-2026-05-16-182 committed as `bb35c6f`.
- If AT-183 validates a clear Rust slice, AT-184 can code the driver local consumer with TDD.
- AT-184 is clear enough to start after this commit: add no-op source for `()`, inject pending source into `DownloadJobDriver`, and add focused driver tests.
