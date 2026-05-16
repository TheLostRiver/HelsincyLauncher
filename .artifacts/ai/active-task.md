# Active Atomic Task

## Identity

- task id: AT-2026-05-16-182
- title: Add downloads pending resume work source drain
- status: completed

## Goal

按 AT-181 的实现文档，把 `InMemoryDownloadResumeWorkScheduler` 从“只能登记/整体 drain pending work”的壳推进到“可被未来 driver 侧按 job id 消费”的最小代码片：新增 downloads-owned `DownloadPendingResumeWorkSource` 边界，实现 job-id-scoped drain，并用 TDD 证明 draining 一个 job 不会吞掉其他 job 的 pending work，空 job drain 返回空集合。

本轮只覆盖 source/drain 语义，不把它接进 `DownloadJobDriver` 或 `kernel-jobs` 执行循环。

## Scope

- in scope:
  - `crates/module-downloads/src/facade/mod.rs`
  - `crates/module-downloads/src/lib.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - `DownloadJobDriver` integration
  - `kernel-jobs::JobDriver` API changes or runtime execution loop
  - composition-root wiring changes
  - host transport, frontend IPC, UI projection
  - SQLite schema or work-item persistence
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation or new checkpoint repository methods
  - unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. crates/module-downloads/src/lib.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

## 控制性文档

Required context already read in scoped snippets before coding:

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriKernelJobsRuntimeDesign.md
10. docs/TauriCompositionRootWiringDesign.md
11. docs/TauriFirstCrateApiDrafts.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md
14. superpowers:test-driven-development skill

Additional code context to read before RED:

1. `crates/module-downloads/src/facade/mod.rs` test helper area
2. `crates/module-downloads/src/lib.rs` export list

## Hypothesis

- falsifiable local hypothesis: adding a trait-level `drain_pending_resume_work(&JobId)` boundary on the existing in-memory scheduler can prove driver-ready pending-work consumption semantics without touching driver integration, shared runtime, composition wiring, transport, frontend, persistence, or concrete IO.

## Cheap Check

- RED/GREEN focused test:
  - `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml pending_resume_work_source`
- then full module test:
  - `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. Read existing test helper style in `facade/mod.rs`.
2. Write focused RED tests for job-scoped pending-work drain semantics.
3. Observe RED failure caused by missing `DownloadPendingResumeWorkSource` / method.
4. Implement the minimal trait and in-memory scheduler behavior.
5. Export the new source trait through `crates/module-downloads/src/lib.rs`.
6. Update README_IMPL and PWF records.
7. Run focused test, full module test, format check, scoped diff check, and path-limited status.
8. Commit only AT-182 files locally.

## Validation Result

- passed
- RED focused test failed with unresolved `DownloadPendingResumeWorkSource` and missing `drain_pending_resume_work()`.
- GREEN implementation added `DownloadPendingResumeWorkSource`, job-id-scoped draining for `InMemoryDownloadResumeWorkScheduler`, and crate-entry export.
- Focused `pending_resume_work_source` tests passed: 2 passed, 0 failed.
- Full `launcher-module-downloads` suite passed: 24 unit tests passed, 0 failed; doc tests 0.
- `cargo fmt -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` passed.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` showed only AT-182 files.

## Notes

- AT-2026-05-16-181 committed as `ccb0eac`.
- Preserve existing English comments and add concise Chinese companion comments for new public declarations.
- Do not implement actual download execution or claim driver consumption is wired into runtime.
- Next slice must reassess whether to document or implement a `DownloadJobDriver` local consumer method, because current `kernel-jobs::JobDriver` still has no execution `run()` API.
