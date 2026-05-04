# Active Atomic Task

## Identity

- task id: AT-2026-05-04-056
- title: Standalone code comment standard
- status: completed

## Goal

把仓库级注释规范写成独立文档，并明确：
- 哪些声明默认应有注释，哪些明显代码不应硬写注释
- TypeScript/TSX 与 Rust 分别采用什么注释语法与边界
- 函数体内重点注释与多线程/并发高风险注释的强制规则
- 让这份规范可以从 `docs/README.md` 被发现，而不是只停留在对话里

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - add `docs/TauriCodeCommentStandard.md`
  - update `docs/README.md` to route the new standalone standard
- out of scope:
  - retrofit comments across existing source files
  - add lint rules, CI checks, or code generation for comment enforcement
  - rewrite module docs to match the new comment standard
  - update frontend or backend implementation code

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md
5. docs/TauriCodeCommentStandard.md
6. docs/README.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/README.md
6. docs/ModuleDocumentationStandard.md

## Hypothesis

- falsifiable local hypothesis: If a standalone `docs/TauriCodeCommentStandard.md` is added and routed from `docs/README.md`, and that standard explicitly separates declaration comments, selective function-body comments, Rust public-surface Rustdoc usage, and stricter concurrency annotation rules, then the repository gains one discoverable comment-policy source without forcing blanket rustdoc or low-signal comment noise.

## Cheap Check

- `rg "TauriCodeCommentStandard|Doxygen|Rustdoc|并发|函数体" docs/TauriCodeCommentStandard.md docs/README.md`

## Validation Gate

1. `rg "TauriCodeCommentStandard|Doxygen|Rustdoc|并发|函数体" docs/TauriCodeCommentStandard.md docs/README.md`
2. `git -C q:\DEV\MyEpicLauncher diff --check -- .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md docs/TauriCodeCommentStandard.md docs/README.md`

## Validation Result

- passed

## 安全恢复点

- AT-056 已完成并通过 docs-only 窄验证；仓库现在已有独立注释规范文档和 docs 路由，Git 发布路径已缩小到可工作的异步终端会话。
