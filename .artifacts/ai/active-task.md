# Active Atomic Task

## Identity

- task id: AT-2026-05-08-107
- title: Create Windsurf repo rules mapping
- status: completed

## Goal

在不改动现有 Copilot 规则文件、不开第二套计划协议、也不引入 Windsurf 专用并行工作流的前提下，为当前仓库生成一份可直接给 Windsurf 消费的 repo-local 规则文件：

- `.windsurfrules`

本轮只把现有 strict-doc-driven-development 和 `.artifacts/ai` 事务协议投影成 plain-text Windsurf 规则，不改后端代码、不改模块设计、不重写已有 Copilot skill frontmatter。

## Scope

- in scope:
  - add `.windsurfrules`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change any backend or frontend runtime behavior
  - replace `.artifacts/ai` with a second planning or memory surface
  - rewrite `.github/skills/strict-doc-driven-development/SKILL.md` into a non-Copilot format
  - introduce `.windsurf/` planning files or any root legacy planning files as a second source of truth
  - touch unrelated dirty frontend, pen, sqlite, or lockfile changes already present in the worktree

## Allowed Files

1. .windsurfrules
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIContextManagementIntegrationDesign.md
6. .github/copilot-instructions.md
7. .github/skills/strict-doc-driven-development/SKILL.md
8. .artifacts/ai/README.md

## Hypothesis

- falsifiable local hypothesis: If the repo adds a root `.windsurfrules` file that restates the existing strict-doc and `.artifacts/ai` transaction protocol in plain instructions instead of Copilot-specific skill metadata, then Windsurf can follow the same repository workflow without introducing a second planning source or changing any runtime behavior.

## Cheap Check

- `git -C q:\DEV\MyEpicLauncher diff --check -- .windsurfrules .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. `git -C q:\DEV\MyEpicLauncher diff --check -- .windsurfrules .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`
2. VS Code diagnostics should report no errors for `.windsurfrules` and the touched task-record files.

## Validation Result

- passed

## Notes

- The workspace scan found no existing Windsurf-specific repo rule surface, no `.windsurf/` directory, and no adjacent generic agent rule files such as `AGENTS.md`.
- Reusing `.github/skills/strict-doc-driven-development/SKILL.md` verbatim is not the right move for Windsurf because its frontmatter is Copilot-specific; the safe move is to restate only the operational rules in plain text.
- A repo-root `.windsurfrules` file is the smallest isolated compatibility layer because it does not require rewriting existing Copilot instructions or reopening the `.artifacts/ai` protocol design.
- This slice is docs and workflow text only; no backend, frontend, or transport behavior is changed.

## 安全恢复点

- Windsurf 兼容切片已经收敛到 `.windsurfrules` 单文件规则投影；若中断，恢复时先对 `.windsurfrules` 和 5 个任务记录文件做 scoped `git diff --check`，确认无格式问题后再决定是否发布。

## Completion Summary

- The repo now has a root `.windsurfrules` file that restates the current strict-doc workflow, `.artifacts/ai` source-of-truth rule, validation order, architecture guardrails, and comment-language defaults in plain instructions for Windsurf.
- Scoped `git diff --check` passed for `.windsurfrules` and the touched task-record files.
- VS Code diagnostics reported no errors for `.windsurfrules` or the touched task-record files.

## Completion
- AT-2026-05-06-091 has been published as commit `f20e4f5`.
- AT-2026-05-06-092 has been published as commit `c5b6f33`.
- AT-2026-05-07-093 has been published as commit `d8fbbc8`.
- AT-2026-05-07-094 has been published as commit `39ba47d`.
- AT-2026-05-07-095 has been published as commit `f022abe`.
- AT-2026-05-07-096 has been published as commit `5b5a96a`.
- AT-2026-05-08-097 has been published as commit `367b4b6`.
- AT-2026-05-08-098 has been published as commit `7260673`.
- AT-2026-05-08-099 has been published as commit `2792762`.
- AT-2026-05-08-100 has been published as commit `fab77ce`.
- AT-2026-05-08-101 has been published as commit `340bd13`.
- AT-2026-05-08-102 has been published as commit `7fa1bda`.
- AT-2026-05-08-103 has been published as commit `6fcb6e3`.
- AT-2026-05-08-104 has been published as commit `c35ffaa`.
- AT-2026-05-08-105 has been published as commit `7b4b502`.
- AT-2026-05-08-106 has been published as commit `ec3dc63`.


