---
name: focused-module-implementer
description: Docs-first autonomous implementation for a user-specified module in this repository. Use when the user asks Codex to focus on one module and keep going autonomously, finish the next module slice, plan/code/test/commit/push in small atomic tasks, or explicitly says the budget is sufficient and Codex should not stop except for serious blockers.
---

# Focused Module Implementer

## Core Rule

Do not start coding before reading the relevant project and module documents. Treat the user-specified module as the focus boundary: understand what it owns, what it must not own, and which adjacent contracts it depends on before editing code.

Assume the user wants autonomous execution. Plan when planning is useful, code when the next atomic task is clear, test before claiming progress, commit after each completed atomic task, and attempt to push. If push fails, record the failure locally and continue with the next task unless the failure blocks local work.

## Intake

1. Identify the module named by the user. If the module is ambiguous, ask one concise question.
2. Check `git status` before edits. Preserve unrelated user changes.
3. If `.artifacts/ai/` exists, use it as the local task record. Read the active plan/progress/findings/handoff enough to continue without duplicating work.
4. Define the smallest useful atomic task. Avoid large mixed tasks.

## Required Reading Before Code

Always read these first:

- `README.md`
- `CONTRIBUTING.md`
- `docs/README.md` if present
- the active `.artifacts/ai/` files when the repo task protocol is active
- module docs under `docs/modules/<module>/`

For a module with docs, read only that module's docs unless a cross-module boundary is truly involved:

- `README_ARCH.md` for ownership and boundaries
- `README_API.md` for public contracts
- `README_FLOW.md` for lifecycle and state transitions
- `README_IMPL.md` for implementation roadmap and current slice details

For the downloads module, treat these as the default required module docs:

- `docs/modules/downloads/README_ARCH.md`
- `docs/modules/downloads/README_API.md`
- `docs/modules/downloads/README_FLOW.md`
- `docs/modules/downloads/README_IMPL.md`

Read additional architecture docs only when they are relevant to the task:

- `docs/TauriRewriteArchitectureBlueprint.md` for overall architecture boundaries
- `docs/TauriArchitecturePrinciplesDesign.md` for ownership and layering principles
- `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` for crate/module placement
- `docs/TauriTestingStrategyAndQualityGateDesign.md` for verification expectations
- `docs/TauriAIDevelopmentTransactionProtocolDesign.md` for task transaction rules
- `docs/TauriIPCAndStateContractsDesign.md` for IPC/state projection work
- `docs/TauriKernelJobsRuntimeDesign.md` for kernel job runtime, scheduler, or execution state work
- `docs/TauriDownloadRuntimeDesign.md` for downloads runtime or segment execution work
- `docs/TauriErrorHandlingAndProjectionDesign.md` for error mapping or state projection
- `docs/TauriSecurityCredentialsAndPermissionsDesign.md` for filesystem, credentials, permissions, or path safety
- `docs/TauriCodeCommentStandard.md` before adding non-obvious comments in Rust or TypeScript

Do not read unrelated module implementation docs just to be thorough. If touching another module, write down why it is in scope.

## Context Summary Gate

Before editing, summarize locally or in the task record:

- module purpose
- owned crates/files likely involved
- public API or IPC surface affected
- out-of-scope areas
- tests that should prove the slice
- the next atomic task

If this summary cannot be written, keep reading instead of coding.

## Atomic Execution Loop

For each atomic task:

1. Update the local task record if `.artifacts/ai/` is active.
2. Write or adjust the focused test first when behavior changes.
3. Implement the minimum code needed for the slice.
4. Add concise Chinese comments only for non-obvious logic or important boundaries. Do not comment obvious code.
5. Run the focused test. Then run the smallest broader gate that could catch integration regressions.
6. Update relevant docs when behavior, boundary, API, or roadmap status changes.
7. Run `git diff` and `git status`.
8. Commit the completed atomic task.
9. Attempt to push. If push fails for network/auth/remote reasons, record it and continue locally.

Keep each commit focused. Do not batch unrelated fixes into one task just because they are nearby.

## Testing Expectations

Never say a task is complete without verification. Prefer focused commands first, then broader checks:

- Rust crate work: `cargo test -p <crate> <test-name>` then relevant crate/workspace checks
- Tauri host or composition work: include composition/desktop smoke tests when touched
- Frontend work: use `npm run lint`, `npm run build`, or targeted checks based on the changed surface
- Documentation-only work: run available format/link checks if present; otherwise inspect changed docs manually

If a command cannot run, record the exact reason and choose the next-best local proof.

## Failure Policy

Continue autonomously unless there is a serious blocker.

For a serious bug:

1. Reproduce or isolate it first.
2. Try up to five distinct fix attempts.
3. After each failed attempt, record what changed and what evidence was observed.
4. If five attempts fail, stop implementation, record the blocker in `.artifacts/ai/handoff.md` or another local task record, and tell the user what remains.

Do not spin indefinitely. Do not hide failed tests.

## Boundaries

- Stay focused on the user-specified module.
- Do not move business truth into the frontend when backend ownership is documented.
- Do not change public `DL_*`, IPC, scheduler, manifest, SQLite, or composition-root contracts unless the current atomic task explicitly requires it.
- Do not rewrite architecture documents broadly as part of implementation.
- Do not revert unrelated user or agent changes.
- Do not skip documentation reading because the code looks obvious.
