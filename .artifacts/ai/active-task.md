# Active Atomic Task

## Identity

- task id: AT-2026-05-17-218
- title: Add documentation budget rules
- status: completed

## Goal

Add concise repository rules that prevent module implementation documents from becoming per-task logs, while preserving strict doc-driven development and `.artifacts/ai` as the task execution record.

## Scope

- in scope:
  - `docs/ModuleDocumentationStandard.md`
  - `docs/README.md`
  - `.github/copilot-instructions.md`
  - `.github/skills/strict-doc-driven-development/SKILL.md`
  - `.windsurf/rules/repo-workflow.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - rewriting existing large module implementation history
  - changing backend runtime behavior
  - changing hooks or generated workflow scripts
  - editing unrelated frontend, sqlite, `.codex`, Cargo.lock, or `src/` changes

## Allowed Files

1. docs/ModuleDocumentationStandard.md
2. docs/README.md
3. .github/copilot-instructions.md
4. .github/skills/strict-doc-driven-development/SKILL.md
5. .windsurf/rules/repo-workflow.md
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md
10. .artifacts/ai/handoff.md

## Required Context Read

Read this turn before writing:

1. docs/ModuleDocumentationStandard.md
2. docs/README.md update routing section
3. .github/copilot-instructions.md
4. .github/skills/strict-doc-driven-development/SKILL.md
5. .windsurf/rules/repo-workflow.md
6. current PWF task plan and handoff tails

## Hypothesis

- falsifiable local hypothesis: adding a small "documentation budget" rule to the central workflow/documentation entry points is enough to stop future README_IMPL overgrowth without weakening the requirement to read controlling docs before backend coding.

## Cheap Check

1. Add the concise documentation-budget rule to module docs, docs navigation, Copilot rules, strict-doc skill, and Windsurf projection.
2. Update PWF records, including AT-217 final commit `37765ef`.
3. Run scoped `git diff --check` and path-limited status.

## Validation Gate

1. Central docs clearly separate durable docs from task logs.
2. Rules explicitly send task logs, validation output, handoff notes, and commit ids to `.artifacts/ai`.
3. Rules explicitly discourage long per-AT `README_IMPL.md` completion logs.
4. Scoped `git diff --check` passes.
5. Commit only AT-218 files locally, then push `main` to `origin`.

## Validation Result

- Central workflow and documentation entry points now separate durable `docs/` content from per-task execution logs.
- Task logs, validation output, handoff notes, and commit ids are explicitly routed to `.artifacts/ai/`.
- Long per-AT `README_IMPL.md` completion logs are discouraged in favor of short durable current-state notes.
- Scoped `git diff --check` passed for the AT-218 file set with CRLF normalization warnings only.
- Commit and push are pending for the AT-218 file set.

## Notes

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- AT-2026-05-17-210 final commit is `2f9e828` and is already pushed to `origin/main`.
- AT-2026-05-17-211 final commit is `1d31f56` and is already pushed to `origin/main`.
- AT-2026-05-17-212 final commit is `ed27996` and is already pushed to `origin/main`.
- AT-2026-05-17-213 final commit is `38c32b2` and is already pushed to `origin/main`.
- AT-2026-05-17-214 final commit is `c92be25` and is already pushed to `origin/main`.
- AT-2026-05-17-215 final commit is `4ef3f10` and is already pushed to `origin/main`.
- AT-2026-05-17-216 final commit is `1094c10` and is already pushed to `origin/main`.
- AT-2026-05-17-217 final commit is `37765ef` and is already pushed to `origin/main`.
