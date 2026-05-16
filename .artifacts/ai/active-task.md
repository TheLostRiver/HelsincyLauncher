# Active Atomic Task

## Identity

- task id: AT-2026-05-16-171
- title: Project downloads resume outcome through host transport
- status: completed

## Goal

把 AT-170 新增的 `DownloadResumeOutcome` 安全投影到 `src-tauri` host transport：`RuntimeAccepted` 继续返回 accepted-job 语义，`AlreadyComplete` 返回稳定的已完成 resume outcome，不再伪装成 `accepted: true`，也不再让 all-sealed resume 卡在 `DOWNLOADS_NOT_WIRED`。

本轮只覆盖后端 transport/outcome 边界：

- add a RED mapper/transport test first
- introduce a host-side resume outcome DTO
- switch `downloads_resume` to `resume_download_outcome`
- keep start/Fab/Engines accepted-job mappers unchanged
- keep segment details out of transport
- update README_IMPL implementation state

## Scope

- in scope:
  - `src-tauri/src/commands/mod.rs`
  - `src-tauri/src/commands/downloads.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - frontend files
  - SQLite schema or concrete segment persistence
  - concrete scheduler/fetch/write/verify execution
  - `kernel-jobs` segment payload or completion API changes
  - changing start/Fab/Engines accepted-job transport shape
  - exposing segment-level completion details
  - unrelated dirty worktree files

## Allowed Files

1. src-tauri/src/commands/mod.rs
2. src-tauri/src/commands/downloads.rs
3. docs/modules/downloads/README_IMPL.md
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md
8. .artifacts/ai/handoff.md

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
11. docs/TauriIPCAndStateContractsDesign.md
12. docs/TauriErrorHandlingAndProjectionDesign.md
13. docs/TauriTestingStrategyAndQualityGateDesign.md
14. docs/TauriAIDevelopmentTransactionProtocolDesign.md
15. current `src-tauri/src/commands/mod.rs` and `downloads.rs` snippets
16. current `crates/module-downloads/src/facade/mod.rs` outcome snippets

## Hypothesis

- falsifiable local hypothesis: a focused host command mapper test can force `DownloadResumeOutcome::AlreadyComplete` to map into a non-accepted host DTO while `RuntimeAccepted` still maps through accepted-job semantics, without changing frontend, persistence, scheduler, or kernel-jobs contracts.

## Cheap Check

- add a RED test for the host mapper and confirm it fails because no `DownloadResumeOutcomeDto` / mapper exists yet.

## Validation Gate

1. Read required root, docs index, module, runtime, IPC, error, testing, collaboration, README_IMPL, and current transport/facade snippets before editing Rust code.
2. Add RED mapper/transport test before production code.
3. Confirm RED fails for the expected missing transport outcome mapper.
4. Add only the minimal DTO/mapper and downloads handler change needed to pass.
5. Update README_IMPL current-state wording.
6. Run focused host test, downloads module test if touched, transport smoke, formatter/checks, and scoped `git diff --check`.

## Validation Result

- passed
- RED confirmed with focused compile failure: `maps_download_resume` failed because `DownloadResumeOutcomeDto` and `map_download_resume_outcome_result` did not exist.
- GREEN confirmed with focused mapper tests pass after adding the downloads resume outcome DTO/mapper and switching `downloads_resume` to `resume_download_outcome`.
- Focused host mapper validation passed: `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml maps_download_resume` reported 2 passed, 0 failed.
- Host transport smoke passed: `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` reported 1 passed, 0 failed.
- `cargo fmt --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml -p my-epic-launcher-desktop` completed successfully.
- Scoped `git diff --check` passed for host commands, README_IMPL, and PWF files with CRLF conversion warnings only.
- AT-171 is committed locally as a host transport/docs/PWF slice.

## Notes

- AT-2026-05-16-170 added module-owned `DownloadResumeOutcome`.
- Resume point: choose whether to continue downloads scheduler/driver payload design or add concrete adapter coverage for resume outcome branches.
- Direct `origin/main` push remains intentionally skipped without explicit approval.
