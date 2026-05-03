# Active Atomic Task

## Identity

- task id: AT-2026-05-03-009
- title: Bootstrap first Cargo workspace member
- status: committed

## Goal

- exact local outcome: Create the first valid Rust workspace entry for the current repo by pairing the root `Cargo.toml` with a minimal `src-tauri` member stub that makes the documented metadata gate real.

## Scope

- in scope:
  - add the root `Cargo.toml` workspace manifest for backend skeleton Phase A kickoff
  - add the smallest `src-tauri` package stub needed for `cargo metadata` to treat the workspace as valid
  - update `.artifacts/ai` records so the repo workflow explicitly tracks the backend skeleton kickoff
  - surface the A1 documentation gap exposed by Cargo's validation behavior
- out of scope:
  - adding `tauri.conf.json`, `main.rs`, `bootstrap.rs`, or `state.rs`
  - adding any crate under `crates/`
  - implementing any backend service, module, or adapter code

## Allowed Files

1. Cargo.toml
2. src-tauri/Cargo.toml
3. src-tauri/src/lib.rs
4. docs/TauriBackendSkeletonImplementationDesign.md
5. .artifacts/ai/**

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
7. docs/TauriCompositionRootWiringDesign.md
8. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md
11. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: Cargo requires at least one real workspace member with a target for the documented metadata gate to pass, so pairing the root workspace manifest with a minimal `src-tauri` library stub should make `cargo metadata --format-version 1` succeed without dragging in broader Tauri host wiring.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo metadata --format-version 1 --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` after creating the root manifest.

## Validation Gate

1. `cargo metadata --format-version 1 --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git diff --check`

## Validation Result

- `cargo metadata --format-version 1 --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed after the root workspace was bridged to a minimal `src-tauri` lib stub.
- The repair also confirmed the doc gap: the metadata gate is not valid for a zero-member virtual workspace.

## 需要更新的文档和日志

1. Cargo.toml
2. src-tauri/Cargo.toml
3. src-tauri/src/lib.rs
4. docs/TauriBackendSkeletonImplementationDesign.md
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap first Cargo workspace member
2. push command plan: git push

## 停止条件

1. missing or conflicting docs about A1 workspace shape
2. `cargo metadata` still fails after adding the minimal `src-tauri` member stub
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: open the next host bootstrap slice to add `tauri.conf.json`, `main.rs`, `bootstrap.rs`, and `state.rs` on top of the validated `src-tauri` member stub.