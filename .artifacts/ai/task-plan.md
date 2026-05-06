# AI Task Plan

## Goal

Use the stabilized `.artifacts/ai` workflow to drive current-repo backend skeleton work and adjacent documentation-governance improvements without regressing the single-source-of-truth task protocol.

## Current Phase

Phase 23 - Backend comment rollout

## Current Focus

- AT-2026-05-06-082 has been published as commit `bfdbf9a` after documenting the crate-entry boundary in `crates/module-engines/src/lib.rs`.
- AT-2026-05-06-083 has been published as commit `92696c0` after documenting the crate-entry boundary in `crates/module-downloads/src/lib.rs`.
- AT-2026-05-06-084 has been published as commit `ec0f940` after documenting the crate-entry boundary in `crates/module-fab/src/lib.rs`.
- AT-2026-05-06-085 has been published as commit `bf96bb2` after documenting the contracts-entry boundary in `crates/module-fab/src/contracts/mod.rs`.
- AT-2026-05-06-086 has now completed the next missing-comment-only slice by documenting the public command DTOs in `crates/module-fab/src/contracts/commands.rs`.
- The next action is to run the scoped diff and diagnostics, publish only this slice, and then pause for user confirmation.

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
- **Status:** complete

### Phase 6: Documentation Governance Benchmark

- Outcome: compare the repo's collaboration and architecture docs against Codex-Manager, record the main gaps, and repair the README entry surface to match the real repo state.
- Atomic tasks: AT-2026-05-03-034
- **Status:** complete

### Phase 7: Contributor Collaboration Guide

- Outcome: add a contributor-facing collaboration guide that captures current-repo boundaries, validation, risk hotspots, and document ownership without replacing the strict-doc protocol.
- Atomic tasks: AT-2026-05-03-035
- **Status:** complete

### Phase 8: Current-repo Architecture Overview

- Outcome: add a current-repo architecture overview that maps the live repo shape, entrypoints, runtime chain, hotspots, and suggested landing zones for contributors.
- Atomic tasks: AT-2026-05-03-036
- **Status:** complete

### Phase 9: Docs Map Overview

- Outcome: add `docs/README.md` as the routing map for principles, topic docs, current-repo docs, governance docs, and module docs.
- Atomic tasks: AT-2026-05-03-037
- **Status:** complete

### Phase 10: Review Drift Cleanup

- Outcome: repair the post-review drift between live task records and published entry-layer documents.
- Atomic tasks: AT-2026-05-03-038
- **Status:** complete

### Phase 11: Fab Inventory Query Wiring

- Outcome: wire the first real backend-owned Fab inventory query through the projection repository path while keeping the current adapter on a cold-start empty-page stub.
- Atomic tasks: AT-2026-05-03-039
- **Status:** complete

### Phase 12: Fab Asset Detail Query Wiring

- Outcome: wire the Fab asset-detail query through the local projection path while keeping the current adapter on a cold-start placeholder stub.
- Atomic tasks: AT-2026-05-03-040
- **Status:** complete

### Phase 13: Fab Sync Job Acceptance

- Outcome: make the Fab sync-inventory command return a backend-owned accepted job while keeping real runtime/provider execution deferred.
- Atomic tasks: AT-2026-05-03-041
- **Status:** complete

### Phase 14: Fab Startup Prewarm Job Acceptance

- Outcome: make the Fab startup-prewarm command return a backend-owned accepted job while keeping stage-3 startup orchestration and real runtime/provider execution deferred.
- Atomic tasks: AT-2026-05-03-042
- **Status:** complete

### Phase 15: Startup Stage-3 Fab Prewarm Orchestration

- Outcome: make the startup facade explicitly trigger Fab prewarm during stage 3 when capability gating allows it.
- Atomic tasks: AT-2026-05-03-043
- **Status:** complete

### Phase 16: Shared Job Runtime Bundle

