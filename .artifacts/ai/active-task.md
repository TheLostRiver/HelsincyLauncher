# Active Atomic Task

## Identity

- task id: AT-2026-05-03-037
- title: Add docs map overview
- status: committed

## Goal

- exact local outcome: Add `docs/README.md` as a docs map that explains which documents own which topics, what should stay out of `README.md`, and how to navigate principles, topic docs, current-repo docs, and module docs, then expose that map from `README.md`.

## Scope

- in scope:
  - add `docs/README.md`
  - update `README.md` for docs-map discoverability
  - update `.artifacts/ai` records for the AT-037 docs slice
- out of scope:
  - changing backend Rust/Tauri implementation code
  - rewriting the existing deep architecture/topic docs
  - touching user-owned frontend worktree changes

## Allowed Files

1. docs/README.md
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
3. docs/TauriDocumentationBenchmarkAgainstCodexManager.md
4. README.md
5. CONTRIBUTING.md
6. docs/TauriRewriteArchitectureBlueprint.md
7. docs/TauriArchitecturePrinciplesDesign.md
8. docs/TauriCurrentRepoArchitectureOverview.md
9. docs/ModuleDocumentationStandard.md

## Hypothesis

- falsifiable local hypothesis: If the repo adds one `docs/README.md` that groups the current documentation into principles, topic docs, current-repo entry docs, governance docs, and module docs, and if `README.md` links to it explicitly, then the benchmarked P3 gap will be materially closed without forcing more routing guidance back into the root README.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify `docs/README.md` exists, `README.md` links to it, and the map contains explicit sections for doc ownership layers, “what not to put back in README”, and update-routing guidance.

## Validation Gate

1. grep search for the new docs-map link and required section headings
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai README.md docs`

## Validation Result

- `README.md` grep checks passed: the docs map is linked from the workflow/governance section.
- `docs/README.md` grep checks passed: the map contains explicit sections for doc layering, what should stay out of `README.md`, and update-routing guidance.
- `git diff --check -- README.md docs/README.md .artifacts/ai` produced no blocking output for the AT-037 docs slice.

## 需要更新的文档和日志

1. docs/README.md
2. README.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Add docs map overview
2. push command plan: git push

## 停止条件

1. the docs map requires rewriting multiple existing docs instead of routing between them
2. the map cannot stay accurate without broad implementation edits outside the allowed files
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage `docs/README.md`, `README.md`, and the AT-037 record updates, commit the docs slice, then decide whether to continue broader documentation governance or return to backend integration.