# Active Atomic Task

## Identity

- task id: AT-2026-05-16-185
- title: Define downloads composition shared scheduler/source wiring boundary
- status: completed

## Goal

在开始 composition-root Rust wiring 前，先把 downloads resume pending-work scheduler/source 的装配边界写清楚：同一个 `InMemoryDownloadResumeWorkScheduler` 必须同时作为 facade 的 `DownloadResumeWorkScheduler` 和 driver 的 `DownloadPendingResumeWorkSource`，避免 facade 注册的 pending work 与 driver drain 看到两套不同内存队列。

本轮只覆盖：

- README_IMPL 中的 composition shared scheduler/source wiring 规则；
- 明确 composition-root 只负责 assembly，不执行业务 resume/fetch/write/verify；
- 明确下一 Rust 切片的最小可验证范围；
- 更新 `.artifacts/ai` 任务记录。

## Scope

- in scope:
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust production code changes
  - composition-root wiring implementation
  - `kernel-jobs::JobDriver` API changes or runtime execution loop
  - host transport, frontend IPC, UI projection
  - SQLite schema or durable work-item persistence
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation, snapshot mutation, completion APIs, or event publication
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
7. docs/modules/downloads/README_IMPL.md sections 7.9-7.10
8. docs/TauriCompositionRootWiringDesign.md
9. docs/TauriKernelJobsRuntimeDesign.md
10. docs/TauriDownloadRuntimeDesign.md

## Hypothesis

- falsifiable local hypothesis: a docs-only boundary can define the next composition-root wiring slice clearly enough that the later Rust task can share one scheduler/source instance without widening into runtime execution, host transport, frontend, SQLite persistence, fetch/write/verify, checkpoint mutation, or snapshot completion.

## Cheap Check

- Read back the new README_IMPL section and PWF current phase.
- `git diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`
- `git status --short -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Document shared scheduler/source ownership and assembly-only rules.
2. Document the minimum next Rust wiring slice and verification path.
3. Update PWF records.
4. Run readback, scoped diff check, and path-limited status.
5. Commit only AT-185 files locally.

## Validation Result

- passed
- README_IMPL section 7.11 now defines the composition shared scheduler/source wiring boundary.
- Readback confirmed the section states one `InMemoryDownloadResumeWorkScheduler` must feed both facade preparation and driver drain.
- Task-plan readback confirmed Phase 60 is the current phase.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` showed only AT-185 files.

## Notes

- AT-2026-05-16-184 is committed as `a710cfc`.
- The next Rust slice should not begin fetch/write/verify execution; it should only wire the same scheduler/source instance through composition-root and prove the driver can drain work prepared through the facade path.
