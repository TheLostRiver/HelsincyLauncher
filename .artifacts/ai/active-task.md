# Active Atomic Task

## Identity

- task id: AT-2026-05-05-070
- title: Prefix workspace slash commands with hsy
- status: completed

## Goal

统一仓库级 slash prompt 命名，避免和其他工作区命令冲突：把现有 workspace prompt 全部改为 `hsy-XXX` 形式，并同步更新当前规范与任务记录中的命令引用。

本轮只调整 prompt 文件名、prompt `name` 和相关文档引用，不改 hook 行为、不改业务代码，也不扩展新的命令能力。

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
  - update `docs/TauriCodeCommentStandard.md`
  - replace `.github/prompts/plan-atomic-task.prompt.md`
  - replace `.github/prompts/plan-backend-skeleton.prompt.md`
  - replace `.github/prompts/plan-doc-review.prompt.md`
  - replace `.github/prompts/resume-from-handoff.prompt.md`
  - replace `.github/prompts/comment-zh.prompt.md`
  - replace `.github/prompts/comment-en.prompt.md`
- out of scope:
  - change any backend or frontend runtime behavior
  - change hook dispatch or workflow language mode semantics
  - add new prompt capabilities beyond the rename
  - rewrite unrelated historical logs outside the touched task records
  - modify backend or frontend source files

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. .artifacts/ai/handoff.md
6. docs/TauriCodeCommentStandard.md
7. .github/prompts/plan-atomic-task.prompt.md
8. .github/prompts/plan-backend-skeleton.prompt.md
9. .github/prompts/plan-doc-review.prompt.md
10. .github/prompts/resume-from-handoff.prompt.md
11. .github/prompts/comment-zh.prompt.md
12. .github/prompts/comment-en.prompt.md
13. .github/prompts/hsy-plan-atomic-task.prompt.md
14. .github/prompts/hsy-plan-backend-skeleton.prompt.md
15. .github/prompts/hsy-plan-doc-review.prompt.md
16. .github/prompts/hsy-resume-from-handoff.prompt.md
17. .github/prompts/hsy-comment-zh.prompt.md
18. .github/prompts/hsy-comment-en.prompt.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriCodeCommentStandard.md
6. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If all repository-local workspace prompts are renamed to `hsy-XXX` at both the filename and frontmatter `name` level, and the normative docs switch to the same names, then the repo command surface will avoid common command-name collisions without changing any underlying workflow behavior.

## Cheap Check

- `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/*.prompt.md`

## Validation Gate

1. `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/*.prompt.md`

## Validation Result

- passed

## Notes

- `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md docs/TauriCodeCommentStandard.md .github/prompts/*.prompt.md` produced no output.
- `.github/prompts/*.prompt.md` now resolves to exactly six `hsy-` prefixed prompt files with no remaining unprefixed prompt files.
- VS Code diagnostics stayed clean for `docs/TauriCodeCommentStandard.md` and the six active `hsy-` prompt files.

## 安全恢复点

- hsy 前缀统一收敛在 `docs/TauriCodeCommentStandard.md` 和 `.github/prompts/*.prompt.md`；若中断，恢复时直接把六个 prompt 全部切到 `hsy-XXX` 并立刻跑 scoped `git diff --check`。

## Completion

- completed slice: `docs/TauriCodeCommentStandard.md`, `.github/prompts/hsy-plan-atomic-task.prompt.md`, `.github/prompts/hsy-plan-backend-skeleton.prompt.md`, `.github/prompts/hsy-plan-doc-review.prompt.md`, `.github/prompts/hsy-resume-from-handoff.prompt.md`, `.github/prompts/hsy-comment-zh.prompt.md`, `.github/prompts/hsy-comment-en.prompt.md`
- publication step: pending commit and push in this turn
