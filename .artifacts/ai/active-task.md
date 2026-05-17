# Active Atomic Task

## Identity

- task id: AT-2026-05-17-189
- title: Define downloads driver execution boundary
- status: completed

## Goal

在 segment checkpoint facts 已经可持久化之后，先明确 downloads driver execution boundary：当前 Rust 里 `kernel-jobs::JobDriver` 仍只有 `restore()`，没有真实 `run()` 回调；因此下一步不能直接进入 HTTP fetch、staging write 或 verifier，而要先定义 driver 何时、如何消费 pending work、如何与 checkpoint 和 runtime snapshot 分层。

本轮只覆盖：

- README_IMPL driver execution boundary；
- 明确当前 Rust reality 与未来 runtime `run()` 设计之间的差异；
- 明确下一 Rust slice 的最小候选；
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
  - `kernel-jobs::JobDriver` API changes
  - driver execution implementation
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - checkpoint mutation implementation beyond already persisted segment facts
  - host transport, frontend IPC, UI projection
  - composition-root wiring changes
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
7. docs/modules/downloads/README_IMPL.md section 7.12
8. docs/TauriKernelJobsRuntimeDesign.md driver/runtime sections
9. docs/TauriDownloadRuntimeDesign.md scheduler/fetcher/writer/verifier/checkpoint sections
10. docs/TauriTestingStrategyAndQualityGateDesign.md backend test matrix

## Hypothesis

- falsifiable local hypothesis: a docs-only boundary can identify the next safe Rust slice without falsely claiming real runtime execution exists or prematurely implementing concrete fetch/write/verify behavior.

## Cheap Check

- Read back the new README_IMPL section and PWF current phase.
- `git diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`
- `git status --short -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Document current runtime reality versus future `run()` design.
2. Document execution-turn ownership, pending-work consumption rules, checkpoint/snapshot ordering, and out-of-scope fetch/write/verify boundaries.
3. Identify the next Rust slice candidate.
4. Update PWF records.
5. Run readback, scoped diff check, and path-limited status.
6. Commit only AT-189 files locally.

## Validation Notes

- README_IMPL readback passed locally.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited status showed only the six AT-189 files modified.

## Validation Result

- passed; committed locally with the AT-189 docs/PWF file set

## Notes

- AT-2026-05-16-188 committed as `4e3e5ac`.
- AT-2026-05-17-189 committed locally; verify the final amended hash with `git log --oneline -1`.
- Push to `origin/main` was rejected earlier by safety review; do not work around it.
