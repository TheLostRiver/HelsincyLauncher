# AI Task Plan

## Goal

Use the stabilized `.artifacts/ai` workflow to drive current-repo backend skeleton bootstrap without regressing the single-source-of-truth task protocol.

## Current Phase

Phase 5 - Backend Skeleton Bootstrap

## Current Focus

- Use the clean post-D1 baseline as the starting point for the next documented backend slice.
- Keep the next slice focused on one documented boundary at a time.

## Phases

### Phase 1: Bootstrap and Localization

- Outcome: switched workflow surfaces to `.artifacts/ai` and introduced zh-CN/en language support.
- Atomic tasks: AT-2026-05-03-001, AT-2026-05-03-002
- **Status:** complete

### Phase 2: Integration and Recovery

- Outcome: mapped planning-with-files onto `.artifacts/ai`, retired legacy root files, and repaired recovery/control-path bugs.
- Atomic tasks: AT-2026-05-03-003, AT-2026-05-03-004, AT-2026-05-03-005, AT-2026-05-03-006
- **Status:** complete

### Phase 3: Command Entry Points

- Outcome: added explicit workspace slash-command prompts for planning and resume flows.
- Atomic tasks: AT-2026-05-03-007
- **Status:** complete

### Phase 4: Record Schema Normalization

- Outcome: align live workflow records, templates, and session bootstrap output to one repo-specific schema inspired by planning-with-files.
- Atomic tasks: AT-2026-05-03-008
- **Status:** complete

### Phase 5: Backend Skeleton Bootstrap

- Outcome: establish the current-repo Rust/Tauri scaffold in atomic slices A1-A3 without disturbing the existing Next.js frontend prototype.
- Atomic tasks: AT-2026-05-03-009, AT-2026-05-03-010, AT-2026-05-03-011
- **Status:** in_progress

## Atomic Task Ledger

1. AT-2026-05-03-001 - committed - switched hooks, repo instructions, and workflow templates to `.artifacts/ai` and bootstrapped the new task records.
2. AT-2026-05-03-002 - committed - added zh-CN and en language modes for the repo workflow skill, translated the strict-doc skill assets to Chinese-first wording, and localized the repo-specific hook messages.
3. AT-2026-05-03-003 - committed - defined the mapping that lets planning-with-files act as the context-management layer while `.artifacts/ai` remains the only authoritative task record set.
4. AT-2026-05-03-004 - committed - retargeted the repo-local planning-with-files copy so its hooks, init flow, stop-time checks, and catchup flow operate on `.artifacts/ai` instead of root planning files.
5. AT-2026-05-03-005 - committed - archived the legacy root planning files under `.artifacts/ai/legacy-root-planning/` and removed the root copies from the active repo surface.
6. AT-2026-05-03-006 - committed - repaired the workflow control surfaces so committed tasks stop injecting as current work and repo-local recovery now covers `active-task.md` and `handoff.md` without pointing at user-global `.claude` copies.
7. AT-2026-05-03-007 - committed - added workspace slash-command prompt entry points that wrap the repo's strict-doc and planning-with-files workflow without introducing a second planning source.
8. AT-2026-05-03-008 - committed - normalized the live `.artifacts/ai` records plus repo-local planning templates and bootstrap output into one section-based schema that remains compatible with the existing AT ledger and hooks.
9. AT-2026-05-03-009 - committed - created the root Rust workspace manifest plus the smallest valid `src-tauri` member stub so backend skeleton Phase A can pass the documented metadata gate.
10. AT-2026-05-03-010 - committed - persisted the generated `Cargo.lock` so the validated workspace baseline is fully recorded and the worktree returns to clean.
11. AT-2026-05-03-011 - committed - added the thin host bootstrap surface under `src-tauri` so the desktop host crate can pass `cargo check` without composition-root yet.
12. AT-2026-05-03-012 - committed - ignored Rust `target/` artifacts so backend validation no longer leaves the repo dirty.
13. AT-2026-05-03-013 - committed - added a local README under `.artifacts/ai/` that clearly separates active workflow records from the archived legacy root planning files.
14. AT-2026-05-03-014 - committed - bootstrapped the `launcher-kernel-foundation` crate shell and connected it to the root workspace.
15. AT-2026-05-03-015 - committed - persisted the generated `Cargo.lock` entry for `launcher-kernel-foundation` so the repo returns to a clean post-B1 baseline.
16. AT-2026-05-03-016 - committed - implemented the first real foundation contract surface and the named `foundation_contract_smoke` test.
17. AT-2026-05-03-017 - committed - persisted the dependency-expanded `Cargo.lock` produced by the validated foundation B2 slice.
18. AT-2026-05-03-018 - committed - bootstrapped the `launcher-kernel-jobs` crate shell and minimal shared runtime surface.
19. AT-2026-05-03-019 - committed - persisted the `Cargo.lock` delta produced by the validated kernel-jobs B3 slice.
20. AT-2026-05-03-020 - committed - bootstrapped the `launcher-module-fab` crate shell and public contracts/facade boundary.
21. AT-2026-05-03-021 - committed - persisted the `Cargo.lock` delta produced by the validated module-fab C1 slice.
22. AT-2026-05-03-022 - committed - bootstrapped the `launcher-module-downloads` crate shell and public contracts/facade boundary.
23. AT-2026-05-03-023 - committed - persisted the `Cargo.lock` delta produced by the validated module-downloads C2 slice.
24. AT-2026-05-03-024 - committed - bootstrapped the `launcher-adapter-storage-sqlite` crate shell and minimal repository constructors.
25. AT-2026-05-03-025 - committed - persisted the `Cargo.lock` delta produced by the validated sqlite adapter C3 slice.
26. AT-2026-05-03-026 - committed - bootstrapped the `launcher-adapter-provider-fab` crate shell and minimal provider adapter constructor.
27. AT-2026-05-03-027 - committed - persisted the `Cargo.lock` delta produced by the validated Fab provider adapter C4 slice.
28. AT-2026-05-03-028 - committed - bootstrapped the `launcher-composition-root` crate shell and public API surface.
29. AT-2026-05-03-029 - committed - persisted the `Cargo.lock` delta produced by the validated composition-root D1 slice.

## Key Questions

1. What is the smallest valid Phase A slice that creates a Rust workspace entry point without dragging in nonexistent packages?
2. Which exact host surfaces are enough to make `cargo check -p my-epic-launcher-desktop` meaningful without pulling in real backend wiring too early?

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| `.artifacts/ai` remains the only authoritative task record set | Prevents competing planning surfaces and stale recovery paths |
| The task-plan keeps a numbered AT ledger | Preserves compatibility with stop-hook and `check-complete` parsing |
| Record files should use a hybrid schema | planning-with-files contributes readable sections, while strict-doc keeps explicit atomic-task semantics |
| Backend skeleton kickoff needs a real first member | Cargo metadata rejects an empty virtual workspace, so the first valid slice must include a minimal target-bearing member |

## Follow-up Queue

1. Integrate planning-with-files' 2-action checkpoint cadence more explicitly into repo-level reminders if the adapter slice alone is not enough.
2. After AT-2026-05-03-029 passes, choose the next documented backend slice from a clean post-D1 baseline.

## Legacy Note

1. The old root planning history now lives under `.artifacts/ai/legacy-root-planning/` and is not the source of truth for new tasks.
2. Root `task_plan.md`, `progress.md`, and `findings.md` were removed from repo root in AT-2026-05-03-005 to prevent the legacy workflow from re-entering active execution paths.