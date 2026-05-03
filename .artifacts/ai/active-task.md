# Active Atomic Task

## Identity

- task id: AT-2026-05-03-011
- title: Add host bootstrap surface
- status: committed

## Goal

- exact local outcome: Extend `src-tauri` from a bare workspace member stub into a thin host shell with `tauri.conf.json`, `main.rs`, `bootstrap.rs`, `state.rs`, and an updated manifest that can pass `cargo check -p my-epic-launcher-desktop` without pulling in composition-root yet.

## Scope

- in scope:
  - update `src-tauri/Cargo.toml` so the host crate has an explicit binary target alongside the existing library target
  - add `src-tauri/tauri.conf.json`
  - add `src-tauri/src/bootstrap.rs`, `src-tauri/src/state.rs`, and `src-tauri/src/main.rs`
  - upgrade `src-tauri/src/lib.rs` into a small public bootstrap surface with a crate-local placeholder service handle
  - update `.artifacts/ai` records for this slice
- out of scope:
  - adding real Tauri command registration or `DesktopAppServices` from composition-root
  - adding `build.rs`, host smoke tests, or any crate under `crates/`
  - implementing real backend logic behind the host surface

## Allowed Files

1. src-tauri/Cargo.toml
2. src-tauri/tauri.conf.json
3. src-tauri/src/lib.rs
4. src-tauri/src/bootstrap.rs
5. src-tauri/src/state.rs
6. src-tauri/src/main.rs
7. .artifacts/ai/**

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
6. docs/TauriCompositionRootWiringDesign.md
7. docs/TauriDevelopmentEnvironmentBootstrapDesign.md
8. .github/copilot-instructions.md
9. .github/skills/strict-doc-driven-development/SKILL.md
10. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `src-tauri` gets a thin crate-local bootstrap surface, placeholder shared state handle, explicit binary target, and config stub, then `cargo check -p my-epic-launcher-desktop` will pass without requiring composition-root or real Tauri transport wiring yet.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` after adding the host shell files.

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git diff --check`

## Validation Result

- `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the thin host shell files in place.
- The host crate is still crate-local and placeholder-based; it does not yet depend on composition-root or real backend crates.

## 需要更新的文档和日志

1. src-tauri/Cargo.toml
2. src-tauri/tauri.conf.json
3. src-tauri/src/lib.rs
4. src-tauri/src/bootstrap.rs
5. src-tauri/src/state.rs
6. src-tauri/src/main.rs
7. .artifacts/ai/active-task.md
8. .artifacts/ai/task-plan.md
9. .artifacts/ai/progress.md
10. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Add host bootstrap surface
2. push command plan: git push

## 停止条件

1. the host shell starts depending on composition-root or real backend crates prematurely
2. `cargo check -p my-epic-launcher-desktop` fails because the host shell still lacks required crate-local surfaces
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: choose the next Phase 5 slice between host-side smoke wiring and the first kernel crate bootstrap.