# Active Atomic Task

## Identity

- task id: AT-2026-05-05-069
- title: Comment language control docs and slash commands
- status: completed

## Goal

收紧仓库注释规范，让新增或改写的代码注释默认使用简体中文，并为需要英文注释的开发者补上显式的 slash 命令入口：
- `docs/TauriCodeCommentStandard.md`
- `.github/prompts/comment-zh.prompt.md`
- `.github/prompts/comment-en.prompt.md`

本轮只调整注释语言规范和 prompt 入口，不改业务代码、不改现有 hook 语言模式，也不改 backend 行为。

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
  - update `docs/TauriCodeCommentStandard.md`
  - add `.github/prompts/comment-zh.prompt.md`
  - add `.github/prompts/comment-en.prompt.md`
- out of scope:
  - change any backend or frontend runtime behavior
  - overload `.artifacts/ai/language-mode.txt` to mean comment language
  - add comment rewrites across existing source files
  - modify unrelated architecture docs or hook scripts

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. .artifacts/ai/handoff.md
6. docs/TauriCodeCommentStandard.md
7. .github/prompts/comment-zh.prompt.md
8. .github/prompts/comment-en.prompt.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriCodeCommentStandard.md
6. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If the comment standard explicitly states that code comments default to simplified Chinese and the repo exposes prompt-based `/comment-zh` and `/comment-en` switches for future comment-authoring work, then developers can request English comments without weakening the default Chinese rule or overloading the existing workflow-language controls.

## Cheap Check

- `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/comment-zh.prompt.md .github/prompts/comment-en.prompt.md`

## Validation Gate

1. `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/comment-zh.prompt.md .github/prompts/comment-en.prompt.md`

## Validation Result

- passed

## Notes

- `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/comment-zh.prompt.md .github/prompts/comment-en.prompt.md` produced no output.
- VS Code diagnostics stayed clean for `docs/TauriCodeCommentStandard.md`, `.github/prompts/comment-zh.prompt.md`, and `.github/prompts/comment-en.prompt.md`.

## 安全恢复点

- 默认中文注释和显式英文切换约束已经收敛到 `docs/TauriCodeCommentStandard.md` 与 `.github/prompts/comment-*.prompt.md`；若中断，恢复时直接完成这三个文件并立刻跑 scoped `git diff --check`。

## Completion

- completed slice: `docs/TauriCodeCommentStandard.md`, `.github/prompts/comment-zh.prompt.md`, `.github/prompts/comment-en.prompt.md`
- publication step: pending commit and push in this turn

- pending
