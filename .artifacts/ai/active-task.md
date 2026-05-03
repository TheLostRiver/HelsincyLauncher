# Active Atomic Task

## Identity

- task id: AT-2026-05-03-033
- title: Register transport bootstrap smoke shell
- status: committed

## Goal

- exact local outcome: Implement the minimal E2 host bootstrap shell by replacing the placeholder host state with real `DesktopAppServices` wiring, moving shared command exports into `src-tauri/src/commands/mod.rs`, updating the testable bootstrap path in `lib.rs` and `main.rs`, and adding the named `transport_wiring_smoke` test.

## Scope

- in scope:
  - update `src-tauri/src/lib.rs`
  - update `src-tauri/src/bootstrap.rs`
  - update `src-tauri/src/main.rs`
  - update `src-tauri/src/state.rs`
  - add `src-tauri/src/commands/mod.rs`
  - add `src-tauri/tests/transport_wiring_smoke.rs`
  - update `.artifacts/ai` records for the E2 slice
- out of scope:
  - touching the user-owned frontend worktree changes
  - adding real Tauri runtime registration or window startup
  - modifying composition-root or module crates beyond consuming their public APIs

## Allowed Files

1. src-tauri/src/lib.rs
2. src-tauri/src/bootstrap.rs
3. src-tauri/src/main.rs
4. src-tauri/src/state.rs
5. src-tauri/src/commands/mod.rs
6. src-tauri/tests/transport_wiring_smoke.rs
7. .artifacts/ai/active-task.md
8. .artifacts/ai/task-plan.md
9. .artifacts/ai/progress.md
10. .artifacts/ai/findings.md

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
8. docs/TauriFirstCrateApiDrafts.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `src-tauri` routes `build_desktop_host_bootstrap()` through real `build_desktop_services()`, exposes a shared host state handle around `DesktopAppServices`, centralizes command names in `commands/mod.rs`, and adds a host smoke test that inspects registration plus calls one command handler, then `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without requiring a real Tauri runtime.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short -- .artifacts/ai src-tauri`

## Validation Result

- `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with one named host smoke test proving the bootstrap surface wires registered command names, shared state, and a real handler call path.
- `git diff --check` surfaced only the existing CRLF warnings from untouched user frontend files, and `git status --short -- .artifacts/ai src-tauri` confirmed this slice contains only the expected E2 host files plus AT-033 record updates.

## 需要更新的文档和日志

1. src-tauri/src/lib.rs
2. src-tauri/src/bootstrap.rs
3. src-tauri/src/main.rs
4. src-tauri/src/state.rs
5. src-tauri/src/commands/mod.rs
6. src-tauri/tests/transport_wiring_smoke.rs
7. .artifacts/ai/active-task.md
8. .artifacts/ai/task-plan.md
9. .artifacts/ai/progress.md
10. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Register transport bootstrap smoke shell
2. push command plan: git push

## 停止条件

1. E2 requires touching unrelated frontend files or adding real Tauri runtime machinery to pass the smoke test
2. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke` fails for reasons outside the E2 host file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the validated E2 host files plus the AT-033 records, commit the slice, then choose the next post-E2 validation or integration step while leaving user frontend edits untouched.