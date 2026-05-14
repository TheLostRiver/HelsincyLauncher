# Active Atomic Task

## Identity

- task id: AT-2026-05-15-150
- title: Add downloads control transport smoke coverage
- status: completed

## Goal

继续 Phase 28 Backend Development Scope Recovery，在 AT-149 已经把 downloads pause/cancel 接到 shared `JobRuntime` 控制端口之后，用宿主 transport smoke test 验证 start/pause/cancel 这条链路能从 `src-tauri` handler 穿到 composition-root 后端服务。

本轮只覆盖：

- `src-tauri/tests/transport_wiring_smoke.rs` downloads start/pause/cancel smoke coverage

## Scope

- in scope:
  - call `downloads_start` through the existing host smoke test and assert accepted-job projection
  - use the accepted `job_id` to call `downloads_pause`
  - call `downloads_cancel` through the same host transport path
  - update `.artifacts/ai/active-task.md`
  - update `.artifacts/ai/task-plan.md`
  - update `.artifacts/ai/progress.md`
  - update `.artifacts/ai/findings.md`
  - update `.artifacts/ai/handoff.md`
- out of scope:
  - change downloads production behavior
  - change `resume_download` semantics
  - touch frontend files, sqlite files, `Cargo.lock`, `.codex`, `src/`, or other unrelated dirty worktree files

## Allowed Files

1. src-tauri/tests/transport_wiring_smoke.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## 控制性文档

1. README.md
2. CONTRIBUTING.md
3. docs/TauriDownloadRuntimeDesign.md
4. docs/TauriKernelJobsRuntimeDesign.md
5. docs/TauriCurrentRepoArchitectureOverview.md
6. .artifacts/ai/task-plan.md

## Hypothesis

- falsifiable local hypothesis: If the host transport layer is correctly wired to the composition-root downloads facade and shared job runtime, then `transport_wiring_smoke` can start a downloads job and successfully call pause/cancel through `commands::downloads::*` without receiving `DOWNLOADS_NOT_WIRED`.

## Cheap Check

- `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke`

## Validation Gate

1. Run `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke`
2. Run `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- src-tauri/tests/transport_wiring_smoke.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md`

## Validation Result

- passed
- `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` passed with 1 integration test.
- `git -c safe.directory=D:/DEV/MyEpicLauncher diff --check -- src-tauri/tests/transport_wiring_smoke.rs .artifacts/ai/active-task.md .artifacts/ai/task-plan.md .artifacts/ai/progress.md .artifacts/ai/findings.md .artifacts/ai/handoff.md` passed; Git only reported Windows LF-to-CRLF working-copy warnings.

## Notes

- AT-2026-05-15-149 completed and was committed locally as `e774628`.
- Direct `origin/main` push remains blocked by safety review; per user rule, continue without bypassing push review.
- This is a verification-only backend host smoke slice. It intentionally leaves `resume_download` unchanged because its `AcceptedJob` return needs a separate resume-acceptance design.

## Completion Summary

- Added host transport smoke coverage proving downloads start returns an accepted job and that pause/cancel calls remain callable through `commands::downloads::*`.
- Production behavior was not changed in this slice.
