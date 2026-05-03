# Active Atomic Task

## Identity

- task id: AT-2026-05-03-026
- title: Bootstrap fab provider adapter crate
- status: committed

## Goal

- exact local outcome: Create the first `launcher-adapter-provider-fab` crate shell, wire it into the root workspace, and expose only the minimal Fab provider adapter config plus constructor surface that the controlling docs require, while keeping remote auth, HTTP transport, and payload mapping out of scope.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/adapter-provider-fab/Cargo.toml`
  - add `crates/adapter-provider-fab/src/lib.rs`
  - update `.artifacts/ai` records for the C4 slice
- out of scope:
  - adding real provider auth, HTTP client wiring, or response mapping
  - adding composition-root wiring or module-owned port traits
  - touching `src-tauri` or storage adapters

## Allowed Files

1. Cargo.toml
2. crates/adapter-provider-fab/Cargo.toml
3. crates/adapter-provider-fab/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

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
5. docs/TauriBackendSkeletonImplementationDesign.md
6. docs/TauriFirstCrateApiDrafts.md
7. docs/TauriFabInventoryLoadingDesign.md
8. docs/TauriRepositoryPortsAndAdapterDesign.md
9. docs/TauriCompositionRootWiringDesign.md
10. .github/copilot-instructions.md
11. .github/skills/strict-doc-driven-development/SKILL.md
12. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-adapter-provider-fab` is introduced as a workspace member that exports only provider adapter config plus the minimal `EpicFabCatalogProviderAdapter` constructor surface, then `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling remote auth, HTTP transport, or payload mapping details into the first provider adapter stub.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the minimal Fab provider adapter config and constructor shell in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected workspace-member `Cargo.lock` update.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/adapter-provider-fab/Cargo.toml
3. crates/adapter-provider-fab/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap fab provider adapter crate
2. push command plan: git push

## 停止条件

1. the slice starts introducing remote auth, HTTP client wiring, or payload mapping behavior into `launcher-adapter-provider-fab`
2. `cargo check -p launcher-adapter-provider-fab` fails for reasons outside the C4 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the C4 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.