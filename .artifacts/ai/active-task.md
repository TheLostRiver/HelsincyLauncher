# Active Atomic Task

## Identity

- task id: AT-2026-05-03-031
- title: Add transport facade command shell
- status: committed

## Goal

- exact local outcome: Implement the minimal E1 host transport command shell by adding `src-tauri` Fab and Downloads command modules, wiring them against `DesktopAppServices`, and providing thin result-envelope mappers so the desktop host crate can compile without starting the full E2 registration path.

## Scope

- in scope:
  - update `src-tauri/Cargo.toml`
  - update `src-tauri/src/lib.rs`
  - add `src-tauri/src/commands/fab.rs`
  - add `src-tauri/src/commands/downloads.rs`
  - update `.artifacts/ai` records for the E1 slice
- out of scope:
  - changing `src-tauri/src/bootstrap.rs`, `state.rs`, or `main.rs`
  - registering Tauri commands or shared state injection
  - adding host-level smoke tests or real Tauri runtime behavior

## Allowed Files

1. src-tauri/Cargo.toml
2. src-tauri/src/lib.rs
3. src-tauri/src/commands/fab.rs
4. src-tauri/src/commands/downloads.rs
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md

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
8. docs/TauriRustTsSchemaDesign.md
9. docs/TauriFirstCrateApiDrafts.md
10. .github/copilot-instructions.md
11. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `src-tauri` adds minimal Fab and Downloads command modules that take `DesktopAppServices`, map facade `AppResult` values into thin command/query envelopes, and keep any not-yet-wired read models as explicit stub fallbacks, then `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without widening into E2 registration or shared-state changes.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo check -p my-epic-launcher-desktop --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed after the host command shell consumed `DesktopAppServices` through thin result-envelope mappers.
- `git diff --check` passed. `git status --short` confirmed the E1 file set plus one adjacent `Cargo.lock` delta for the desktop crate dependency list; unrelated user frontend edits in `app/page.tsx`, `components/Sidebar.tsx`, and `components/EngineManagementContent.tsx` were detected and intentionally left untouched.

## 需要更新的文档和日志

1. src-tauri/Cargo.toml
2. src-tauri/src/lib.rs
3. src-tauri/src/commands/fab.rs
4. src-tauri/src/commands/downloads.rs
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Add transport facade command shell
2. push command plan: git push

## 停止条件

1. E1 requires changing host bootstrap, state injection, or `main.rs` to compile
2. `cargo check -p my-epic-launcher-desktop` fails for reasons outside the E1 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the validated E1 host command files plus the AT-031 records, then open one tiny cleanup slice to persist the adjacent desktop-host `Cargo.lock` delta before E2.