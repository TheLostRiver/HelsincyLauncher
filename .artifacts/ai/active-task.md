# Active Atomic Task

## Identity

- task id: AT-2026-05-03-030
- title: Wire composition root smoke shell
- status: committed

## Goal

- exact local outcome: Implement the minimal D2 composition-root wiring shell by moving the public API into `bootstrap.rs` and `startup.rs`, making `build_desktop_services()` return assembled desktop facades without triggering startup side effects, and adding the named `bootstrap_wiring_smoke` test.

## Scope

- in scope:
  - update `crates/composition-root/src/lib.rs`
  - add `crates/composition-root/src/bootstrap.rs`
  - add `crates/composition-root/src/startup.rs`
  - add `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
  - update `.artifacts/ai` records for the D2 slice
- out of scope:
  - wiring `src-tauri` to the real composition-root services
  - adding real runtime restore, prewarm, SQL, or provider IO behavior
  - changing module or adapter public APIs

## Allowed Files

1. Cargo.lock
2. crates/composition-root/src/lib.rs
3. crates/composition-root/src/bootstrap.rs
4. crates/composition-root/src/startup.rs
5. crates/composition-root/tests/bootstrap_wiring_smoke.rs
6. .artifacts/ai/active-task.md
7. .artifacts/ai/task-plan.md
8. .artifacts/ai/progress.md
9. .artifacts/ai/findings.md

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
7. docs/TauriFirstCrateApiDrafts.md
8. docs/TauriKernelJobsRuntimeDesign.md
9. .github/copilot-instructions.md
10. .github/skills/strict-doc-driven-development/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-composition-root` assembles concrete storage/provider adapters into the existing Fab and Downloads facades inside `build_desktop_services()`, keeps startup stage methods explicit no-ops, and adds a direct smoke test that only checks assembly, then `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling real runtime side effects into D2.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`
3. `git -C q:\DEV\MyEpicLauncher status --short`

## Validation Result

- `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with one integration smoke test proving `build_desktop_services()` assembles the Fab, Downloads, and Startup facade handles.
- `git diff --check` passed, and `git status --short` showed only the expected D2 code files plus AT-030 record updates before commit.

## 需要更新的文档和日志

1. crates/composition-root/src/lib.rs
2. crates/composition-root/src/bootstrap.rs
3. crates/composition-root/src/startup.rs
4. crates/composition-root/tests/bootstrap_wiring_smoke.rs
5. .artifacts/ai/active-task.md
6. .artifacts/ai/task-plan.md
7. .artifacts/ai/progress.md
8. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Wire composition root smoke shell
2. push command plan: git push

## 停止条件

1. D2 requires widening into `src-tauri` transport wiring or adding real runtime side effects to pass the smoke test
2. the composition-root smoke test fails for reasons outside the D2 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: stage the validated D2 composition-root files plus the AT-030 record files, commit the slice, then start the Phase E transport task from the clean post-D2 baseline.