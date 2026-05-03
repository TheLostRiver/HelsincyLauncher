# Active Atomic Task

## Identity

- task id: AT-2026-05-03-008
- title: Normalize AI record schema
- status: committed

## Goal

- exact local outcome: Normalize the repo's live `.artifacts/ai` records plus repo-local planning templates and bootstrap output into one planning-with-files-inspired schema that still preserves strict-doc atomic-task semantics.

## Scope

- in scope:
  - restructure `.artifacts/ai/active-task.md`, `task-plan.md`, `progress.md`, and `findings.md` into clearer section-based documents
  - align planning-with-files default templates and init-session output to the same schema
  - preserve the numbered atomic-task ledger that repo hooks and `check-complete` already parse
- out of scope:
  - changing hook recovery logic, stop-hook parsing rules, or slash-command behavior beyond schema alignment
  - reintroducing root planning files or any second planning directory
  - backend feature work

## Allowed Files

1. .artifacts/ai/**
2. .github/skills/planning-with-files/templates/**
3. .github/skills/planning-with-files/scripts/init-session.ps1
4. .github/skills/planning-with-files/scripts/init-session.sh

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIContextManagementIntegrationDesign.md
6. .github/copilot-instructions.md
7. .github/skills/planning-with-files/SKILL.md
8. .github/skills/strict-doc-driven-development/active-atomic-task-template.md

## Hypothesis

- falsifiable local hypothesis: The disorder comes from live records, planning templates, and bootstrap scripts using different schemas; if those three surfaces share one hybrid format, the files will become predictable without breaking the repo's single-active-task protocol.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Confirm the new task-plan schema still preserves numbered AT status lines, then run `get_errors` on the touched markdown/template files and `git diff --check`.

## Validation Gate

1. `get_errors` on the touched markdown/template files
2. `git diff --check`

## Validation Result

- `get_errors` reported no diagnostics for the normalized markdown, template, and bootstrap-script files.
- `bash -n .github/skills/planning-with-files/scripts/init-session.sh` and `git diff --check` both passed after correcting the Windows bash path form used during validation.

## 需要更新的文档和日志

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. .github/skills/planning-with-files/templates/task_plan.md
6. .github/skills/planning-with-files/templates/progress.md
7. .github/skills/planning-with-files/templates/findings.md
8. .github/skills/planning-with-files/scripts/init-session.ps1
9. .github/skills/planning-with-files/scripts/init-session.sh

## 验证后的 Git 动作

1. commit message plan: Normalize AI record schema
2. push command plan: git push

## 停止条件

1. missing or conflicting docs
2. any schema change that would break the task ledger or single-active-task protocol
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: use the normalized schema as the default for the next workflow slice and only revisit it if a hook, template, or bootstrap surface drifts again.