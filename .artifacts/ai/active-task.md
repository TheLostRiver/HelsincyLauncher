# Active Atomic Task

## Identity

- task id: AT-2026-05-03-022
- title: Bootstrap module downloads crate
- status: committed

## Goal

- exact local outcome: Create the first `launcher-module-downloads` crate shell, wire it into the root workspace, and expose only the public `contracts` plus `DownloadFacade` boundary that the controlling docs require, while keeping scheduler, checkpoint, and manifest-provider internals out of scope.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/module-downloads/Cargo.toml`
  - add `crates/module-downloads/src/lib.rs`
  - add minimal public contract files under `crates/module-downloads/src/contracts/`
  - add `crates/module-downloads/src/facade/mod.rs`
  - update `.artifacts/ai` records for the C2 slice
- out of scope:
  - adding application, ports, or runtime scheduler implementations
  - adding real checkpoint, manifest, staging, or transport behavior
  - touching adapter crates, composition-root, or `src-tauri`

## Allowed Files

1. Cargo.toml
2. crates/module-downloads/Cargo.toml
3. crates/module-downloads/src/lib.rs
4. crates/module-downloads/src/contracts/mod.rs
5. crates/module-downloads/src/contracts/dto.rs
6. crates/module-downloads/src/contracts/queries.rs
7. crates/module-downloads/src/contracts/commands.rs
8. crates/module-downloads/src/contracts/events.rs
9. crates/module-downloads/src/facade/mod.rs
10. .artifacts/ai/active-task.md
11. .artifacts/ai/task-plan.md
12. .artifacts/ai/progress.md
13. .artifacts/ai/findings.md

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
7. docs/TauriDownloadRuntimeDesign.md
8. docs/TauriIPCAndStateContractsDesign.md
9. docs/modules/downloads/README_ARCH.md
10. docs/modules/downloads/README_API.md
11. docs/modules/downloads/README_FLOW.md
12. .github/copilot-instructions.md
13. .github/skills/strict-doc-driven-development/SKILL.md
14. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-module-downloads` is introduced as a workspace member that exports only small public contracts and a placeholder `DownloadFacade` over generic dependencies, then `cargo check -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling scheduler, checkpoint, or manifest-provider details into the module boundary.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the minimal downloads contracts and facade shell in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected workspace-member `Cargo.lock` update.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/module-downloads/Cargo.toml
3. crates/module-downloads/src/lib.rs
4. crates/module-downloads/src/contracts/mod.rs
5. crates/module-downloads/src/contracts/dto.rs
6. crates/module-downloads/src/contracts/queries.rs
7. crates/module-downloads/src/contracts/commands.rs
8. crates/module-downloads/src/contracts/events.rs
9. crates/module-downloads/src/facade/mod.rs
10. .artifacts/ai/active-task.md
11. .artifacts/ai/task-plan.md
12. .artifacts/ai/progress.md
13. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap module downloads crate
2. push command plan: git push

## 停止条件

1. the slice starts introducing scheduler, checkpoint, or manifest-provider business internals into `launcher-module-downloads`
2. `cargo check -p launcher-module-downloads` fails for reasons outside the C2 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the C2 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.