- Outcome: replace the current job-runtime placeholder with a minimal shared runtime host and inject it through composition-root.
- Atomic tasks: AT-2026-05-03-044
- **Status:** complete

### Phase 17: Runtime Snapshot Persistence

- Outcome: persist shared runtime snapshots to sqlite so accepted jobs survive a fresh composition-root rebuild.
- Atomic tasks: AT-2026-05-03-045
- **Status:** complete

### Phase 18: Download Restore Correctness

- Outcome: make downloads stage-2 restore depend on persisted checkpoint facts instead of unconditional resume.
- Atomic tasks: AT-2026-05-04-052
- **Status:** complete

### Phase 19: Download Intake Correctness

- Outcome: make `start_download` preserve request metadata and enqueue priority instead of silently dropping them.
- Atomic tasks: AT-2026-05-04-053
- **Status:** complete

### Phase 20: Engines Verification Intake Correctness

- Outcome: make engine verification return a backend-owned accepted job and expose that path through composition-root and host transport.
- Atomic tasks: AT-2026-05-04-054
- **Status:** complete

### Phase 21: Composition-root Documentation Drift Cleanup

- Outcome: align `docs/TauriCompositionRootWiringDesign.md` with the live engines wiring baseline without overstating startup-stage ownership.
- Atomic tasks: AT-2026-05-04-055
- **Status:** complete

### Phase 22: Code Comment Governance

- Outcome: publish a standalone repository comment standard that defines declaration coverage, language-specific syntax, selective body comments, and stronger concurrency-comment rules.
- Atomic tasks: AT-2026-05-04-056
- **Status:** complete

### Phase 23: Backend Comment Rollout

- Outcome: apply the new repository comment standard to backend Rust/Tauri files in small, commit-sized slices of one or two files.
- Atomic tasks: AT-2026-05-04-057, AT-2026-05-04-058, AT-2026-05-04-059, AT-2026-05-04-060, AT-2026-05-04-061, AT-2026-05-04-062, AT-2026-05-04-063, AT-2026-05-04-064, AT-2026-05-04-065, AT-2026-05-04-066, AT-2026-05-04-067, AT-2026-05-04-068, AT-2026-05-05-071, AT-2026-05-05-072, AT-2026-05-05-073, AT-2026-05-05-074, AT-2026-05-05-075, AT-2026-05-05-076, AT-2026-05-05-077, AT-2026-05-05-078, AT-2026-05-05-079, AT-2026-05-05-080, AT-2026-05-05-081, AT-2026-05-06-082, AT-2026-05-06-083, AT-2026-05-06-084, AT-2026-05-06-085, AT-2026-05-06-086
- **Status:** in_progress

### Phase 24: Comment Language Controls

- Outcome: make the repository comment standard default to Chinese comments while exposing explicit slash-command switches for English and Chinese comment authoring.
- Atomic tasks: AT-2026-05-05-069
- **Status:** complete

### Phase 25: Slash Command Prefix Normalization

