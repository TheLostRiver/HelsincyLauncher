# Active Atomic Task

## Identity

- task id: AT-2026-05-03-035
- title: Add contributor collaboration guide
- status: committed

## Goal

- exact local outcome: Add a root `CONTRIBUTING.md` that gives contributors a fast current-repo entry layer for boundaries, commands, validation, risk hotspots, and documentation responsibilities, and expose that guide from `README.md`.

## Scope

- in scope:
  - add `CONTRIBUTING.md`
  - update `README.md` for contributor-guide discoverability
  - update `.artifacts/ai` records for the AT-035 docs slice
- out of scope:
  - changing backend Rust/Tauri implementation code
  - adding the separate current-repo architecture overview from benchmark P2
  - touching user-owned frontend worktree changes

## Allowed Files

1. CONTRIBUTING.md
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
4. docs/TauriAIDevelopmentTransactionProtocolDesign.md
5. docs/TauriTestingStrategyAndQualityGateDesign.md
6. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
7. README.md
8. package.json
9. Cargo.toml

## Hypothesis

- falsifiable local hypothesis: If the repo adds one root `CONTRIBUTING.md` that maps current-repo boundaries, commands, validation gates, risk hotspots, and document ownership without duplicating the deep architecture docs, and if `README.md` links to it explicitly, then the benchmarked P1 gap will be materially closed without weakening the strict-doc protocol.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify `CONTRIBUTING.md` exists, `README.md` links to it, and the guide contains explicit sections for current-repo boundaries, minimal validation, high-risk files, and documentation ownership.

## Validation Gate

1. grep search for the `CONTRIBUTING.md` link and required section headings
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai README.md CONTRIBUTING.md`

## Validation Result

- `README.md` grep checks passed: the contributor guide is linked from the workflow/governance section and explicitly called out in the notes section for contributors.
- `CONTRIBUTING.md` grep checks passed: the guide contains explicit sections for current-repo boundaries, minimal validation, high-risk files, and documentation ownership.
- `git diff --check -- CONTRIBUTING.md README.md .artifacts/ai` produced no blocking output for the AT-035 docs slice.

## 需要更新的文档和日志

1. CONTRIBUTING.md
2. README.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Add contributor collaboration guide
2. push command plan: git push

## 停止条件

1. the contributor guide requires rewriting multiple deep design docs instead of adding an entry-layer surface
2. the guide cannot stay accurate without broad implementation edits outside the allowed files
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage `CONTRIBUTING.md`, `README.md`, and the AT-035 record updates, commit the docs slice, then decide whether to continue into benchmark P2 or return to backend integration.