# Active Atomic Task

## Identity

- task id: AT-2026-05-04-052
- title: DownloadJobDriver real restore — checkpoint verification
- status: completed

## Goal

让 downloads 的 stage-2 restore 不再无条件返回 Resumed，而是显式读取持久化 checkpoint：
- 有 checkpoint 时保持可恢复
- 缺 checkpoint 时返回 FailedAsUnrecoverable，由 startup pipeline 落成 Failed

## Scope

- in scope:
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - expose a minimal `DownloadCheckpointRepository` boundary in `launcher-module-downloads`
  - implement sqlite-backed checkpoint load/save in `launcher-adapter-storage-sqlite`
  - wire `DownloadJobDriver` to the checkpoint repository in composition-root
  - add narrow restore tests for missing and present download checkpoints
- out of scope:
  - full segmented checkpoint payload design
  - real staging-file verification
  - download execution runtime
  - frontend UI changes
  - AT-053 and AT-054 remediation

## Allowed Files

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. crates/module-downloads/src/driver.rs
5. crates/module-downloads/src/lib.rs
6. crates/adapter-storage-sqlite/src/lib.rs
7. crates/composition-root/src/bootstrap.rs
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

- falsifiable local hypothesis: If `DownloadJobDriver` loads a persisted checkpoint through a repository before returning `RestoreDisposition::Resumed`, then stage-2 restore will keep queued download jobs resumable only when checkpoint state exists and will otherwise mark them failed through the existing startup pipeline path.

## Cheap Check

- `cargo test -p launcher-composition-root stage2_restore_marks_download_job_failed_without_checkpoint --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`

## Validation Gate

1. `cargo test -p launcher-module-downloads --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `cargo test -p launcher-composition-root bootstrap_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
3. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
4. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- passed

## 安全恢复点

- AT-052 已完成并验证；下一步等待用户确认是否继续处理 AT-053 / AT-054。