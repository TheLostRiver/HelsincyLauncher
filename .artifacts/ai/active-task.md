# Active Atomic Task

## Identity

- task id: AT-2026-05-04-054
- title: Engine verification accepted-job wiring
- status: completed

## Goal

让 engines verification 走通 backend-owned accepted-job 链路，而不是停留在 `ENGINES_NOT_WIRED` stub：
- `EngineFacade::run_verification()` 返回 `AcceptedJob`
- composition-root 暴露 engines facade
- host transport 注册并调用 engines verification command

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - make `EngineFacade::run_verification()` create a backend-owned accepted job
  - expose engines from `DesktopAppServices`
  - add a host command for engine verification and register it
  - add narrow module, composition-root, and transport tests for the accepted-job path
- out of scope:
  - engine list/status query wiring
  - repair planning and repair apply flows
  - engine verification checkpoint persistence
  - frontend UI changes
  - deeper engine restore logic in `EngineJobDriver`

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. crates/module-engines/src/facade/mod.rs
5. crates/composition-root/src/bootstrap.rs
6. crates/composition-root/tests/bootstrap_wiring_smoke.rs
7. src-tauri/Cargo.toml
8. src-tauri/src/commands/mod.rs
9. src-tauri/src/commands/engines.rs
10. src-tauri/tests/transport_wiring_smoke.rs

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriEngineVerificationRepairDesign.md
3. docs/TauriIPCAndStateContractsDesign.md
4. docs/TauriFirstCrateApiDrafts.md
5. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
6. docs/TauriKernelJobsRuntimeDesign.md
7. docs/TauriCompositionRootWiringDesign.md
9. docs/TauriTestingStrategyAndQualityGateDesign.md

## Hypothesis

- falsifiable local hypothesis: If `EngineFacade::run_verification()` enqueues an `engines/verification` job and the composition root plus host command surface both expose that facade, then a narrow module test, a composition-root integration test, and the transport smoke test will all observe a backend-owned `AcceptedJob` instead of `ENGINES_NOT_WIRED` or a missing command path.

## Cheap Check

- `cargo test -p launcher-module-engines run_verification_returns_backend_owned_accepted_job --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. `cargo test -p launcher-module-engines --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root engines_run_verification_enqueues_job --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- passed

## 安全恢复点

- AT-054 已完成并验证；下一步如果继续，就是更宽的 engines repair / query 路径或回头处理遗留文档漂移。