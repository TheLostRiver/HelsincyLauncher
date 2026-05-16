# Active Atomic Task

## Identity

- task id: AT-2026-05-16-187
- title: Define downloads checkpoint mutation boundary
- status: completed

## Goal

在继续任何 fetch/write/verify 或 driver execution 代码前，先把 downloads checkpoint mutation 的 owner、写入时机、repository/SQLite 边界、错误分层和下一 Rust 切片写清楚。

本轮只覆盖：

- README_IMPL checkpoint mutation boundary；
- 明确 command path、scheduler pending queue、driver execution turn 与 checkpoint repository 的职责；
- 明确下一 Rust 切片应优先补 segment checkpoint persistence，而不是直接进入 HTTP/文件写入/校验；
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
  - SQLite schema or adapter implementation changes
  - concrete HTTP fetch, staging writes, verifier/hash execution
  - `kernel-jobs` runtime execution loop or job completion APIs
  - host transport, frontend IPC, UI projection
  - checkpoint mutation implementation
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
7. docs/modules/downloads/README_IMPL.md section 7.11
8. docs/TauriDownloadRuntimeDesign.md checkpoint/resume sections
9. docs/TauriStorageAndDatabaseDesign.md storage placement sections
10. docs/TauriRepositoryPortsAndAdapterDesign.md repository/checkpoint transaction sections

## Hypothesis

- falsifiable local hypothesis: a docs-only boundary can make the checkpoint mutation owner and next persistence slice explicit enough to continue backend work without prematurely implementing fetch/write/verify, runtime completion, host transport, frontend projection, or broad SQLite abstractions.

## Cheap Check

- Read back the new README_IMPL section and PWF current phase.
- `git diff --check -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`
- `git status --short -- docs/modules/downloads/README_IMPL.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. Document command/driver/checkpoint repository ownership.
2. Document durable checkpoint mutation timing and failure behavior.
3. Identify the next Rust slice without opening concrete download execution.
4. Update PWF records.
5. Run readback, scoped diff check, and path-limited status.
6. Commit only AT-187 files locally.

## Validation Result

- passed
- README_IMPL section 7.12 now defines the checkpoint mutation boundary.
- Readback confirmed ownership rules, command path rules, driver/execution-turn rules, persistence rules, and next Rust slice.
- Task-plan readback confirmed Phase 62 is the current phase.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited `git status --short` showed only AT-187 files.
- Local commit completed after validation.

## Notes

- AT-2026-05-16-186 committed as `6a721af`.
- Push to `origin/main` was rejected earlier by safety review; do not work around it.
