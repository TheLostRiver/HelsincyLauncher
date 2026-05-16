# Active Atomic Task

## Identity

- task id: AT-2026-05-16-180
- title: Wire downloads pending resume scheduler in composition root
- status: completed

## Goal

把 AT-179 新增的 `InMemoryDownloadResumeWorkScheduler` 接入 composition-root 的 downloads facade 装配，让桌面服务图使用真实 module-local pending work scheduler shell，而不是继续使用 `()` no-op placeholder。

本轮只覆盖 composition wiring：

- add focused RED smoke proving composition exposes `pending_work()` on the downloads scheduler dependency
- update `DesktopDownloadFacade` concrete type alias
- wire `InMemoryDownloadResumeWorkScheduler::new()` in `build_downloads_module`
- keep driver-side pending-work consumption, fetch/write/verify, checkpoint mutation, SQLite schema, host transport, frontend, and `kernel-jobs` payload changes out of scope

## Scope

- in scope:
  - `crates/composition-root/src/bootstrap.rs`
  - `crates/composition-root/tests/bootstrap_wiring_smoke.rs`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - concrete scheduler/fetch/write/verify execution
  - checkpoint mutation or new checkpoint repository methods
  - frontend files
  - host transport IPC shape
  - SQLite schema or concrete segment persistence
  - `kernel-jobs` segment payload or completion API changes
  - unrelated dirty worktree files

## Allowed Files

1. crates/composition-root/src/bootstrap.rs
2. crates/composition-root/tests/bootstrap_wiring_smoke.rs
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
11. docs/TauriKernelJobsRuntimeDesign.md
12. docs/TauriTestingStrategyAndQualityGateDesign.md
13. docs/TauriAIDevelopmentTransactionProtocolDesign.md

Related architecture/collaboration docs read in scoped snippets:

1. docs/modules/downloads/README_IMPL.md
2. docs/TauriDownloadRuntimeDesign.md
3. docs/TauriKernelJobsRuntimeDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriAIDevelopmentTransactionProtocolDesign.md
6. docs/TauriRewriteArchitectureBlueprint.md
7. docs/TauriArchitecturePrinciplesDesign.md
8. docs/TauriCurrentRepoArchitectureOverview.md
9. docs/TauriCompositionRootWiringDesign.md

## Hypothesis

- falsifiable local hypothesis: composition-root can swap downloads `resume_scheduler` from `()` to `InMemoryDownloadResumeWorkScheduler` without changing public service shape or touching driver execution, transport, frontend, SQLite schema, or `kernel-jobs`.

## Cheap Check

- focused RED/GREEN test: `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml bootstrap_wiring_smoke`

## Validation Gate

1. Read required docs in scoped snippets before editing README_IMPL.
2. Write focused RED composition smoke for scheduler shell wiring.
3. Observe the focused test fail while composition still uses `()` placeholder.
4. Wire `InMemoryDownloadResumeWorkScheduler` in composition-root.
5. Update README_IMPL current state and PWF records.
6. Run focused composition smoke and relevant package tests.
7. Run scoped `git diff --check` and path-limited `git status --short`.
8. Commit only the AT-179 slice locally.

## Validation Result

- passed
- Required README, docs map, CONTRIBUTING, downloads README_IMPL, composition-root wiring design, backend crate layout/API, testing strategy, and AI transaction protocol snippets were read before coding.
- RED: focused composition smoke failed because current `resume_scheduler` was `()` and had no `pending_work()` method.
- GREEN: wired `InMemoryDownloadResumeWorkScheduler` into `DesktopDownloadFacade` and `build_downloads_module`.
- Updated `bootstrap_wiring_smoke` to assert the assembled downloads facade exposes an empty pending-work scheduler.
- Updated README_IMPL to record that composition now wires the in-memory scheduler shell instead of `()`.
- Focused `bootstrap_wiring_smoke` passed.
- Full `launcher-composition-root` suite passed: lib tests 5 passed, integration tests 7 passed, doc tests 0.
- Initial `cargo fmt -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check` failed on smoke-test line wrapping; after `cargo fmt`, the same format check passed.
- Scoped `git diff --check` passed for AT-180 files with CRLF warnings only.
- Driver-side pending-work consumption, fetch/write/verify, checkpoint mutation, SQLite schema, host transport, frontend, and `kernel-jobs` payloads remain unchanged.

## Notes

- AT-2026-05-16-179 committed pending scheduler shell as `4d0c23b`.
- User approved starting AT-180 with "开始".
- Direct `origin/main` push remains intentionally skipped without explicit approval.
- Current unrelated worktree state: preserve `crates/composition-root/src/startup.rs` formatting side effect and other unrelated dirty files.
