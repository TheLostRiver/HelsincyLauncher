# Active Atomic Task

## Identity

- task id: AT-2026-05-15-160
- title: Add downloads resume manifest provider boundary
- status: complete

## Goal

在 `resume_download` 已经完成 job lookup、checkpoint lookup 和 staging validation 之后，补上最小 manifest provider 端口边界，并证明 facade 会在 runtime enqueue 之前重建或读取 manifest plan。

本轮只覆盖：

- `DownloadManifestProviderPort` 的最小 trait 边界
- `DownloadManifestPlan` 的最小句柄
- `resume_download` 在 staging 之后调用 manifest provider
- 保持完整 runtime resume enqueue 仍为后续切片

## Scope

- in scope:
  - update `crates/module-downloads/src/facade/mod.rs`
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change frontend files
  - change host transport or composition-root wiring
  - implement concrete provider/adapter behavior
  - enqueue resumed runtime jobs
  - change sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. crates/module-downloads/src/facade/mod.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/README.md
4. docs/modules/downloads/README_ARCH.md
5. docs/modules/downloads/README_API.md
6. docs/modules/downloads/README_FLOW.md
7. docs/modules/downloads/README_IMPL.md
8. docs/TauriDownloadRuntimeDesign.md
9. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
10. docs/TauriFirstCrateApiDrafts.md
11. docs/TauriKernelJobsRuntimeDesign.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md
14. docs/TauriCodeCommentStandard.md

## Hypothesis

- falsifiable local hypothesis: If `resume_download` has a manifest provider port, a focused facade test can prove the flow reaches manifest reconstruction only after module job record, checkpoint, and staging are present, while the operation still returns `DOWNLOADS_NOT_WIRED` until runtime resume enqueue is explicitly designed.

## Cheap Check

- `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download_reconstructs_manifest_after_staging_is_valid`

## Validation Gate

1. Read README, collaboration docs, downloads module docs, `README_IMPL.md`, and related backend/runtime/testing docs before code.
2. Write a RED facade test for manifest reconstruction after staging.
3. Implement the smallest port/plan boundary and keep `()` placeholder compatibility.
4. Run focused resume test, full `launcher-module-downloads` tests, and scoped diff checks.

## Validation Result

- passed
- RED compile failure was observed first: the focused test failed because `DownloadManifestPlan` and `DownloadManifestProviderPort` did not exist.
- Focused GREEN test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml resume_download_reconstructs_manifest_after_staging_is_valid` reported 1 passed, 0 failed.
- Full module test passed: `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml` reported 10 passed, 0 failed.
- Scoped whitespace validation passed: `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- crates/module-downloads/src/facade/mod.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`.

## Notes

- AT-2026-05-15-159 completed and was committed locally as `c6c6f44`.
- User requires module docs under `docs/modules` and implementation docs to be read before module backend code.
- User prefers preserving existing English comments and adding Chinese companions; new code comments in this slice should be bilingual where declaration-level comments are added.