- Outcome: prefix all repository-local workspace slash commands with `hsy-` to reduce collisions with commands from other workspaces or extensions.
- Atomic tasks: AT-2026-05-05-070
- **Status:** complete

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
30. AT-2026-05-03-030 - committed - wired the composition-root smoke shell and added the named bootstrap wiring smoke test.
31. AT-2026-05-03-031 - committed - added the minimal host transport command shell for Fab and Downloads.
32. AT-2026-05-03-032 - committed - persisted the adjacent desktop-host `Cargo.lock` delta.
33. AT-2026-05-03-033 - committed - registered the host bootstrap smoke shell and added the named transport wiring smoke test.
34. AT-2026-05-03-034 - committed - benchmarked the repo documentation against Codex-Manager, recorded the optimization plan, and repaired the README entry surface to match the current repo state.
35. AT-2026-05-03-035 - committed - added a root contributor collaboration guide and exposed it from README as the benchmarked P1 entry layer.
36. AT-2026-05-03-036 - committed - added a current-repo architecture overview and exposed it from README as the benchmarked P2 entry layer.
37. AT-2026-05-03-037 - committed - added `docs/README.md` as the benchmarked P3 docs map and exposed it from README.
38. AT-2026-05-03-038 - committed - repaired the focused review drift in `.artifacts/ai` and the current-repo architecture overview.
39. AT-2026-05-03-039 - committed - wired the Fab inventory list query through the projection-repository path and retired the `FAB_NOT_WIRED` fallback for that query.
40. AT-2026-05-03-040 - committed - wired the Fab asset-detail query through the local projection path and retired the transport-owned `FAB_NOT_WIRED` fallback for that detail route.
41. AT-2026-05-03-041 - committed - made the Fab sync-inventory command return a backend-owned accepted job without opening startup prewarm or real runtime wiring.
42. AT-2026-05-03-042 - committed - made the Fab startup-prewarm command return a backend-owned accepted job without opening startup stage-3 orchestration or real runtime wiring.
43. AT-2026-05-03-043 - committed - wired config-gated Fab prewarm orchestration into startup stage 3 without opening real runtime execution or host ordering changes.
44. AT-2026-05-03-044 - committed - built a minimal shared job runtime host, injected it through composition-root, and moved Fab accepted-job paths onto that runtime bundle.
45. AT-2026-05-03-045 - in_progress - persist shared runtime snapshots to sqlite so accepted jobs survive a fresh composition-root rebuild.
46. AT-2026-05-04-052 - completed - wired a checkpoint-backed download restore driver, added sqlite checkpoint persistence, and validated missing/present checkpoint behavior through module and composition-root tests.
47. AT-2026-05-04-053 - completed - persisted start_download request metadata into the downloads job repository, honored request priority during runtime enqueue, and validated the behavior through module and composition-root tests.
48. AT-2026-05-04-054 - completed - wired engine verification to return a backend-owned accepted job, exposed engines through composition-root and host transport, and validated the path through module, composition-root, and transport tests.
49. AT-2026-05-04-055 - completed - repaired `TauriCompositionRootWiringDesign.md` so the current wiring scope, service sketches, sequence, and smoke-test matrix acknowledge engines without implying startup orchestration already exists.
50. AT-2026-05-04-056 - completed - published `docs/TauriCodeCommentStandard.md`, routed it from `docs/README.md`, and kept the slice docs-only without widening into repo-wide comment retrofits.
51. AT-2026-05-04-057 - completed - annotated the first backend slice in `module-fab` by documenting the facade boundary and restore drivers, then validated the slice and prepared it for publication before waiting for user confirmation.
52. AT-2026-05-04-058 - completed - annotated the desktop host bootstrap boundary and shared transport command/DTO mapping surface, then validated the slice and prepared it for publication before asking whether to continue.
53. AT-2026-05-04-059 - completed - annotated the composition-root bootstrap assembly owner, then validated the slice and prepared it for publication before asking whether to continue.
54. AT-2026-05-04-060 - completed - annotated the startup pipeline boundary in composition-root, then validated the slice and prepared it for publication before asking whether to continue.
55. AT-2026-05-04-061 - completed - annotated the desktop host state handle boundary, then validated and prepared the slice for publication.
56. AT-2026-05-04-062 - completed - annotated the downloads contracts aggregation boundary, then validated the slice and prepared it for publication before asking whether to continue.
57. AT-2026-05-04-063 - completed - annotated the downloads command/query input contracts, then validated the slice and prepared it for publication before asking whether to continue.
58. AT-2026-05-04-064 - completed - annotated the downloads read-model and event contracts, then validated the slice and prepared it for publication before asking whether to continue.
59. AT-2026-05-04-065 - completed - annotated the desktop host downloads transport handlers, then validated the slice and prepared it for publication before asking whether to continue.
60. AT-2026-05-04-066 - completed - annotated the desktop host engines transport handler, then validated the slice and prepared it for publication before asking whether to continue.
61. AT-2026-05-04-067 - completed - annotated the desktop host Fab transport handlers, then validated the slice and prepared it for publication before asking whether to continue.
62. AT-2026-05-04-068 - completed - annotated the desktop host crate entry surface, then validated the slice and prepared it for publication before asking whether to continue.
63. AT-2026-05-05-069 - completed - documented Chinese-by-default comment language and added prompt-based comment-language switches for future comment-authoring work, later normalized under the `hsy-` prefix.
64. AT-2026-05-05-070 - completed - renamed all repository workspace prompts to `hsy-XXX` and updated normative references so the repo command surface avoids name collisions.
65. AT-2026-05-05-071 - completed - annotated the downloads facade public boundary, validated the slice, and prepared it for publication before asking whether to continue.
66. AT-2026-05-05-072 - completed - rewrote the downloads restore-driver comments from English to Chinese, validated the slice, and prepared it for publication before asking whether to continue.
67. AT-2026-05-05-073 - completed - rewrote the downloads contracts entry comments from English to Chinese, validated the slice, and prepared it for publication before asking whether to continue.
68. AT-2026-05-05-074 - completed - rewrote the downloads command contracts comments from English to Chinese, validated the slice, and prepared it for publication before asking whether to continue.
69. AT-2026-05-05-075 - completed - rewrote the downloads read-model DTO comments from English to Chinese, validated the slice, and prepared it for publication before asking whether to continue.
70. AT-2026-05-05-076 - completed - rewrote the downloads event contract comments from English to Chinese, validated the slice, and prepared it for publication before asking whether to continue.
71. AT-2026-05-05-077 - completed - added missing checkpoint-boundary comments in the downloads driver without rewriting already-correct English comments, validated the slice, and prepared it for publication before asking whether to continue.
72. AT-2026-05-05-078 - completed - added missing record-state variant comments in the downloads facade without rewriting already-correct comments, validated the slice, and prepared it for publication before asking whether to continue.
73. AT-2026-05-05-079 - completed - added missing declaration comments to the public engine contract DTOs without rewriting already-correct comments in adjacent host files.
74. AT-2026-05-05-080 - completed - added missing declaration comments to the public engine facade boundary without rewriting already-correct comments or widening into test repair.
75. AT-2026-05-05-081 - completed - added the missing file-entry comment to the engine restore driver while preserving the existing correct English struct comment.
76. AT-2026-05-06-082 - completed - added the missing file-entry comment to the engines crate entry without widening into re-export rewrites.
77. AT-2026-05-06-083 - completed - added the missing file-entry comment to the downloads crate entry while preserving its existing export wiring and acceptable adjacent comments.
78. AT-2026-05-06-084 - completed - added the missing file-entry comment to the fab crate entry while preserving its existing export wiring and acceptable adjacent comments.
79. AT-2026-05-06-085 - completed - added the missing file-entry comment to the Fab contracts aggregation entry while preserving its existing export wiring and acceptable adjacent comments.
80. AT-2026-05-06-086 - completed - added the missing declaration comments to the Fab command request DTOs while preserving their existing payload shape and adjacent comments.

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

1. After AT-2026-05-03-045, decide whether the next backend slice should open stage-2 restore orchestration, lease handling, broader downloads runtime behavior, or another narrow backend path.
2. If backend work pauses after this query slice, resume from the validated host transport baseline while leaving user frontend edits untouched.

## Legacy Note

1. The old root planning history now lives under `.artifacts/ai/legacy-root-planning/` and is not the source of truth for new tasks.
2. Root `task_plan.md`, `progress.md`, and `findings.md` were removed from repo root in AT-2026-05-03-005 to prevent the legacy workflow from re-entering active execution paths.