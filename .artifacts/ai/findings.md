# AI Findings & Decisions

## Requirements

- Keep `.artifacts/ai` as the only authoritative workflow record set.
- Preserve the repo's single-active-task protocol and compatibility with existing hook parsing.
- Normalize the live records so they look intentional and match the repo-local planning-with-files workflow.

## Research Findings

- The stale startup context was caused by .github/hooks/scripts/session-start.ps1 and session-start.sh reading root task_plan.md.
- The stricter repo instructions and workflow templates still referenced root task_plan.md, progress.md, and findings.md, which conflicted with docs/TauriAIDevelopmentTransactionProtocolDesign.md.
- The protocol document defines .artifacts/ai/active-task.md, task-plan.md, progress.md, findings.md, and handoff.md as the intended local workflow records.
- New sessions should bootstrap .artifacts/ai records instead of falling back to the legacy root planning workflow.
- The repo-specific workflow is now aligned to .artifacts/ai across session-start, pre-tool-use, post-tool-use, error handling, agent-stop, instructions, and strict-doc templates.
- .sh hooks need LF preservation at the git layer; without .gitattributes, future git operations can reintroduce CRLF and break bash parsing.
- The clearest place to offer user language choice is the repo-local workflow records under .artifacts/ai/, because hooks can read a language preference file there without changing the workflow semantics.
- Session-start is the highest-value first localization target because it is the first user-visible reminder and the current strict-doc skill already anchors there.
- Windows PowerShell 5.1 is safer when scripts stay ASCII-only and localized Chinese text lives in external asset files instead of inline literals inside .ps1.
- The remaining repo-specific hook messages can share one bilingual message catalog per language instead of duplicating workflow logic in each script.
- error-occurred.ps1 was still fragile even before localization because using the automatic $input variable made stdin JSON parsing unreliable; reading into a dedicated local string fixes the hook and keeps the localized error reminder working.
- The legacy strict-doc session-start.txt and session-bootstrap.txt files are already gone, so the bilingual locale-specific assets are the only active startup reminder sources.
- planning-with-files and strict-doc-driven-development are not competing at the responsibility level; the conflict appears only when both are allowed to own active planning files or hook injection surfaces.
- docs/TauriAIDevelopmentTransactionProtocolDesign.md already expects a layered design across project instructions, on-demand skills, and hooks, so integrating planning-with-files as a context-management layer is consistent with the existing protocol.
- The critical integration rule is not "pick one skill" but "preserve a single authoritative task record set under `.artifacts/ai` while borrowing planning-with-files' context checkpoint and recovery discipline."
- `check-complete` cannot be pointed at the repo's `.artifacts/ai/task-plan.md` unchanged because the repo records atomic task status as numbered `AT-... - in_progress|committed` lines instead of Manus-style `### Phase` blocks.
- Shell validation on Windows is sensitive to `.sh` line endings even when the PowerShell side works, so repo-local shell adapters need explicit LF preservation and syntax checks.
- Once hooks, instructions, and planning-with-files scripts are fully retargeted to `.artifacts/ai`, leaving historical `task_plan.md`, `progress.md`, and `findings.md` at repo root only preserves ambiguity rather than compatibility.
- The least risky B5 decision is to archive the legacy root planning files under `.artifacts/ai/legacy-root-planning/` and remove the root copies, because git history and the archive both preserve recoverability without exposing a second active-looking workflow surface.
- The pre-tool-use hooks must treat only non-terminal active-task states as injectable current work; otherwise a `committed` task keeps leaking stale scope and allowed-files constraints into the next slice.
- In this repo, "remove the global `.claude` path" means repo-local workflow guidance and hooks should stop hardcoding user-profile paths like `.claude/skills/planning-with-files/...` and should instead execute the checked-in scripts under `.github/skills/planning-with-files/...`.
- Session catchup is not protocol-aligned if it only tracks `task-plan.md`, `progress.md`, and `findings.md`; the repo's actual recovery surface also includes `active-task.md` for the current slice and `handoff.md` for suspend/resume state.
- In VS Code chat, explicit workspace slash commands are exposed via `.github/prompts/*.prompt.md`; existing `user-invocable` skills show up under `/`, but they do not automatically create named `/plan-xxx` entry points.
- For this repository, prompt files are the least disruptive way to expose named workflow commands because they can reference the existing strict-doc and planning-with-files protocol while keeping `.artifacts/ai` as the only authoritative task record set.
- The current disorder is structural, not just cosmetic: live `.artifacts/ai` files, planning-with-files templates, and init-session bootstrap output currently use different schemas.
- The task-plan schema remains compatible with the existing stop hooks as long as the numbered ledger lines keep the form `N. AT-... - status - description`, even when richer planning sections are added around them.
- The repo still has no root `Cargo.toml` and no `src-tauri/` directory, so backend skeleton Phase A must truly start from A1 rather than pretending a host scaffold already exists.
- The backend skeleton implementation doc explicitly fixes A1 as the smallest safe slice: create only the root workspace manifest and do not pre-register nonexistent member paths.
- Cargo metadata rejects a purely virtual workspace with zero members, and it also rejects a member manifest that has no target file. The smallest passing shape is a workspace plus at least one real member with `src/lib.rs` or an equivalent target.
- The smallest current-repo member that satisfies the metadata gate without overcommitting to full host wiring is `src-tauri/Cargo.toml` plus `src-tauri/src/lib.rs`.
- The next safe host slice can stay crate-local: a placeholder service handle plus thin bootstrap/main surfaces are enough to make `cargo check -p my-epic-launcher-desktop` meaningful before composition-root exists.
- A crate-local host shell with `bootstrap.rs`, `state.rs`, `main.rs`, and an explicit bin target is sufficient to make the `my-epic-launcher-desktop` package compile before any real Tauri transport or composition-root wiring exists.
- The repo root must ignore Rust `target/`; otherwise even a narrow package-scoped `cargo check` leaves untracked incremental artifacts and breaks the clean-worktree expectation after backend validation.
- Even with the skill retargeted correctly, users can still misread `.artifacts/ai/legacy-root-planning/` as live state unless the active `.artifacts/ai/` directory itself carries a local README that states the source-of-truth boundary.
- The next backend atomic slice after `src-tauri` B1-B3 is documented as backend skeleton task B1: add `crates/kernel-foundation` and update the root workspace membership at the same time, not earlier and not later.
- Introducing a new workspace package can dirty `Cargo.lock` even when the B1 code slice is correct, so the repo should continue treating lockfile persistence as a tiny adjacent cleanup slice instead of silently widening the crate bootstrap task.
- The current B2 task-table row is slightly stale against the committed B1 baseline: implementing the documented foundation surface also requires `crates/kernel-foundation/Cargo.toml`, `crates/kernel-foundation/src/lib.rs`, and `crates/kernel-foundation/src/result.rs`, so the controlling doc should be corrected before continuing.
- The first dependency-bearing foundation slice can expand `Cargo.lock` substantially because the repo had not resolved `chrono` and `serde_json` trees before; this is still a lockfile cleanup concern, not a reason to widen the B2 code task.
- The B3 `launcher-kernel-jobs` slice does not need the full later runtime directory tree yet; the current controlling docs already permit a narrower shell with `lib.rs`, `model.rs`, and `runtime.rs` as long as it exposes only shared job state, snapshot, and runtime traits.
- The `launcher-kernel-jobs` B3 lockfile delta is smaller than the foundation one because it only adds the new workspace package entry and reuses already-resolved foundation dependencies; it should still be persisted as a separate cleanup slice.
- For C1, `launcher-module-fab` only needs small public contracts and a placeholder `FabFacade` over generic dependencies; the actual repository/provider port traits and use case implementations belong to later slices and should not be pulled into the first module shell.
- The `launcher-module-fab` C1 lockfile delta should remain a separate cleanup slice as well; the package reuses already-resolved workspace dependencies, so the adjacent `Cargo.lock` change is expected and should not be folded back into the code task.
- For C2, `launcher-module-downloads` should mirror the same shell-first pattern: expose only small public contracts and a placeholder `DownloadFacade`, while download scheduler, checkpoint, manifest-provider, staging-store, and transport details stay deferred to later slices.
- The `launcher-module-downloads` C2 lockfile delta should remain a separate cleanup slice as well; the package reuses already-resolved workspace dependencies, so the adjacent `Cargo.lock` change is expected and should not be folded back into the code task.
- For C3, `launcher-adapter-storage-sqlite` should stay shell-first as well: expose only adapter config plus minimal repository constructor surfaces, while schema design, migrations, connection strategy, and actual SQL remain deferred to later slices.
- The `launcher-adapter-storage-sqlite` C3 lockfile delta should remain a separate cleanup slice as well; the package reuses already-resolved workspace dependencies, so the adjacent `Cargo.lock` change is expected and should not be folded back into the code task.
- For C4, `launcher-adapter-provider-fab` should stay shell-first too: expose only provider adapter config plus the `EpicFabCatalogProviderAdapter` constructor surface, while remote auth, HTTP wiring, and payload mapping remain deferred to later slices.
- The `launcher-adapter-provider-fab` C4 lockfile delta should remain a separate cleanup slice as well; the package reuses already-resolved workspace dependencies, so the adjacent `Cargo.lock` change is expected and should not be folded back into the code task.
- For D1, `launcher-composition-root` should start as a pure public API shell: expose `DesktopBootstrapConfig`, `DesktopAppServices`, `StartupPipelineFacade`, and `build_desktop_services()`, while the actual wiring implementation and bootstrap smoke test remain deferred to D2.
- The `launcher-composition-root` D1 lockfile delta should remain a separate cleanup slice as well; the package reuses already-resolved workspace dependencies, so the adjacent `Cargo.lock` change is expected and should not be folded back into the code task.
- For D2, the first composition-root wiring slice should stay smoke-test oriented: assemble the already-created adapter stubs into Fab and Downloads facades, keep startup stage methods explicit and side-effect free, and let the named `bootstrap_wiring_smoke` test prove only that `build_desktop_services()` returns the three facade handles.
- A thin D2 builder layout is enough for the first composition-root wiring slice: `build_storage_adapters`-style config validation can live in helper functions, the startup facade can expose async stage methods as explicit no-ops, and the integration smoke test can verify assembly by inspecting facade deps instead of requiring real runtime execution.
- For E1, the smallest host transport slice should stay command-shell only: add Fab and Downloads handler modules that consume `DesktopAppServices`, keep result mapping thin and local to `src-tauri`, and defer actual command registration plus shared-state injection to E2.
- The E1 host slice can safely stub read-only query responses on `*_NOT_WIRED` facade errors while still routing commands through `DesktopAppServices`; adding these host crate dependencies also produces a tiny `Cargo.lock` adjacency that should be persisted in a separate cleanup slice, without touching unrelated user frontend worktree edits.
- When the worktree also contains unrelated user frontend changes, backend lockfile cleanup should use path-scoped `git status` checks and selective staging so the backend slice can still be committed and pushed without sweeping user UI work into the same commit.
- For E2, the smallest host smoke slice should keep the runtime fakeable: centralize command names in `src-tauri/src/commands/mod.rs`, build a real `DesktopAppServices` handle inside the testable bootstrap surface, and let `transport_wiring_smoke` prove registration/state/handler path without needing a live Tauri runtime.
- The host transport slice can stay fully testable without the real Tauri crate: `build_desktop_host_bootstrap()` can return registered command names plus a shared `DesktopAppServicesHandle`, and the named smoke test can call an async handler with a ready future poll helper instead of spinning up a runtime.
- External benchmarking against Codex-Manager showed that MyEpicLauncher's main documentation weakness is not lack of deep design detail; it is the absence of a flatter contributor-facing entry layer above the strict-doc and architecture stack.
- Codex-Manager's CONTRIBUTING document adds high-value path ownership, risk hotspots, size-threshold warnings, submit/PR expectations, and doc-maintenance boundaries that MyEpicLauncher does not currently expose in one place.
- Codex-Manager's ARCHITECTURE document adds high-value current-repo navigation artifacts: directory duties, entrypoint indexes, request-chain summaries, current hotspots, and suggested landing zones.
- README drift is a concrete symptom of that missing entry layer: before AT-034, the root entry surface still claimed the repo lacked `Cargo.toml` / `src-tauri/` and still referenced root `task_plan.md` / `progress.md` / `findings.md` after the repo had already moved to `.artifacts/ai/` and landed the backend skeleton baseline.
- The most pragmatic documentation rollout after AT-034 is: keep README aligned first, then add a contributor-facing `CONTRIBUTING.md`, then add a current-repo architecture overview, then add `docs/README.md` as the docs map.
- AT-035 confirmed that a root `CONTRIBUTING.md` can close the main P1 gap without duplicating the deep architecture stack, as long as it stays focused on current-repo boundaries, commands, validation, hotspots, and docs ownership.
- The contributor guide should link forward to the AI transaction protocol and deep design docs rather than restating their full content; otherwise the new entry layer will drift into a second architecture source of truth.
- In this repo, contributor-facing hotspots are not only large files; they are also control-surface files such as `README.md`, `.github/copilot-instructions.md`, `.github/skills/strict-doc-driven-development/SKILL.md`, `src-tauri/src/lib.rs`, and `crates/composition-root/src/bootstrap.rs`.
- AT-036 confirmed that the missing P2 layer is not another principles doc; it is a current-state map of the live repo shape, key entrypoints, validation entrypoints, and landing zones.
- The most useful current-repo architecture content is concrete and file-anchored: `src-tauri/src/bootstrap.rs`, `src-tauri/src/commands/mod.rs`, `crates/composition-root/src/bootstrap.rs`, and `src-tauri/tests/transport_wiring_smoke.rs` explain more about today's repo than repeating high-level boundaries.
- The architecture overview should stay narrowly scoped to “where things live and how they connect today”; if it starts re-explaining all non-goals and principles from the blueprint, it will drift into a duplicate source.
- AT-037 confirmed that the docs-map layer is most useful when it is opinionated about ownership and routing: which docs are first-entry docs, which are principles, which describe the current repo, which are topic docs, and which are module docs.
- `docs/README.md` should not become a second README; its value is in telling contributors where to go next and what does not belong back in the root README.
- AT-038 confirmed that post-review repair slices must reset both `task-plan.md` current-focus text and `progress.md` current-status text together; fixing only one surface recreates recovery drift immediately.
- Once an entry-layer document is exposed from the root README, it should not keep draft-status wording unless the repo explicitly wants users to treat it as non-authoritative.

