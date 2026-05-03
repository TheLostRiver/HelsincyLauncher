# Active Atomic Task

## Identity

- task id: AT-2026-05-03-036
- title: Add current-repo architecture overview
- status: committed

## Goal

- exact local outcome: Add a current-repo architecture overview document that helps contributors quickly understand the live repo shape, key entrypoints, request/bootstrapping chain, structural hotspots, and suggested landing zones, then expose it from `README.md`.

## Scope

- in scope:
  - add `docs/TauriCurrentRepoArchitectureOverview.md`
  - update `README.md` for current-repo architecture discoverability
  - update `.artifacts/ai` records for the AT-036 docs slice
- out of scope:
  - changing backend Rust/Tauri implementation code
  - rewriting the blueprint or deep architecture principle docs
  - touching user-owned frontend worktree changes

## Allowed Files

1. docs/TauriCurrentRepoArchitectureOverview.md
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
4. docs/TauriRewriteArchitectureBlueprint.md
5. docs/TauriArchitecturePrinciplesDesign.md
6. README.md
7. src-tauri/src/lib.rs
8. src-tauri/src/bootstrap.rs
9. src-tauri/src/commands/mod.rs
10. crates/composition-root/src/bootstrap.rs
11. crates/composition-root/src/startup.rs
12. src-tauri/tests/transport_wiring_smoke.rs

## Hypothesis

- falsifiable local hypothesis: If the repo adds one current-repo architecture overview that maps the live directory duties, key entrypoints, host-to-composition bootstrap chain, structural hotspots, and suggested landing zones, and if `README.md` links to it explicitly, then the benchmarked P2 gap will be materially closed without duplicating the blueprint.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Verify the new overview doc exists, `README.md` links to it, and the doc contains explicit sections for current repo shape, key entrypoints, runtime/boot chain, and landing-zone guidance.

## Validation Gate

1. grep search for the new architecture-overview link and required section headings
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai README.md docs`

## Validation Result

- `README.md` grep checks passed: the current-repo architecture overview is linked from the architecture section.
- `docs/TauriCurrentRepoArchitectureOverview.md` grep checks passed: the document contains explicit sections for current repo shape, key entrypoints, startup/call chain, and landing-zone guidance.
- `git diff --check -- README.md docs/TauriCurrentRepoArchitectureOverview.md .artifacts/ai` produced no blocking output for the AT-036 docs slice.

## 需要更新的文档和日志

1. docs/TauriCurrentRepoArchitectureOverview.md
2. README.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Add current-repo architecture overview
2. push command plan: git push

## 停止条件

1. the overview requires rewriting deep design docs instead of mapping the current repo shape
2. the overview cannot stay accurate without broad implementation edits outside the allowed files
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage `docs/TauriCurrentRepoArchitectureOverview.md`, `README.md`, and the AT-036 record updates, commit the docs slice, then decide whether to continue into benchmark P3 or return to backend integration.