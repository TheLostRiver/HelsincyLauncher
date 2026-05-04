# Active Atomic Task

## Identity

- task id: AT-2026-05-04-053
- title: StartDownload request propagation and persistence
- status: completed

## Goal

让 downloads 的 start_download 不再丢弃请求内容，而是：
- 使用 request.priority 进入 runtime enqueue
- 把 target_id / kind / install_intent 持久化进 download job repository

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - expose a minimal `DownloadJobRepository` boundary in `launcher-module-downloads`
  - persist start request metadata through the sqlite downloads repository
  - honor `StartDownloadRequestDto.priority` when enqueueing the job runtime request
  - add narrow tests for module-level request propagation and composition-root persistence
- out of scope:
  - manifest resolution and real download execution
  - download snapshot extension payload redesign
  - pause / cancel / resume use case wiring
  - frontend UI changes
  - AT-054 remediation

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. crates/module-downloads/src/driver.rs
5. crates/module-downloads/src/facade/mod.rs
6. crates/module-downloads/src/lib.rs
7. crates/adapter-storage-sqlite/src/lib.rs
8. crates/composition-root/tests/bootstrap_wiring_smoke.rs

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriDownloadRuntimeDesign.md
3. docs/TauriRepositoryPortsAndAdapterDesign.md
4. docs/TauriFirstCrateApiDrafts.md
5. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
6. docs/TauriStartupPipelineDesign.md
7. docs/TauriKernelJobsRuntimeDesign.md
8. docs/TauriCompositionRootWiringDesign.md
9. docs/TauriTestingStrategyAndQualityGateDesign.md

## Hypothesis

- falsifiable local hypothesis: If `DownloadFacade::start_download()` persists a download job record from the request before enqueueing, and forwards `request.priority` into the runtime request, then a narrow unit test and a composition-root integration test will both observe that the request is no longer silently discarded.

## Cheap Check

- `cargo test -p launcher-module-downloads start_download_persists_request_metadata_and_enqueue_priority --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. `cargo test -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root downloads_start_persists_request_metadata --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- passed

## 安全恢复点

- AT-053 已完成并验证；下一步可继续处理 AT-054。