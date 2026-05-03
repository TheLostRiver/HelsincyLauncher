# Active Atomic Task

## Identity

- task id: AT-2026-05-03-034
- title: Benchmark documentation against Codex-Manager
- status: committed

## Goal

- exact local outcome: Compare MyEpicLauncher's collaboration-constraint docs and architecture docs against Codex-Manager's CONTRIBUTING and ARCHITECTURE documents, write a detailed local benchmark and optimization record, and repair the smallest discoverability drift in `README.md` so the new benchmark is not isolated.

## Scope

- in scope:
  - add `docs/TauriDocumentationBenchmarkAgainstCodexManager.md`
  - update `README.md` for current-state accuracy and benchmark discoverability
  - update `.artifacts/ai` records for the AT-034 docs slice
- out of scope:
  - changing backend Rust/Tauri code
  - rewriting the deep design docs under `docs/`
  - touching user-owned frontend worktree changes

## Allowed Files

1. docs/TauriDocumentationBenchmarkAgainstCodexManager.md
2. README.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. .github/copilot-instructions.md
2. .github/skills/strict-doc-driven-development/SKILL.md
3. docs/TauriRewriteArchitectureBlueprint.md
4. docs/TauriArchitecturePrinciplesDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md
6. docs/TauriTestingStrategyAndQualityGateDesign.md
7. README.md

## Hypothesis

- falsifiable local hypothesis: MyEpicLauncher already has stronger deep architecture and AI workflow rules than Codex-Manager, but it lacks a flatter contributor-facing overview layer; if this slice adds one benchmark document and repairs the README entry surface to point at the real repo state plus the new benchmark, then the main comparison gaps will be captured without weakening the existing strict-doc stack.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify the new benchmark doc exists, `README.md` no longer claims the repo lacks `Cargo.toml` or `src-tauri/`, and no stale root planning file paths remain in the edited entry surface.

## Validation Gate

1. grep search for the new benchmark link and stale README claims
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai README.md docs`

## Validation Result

- Root `README.md` grep checks passed: the new benchmark entry exists and the stale pre-backend / root-planning claims no longer appear in the root entry surface.
- `git -C q:\DEV\MyEpicLauncher diff --check` produced no output for the docs slice.
- Repo-scoped changed-file review shows the AT-034 slice is limited to `README.md`, `docs/TauriDocumentationBenchmarkAgainstCodexManager.md`, and `.artifacts/ai` record updates.

## 需要更新的文档和日志

1. docs/TauriDocumentationBenchmarkAgainstCodexManager.md
2. README.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: deferred; no git commit unless explicitly requested by the user
2. push command plan: deferred

## 停止条件

1. the comparison requires rewriting multiple existing design docs instead of recording a bounded benchmark
2. the README repair would require broad repo-wide documentation rewrites outside the allowed files
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: choose whether the next docs slice should implement P1 (`CONTRIBUTING.md`) or P2 (current-repo architecture overview), or resume backend integration from the validated post-E2 baseline.