## Technical Decisions

| Decision | Rationale |
|----------|-----------|
| `.artifacts/ai` remains the only authoritative task record set | Prevents dual planning surfaces and stale recovery paths |
| The task-plan keeps a numbered AT ledger | `check-complete` and stop hooks already parse that shape |
| The repo should use one hybrid schema across live records, templates, and bootstrap output | planning-with-files readability is useful, but strict-doc semantics must stay explicit |
| Backend skeleton kickoff begins with a workspace plus a minimal `src-tauri` lib stub | Cargo metadata requires one real target-bearing member before the documented gate can pass |
| Preserve the deep design docs and add a flatter contributor-facing entry layer above them | Lowers onboarding friction without weakening the repo's strict architecture and AI transaction constraints |

## Issues Encountered

| Issue | Resolution |
|-------|------------|
| Generic planning templates and repo live files drifted into different shapes | Normalize both the live records and the template/bootstrap sources together |
| Sync terminal commands did not preserve the intended repo cwd for verification and commit flow | Use async terminal commands with explicit repo paths when git output matters |
| The backend skeleton doc's A1 assumption conflicted with actual Cargo behavior | Surface the gap in findings/progress and bridge the workspace root to the smallest valid `src-tauri` stub |

## Resources

- docs/TauriRewriteArchitectureBlueprint.md
- docs/TauriArchitecturePrinciplesDesign.md
- docs/TauriAIDevelopmentTransactionProtocolDesign.md
- docs/TauriTestingStrategyAndQualityGateDesign.md
- docs/TauriAIContextManagementIntegrationDesign.md
- .github/skills/planning-with-files/SKILL.md
- .github/skills/planning-with-files/templates/task_plan.md
- .github/skills/planning-with-files/templates/progress.md
- .github/skills/planning-with-files/templates/findings.md
- .github/skills/strict-doc-driven-development/active-atomic-task-template.md

## Visual/Browser Findings

- None in this slice.