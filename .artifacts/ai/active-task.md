# Active Atomic Task

## Identity

- task id: AT-2026-05-16-181
- title: Define downloads driver pending-work consumption boundary
- status: completed

## Goal

把 AT-179/AT-180 已经落地的 `InMemoryDownloadResumeWorkScheduler` 继续向 driver 侧推进，但本轮只先写清楚实现边界：当前 `kernel-jobs` 的真实 `JobDriver` 接口只有 `restore()`，所以不能假装已经存在完整 `run()` 执行循环；需要在 `docs/modules/downloads/README_IMPL.md` 中定义下一片最小 Rust slice 应该怎样让 downloads driver 消费 pending resume work，同时保持 fetch/write/verify/checkpoint mutation 等真实执行面继续后置。

本轮覆盖：

- document how prepared `DownloadPendingResumeWork` should be consumed after shared runtime owns a job execution turn;
- define whether the driver should depend on the concrete `InMemoryDownloadResumeWorkScheduler` or a narrow module-local trait/port;
- document drain timing, empty queue behavior, job-id filtering, and failure behavior;
- decide the next code slice shape only if it can be implemented without inventing undocumented runtime execution semantics.

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code for AT-181
  - concrete HTTP fetch, staging write, sparse write, verifier, or hash execution
  - checkpoint mutation or new checkpoint repository methods
  - SQLite schema or concrete work-item persistence
  - host transport, frontend IPC, UI projection, or `kernel-jobs` segment payloads
  - changing `kernel-jobs::JobDriver` in the docs-only task
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

Related Rust surfaces read for factual boundary checks:

1. crates/module-downloads/src/driver.rs
2. crates/module-downloads/src/facade/mod.rs
3. crates/kernel-jobs/src/runtime.rs
4. crates/composition-root/src/bootstrap.rs

## Hypothesis

- falsifiable local hypothesis: the next downloads backend slice should first introduce a narrow pending-work consumption boundary for the downloads driver, rather than depending directly on concrete scheduler internals or widening `kernel-jobs`, transport, frontend, SQLite, or concrete download IO.

## Cheap Check

- docs-only validation:
  - read back the new README_IMPL section;
  - run scoped `git diff --check`;
  - run path-limited `git status --short` for AT-181 files.

## Validation Gate

1. Confirm AT-180 is already committed and preserve unrelated dirty files.
2. Read required docs and relevant Rust entry points in small snippets.
3. Update PWF records to AT-181.
4. Add README_IMPL boundary for driver-side pending-work consumption.
5. Update progress/findings/handoff.
6. Run docs-only validation.
7. Commit only AT-181 files locally.

## Validation Result

- passed
- README_IMPL section `7.9 Driver Pending-Work Consumption Boundary` was read back after editing.
- The new boundary explicitly records that current `kernel-jobs::JobDriver` has no `run()` API yet, so `restore()` must remain a checkpoint recovery gate and must not drain in-memory pending work.
- The documented next Rust slice is a job-id-scoped `DownloadPendingResumeWorkSource` / drain boundary implemented on `InMemoryDownloadResumeWorkScheduler`.
- Scoped `git diff --check` passed for AT-181 files with CRLF warnings only.
- Path-limited `git status --short` showed only the six AT-181 files.

## Notes

- AT-2026-05-16-180 is committed as `d3b1b7d`.
- Current `kernel-jobs::JobDriver` has `module()`, `kind()`, and `restore()` only; docs that mention `run()` are future design, not current Rust API.
- `InMemoryDownloadResumeWorkScheduler` is cloneable and exposes `pending_work()` / `drain_pending_work()`, but the next boundary should avoid binding `DownloadJobDriver` directly to that concrete type unless README_IMPL explicitly justifies it.
- Next Rust slice is explicit enough to start AT-182 after this docs-only commit: add the job-scoped pending-work source/drain semantics with focused module tests, without driver/runtime/transport/frontend/persistence changes.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
