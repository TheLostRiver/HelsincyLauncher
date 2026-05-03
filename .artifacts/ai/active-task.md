# Active Atomic Task

## Identity

- task id: AT-2026-05-03-028
- title: Bootstrap composition root crate
- status: committed

## Goal

- exact local outcome: Create the first `launcher-composition-root` crate shell, wire it into the root workspace, and expose only the public composition-root API shell that the controlling docs require, while keeping real bootstrap wiring, concrete assembly, and smoke tests out of scope.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/composition-root/Cargo.toml`
  - add `crates/composition-root/src/lib.rs`
  - update `.artifacts/ai` records for the D1 slice
- out of scope:
  - implementing real bootstrap wiring, runtime assembly, or smoke tests
  - touching `src-tauri` commands or module/adapter internals
  - adding `bootstrap.rs` or `startup.rs` before D2

## Allowed Files

1. Cargo.toml
2. crates/composition-root/Cargo.toml
3. crates/composition-root/src/lib.rs
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
6. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
7. docs/TauriFirstCrateApiDrafts.md
8. docs/TauriCompositionRootWiringDesign.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md
11. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-composition-root` is introduced as a workspace member that exports only `DesktopBootstrapConfig`, `DesktopAppServices`, `StartupPipelineFacade`, and a placeholder `build_desktop_services()`, then `cargo check -p launcher-composition-root --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling real wiring logic or concrete assembly into D1.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-composition-root --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-composition-root --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-composition-root --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the composition-root public API shell in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected workspace-member `Cargo.lock` update.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/composition-root/Cargo.toml
3. crates/composition-root/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap composition root crate
2. push command plan: git push

## 停止条件

1. the slice starts introducing real wiring, bootstrap logic, or concrete assembly into `launcher-composition-root`
2. `cargo check -p launcher-composition-root` fails for reasons outside the D1 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the D1 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.