# Active Atomic Task

## Identity

- task id: AT-2026-05-17-219
- title: Prove downloads policy host transport runtime application
- status: completed

## Goal

Add a narrow host transport smoke assertion proving `downloads_update_policy` enters the already-wired downloads facade through `src-tauri` and updates the shared runtime policy snapshot through composition-root wiring.

## Scope

- in scope:
  - `src-tauri/tests/transport_wiring_smoke.rs`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - frontend settings UI
  - new host commands or DTO shapes
  - scheduler loops, active-job rescheduling, leases, snapshots migration, pending resume work mutation
  - concrete HTTP/file/hash execution, retry/backoff, terminal completion
  - SQLite schema changes or unrelated dirty files
  - module implementation documentation churn unless a durable boundary changes

## Allowed Files

1. src-tauri/tests/transport_wiring_smoke.rs
2. .artifacts/ai/active-task.md
3. .artifacts/ai/task-plan.md
4. .artifacts/ai/progress.md
5. .artifacts/ai/findings.md
6. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. README.md and docs/README.md relevant routing sections.
2. docs/modules/downloads/README_ARCH.md.
3. docs/modules/downloads/README_API.md.
4. docs/modules/downloads/README_FLOW.md.
5. docs/modules/downloads/README_IMPL.md section 7.28.
6. docs/TauriIPCAndStateContractsDesign.md downloads command/query section.
7. docs/TauriCompositionRootWiringDesign.md host transport/composition sections.
8. docs/TauriTestingStrategyAndQualityGateDesign.md transport smoke gate sections.
9. docs/TauriAIDevelopmentTransactionProtocolDesign.md task lifecycle sections.
10. Current `src-tauri` transport test and downloads command handler code.

## Hypothesis

- falsifiable local hypothesis: adding a RED host transport smoke assertion for `downloads_update_policy` will currently prove whether the existing command path reaches the composition-root runtime policy applier; if the assertion fails, the fix must remain in host/composition wiring only.

## Cheap Check

1. Add the smallest transport smoke assertion around `downloads_update_policy`.
2. Run the focused test first and confirm the new assertion fails if runtime application is not visible through the host path.
3. If RED already passes because previous wiring is complete, record that no production code change is needed.
4. Run `cargo test -p my-epic-launcher-desktop transport_wiring_smoke --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`.
5. Run scoped `git diff --check` for the AT-219 file set.

## Validation Gate

1. Host transport test exercises `commands::downloads::downloads_update_policy(...)`.
2. The test verifies the command returns success through the shared transport envelope.
3. The test verifies `services.downloads.deps().job_runtime.policy().max_concurrent_jobs` reflects the requested policy slots.
4. No frontend, scheduler, concrete IO, retry/backoff, terminal completion, or SQLite schema behavior changes.
5. Commit only AT-219 files locally, then push `main` to `origin`.

## Validation Result

- Added a host transport smoke assertion for `commands::downloads::downloads_update_policy(...)`.
- The command returns a success envelope, `downloads_get_policy(...)` reads back the persisted policy, and `services.downloads.deps().job_runtime.policy().max_concurrent_jobs` reflects the requested `concurrency_slots`.
- No production Rust behavior changes were needed; the existing AT-217 wiring already satisfied the host path assertion.
- `cargo test -p my-epic-launcher-desktop --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml transport_wiring_smoke` passed with 1 test passed / 0 failed.
- `rustfmt --check src-tauri\tests\transport_wiring_smoke.rs` passed after formatting only the touched transport smoke test file.
- Scoped `git diff --check` passed for the AT-219 file set with CRLF normalization warnings only.
- Commit and push are pending for the AT-219 file set.

## Notes

- AT-2026-05-17-218 final commit is `5aae7f1` and is already pushed to `origin/main`.
