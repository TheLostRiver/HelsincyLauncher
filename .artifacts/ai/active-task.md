# Active Atomic Task

## Identity

- task id: AT-2026-05-04-055
- title: Composition-root engines doc drift cleanup
- status: completed

## Goal

让 `docs/TauriCompositionRootWiringDesign.md` 与当前 composition-root 基线重新一致：
- 不再把 `module-engines` 记录为 deferred
- `DesktopAppServices` / module bundle / smoke-test 矩阵体现 engines facade
- 同时保留“startup pipeline 目前未直接编排 engines”这一边界

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `docs/TauriCompositionRootWiringDesign.md` to reflect the current engines wiring baseline
  - align `Current Wiring Scope`, `DesktopAppServices`, wiring sequence, module bundle, and smoke-test text with the live implementation
  - keep startup ownership boundaries explicit so the doc does not imply engines startup orchestration already exists
- out of scope:
  - further code changes in `launcher-composition-root`
  - engines list/status/repair feature work
  - changing the stricter facade-only architecture rule to bless every current implementation detail
  - frontend UI changes

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. docs/TauriCompositionRootWiringDesign.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriCompositionRootWiringDesign.md
3. docs/TauriEngineVerificationRepairDesign.md
4. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
5. docs/TauriKernelJobsRuntimeDesign.md
6. docs/TauriTestingStrategyAndQualityGateDesign.md

## Hypothesis

- falsifiable local hypothesis: If `docs/TauriCompositionRootWiringDesign.md` is updated so its scope table, service sketches, wiring sequence, and smoke-test matrix all acknowledge the already-wired engines facade while keeping startup-pipeline ownership narrow, then the stale “module-engines is deferred” guidance disappears without implying that engines startup orchestration already exists.

## Cheap Check

- `rg "launcher-module-engines|pub engines: Arc<EngineFacade|build_engines_module|四个 facade 都存在" docs/TauriCompositionRootWiringDesign.md`

## Validation Gate

1. `rg "launcher-module-engines|pub engines: Arc<EngineFacade|build_engines_module|四个 facade 都存在" docs/TauriCompositionRootWiringDesign.md`
2. `git -C q:\DEV\MyEpicLauncher diff --check -- docs/TauriCompositionRootWiringDesign.md`

## Validation Result

- passed

## 安全恢复点

- AT-055 已完成并验证；当前 composition-root 文档已承认 engines facade 基线，但 startup pipeline 仍未接入 engines 编排，这个边界保持显式。