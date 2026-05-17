# Active Atomic Task

## Identity

- task id: AT-2026-05-17-191
- title: Define downloads segment execution ports boundary
- status: completed

## Goal

AT-190 已经把 `DownloadJobDriver` 的 module-local execution-turn 分类落地，但实现文档仍然把该能力描述为“下一 Rust slice”。本轮只做文档收敛：

1. 把 README_IMPL 7.13 更新为当前 Rust reality；
2. 明确 local execution-turn 已经完成什么、仍然没有完成什么；
3. 在 README_IMPL 中新增下一段 driver segment execution ports boundary；
4. 明确后续编码不能直接写 HTTP/disk/hash 具体实现，必须先通过端口/DTO/测试边界。

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
  - Rust tests
  - `kernel-jobs::JobDriver` trait changes
  - runtime `run()` implementation
  - concrete HTTP fetch, staging writes, hash/length verifier
  - checkpoint mutation implementation changes
  - composition-root, host transport, frontend, SQLite schema changes
  - unrelated dirty worktree files

## Allowed Files

1. docs/modules/downloads/README_IMPL.md
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Already read in scoped snippets this session:

1. `docs/modules/downloads/README_IMPL.md` section 7.13
2. `.artifacts/ai/handoff.md`
3. `.artifacts/ai/task-plan.md` current phase and ledger
4. AT-190 git log/status confirmation

## Hypothesis

- falsifiable local hypothesis: a docs-only boundary can make the next code task explicit without falsely claiming real download IO or runtime `run()` exists.

## Cheap Check

1. Read back README_IMPL sections 7.13 and the new segment execution ports boundary.
2. Run scoped `git diff --check` for the docs/PWF file set.
3. Run path-limited `git status --short` for the docs/PWF file set.

## Validation Gate

1. README_IMPL reflects AT-190 as current Rust reality.
2. README_IMPL defines the next safe code slice and explicitly excludes concrete IO/runtime completion.
3. PWF records are updated.
4. Scoped diff/status checks pass.
5. Commit only AT-191 files locally.

## Validation Result

- README_IMPL readback confirmed section 7.13 now records AT-190 current Rust reality.
- README_IMPL readback confirmed new section 7.14 defines the next segment execution ports boundary and first Rust slice.
- Scoped `git diff --check` passed with CRLF warnings only.
- Path-limited status showed only AT-191 docs/PWF files in the intended commit set.
- Local commit: completed with the AT-191 docs/PWF file set; verify the final amended hash with `git log --oneline -1`.

## Notes

- AT-2026-05-17-190 committed locally as `f5d950d`.
- Push remains skipped unless a safe push path is explicitly authorized later.
