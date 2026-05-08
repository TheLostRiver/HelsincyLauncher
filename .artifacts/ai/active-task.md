# Active Atomic Task

## Identity

- task id: AT-2026-05-08-108
- title: Move Windsurf rules into folder surface
- status: completed

## Goal

在不改动现有 Copilot 规则文件、不开第二套计划协议、也不引入 Windsurf 专用并行工作流的前提下，把当前 Windsurf 兼容层从根 `.windsurfrules` 迁到 `.windsurf/rules` 目录：

- delete `.windsurfrules`
- add `.windsurf/rules/repo-workflow.md`

本轮只迁移已存在的 Windsurf 规则内容和任务记录，不改后端代码、不改模块设计，也不新增第二份规则真相。

## Scope

- in scope:
  - delete `.windsurfrules`
  - add `.windsurf/rules/repo-workflow.md`
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
  - keep both `.windsurfrules` and `.windsurf/rules` as parallel active rule surfaces
  - touch unrelated dirty frontend, pen, sqlite, or lockfile changes already present in the worktree

## Allowed Files

1. .windsurfrules
2. .windsurf/rules/repo-workflow.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

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

- falsifiable local hypothesis: If the repo moves the existing Windsurf compatibility content from root `.windsurfrules` into a single file under `.windsurf/rules/` and removes the root file, then the workspace will satisfy the user's requested Windsurf folder-based rule surface without creating parallel rule entrypoints or changing runtime behavior.

## Cheap Check

- `git -C q:\DEV\MyEpicLauncher diff --check -- .windsurfrules .windsurf/rules/repo-workflow.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Gate

1. `git -C q:\DEV\MyEpicLauncher diff --check -- .windsurfrules .windsurf/rules/repo-workflow.md .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`
2. VS Code diagnostics should report no errors for `.windsurf/rules/repo-workflow.md` and the touched task-record files.

## Validation Result

- passed

## Notes

- The user explicitly wants the Windsurf compatibility surface to live under `.windsurf/rules` instead of the repo-root `.windsurfrules` file.
- Reusing `.github/skills/strict-doc-driven-development/SKILL.md` verbatim is not the right move for Windsurf because its frontmatter is Copilot-specific; the safe move is to restate only the operational rules in plain text.
- The smallest safe migration is to move the already-published plain-text rule content into a single `.windsurf/rules` markdown file and delete the root `.windsurfrules` file so the workspace keeps only one Windsurf rule entrypoint.
- This slice is docs and workflow text only; no backend, frontend, or transport behavior is changed.
- Scoped `git diff --check` returned clean for the root file deletion, the new `.windsurf/rules/repo-workflow.md` file, and the touched task-record files, and VS Code diagnostics reported no errors for the touched text files.

## 安全恢复点

- Windsurf 目录化迁移切片已经完成并通过聚焦校验；若中断，恢复时只发布根 `.windsurfrules` 删除、`.windsurf/rules/repo-workflow.md` 新增和 5 个任务记录文件，不要扩大到其他 agent 或后端文件。

## Completion Summary

- The Windsurf compatibility surface now lives at `.windsurf/rules/repo-workflow.md`.
- The repo-root `.windsurfrules` file has been removed to avoid a second active rule entrypoint.
- Scoped `git diff --check` passed and VS Code diagnostics reported no errors for the touched text files.

## 安全恢复点

- Windsurf 兼容切片已经收敛到 `.windsurf/rules/repo-workflow.md` 单文件迁移；若中断，恢复时先对根 `.windsurfrules` 删除、目录规则文件新增和 5 个任务记录文件做 scoped `git diff --check`，确认无格式问题后再决定是否发布。

## Completion Summary

- AT-2026-05-08-107 has been published as commit `a17e9f7`.

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


