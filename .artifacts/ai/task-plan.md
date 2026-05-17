# AI Task Plan

## Goal

Use the stabilized `.artifacts/ai` workflow to drive current-repo backend skeleton work and adjacent documentation-governance improvements without regressing the single-source-of-truth task protocol.

## Current Phase

Phase 102 - Accepted Execution State Projection Boundary

## Current Focus

- AT-2026-05-17-221 was published as commit `89d3a19` after adding the minimal `kernel-jobs` execution-turn contract with focused TDD.
- AT-2026-05-17-222 was published as commit `feddcfc` after documenting the next one-shot shared runtime dispatch boundary.
- AT-2026-05-17-223 was published as commit `f87df03` after adding the TDD-backed one-shot `kernel-jobs` dispatch method.
- AT-2026-05-17-224 was published as commit `597c0e5` after documenting the safe downloads driver `run(...)` override boundary.
- AT-2026-05-17-225 was published as commit `c5b0695` after implementing the guarded downloads driver `run(...)` override.
- AT-2026-05-17-226 was published as commit `d2d5405` after covering the remaining guarded `run(...)` deferred branches and fixing non-mutating checkpoint helper semantics.
- AT-2026-05-17-227 is complete locally after defining the accepted execution state projection boundary; commit/push pending.
- AT-2026-05-17-220 was published as commit `aa8d6e3` after documenting the shared runtime execution-turn boundary.
- AT-2026-05-17-219 was published as commit `f618718` after adding host transport smoke coverage for downloads policy runtime application.
- AT-2026-05-17-218 was published as commit `5aae7f1` after adding documentation-budget rules.
- AT-2026-05-06-082 has been published as commit `bfdbf9a` after documenting the crate-entry boundary in `crates/module-engines/src/lib.rs`.
- AT-2026-05-06-083 has been published as commit `92696c0` after documenting the crate-entry boundary in `crates/module-downloads/src/lib.rs`.
- AT-2026-05-06-084 has been published as commit `ec0f940` after documenting the crate-entry boundary in `crates/module-fab/src/lib.rs`.
- AT-2026-05-06-085 has been published as commit `bf96bb2` after documenting the contracts-entry boundary in `crates/module-fab/src/contracts/mod.rs`.
- AT-2026-05-06-086 has been published as commit `8b4f751` after documenting the public command DTOs in `crates/module-fab/src/contracts/commands.rs`.
- AT-2026-05-06-087 has been published as commit `0d33c98` after documenting the public event union and variants in `crates/module-fab/src/contracts/events.rs`.
- AT-2026-05-06-088 has been published as commit `f9b7512` after documenting the public query DTOs in `crates/module-fab/src/contracts/queries.rs`.
- AT-2026-05-06-089 has been published as commit `83dd236` after documenting the public read-model declarations in `crates/module-fab/src/contracts/dto.rs`.
- AT-2026-05-06-090 has been published as commit `9c44f56` after documenting the Fab provider adapter shell in `crates/adapter-provider-fab/src/lib.rs`.
- AT-2026-05-06-091 has been published as commit `f20e4f5` after documenting the SQLite storage adapter file-entry and shared config cluster in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-06-092 has been published as commit `c5b6f33` after documenting the Fab projection repository shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-07-093 has been published as commit `d8fbbc8` after documenting the Fab sync cursor repository shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-07-094 has been published as commit `39ba47d` after documenting the Fab media metadata repository shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-07-095 has been published as commit `f022abe` after documenting the download job repository shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-07-096 has been published as commit `5b5a96a` after documenting the download checkpoint repository shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-08-097 has been published as commit `367b4b6` after documenting the shared job snapshot store shell in `crates/adapter-storage-sqlite/src/lib.rs`.
- AT-2026-05-08-098 has been published as commit `7260673` after documenting the shared foundation error contract in `crates/kernel-foundation/src/error.rs`.
- AT-2026-05-08-099 has been published as commit `2792762` after documenting the shared foundation paging contract in `crates/kernel-foundation/src/paging.rs`.
- AT-2026-05-08-100 has been published as commit `fab77ce` after documenting the shared foundation result alias in `crates/kernel-foundation/src/result.rs`.
- AT-2026-05-08-101 has been published as commit `340bd13` after documenting the foundation crate entry boundary in `crates/kernel-foundation/src/lib.rs`.
- AT-2026-05-08-102 has been published as commit `7fa1bda` after documenting the shared foundation clock boundary in `crates/kernel-foundation/src/clock.rs`.
- AT-2026-05-08-103 has been published as commit `6fcb6e3` after documenting the shared foundation time wrapper boundary in `crates/kernel-foundation/src/time.rs`.
- AT-2026-05-08-104 has been published as commit `c35ffaa` after documenting the shared foundation ID wrapper boundary in `crates/kernel-foundation/src/ids.rs`.
- AT-2026-05-08-105 has been published as commit `7b4b502` after documenting the shared jobs crate entry boundary in `crates/kernel-jobs/src/lib.rs`.
- AT-2026-05-08-106 has been published as commit `ec3dc63` after documenting the `JobState` / `JobUiState` declaration cluster in `crates/kernel-jobs/src/model.rs`.
- The user then requested a Windsurf-specific repo rules translation instead of continuing the backend comment rollout immediately.
- AT-2026-05-08-107 has been published as commit `a17e9f7` after adding a repo-root `.windsurfrules` file that maps the strict-doc and `.artifacts/ai` workflow into plain instructions.
- The user then requested that the Windsurf compatibility surface use `.windsurf/rules` instead of the repo-root `.windsurfrules` file.
- AT-2026-05-08-108 completed the relocation slice by moving the Windsurf compatibility rules into `.windsurf/rules/repo-workflow.md` and deleting the root `.windsurfrules` file.
- The focused validation gates for AT-2026-05-08-108 passed: scoped `git diff --check` returned clean and VS Code diagnostics reported no errors for the touched text files.
- The user chose to keep `.artifacts/ai` as the authoritative task record surface instead of moving the files back to the repo root.
- AT-2026-05-14-109 repaired the repo-local PWF path resolver so `pwf-doctor` recognizes `.artifacts/ai/task-plan.md` and reports the existing planning files as healthy.
- The focused validation gates for AT-2026-05-14-109 passed: the resolver assertion first failed before the fix, then passed after the fix, and `pwf-doctor` reported `active plan: ok` plus `planning files: ok`.
- Stop hook then correctly pointed back to the only remaining incomplete phase, Phase 23 Backend Comment Rollout.
- AT-2026-05-14-110 added the next narrow `kernel-jobs` comments for `JobPriority` and `JobProgress`, but final Rust validation is blocked because the current shell cannot find `cargo`.
- The user installed Rust; `cargo 1.95.0` and `rustc 1.95.0` are now visible in the current shell.
- AT-2026-05-14-110 completed after `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` passed.
- AT-2026-05-14-111 completed after documenting `AcceptedJob` and `EnqueueJobRequest<E>` in `crates/kernel-jobs/src/model.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-112 completed after documenting `RestoreDisposition` in `crates/kernel-jobs/src/model.rs` and publishing commit `3e54e3a`.
- AT-2026-05-14-113 completed after documenting `JobSnapshot<E>` in `crates/kernel-jobs/src/model.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-114 completed after documenting `JobSnapshotDto` in `crates/kernel-jobs/src/model.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-115 completed after documenting `RuntimeQueuePolicy` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-116 completed after documenting `JobDriver<E>` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-117 completed after documenting `JobDriverRegistry<E>` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-118 completed after documenting `JobSnapshotStore<E>` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-119 completed after documenting `InMemoryJobSnapshotStore` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-120 completed after documenting `SharedJobRuntimeHost` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-121 completed after documenting `JobRuntime` in `crates/kernel-jobs/src/runtime.rs` and validating with the same `launcher-kernel-jobs` lib check.
- AT-2026-05-14-122 completed after localizing `EngineJobDriver` comments in `crates/module-engines/src/driver.rs` and validating `launcher-module-engines`.
- AT-2026-05-14-123 completed after localizing downloads query contract comments in `crates/module-downloads/src/contracts/queries.rs` and validating `launcher-module-downloads`.
- AT-2026-05-14-124 completed after localizing SQLite `job_snapshots.recoverable` migration comments in `crates/adapter-storage-sqlite/src/lib.rs` and validating `launcher-adapter-storage-sqlite`.
- AT-2026-05-14-125 completed after localizing desktop host state comments in `src-tauri/src/state.rs` and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-126 completed after localizing desktop host crate entry comments in `src-tauri/src/lib.rs` and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-127 completed after localizing desktop bootstrap comments in `src-tauri/src/bootstrap.rs` and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-128 completed after localizing engines transport command comments in `src-tauri/src/commands/engines.rs` and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-128 was committed locally as `206e603`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-129 completed after localizing `src-tauri/src/commands/jobs.rs` shared jobs query comments and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-129 was committed locally as `f7155cd`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-130 completed after localizing `src-tauri/src/commands/fab.rs` transport comments and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-130 was committed locally as `4d0e1f2`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-131 completed after localizing `src-tauri/src/commands/downloads.rs` transport comments and validating `my-epic-launcher-desktop`.
- AT-2026-05-14-131 was committed locally as `af04875`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-132 completed after localizing `crates/module-fab/src/driver.rs` restore driver comments and validating `launcher-module-fab`.
- AT-2026-05-14-132 was committed locally as `8444c7f`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-133 completed after localizing the first `crates/module-fab/src/facade/mod.rs` facade boundary comments and validating `launcher-module-fab`.
- AT-2026-05-14-133 was committed locally as `fab9b4b`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-134 completed after adding Chinese companion comments for the `()` fallback comments in `crates/module-fab/src/facade/mod.rs` and validating `launcher-module-fab`.
- AT-2026-05-14-134 was committed locally as `5ab45ab`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- User preference changed: when an English comment already exists, keep it and add a Chinese companion comment instead of replacing the English text.
- AT-2026-05-14-135 completed after adding Chinese companion comments for the next `crates/module-fab/src/facade/mod.rs` method-comment slice and validating `launcher-module-fab`.
- AT-2026-05-14-135 was committed locally as `8750d58`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-136 completed after adding Chinese companion comments for the shared transport file header and command registry in `src-tauri/src/commands/mod.rs`, then validating `my-epic-launcher-desktop`.
- AT-2026-05-14-136 was committed locally as `ec00e89`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-137 completed after adding Chinese companion comments for the `AppErrorDto` error envelope in `src-tauri/src/commands/mod.rs`, then validating `my-epic-launcher-desktop`.
- AT-2026-05-14-137 was committed locally as `0b727de`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-138 completed after adding Chinese companion comments for the result envelope and accepted-job DTO cluster in `src-tauri/src/commands/mod.rs`, then validating `my-epic-launcher-desktop`.
- AT-2026-05-14-138 was committed locally as `c32f12f`; direct `origin/main` push remains blocked by safety review, so continue without bypassing it.
- AT-2026-05-14-139 completed after adding Chinese companion comments for the shared transport mapper cluster in `src-tauri/src/commands/mod.rs`, then validating `my-epic-launcher-desktop`.
- AT-2026-05-15-148 completed and was committed locally as `a13a2e6`, recording backend comment rollout completion and starting Phase 28 backend scope recovery.
- AT-2026-05-15-149 completed and was committed locally as `e774628`, wiring downloads pause/cancel facade methods to shared runtime control ports while leaving resume out of scope.
- AT-2026-05-15-150 completed and was committed locally as `958a0e6`, adding host transport smoke coverage for downloads start/pause/cancel without changing production behavior or resume semantics.
- AT-2026-05-15-151 completed and was committed locally as `a6fc28a`, refreshing the downloads facade file header so it matches the current start/pause/cancel wiring state.
- AT-2026-05-15-152 completed and was committed locally as `1397ec7`, closing Phase 28 backend recovery records and leaving `resume_download` as the next checkpoint-aware design/RED-test candidate.
- AT-2026-05-15-153 completed and was committed locally as `c05d132`, recording the docs-first resume-download design boundary and confirming the next implementation slice must start with a checkpoint-loading RED test.
- AT-2026-05-15-154 completed and was committed locally as `71b0ee1`, closing Phase 29 records and keeping `resume_download` implementation gated on design approval.
- AT-2026-05-15-155 completed after adding the checkpoint-aware `resume_download` RED test and minimal checkpoint read while leaving full resume orchestration out of scope.
- AT-2026-05-15-155 was committed locally as `645dd93`.
- AT-2026-05-15-156 completed after adding stable `DL_CHECKPOINT_MISSING` semantics to `resume_download` while keeping full resume orchestration out of scope.
- AT-2026-05-15-156 was committed locally as `b3bfb1f`.
- AT-2026-05-15-157 is in progress after selecting the next document-backed resume slice: module job lookup before checkpoint.
- AT-2026-05-15-157 completed after adding stable `DL_JOB_NOT_FOUND` semantics before checkpoint loading while keeping staging, manifest, and runtime resume orchestration out of scope.
- AT-2026-05-15-157 was committed locally as `2dc46c4`.
- AT-2026-05-15-158 is in progress after selecting the next document-backed resume slice: staging validation boundary after job and checkpoint.
- AT-2026-05-15-158 completed after adding the minimal `DownloadStagingObjectStore` boundary and proving `resume_download` reaches staging validation after job and checkpoint.
- AT-2026-05-15-158 was committed locally as `cd5e848`.
- AT-2026-05-15-159 is in progress after the user required module implementation docs and pre-code module-document reading rules.
- AT-2026-05-15-159 completed after adding downloads `README_IMPL.md`, a module implementation template, and README_IMPL discoverability rules.
- AT-2026-05-15-160 is in progress after reading the downloads implementation guide and selecting the next document-backed resume slice: manifest provider boundary after staging validation.
- AT-2026-05-15-160 completed after adding the minimal `DownloadManifestProviderPort` and proving `resume_download` reaches manifest reconstruction after staging validation while runtime enqueue remains out of scope.
- AT-2026-05-15-161 is in progress after selecting the next document-backed prerequisite for completed-segment sealing: segment/checkpoint/resume-decision data shape documentation.
- AT-2026-05-15-161 completed after documenting the manifest segment, segment checkpoint, resume decision actions, and invariants required before coding completed-segment sealing.
- AT-2026-05-15-162 is in progress after selecting the first code slice from that data shape: completed checkpoint segments become sealed resume decisions and are not runtime enqueue candidates.
- AT-2026-05-15-162 completed after adding manifest segment/checkpoint/decision types and proving completed segment checkpoints become sealed non-enqueue decisions.
- AT-2026-05-15-163 is in progress after selecting the next explicit decision branch: partial checkpoint segments become `resume_partial` enqueue candidates without runtime enqueue.
- AT-2026-05-15-163 completed after adding the partial checkpoint -> `ResumePartial` decision branch while keeping runtime enqueue out of scope.
- AT-2026-05-15-164 is in progress after confirming the `RejectMismatch` branch already exists but lacks focused safety coverage for stale segment checkpoint facts.
- AT-2026-05-15-164 completed after adding focused mismatch rejection coverage; the test passed without production edits because the safety branch already existed.
- AT-2026-05-15-165 is in progress after the implementation guide made the next decision gap explicit: `queue_remaining` must be covered before runtime enqueue work.
- AT-2026-05-15-165 completed after adding focused queue-remaining coverage; the test passed without production edits because the fallback branch already existed.
- AT-2026-05-15-166 is in progress to document the runtime-enqueue boundary before any Rust code starts handing resume decisions to `JobRuntime`.
- AT-2026-05-15-166 completed and committed locally after updating README_IMPL with the job-level runtime-enqueue boundary and passing scoped doc/Git validation.
- AT-2026-05-15-167 completed and committed locally after adding the TDD-backed job-level runtime enqueue boundary inside `module-downloads` and refreshing README_IMPL current state.
- AT-2026-05-15-168 completed and committed locally after adding a stable downloads-domain error projection for `resume_download` segment mismatch decisions.
- AT-2026-05-16-169 completed and committed locally after documenting the all-sealed completion boundary and current `AcceptedJob` contract gap before Rust behavior changes.
- AT-2026-05-16-170 completed and committed locally after adding the module-owned resume outcome boundary for all-sealed plans while preserving the current host transport `AcceptedJob` entry.
- AT-2026-05-16-171 completed and committed locally after projecting the module-owned downloads resume outcome through host transport without exposing segment details or changing unrelated accepted-job paths.
- AT-2026-05-16-172 completed the downloads-owned scheduler/driver payload boundary documentation before Rust code starts handing `resume_partial` / `queue_remaining` work to a scheduler.
- AT-2026-05-16-173 completed and committed locally as `1a698f9`, adding the module-local downloads resume work plan derivation.
- AT-2026-05-16-174 is in progress to document the scheduler/driver boundary before introducing a scheduler port in Rust.
- AT-2026-05-16-174 completed and committed locally as `6929fa9`, defining the scheduler/driver boundary.
- AT-2026-05-16-175 is in progress to add the minimal scheduler port and schedule work plans before runtime enqueue.
- AT-2026-05-16-175 completed and committed locally as `8846a40`, adding the scheduler port and composition placeholder.
- AT-2026-05-16-176 is in progress to prove scheduler failures skip shared runtime enqueue.
- AT-2026-05-16-176 completed and committed locally as `edec23d`, proving scheduler failures skip runtime enqueue.
- AT-2026-05-16-177 completed and was committed locally as `31942bd`, proving all-sealed resumes do not touch scheduler/runtime work.
- AT-2026-05-16-178 completed and was committed locally as `41c3be4`, documenting the concrete downloads scheduler execution boundary.
- AT-2026-05-16-179 completed and was committed locally as `4d0c23b`, adding the TDD-backed module-local pending resume work queue/scheduler shell.
- AT-2026-05-16-180 completed the composition-root wiring for `InMemoryDownloadResumeWorkScheduler` and is ready for the local commit.

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
- Atomic tasks: AT-2026-05-04-057, AT-2026-05-04-058, AT-2026-05-04-059, AT-2026-05-04-060, AT-2026-05-04-061, AT-2026-05-04-062, AT-2026-05-04-063, AT-2026-05-04-064, AT-2026-05-04-065, AT-2026-05-04-066, AT-2026-05-04-067, AT-2026-05-04-068, AT-2026-05-05-071, AT-2026-05-05-072, AT-2026-05-05-073, AT-2026-05-05-074, AT-2026-05-05-075, AT-2026-05-05-076, AT-2026-05-05-077, AT-2026-05-05-078, AT-2026-05-05-079, AT-2026-05-05-080, AT-2026-05-05-081, AT-2026-05-06-082, AT-2026-05-06-083, AT-2026-05-06-084, AT-2026-05-06-085, AT-2026-05-06-086, AT-2026-05-06-087, AT-2026-05-06-088, AT-2026-05-06-089, AT-2026-05-14-110, AT-2026-05-14-111, AT-2026-05-14-112, AT-2026-05-14-113, AT-2026-05-14-114, AT-2026-05-14-115, AT-2026-05-14-116, AT-2026-05-14-117, AT-2026-05-14-118, AT-2026-05-14-119, AT-2026-05-14-120, AT-2026-05-14-121, AT-2026-05-14-122, AT-2026-05-14-123, AT-2026-05-14-124, AT-2026-05-14-125, AT-2026-05-14-126, AT-2026-05-14-127, AT-2026-05-14-128, AT-2026-05-14-129, AT-2026-05-14-130, AT-2026-05-14-131, AT-2026-05-14-132, AT-2026-05-14-133, AT-2026-05-14-134, AT-2026-05-14-135, AT-2026-05-14-136, AT-2026-05-14-137, AT-2026-05-14-138, AT-2026-05-14-139
- Additional completion sweep: AT-2026-05-14-140, AT-2026-05-14-141, AT-2026-05-14-142, AT-2026-05-14-143, AT-2026-05-14-144, AT-2026-05-14-145, AT-2026-05-14-146, AT-2026-05-15-147
- **Status:** complete

### Phase 24: Comment Language Controls

- Outcome: make the repository comment standard default to Chinese comments while exposing explicit slash-command switches for English and Chinese comment authoring.
- Atomic tasks: AT-2026-05-05-069
- **Status:** complete

### Phase 25: Slash Command Prefix Normalization

- Outcome: prefix all repository-local workspace slash commands with `hsy-` to reduce collisions with commands from other workspaces or extensions.
- Atomic tasks: AT-2026-05-05-070
- **Status:** complete

### Phase 26: External Agent Compatibility

- Outcome: add a Windsurf-compatible repository rules file that restates the strict-doc and `.artifacts/ai` workflow in plain instructions without creating a second planning source.
- Atomic tasks: AT-2026-05-08-107, AT-2026-05-08-108
- **Status:** complete

### Phase 27: PWF Doctor Compatibility Repair

- Outcome: make the repo-local Plan With Files doctor recognize `.artifacts/ai` as the active task record surface without recreating root planning files or `.planning` state.
- Atomic tasks: AT-2026-05-14-109
- **Status:** complete

### Phase 28: Backend Development Scope Recovery

- Outcome: after completing backend comment rollout, re-read the current README, architecture, collaboration, and module documents in small batches, identify the next backend-only development slice, and keep frontend/unrelated dirty work untouched.
- Atomic tasks: AT-2026-05-15-148, AT-2026-05-15-149, AT-2026-05-15-150, AT-2026-05-15-151, AT-2026-05-15-152
- **Status:** complete

### Phase 29: Downloads Resume Design Boundary

- Outcome: begin the next backend-only downloads resume work by reading the required docs first, preserving frontend/composition-root boundaries, and selecting a checkpoint-aware RED-test slice before any behavior change.
- Atomic tasks: AT-2026-05-15-153, AT-2026-05-15-154
- **Status:** complete

### Phase 30: Downloads Resume Checkpoint Slice

- Outcome: start the approved backend implementation by proving `resume_download` explicitly reads `DownloadCheckpointRepository` before any full manifest/staging/runtime resume orchestration is attempted.
- Atomic tasks: AT-2026-05-15-155
- **Status:** complete

### Phase 31: Downloads Resume Missing Checkpoint Semantics

- Outcome: give the explicit user `resume_download` command a stable downloads-domain error when the checkpoint repository has no checkpoint for the requested job.
- Atomic tasks: AT-2026-05-15-156
- **Status:** complete

### Phase 32: Downloads Resume Job Lookup Semantics

- Outcome: make explicit user `resume_download` check the module-owned download job record before checkpoint loading, returning a stable downloads-domain error when the job record is missing.
- Atomic tasks: AT-2026-05-15-157
- **Status:** complete

### Phase 33: Downloads Resume Staging Validation Boundary

- Outcome: introduce the minimal `DownloadStagingObjectStore` boundary and prove explicit user `resume_download` reaches staging validation after job and checkpoint are present.
- Atomic tasks: AT-2026-05-15-158
- **Status:** complete

### Phase 34: Downloads Module Implementation Documentation

- Outcome: add a downloads module implementation document that records required pre-code reading order, Rust landing zones, current implementation state, and validation gates.
- Atomic tasks: AT-2026-05-15-159
- **Status:** complete

### Phase 35: Downloads Resume Manifest Boundary

- Outcome: introduce the minimal manifest provider port and prove explicit user `resume_download` reaches manifest reconstruction after job, checkpoint, and staging are present.
- Atomic tasks: AT-2026-05-15-160
- **Status:** complete

### Phase 36: Downloads Resume Segment Shape Documentation

- Outcome: document the minimal manifest segment, segment checkpoint, and resume decision data shapes required before coding completed-segment sealing.
- Atomic tasks: AT-2026-05-15-161
- **Status:** complete

### Phase 37: Downloads Resume Sealed Segment Decision

- Outcome: add the minimal in-memory data structures and pure resume decision behavior that marks matching completed segment checkpoints as sealed.
- Atomic tasks: AT-2026-05-15-162
- **Status:** complete

### Phase 38: Downloads Resume Partial Segment Decision

- Outcome: add the pure resume decision behavior that marks matching partial segment checkpoints as `resume_partial` enqueue candidates.
- Atomic tasks: AT-2026-05-15-163
- **Status:** complete

### Phase 39: Downloads Resume Mismatch Rejection Coverage

- Outcome: add focused module coverage proving stale manifest/checkpoint boundary facts become `reject_mismatch` decisions and are not runtime enqueue candidates.
- Atomic tasks: AT-2026-05-15-164
- **Status:** complete

### Phase 40: Downloads Resume Queue Remaining Coverage

- Outcome: add focused module coverage proving manifest segments with no safe checkpoint become `queue_remaining` decisions and remain runtime enqueue candidates.
- Atomic tasks: AT-2026-05-15-165
- **Status:** complete

### Phase 41: Downloads Resume Runtime Enqueue Boundary Documentation

- Outcome: document the minimal job-level runtime enqueue boundary for `resume_download` after segment decisions are complete, without opening Rust code, persistence, host, or frontend changes.
- Atomic tasks: AT-2026-05-15-166
- **Status:** complete

### Phase 42: Downloads Resume Runtime Enqueue Boundary

- Outcome: prove and implement the first job-level `JobRuntime::enqueue()` boundary for `resume_download` after segment decisions, without segment persistence, scheduler execution, host transport, or frontend changes.
- Atomic tasks: AT-2026-05-15-167
- **Status:** complete

### Phase 43: Downloads Resume Mismatch Error Projection

- Outcome: prove and implement a stable downloads-domain error for `resume_download` when derived segment decisions contain `reject_mismatch`, without runtime enqueue, persistence, host transport, or frontend changes.
- Atomic tasks: AT-2026-05-15-168
- **Status:** complete

### Phase 44: Downloads Resume All-Sealed Completion Boundary

- Outcome: document the no-runtime-candidate all-sealed resume outcome and the current `AcceptedJob` contract gap before any Rust behavior change, without frontend, IPC, persistence, scheduler, or `kernel-jobs` payload changes.
- Atomic tasks: AT-2026-05-16-169
- **Status:** complete

### Phase 45: Downloads Module-Owned Resume Outcome Boundary

- Outcome: introduce a module-owned resume outcome so all-sealed plans can be represented without runtime enqueue, while keeping host transport, IPC shape, persistence, scheduler execution, and `kernel-jobs` payloads unchanged.
- Atomic tasks: AT-2026-05-16-170
- **Status:** complete

### Phase 46: Downloads Resume Outcome Host Projection

- Outcome: project `DownloadResumeOutcome` through the downloads host transport command so already-complete all-sealed resume is not represented as accepted queued work, without frontend, persistence, scheduler, or kernel-jobs payload changes.
- Atomic tasks: AT-2026-05-16-171
- **Status:** complete

### Phase 47: Downloads Resume Scheduler Driver Payload Boundary

- Outcome: document the downloads-owned resume work payload boundary so `resume_partial` and `queue_remaining` can later feed a module scheduler/driver without moving segment plans into `kernel-jobs`, host transport, frontend, or SQLite schema.
- Atomic tasks: AT-2026-05-16-172
- **Status:** complete

### Phase 48: Downloads Resume Work Plan Derivation

- Outcome: introduce the smallest downloads-owned resume work plan/payload derivation so `resume_partial` and `queue_remaining` can be represented as module-local scheduler inputs while `seal_completed` and `reject_mismatch` produce no work item.
- Atomic tasks: AT-2026-05-16-173
- **Status:** complete

### Phase 49: Downloads Resume Scheduler Boundary Documentation

- Outcome: define the downloads-owned scheduler/driver boundary that consumes `DownloadResumeWorkPlan`, including facade call order and failure behavior before any scheduler port code lands.
- Atomic tasks: AT-2026-05-16-174
- **Status:** complete

### Phase 50: Downloads Resume Scheduler Port

- Outcome: add the minimal downloads-owned scheduler port and call it with `DownloadResumeWorkPlan` before job-level runtime enqueue, without concrete execution or persistence changes.
- Atomic tasks: AT-2026-05-16-175
- **Status:** complete

### Phase 51: Downloads Resume Scheduler Failure Guard

- Outcome: prove scheduler boundary failures return before shared runtime enqueue, preserving module-owned work-plan safety.
- Atomic tasks: AT-2026-05-16-176
- **Status:** complete

### Phase 52: Downloads All-Sealed Scheduler Guard

- Outcome: prove all-sealed already-complete resumes do not touch the scheduler boundary or shared runtime enqueue.
- Atomic tasks: AT-2026-05-16-177
- **Status:** complete

### Phase 53: Downloads Scheduler Execution Boundary Documentation

- Outcome: define the downloads-owned scheduler execution boundary, ownership split, forbidden surfaces, and the next minimal Rust slice before implementing concrete fetch/write/verify behavior.
- Atomic tasks: AT-2026-05-16-178
- **Status:** complete

### Phase 54: Downloads Pending Resume Work Scheduler Shell

- Outcome: add a module-local pending resume work queue/scheduler shell behind `DownloadResumeWorkScheduler`, proving pending work registration happens before shared runtime enqueue without concrete fetch/write/verify or persistence changes.
- Atomic tasks: AT-2026-05-16-179
- **Status:** complete

### Phase 55: Downloads Pending Scheduler Composition Wiring

- Outcome: replace the composition-root downloads `resume_scheduler: ()` placeholder with `InMemoryDownloadResumeWorkScheduler`, proving desktop assembly now wires the module-local pending-work scheduler shell without driver execution or transport changes.
- Atomic tasks: AT-2026-05-16-180
- **Status:** complete

### Phase 56: Downloads Driver Pending Work Consumption Boundary

- Outcome: define how downloads driver-side execution should consume module-local pending resume work without pretending the current `kernel-jobs` runtime already has a full `run()` execution loop, and identify the next smallest Rust slice if one is safe.
- Atomic tasks: AT-2026-05-16-181
- **Status:** complete

### Phase 57: Downloads Pending Resume Work Source Drain

- Outcome: add the minimal downloads-owned `DownloadPendingResumeWorkSource` boundary and job-id-scoped drain semantics on `InMemoryDownloadResumeWorkScheduler`, without wiring driver execution or concrete IO.
- Atomic tasks: AT-2026-05-16-182
- **Status:** complete

### Phase 58: DownloadJobDriver Local Pending Work Consumer Boundary

- Outcome: define whether and how `DownloadJobDriver` can own a local pending-work consumer method backed by `DownloadPendingResumeWorkSource` before the shared runtime has a documented execution `run()` API.
- Atomic tasks: AT-2026-05-16-183
- **Status:** complete

### Phase 59: DownloadJobDriver Local Pending Work Consumer

- Outcome: add the TDD-backed local `DownloadJobDriver` pending-work consumer method and injected source constructor while preserving restore/runtime/composition/IO behavior.
- Atomic tasks: AT-2026-05-16-184
- **Status:** complete

### Phase 60: Downloads Composition Shared Scheduler Source Wiring Boundary

- Outcome: define the composition-root assembly boundary that shares one `InMemoryDownloadResumeWorkScheduler` as both the downloads facade scheduler and the driver pending-work source, without changing runtime execution, transport, frontend, SQLite, checkpoint mutation, or concrete IO.
- Atomic tasks: AT-2026-05-16-185
- **Status:** complete

### Phase 61: Downloads Composition Shared Scheduler Source Wiring

- Outcome: wire one shared `InMemoryDownloadResumeWorkScheduler` through composition-root as both downloads facade scheduler and driver pending-work source, with focused TDD and no public API/runtime/transport/IO expansion.
- Atomic tasks: AT-2026-05-16-186
- **Status:** complete

### Phase 62: Downloads Checkpoint Mutation Boundary

- Outcome: define downloads-owned checkpoint mutation timing, repository/SQLite responsibility, failure layering, and the next persistence slice before concrete fetch/write/verify execution begins.
- Atomic tasks: AT-2026-05-16-187
- **Status:** complete

### Phase 63: Downloads SQLite Segment Checkpoint Persistence

- Outcome: persist and load `DownloadSegmentCheckpointRecord` facts through `SqliteDownloadCheckpointRepository` with focused TDD, without opening driver execution, concrete IO, runtime completion, host transport, or frontend projection.
- Atomic tasks: AT-2026-05-16-188
- **Status:** complete

### Phase 64: Downloads Driver Execution Boundary

- Outcome: define the next downloads driver execution boundary after durable segment checkpoint persistence, without changing `kernel-jobs`, implementing concrete IO, or claiming runtime `run()` exists in current Rust.
- Atomic tasks: AT-2026-05-17-189
- **Status:** complete

### Phase 65: Downloads Driver Local Execution-Turn Classification

- Outcome: add a module-local `DownloadJobDriver` execution-turn classification method that reloads checkpoint facts before draining pending work, without changing shared runtime execution, concrete IO, snapshots, transport, frontend, or persistence.
- Atomic tasks: AT-2026-05-17-190
- **Status:** complete

### Phase 66: Downloads Segment Execution Ports Boundary

- Outcome: update downloads README_IMPL after AT-190 and define the next segment execution ports boundary before coding any fetch/write/verify or runtime completion behavior.
- Atomic tasks: AT-2026-05-17-191
- **Status:** complete

### Phase 67: Downloads Segment Execution Request Handoff

- Outcome: add a module-local segment execution request/result/port shell and a driver helper that converts `PendingWorkAccepted` work into stable job-scoped requests without concrete IO or runtime completion.
- Atomic tasks: AT-2026-05-17-192
- **Status:** complete

### Phase 68: Downloads Fake Segment Execution Acceptance

- Outcome: document and add a local driver helper that hands segment execution requests to a fake/narrow execution port in stable order, without concrete HTTP, writes, verification, checkpoint mutation, runtime completion, transport, or frontend work.
- Atomic tasks: AT-2026-05-17-193
- **Status:** complete

### Phase 69: Downloads Fake Segment Completion Result Contract

- Outcome: document and add a module-local fake completed segment execution result shape that can be collected through the existing execution port helper without concrete HTTP, writes, verification, checkpoint mutation, runtime completion, transport, or frontend work.
- Atomic tasks: AT-2026-05-17-194
- **Status:** complete

### Phase 70: Downloads Fake Completed-result Checkpoint Mutation

- Outcome: document and add a local driver helper that reloads downloads checkpoint facts, applies fake completed segment execution results, and saves the updated checkpoint through the existing repository port without concrete IO, SQLite schema changes, runtime completion, transport, or frontend work.
- Atomic tasks: AT-2026-05-17-195
- **Status:** complete

### Phase 71: Downloads Fake Local Resume Execution Orchestration

- Outcome: document and add a module-local driver helper that chains the existing resume execution-turn, segment request handoff, fake execution port, and checkpoint mutation helpers without introducing runtime `run()`, concrete IO, SQLite adapter/schema changes, transport, or frontend behavior.
- Atomic tasks: AT-2026-05-17-196
- **Status:** complete

### Phase 72: Downloads Fake Segment Failure Result Boundary

- Outcome: document the next module-local fake failed segment result contract before coding, preserving the boundary that public `DL_*` execution errors, runtime completion, concrete IO, SQLite adapter/schema changes, transport, composition-root, and frontend behavior remain out of scope.
- Atomic tasks: AT-2026-05-17-197
- **Status:** complete

### Phase 73: Downloads Fake Segment Failure Result Contract

- Outcome: add a module-local failed segment result value that preserves request facts, local failure reason, retryable hint, and downloaded bytes through the existing segment execution port helper without checkpoint mutation, retry/backoff, public error projection, runtime completion, concrete IO, transport, composition-root, or frontend work.
- Atomic tasks: AT-2026-05-17-198
- **Status:** complete

### Phase 74: Downloads Fake Failed-result Checkpoint Mutation Boundary

- Outcome: document the next local failed-result checkpoint mutation helper before coding, preserving the boundary that retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema changes, transport, composition-root, and frontend behavior remain out of scope.
- Atomic tasks: AT-2026-05-17-199
- **Status:** complete

### Phase 75: Downloads Fake Failed-result Checkpoint Mutation

- Outcome: add a local driver helper that reloads downloads checkpoint facts, applies same-job fake failed segment execution results as `Failed` segment checkpoint facts, and saves through the existing repository port without retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema changes, transport, composition-root, or frontend work.
- Atomic tasks: AT-2026-05-17-200
- **Status:** complete

### Phase 76: Downloads Fake Local Mixed-result Checkpoint Orchestration Boundary

- Outcome: document the next local orchestration boundary so `execute_local_resume_turn(...)` can record both completed and failed fake segment results through existing checkpoint mutation helpers without retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema changes, transport, composition-root, or frontend behavior.
- Atomic tasks: AT-2026-05-17-201
- **Status:** complete

### Phase 77: Downloads Fake Local Mixed-result Checkpoint Orchestration

- Outcome: update `execute_local_resume_turn(...)` to record both completed and failed fake segment results through existing checkpoint mutation helpers without retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter/schema changes, transport, composition-root, or frontend behavior.
- Atomic tasks: AT-2026-05-17-202
- **Status:** complete

### Phase 78: Downloads Get-job Snapshot Query Boundary

- Outcome: document the first safe `list/get/policy surfaces` slice as `DownloadsFacade::get_job_snapshot(...)`, using existing module job lookup plus shared runtime snapshot lookup while keeping list pagination, policy persistence, runtime API expansion, adapters, transport, frontend, and concrete execution out of scope.
- Atomic tasks: AT-2026-05-17-203
- **Status:** complete

### Phase 79: Downloads Get-job Snapshot Query Implementation

- Outcome: implement `DownloadsFacade::get_job_snapshot(...)` with focused TDD by composing the downloads module record with the shared runtime snapshot, while leaving `list_jobs`, policy surfaces, runtime list APIs, adapters, transport, frontend, concrete IO, retry/backoff, and terminal completion out of scope.
- Atomic tasks: AT-2026-05-17-204
- **Status:** complete

### Phase 80: Downloads List-jobs Query Boundary

- Outcome: document the first `DownloadsFacade::list_jobs(...)` implementation slice as a downloads-owned repository page and conservative list-item projection, while deferring runtime list APIs, live snapshot joins, policy storage, transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-205
- **Status:** complete

### Phase 81: Downloads List-jobs Query Implementation

- Outcome: implement `DownloadsFacade::list_jobs(...)` with focused TDD using a downloads-owned repository page and conservative list-item projection, while keeping policy surfaces, runtime list APIs, live snapshot joins, transport, frontend, concrete IO, retry/backoff, and terminal completion out of scope.
- Atomic tasks: AT-2026-05-17-206
- **Status:** complete

### Phase 82: Downloads Policy Source Boundary

- Outcome: document the downloads-owned policy source of truth before coding `get_policy(...)` / `update_policy(...)`, while deferring runtime queue-policy mutation, SQLite schema/adapter work, host transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-207
- **Status:** complete

### Phase 83: Downloads Policy Store Implementation

- Outcome: implement `DownloadsFacade::get_policy(...)` and `DownloadsFacade::update_policy(...)` against a downloads-owned policy store/port with clamped user-facing concurrency slots, while deferring runtime queue-policy mutation, SQLite schema/adapter persistence, host transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-208
- **Status:** complete

### Phase 84: Downloads Policy SQLite Persistence Boundary

- Outcome: document the first SQLite policy persistence slice for the existing `DownloadPolicyStore` port, including schema, adapter, composition, and verification boundaries, while deferring runtime queue-policy mutation, global settings, host transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-209
- **Status:** complete

### Phase 85: Downloads SQLite Policy Store Implementation

- Outcome: implement `SqliteDownloadPolicyStore` for the existing downloads-owned `DownloadPolicyStore` port and wire it through composition-root while deferring runtime policy application, global settings, host transport, frontend, concrete IO, retry/backoff, pending work mutation, and terminal completion.
- Atomic tasks: AT-2026-05-17-210
- **Status:** complete

### Phase 86: Downloads Runtime Policy Application Boundary

- Outcome: document startup seeding from persisted downloads policy as the first runtime application slice, while deferring live runtime mutation, scheduler work, transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-211
- **Status:** complete

### Phase 87: Downloads Runtime Policy Startup Seeding

- Outcome: seed the initial shared runtime queue policy from the persisted downloads policy during composition-root startup, while deferring live runtime mutation, scheduler work, transport, frontend, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-212
- **Status:** complete

### Phase 88: Downloads Live Runtime Policy Update Boundary

- Outcome: document the live runtime policy update boundary and select a kernel-jobs policy-control surface as the first safe Rust slice, while deferring downloads facade wiring, composition-root wiring, transport, frontend, scheduler work, active runtime mutation, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-213
- **Status:** complete

### Phase 89: Kernel Jobs Runtime Policy Control

- Outcome: add the kernel-jobs runtime policy control surface so cloned `SharedJobRuntimeHost` handles observe a shared updated queue-policy snapshot, while deferring downloads facade wiring, composition-root wiring, transport, frontend, scheduler work, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-214
- **Status:** complete

### Phase 90: Downloads Runtime Policy Applier Boundary

- Outcome: document the downloads-owned runtime policy applier boundary before Rust coding, keeping the first code slice in `module-downloads` and deferring composition-root concrete wiring, host transport, frontend, scheduler work, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion.
- Atomic tasks: AT-2026-05-17-215
- **Status:** complete

### Phase 91: Downloads Runtime Policy Applier Port

- Outcome: add the downloads-owned runtime policy applier port and default no-op facade path so `update_policy(...)` can hand off normalized persisted policy snapshots without composition-root concrete runtime wiring, transport, frontend, scheduler work, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, or terminal completion.
- Atomic tasks: AT-2026-05-17-216
- **Status:** complete

### Phase 92: Composition-root Downloads Runtime Policy Applier Wiring

- Outcome: wire a private composition-root runtime policy applier from downloads policy updates to `SharedJobRuntimeHost::update_policy(...)`, while keeping host transport, frontend, scheduler work, active jobs/leases/snapshots, pending work, concrete IO, retry/backoff, and terminal completion unchanged.
- Atomic tasks: AT-2026-05-17-217
- **Status:** complete

### Phase 93: Documentation Budget Rules

- Outcome: add concise repository rules that keep durable architecture/module docs separate from per-task logs, while preserving `.artifacts/ai` as the authoritative execution record.
- Atomic tasks: AT-2026-05-17-218
- **Status:** complete

### Phase 94: Downloads Policy Host Transport Smoke

- Outcome: prove the host transport `downloads_update_policy` command path persists the downloads policy and applies the shared runtime policy snapshot through existing composition-root wiring, without frontend, scheduler, concrete IO, retry/backoff, or terminal completion changes.
- Atomic tasks: AT-2026-05-17-219
- **Status:** complete

### Phase 95: Shared Runtime Execution-Turn Boundary

- Outcome: define the next durable backend implementation boundary for a minimal `kernel-jobs` execution-turn contract before coding runtime-owned driver execution.
- Atomic tasks: AT-2026-05-17-220
- **Status:** complete

### Phase 96: Kernel Jobs Execution-Turn Contract

- Outcome: add the minimal module-neutral `kernel-jobs` execution-turn contract and focused fake-driver tests, without wiring concrete downloads execution or terminal runtime completion.
- Atomic tasks: AT-2026-05-17-221
- **Status:** complete

### Phase 97: Shared Runtime Execution Dispatch Boundary

- Outcome: document the next one-shot shared runtime dispatch boundary so the Rust slice can add host-owned snapshot lookup, driver resolution, and one `run(...)` call without scheduler loops, leases, terminal state, downloads IO, transport, frontend, or SQLite schema changes.
- Atomic tasks: AT-2026-05-17-222
- **Status:** complete

### Phase 98: Kernel Jobs One-shot Execution Dispatch

- Outcome: add a TDD-backed `SharedJobRuntimeHost` one-shot dispatch method that loads one snapshot, resolves its driver, and calls `driver.run(...)`, while returning explicit deferred dispositions for missing snapshots/drivers and keeping scheduler loops, leases, terminal state, downloads IO, transport, frontend, and SQLite schema out of scope.
- Atomic tasks: AT-2026-05-17-223
- **Status:** complete

### Phase 99: Downloads Driver Runtime-run Boundary

- Outcome: document how downloads may safely override `JobDriver::run(...)` after shared runtime dispatch exists, including the guard that pending work must not be drained unless an execution-port path is present, while keeping Rust code, concrete IO, retry/backoff, terminal state, transport, frontend, and SQLite schema unchanged.
- Atomic tasks: AT-2026-05-17-224
- **Status:** complete

### Phase 100: Downloads Driver Guarded Run Override

- Outcome: add a guarded `DownloadJobDriver::run(...)` override that defers without draining when no execution port is wired, and accepts a fake/local completed execution turn only through an opt-in segment execution port path, while keeping composition-root wiring, concrete IO, retry/backoff, terminal runtime completion, transport, frontend, and SQLite schema unchanged.
- Atomic tasks: AT-2026-05-17-225
- **Status:** complete

### Phase 101: Downloads Driver Run Deferred Branch Coverage

- Outcome: add focused tests for missing checkpoint, no pending work, and Accepted-only/no checkpoint mutation `DownloadJobDriver::run(...)` branches, while keeping production behavior unchanged unless a branch test exposes a gap.
- Atomic tasks: AT-2026-05-17-226
- **Status:** complete

### Phase 102: Accepted Execution State Projection Boundary

- Outcome: document the first shared-runtime lifecycle mutation after one-shot dispatch: `JobRunDisposition::Accepted` may project a queued snapshot to non-terminal `Running`, while deferred dispatch, leases, terminal state, scheduler loops, downloads IO, transport, frontend, and SQLite schema remain out of scope.
- Atomic tasks: AT-2026-05-17-227
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
81. AT-2026-05-06-087 - completed - added the missing declaration comments to the Fab inventory event union and its variants while preserving the existing event payload shape and adjacent comments.
82. AT-2026-05-06-088 - completed - added the missing declaration comments to the Fab query DTOs while preserving their existing payload shape and adjacent comments.
83. AT-2026-05-06-089 - completed - added the missing declaration comments to the Fab read-model DTOs while preserving their existing payload shape and adjacent comments.
84. AT-2026-05-06-090 - completed - added the missing file-entry and public declaration comments to the Fab provider adapter shell while preserving its current config and constructor surface.
85. AT-2026-05-06-091 - completed - added the missing file-entry and `SqliteStorageAdapterConfig` declaration comments to the SQLite storage adapter shell while preserving existing repository behavior.
86. AT-2026-05-06-092 - completed - added the missing declaration comments to the SQLite Fab projection repository shell while preserving its current stub behavior and config wiring.
87. AT-2026-05-07-093 - completed - added the missing declaration comments to the SQLite Fab sync cursor repository shell while preserving its current config wiring and placeholder behavior.
88. AT-2026-05-07-094 - completed - added the missing declaration comments to the SQLite Fab media metadata repository shell while preserving its current config wiring and placeholder behavior.
89. AT-2026-05-07-095 - completed - added the missing declaration comments to the SQLite download job repository shell while preserving its current config wiring and persistence behavior.
90. AT-2026-05-07-096 - completed - added the missing declaration comments to the SQLite download checkpoint repository shell while preserving its current config wiring and persistence behavior.
91. AT-2026-05-08-097 - completed - added the missing declaration comments to the SQLite job snapshot store shell while preserving its current persistence and recovery behavior.
92. AT-2026-05-08-098 - completed - added the missing declaration comments to the kernel foundation error contract while preserving its current projection semantics and constructor behavior.
93. AT-2026-05-08-099 - completed - added the missing declaration comments to the kernel foundation paging contract while preserving its current cursor and page-shape semantics.
94. AT-2026-05-08-100 - completed - added the missing declaration comments to the kernel foundation result alias while preserving its current error binding semantics.
95. AT-2026-05-08-101 - completed - added the missing crate-entry comment to kernel foundation while preserving its current module and export wiring.
96. AT-2026-05-08-102 - completed - added the missing clock-boundary comments to kernel foundation while preserving the current clock trait and system clock behavior.
97. AT-2026-05-08-103 - completed - added the missing time-wrapper comments to kernel foundation while preserving its current UTC wrapper and conversion semantics.
98. AT-2026-05-08-104 - completed - added the missing id-wrapper comments to kernel foundation while preserving its current string wrapper and UUID generation semantics.
99. AT-2026-05-08-105 - completed - added the missing crate-entry comment to kernel-jobs while preserving its current module and re-export wiring.
100. AT-2026-05-08-106 - completed - added the missing state-enum comments to kernel-jobs while preserving the current enum values and UI-state projection semantics.
101. AT-2026-05-08-107 - completed - added a repo-root `.windsurfrules` file that maps the current strict-doc and `.artifacts/ai` protocol into plain Windsurf rules without creating a second workflow source.
102. AT-2026-05-08-108 - completed - moved the Windsurf rules surface from root `.windsurfrules` into `.windsurf/rules/repo-workflow.md` so the workspace uses the requested folder-based entrypoint without parallel rule files.
103. AT-2026-05-14-109 - completed - repaired repo-local PWF doctor path detection so `.artifacts/ai/task-plan.md`, `progress.md`, and `findings.md` are recognized as the active plan files without moving them to the repo root.
104. AT-2026-05-14-110 - completed - added Chinese declaration comments to `JobPriority` and `JobProgress`, then validated the slice with `cargo check -p launcher-kernel-jobs --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --lib` after Rust was installed.
105. AT-2026-05-14-111 - completed - added Chinese declaration comments to `AcceptedJob` and `EnqueueJobRequest<E>` while preserving their field shape, serde defaults, and generic extension boundary.
106. AT-2026-05-14-112 - completed - added Chinese declaration comments to `RestoreDisposition` and its variants while preserving restore result semantics.
107. AT-2026-05-14-113 - completed - added Chinese declaration comments to `JobSnapshot<E>` and its fields while preserving snapshot field shape, serde defaults, and generic extension boundary.
108. AT-2026-05-14-114 - completed - added Chinese declaration comments to `JobSnapshotDto` and its fields while preserving DTO field shape and conversion behavior.
109. AT-2026-05-14-115 - completed - added Chinese declaration comments to `RuntimeQueuePolicy` and its constructor while preserving queue policy behavior.
110. AT-2026-05-14-116 - completed - added Chinese declaration comments to `JobDriver<E>` and its methods while preserving driver routing and restore behavior.
111. AT-2026-05-14-117 - completed - added Chinese declaration comments to `JobDriverRegistry<E>` and its public methods while preserving registry ownership and lookup behavior.
112. AT-2026-05-14-118 - completed - added Chinese declaration comments to `JobSnapshotStore<E>` and its methods while preserving persistence and recovery-query behavior.
113. AT-2026-05-14-119 - completed - added Chinese declaration comments to `InMemoryJobSnapshotStore` and its shared state while preserving lock and store behavior.
114. AT-2026-05-14-120 - completed - added Chinese declaration comments to `SharedJobRuntimeHost` and its constructor/accessor surface while preserving host behavior.
115. AT-2026-05-14-121 - completed - added Chinese declaration comments to `JobRuntime` and its control methods while preserving runtime control behavior.
116. AT-2026-05-14-122 - completed - localized `EngineJobDriver` restore driver comments while preserving engines placeholder recovery behavior.
117. AT-2026-05-14-123 - completed - localized downloads query contract comments and added the missing `job_id` field comment while preserving query DTO shape.
118. AT-2026-05-14-124 - completed - localized SQLite `job_snapshots.recoverable` migration comments while preserving migration SQL and ignored-error behavior.
119. AT-2026-05-14-125 - completed - localized desktop host state comments while preserving `Arc` ownership and composition-root service graph boundaries.
120. AT-2026-05-14-126 - completed - localized desktop host crate entry comments while preserving module declarations and public re-exports.
121. AT-2026-05-14-127 - completed - localized desktop bootstrap comments while preserving composition-root wiring and startup behavior.
122. AT-2026-05-14-128 - completed - localized engines transport command comments while preserving handler signature and accepted-job projection behavior.
123. AT-2026-05-14-129 - completed - localized shared jobs transport query comments while preserving `JobSnapshotStore::list_resumable` semantics.
124. AT-2026-05-14-130 - completed - localized Fab transport command comments while preserving `FAB_NOT_WIRED` fallback and accepted-job projection behavior.
125. AT-2026-05-14-131 - completed - localized downloads transport command comments while preserving `DOWNLOADS_NOT_WIRED` fallback and accepted-job projection behavior.
126. AT-2026-05-14-132 - completed - localized Fab restore driver comments while preserving current `RestoreDisposition::Resumed` behavior.
127. AT-2026-05-14-133 - completed - localized the first Fab facade boundary comments while preserving public contracts and behavior.
128. AT-2026-05-14-134 - completed - added Chinese companion comments for Fab facade `()` fallback comments while preserving placeholder accepted-job behavior.
129. AT-2026-05-14-135 - completed - added Chinese companion comments for the next Fab facade method-comment slice while preserving existing English comments.
130. AT-2026-05-14-136 - completed - added Chinese companion comments for the shared transport header and command registry while preserving existing English comments.
131. AT-2026-05-14-137 - completed - added Chinese companion comments for the transport error envelope while preserving existing English comments.
132. AT-2026-05-14-138 - completed - added Chinese companion comments for transport result envelopes and accepted-job DTO while preserving existing English comments.
133. AT-2026-05-14-139 - completed - added Chinese companion comments for the shared transport mapper cluster while preserving existing English comments.
134. AT-2026-05-14-140 - completed - added Chinese companion comments for the composition-root bootstrap entry, config, and facade-only service aggregation while preserving existing English comments.
135. AT-2026-05-14-141 - completed - added Chinese companion comments for the composition-root service graph builder and private helper boundary comments while preserving existing English comments.
136. AT-2026-05-14-142 - completed - added Chinese companion comments for the composition-root startup pipeline production boundary and stage comments while preserving existing English comments.
137. AT-2026-05-14-143 - completed - added the missing Chinese crate-entry comment to composition-root while preserving module declarations and re-exports.
138. AT-2026-05-14-144 - completed - added the missing Chinese binary-entry comment to the desktop host main entry while preserving startup delegation.
139. AT-2026-05-14-145 - completed - added Chinese companion comments for composition-root smoke-test step comments while preserving test behavior.
140. AT-2026-05-14-146 - completed - added Chinese companion comments for startup unit-test driver registry comments while preserving test behavior.
141. AT-2026-05-15-147 - completed - added Chinese companion comments for startup unit-test section headers while preserving test behavior.
142. AT-2026-05-15-148 - completed - recorded backend comment rollout completion and transitioned recovery state toward backend development scope planning.
143. AT-2026-05-15-149 - completed - wired downloads pause/cancel facade methods to shared runtime control ports while leaving resume out of scope.
144. AT-2026-05-15-150 - completed - added downloads start/pause/cancel host transport smoke coverage without changing production behavior or resume semantics.
145. AT-2026-05-15-151 - completed - refreshed the downloads facade file header so it no longer describes pause/cancel as C2 stubs.
146. AT-2026-05-15-152 - completed - marked Phase 28 backend recovery complete and recorded checkpoint-aware `resume_download` as the next design/RED-test candidate.
147. AT-2026-05-15-153 - completed - read the required backend/module docs for `resume_download` and confirmed the next implementation slice must prove explicit checkpoint loading with a RED test before changing behavior.
148. AT-2026-05-15-154 - completed - closed Phase 29 records after AT-153 publication and kept `resume_download` implementation gated on design approval.
149. AT-2026-05-15-155 - completed - added checkpoint-aware `resume_download` RED test and minimal checkpoint read while leaving full resume orchestration out of scope.
150. AT-2026-05-15-156 - completed - added stable missing-checkpoint error semantics to `resume_download` while keeping full resume orchestration out of scope.
151. AT-2026-05-15-157 - completed - added stable missing-job error semantics to `resume_download` while keeping staging, manifest, and runtime resume orchestration out of scope.
152. AT-2026-05-15-158 - completed - added minimal staging validation boundary to `resume_download` while keeping manifest reconstruction and runtime enqueue out of scope.
153. AT-2026-05-15-159 - completed - added downloads module implementation documentation and README_IMPL discoverability rules.
154. AT-2026-05-15-160 - completed - added the minimal downloads resume manifest provider boundary after staging validation while keeping runtime enqueue out of scope.
155. AT-2026-05-15-161 - completed - documented the downloads resume segment/checkpoint/resume-decision data shape before coding completed-segment sealing.
156. AT-2026-05-15-162 - completed - added the minimal completed-segment sealing decision without runtime enqueue or SQLite schema changes.
157. AT-2026-05-15-163 - completed - added the minimal partial-segment resume decision without runtime enqueue or persistence changes.
158. AT-2026-05-15-164 - completed - added focused mismatch rejection coverage for stale segment checkpoint facts without runtime enqueue or persistence changes.
159. AT-2026-05-15-165 - completed - added focused queue-remaining coverage for manifest segments without safe checkpoints before runtime enqueue work.
160. AT-2026-05-15-166 - completed - documented the minimal downloads resume runtime-enqueue boundary in README_IMPL, including job-level request shape, decision mapping, and out-of-scope scheduler/persistence/transport boundaries; committed locally.
161. AT-2026-05-15-167 - completed - added a TDD-backed downloads facade slice proving `resume_download` enqueues the existing job id through job-level runtime when resume decisions contain enqueue candidates and no mismatch rejection, refreshed README_IMPL current state, and committed locally.
162. AT-2026-05-15-168 - completed - added a TDD-backed downloads facade slice proving `resume_download` returns `DL_RESUME_SEGMENT_MISMATCH` and does not runtime enqueue when segment checkpoint facts conflict with the manifest; committed locally.
163. AT-2026-05-16-169 - completed - documented the all-sealed completion boundary and current `AcceptedJob` contract gap before changing `resume_download` behavior; committed locally.
164. AT-2026-05-16-170 - completed - added a module-owned resume outcome for all-sealed plans while preserving current host transport compatibility; committed locally.
165. AT-2026-05-16-171 - completed - projected the module-owned downloads resume outcome through host transport without exposing segment details; committed locally.
166. AT-2026-05-16-172 - completed - documented the downloads-owned resume scheduler/driver payload boundary before Rust implementation; committed locally.
167. AT-2026-05-16-173 - completed - added the minimal downloads-owned resume work plan/payload derivation with focused TDD before scheduler execution or persistence work; committed locally as `1a698f9`.
168. AT-2026-05-16-174 - completed - documented the downloads-owned scheduler/driver boundary before introducing a scheduler port in Rust; committed locally as `6929fa9`.
169. AT-2026-05-16-175 - completed - added the minimal downloads-owned scheduler port and call order before job-level runtime enqueue; committed locally as `8846a40`.
170. AT-2026-05-16-176 - completed - added a focused scheduler-failure guard so scheduler errors skip runtime enqueue; committed locally as `edec23d`.
171. AT-2026-05-16-177 - completed - added a focused all-sealed scheduler guard so already-complete resumes do not touch scheduler/runtime work; committed locally as `31942bd`.
172. AT-2026-05-16-178 - completed - documented the downloads-owned scheduler execution boundary, pending-work queue/scheduler shell as the next Rust slice, and unchanged out-of-scope fetch/write/verify/persistence/transport surfaces; committed locally as `41c3be4`.
173. AT-2026-05-16-179 - completed - added a TDD-backed module-local pending resume work queue/scheduler shell behind `DownloadResumeWorkScheduler`, with focused and full downloads module tests passing; committed locally as `4d0c23b`.
174. AT-2026-05-16-180 - completed - wired `InMemoryDownloadResumeWorkScheduler` into composition-root downloads assembly and proved the smoke path exposes the pending-work scheduler; committed locally as `d3b1b7d`.
175. AT-2026-05-16-181 - completed - documented the downloads driver pending-work consumption boundary and identified the next code slice as job-id-scoped pending-work source/drain semantics without widening runtime, transport, frontend, persistence, or concrete IO.
176. AT-2026-05-16-182 - completed - added a TDD-backed pending resume work source/drain boundary for module-local job-scoped consumption without driver/runtime/transport/frontend/persistence changes.
177. AT-2026-05-16-183 - completed - documented the `DownloadJobDriver` local pending-work consumer boundary and defined AT-184 as a TDD-backed local consumer method without changing shared runtime execution semantics.
178. AT-2026-05-16-184 - completed - added the TDD-backed local `DownloadJobDriver` pending-work consumer and committed locally as `a710cfc`.
179. AT-2026-05-16-185 - completed - documented the composition shared scheduler/source wiring boundary before touching composition-root Rust code.
180. AT-2026-05-16-186 - completed - wired the shared downloads resume scheduler/source through composition-root with focused TDD and committed locally.
181. AT-2026-05-16-187 - completed - documented the downloads checkpoint mutation boundary and next segment-checkpoint persistence slice, then committed locally.
182. AT-2026-05-16-188 - completed - persisted downloads segment checkpoint facts through `SqliteDownloadCheckpointRepository` with focused TDD, then committed locally.
183. AT-2026-05-17-189 - completed - documented the downloads driver execution boundary after durable segment checkpoint persistence; scoped readback/status and `git diff --check` passed with CRLF warnings only, then committed locally.
184. AT-2026-05-17-190 - completed - added downloads driver local execution-turn classification with focused TDD, preserving shared runtime and concrete IO boundaries; then committed locally.
185. AT-2026-05-17-191 - completed - updated README_IMPL with the completed local execution-turn reality and defined the next segment execution ports boundary before coding; then committed locally.
186. AT-2026-05-17-192 - completed - added downloads segment execution request handoff shell with focused TDD, keeping concrete IO/runtime completion out of scope; then committed locally.
187. AT-2026-05-17-193 - completed - documented and added fake segment execution port acceptance with focused TDD, keeping concrete IO/runtime completion out of scope; then committed locally, with initial hash `0655ac2` before PWF backfill amend.
188. AT-2026-05-17-194 - completed - documented and added fake segment completion result contract with focused TDD, keeping checkpoint mutation and concrete IO out of scope; then committed locally, with initial hash `0f8a1a2` before PWF backfill amend.
189. AT-2026-05-17-195 - completed - documented and added fake completed-result checkpoint mutation with focused TDD, keeping SQLite adapter changes, concrete IO, runtime completion, transport, and frontend out of scope; then committed locally, with initial hash `182a34b` before PWF backfill amend.
190. AT-2026-05-17-196 - completed - documented and added fake local resume execution orchestration with focused TDD, keeping runtime `run()`, concrete IO, SQLite adapter changes, transport, and frontend out of scope; then committed locally, with initial hash `3d6f4f7` before PWF backfill amend.
191. AT-2026-05-17-197 - completed - defined the fake segment failure result boundary before Rust coding, keeping public `DL_*` execution errors, runtime completion, concrete IO, SQLite adapter changes, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `83315bf` before PWF backfill amend.
192. AT-2026-05-17-198 - completed - added the fake segment failure result contract with focused TDD, keeping checkpoint mutation, retry/backoff, public error projection, runtime completion, concrete IO, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `c4156bb` before PWF backfill amend.
193. AT-2026-05-17-199 - completed - defined the fake failed-result checkpoint mutation boundary, keeping retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter changes, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `fa71553` before PWF backfill amend.
194. AT-2026-05-17-200 - completed - added fake failed-result checkpoint mutation with focused TDD, keeping retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter changes, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `94573e3` before PWF backfill amend.
195. AT-2026-05-17-201 - completed - defined fake local mixed-result checkpoint orchestration boundary, keeping retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter changes, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `9f6402a` before PWF backfill amend.
196. AT-2026-05-17-202 - completed - updated fake local resume execution to persist failed fake results through existing checkpoint helpers, keeping retry/backoff, public error projection, terminal runtime state, concrete IO, SQLite adapter changes, transport, composition-root, and frontend out of scope; then committed locally, with initial hash `eae3c4f` before PWF backfill amend.
197. AT-2026-05-17-203 - completed - documented the downloads get-job snapshot query boundary before Rust coding, after confirming `list_jobs` and policy surfaces need separate future design; then committed locally, with initial hash `98c491b` before PWF backfill amend.
198. AT-2026-05-17-204 - completed - implemented the downloads get-job snapshot query with focused TDD while keeping list/policy/runtime-list/adapter/transport/frontend surfaces out of scope; then committed locally, with initial hash `d1de743` before PWF backfill amend.
199. AT-2026-05-17-205 - completed - documented the downloads list-jobs query boundary before Rust coding, selecting the downloads-owned job repository page as the first read source; then committed locally, with initial hash `c66d3bb` before PWF backfill amend.
200. AT-2026-05-17-206 - completed - implemented the downloads list-jobs query with focused TDD and SQLite adapter compile support while keeping policy/runtime-list/live-join/transport/frontend surfaces out of scope; then committed locally, with initial hash `87b09ab` before PWF backfill amend.
201. AT-2026-05-17-207 - completed - documented the downloads policy source boundary before Rust coding, keeping runtime queue-policy mutation and persistence integration out of scope; then committed locally, with initial hash `07bdcfb` before PWF backfill amend.
202. AT-2026-05-17-208 - completed - implemented the downloads policy store facade semantics with focused TDD, keeping runtime policy application, SQLite persistence, transport, frontend, concrete IO, retry/backoff, and terminal completion out of scope; committed and pushed as `6d8c022`.
203. AT-2026-05-17-209 - completed - documented the downloads policy SQLite persistence boundary before Rust coding, keeping runtime policy application, global settings, transport, frontend, concrete IO, retry/backoff, and terminal completion out of scope; committed and pushed as `41f0b8c`.
204. AT-2026-05-17-210 - completed - implemented the downloads SQLite policy store with focused TDD, project-local test database paths, and composition-root wiring while keeping runtime policy application, global settings, transport, frontend, concrete IO, retry/backoff, pending work, and terminal completion out of scope; committed and pushed as `2f9e828`.
205. AT-2026-05-17-211 - completed - documented the downloads runtime policy application boundary before Rust coding, selecting startup seeding from persisted policy as the first safe slice while deferring live runtime mutation, scheduler work, transport, frontend, concrete IO, retry/backoff, and terminal completion; committed and pushed as `1d31f56`.
206. AT-2026-05-17-212 - completed - implemented startup seeding from persisted downloads policy into the initial shared runtime queue policy with focused composition-root TDD, while keeping live runtime mutation, scheduler work, transport, frontend, concrete IO, retry/backoff, and terminal completion out of scope; committed and pushed as `ed27996`.
207. AT-2026-05-17-213 - completed - documented the live runtime policy update boundary before Rust coding, selecting a kernel-jobs `SharedJobRuntimeHost` policy-control surface as the first safe slice while deferring downloads facade wiring, transport, frontend, scheduler work, active runtime mutation, concrete IO, retry/backoff, and terminal completion; committed and pushed as `38c32b2`.
208. AT-2026-05-17-214 - completed - added the kernel-jobs runtime policy control surface with focused TDD, keeping policy reads by-value while sharing updates across cloned runtime handles and deferring downloads facade wiring, composition-root wiring, host transport, frontend, scheduler work, active job/lease/snapshot changes, pending work, concrete IO, retry/backoff, and terminal completion; committed and pushed as `c92be25`.
209. AT-2026-05-17-215 - completed - documented the downloads runtime policy applier boundary before Rust coding, selecting a downloads-owned facade applier port as the next safe slice while deferring composition-root concrete wiring, host transport, frontend, scheduler work, active runtime mutation, concrete IO, retry/backoff, and terminal completion; committed and pushed as `4ef3f10`.
210. AT-2026-05-17-216 - completed - added the downloads-owned runtime policy applier port with focused TDD while keeping composition-root concrete wiring, direct shared-runtime mutation from downloads code, transport, frontend, scheduler work, active runtime mutation, concrete IO, retry/backoff, and terminal completion out of scope; committed and pushed as `1094c10`.
211. AT-2026-05-17-217 - completed - wired the concrete downloads runtime policy applier in composition-root with focused TDD while keeping host transport, frontend, scheduler work, active runtime mutation, concrete IO, retry/backoff, and terminal completion out of scope; committed and pushed as `37765ef`.
212. AT-2026-05-17-218 - completed - added documentation-budget rules to keep task logs in `.artifacts/ai` and reserve `docs/` updates for durable boundaries, contracts, models, error semantics, wiring, and validation rules; committed and pushed as `5aae7f1`.
213. AT-2026-05-17-219 - completed - added a host transport smoke assertion proving `downloads_update_policy` persists the policy and applies the shared runtime policy snapshot through existing composition-root wiring; committed and pushed as `f618718`.
214. AT-2026-05-17-220 - completed - documented the shared runtime execution-turn boundary before coding a minimal `kernel-jobs` run contract; committed and pushed as `aa8d6e3`.
215. AT-2026-05-17-221 - completed - added the minimal `kernel-jobs` execution-turn contract with focused fake-driver TDD while keeping downloads execution, IO, retry/backoff, leases, terminal completion, transport, frontend, and SQLite schema out of scope; committed and pushed as `89d3a19`.
216. AT-2026-05-17-222 - completed - documented the one-shot shared runtime execution dispatch boundary before Rust coding; committed and pushed as `feddcfc`.
217. AT-2026-05-17-223 - completed - added one-shot `SharedJobRuntimeHost` execution dispatch with focused TDD; committed and pushed as `f87df03`.
218. AT-2026-05-17-224 - completed - documented the downloads driver runtime-run override boundary; committed and pushed as `597c0e5`.
219. AT-2026-05-17-225 - completed - added guarded downloads driver `run(...)` override with focused TDD; committed and pushed as `c5b0695`.
220. AT-2026-05-17-226 - completed - covered guarded downloads driver `run(...)` deferred branches and fixed the non-mutating checkpoint helper gap; committed and pushed as `d2d5405`.
221. AT-2026-05-17-227 - completed - defined the accepted execution state projection boundary before `kernel-jobs` Rust changes; commit/push pending.
90. AT-2026-05-07-096 - completed - added the missing declaration comments to the SQLite download checkpoint repository shell while preserving its current config wiring and checkpoint persistence behavior.

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

1. Commit and push AT-2026-05-17-209, then implement the documented SQLite policy store slice if validation is clean.
2. Leave unrelated dirty frontend, pen, sqlite, Cargo.lock, `.codex`, and `src/` changes untouched unless the user explicitly scopes them into a task.

## Legacy Note

1. The old root planning history now lives under `.artifacts/ai/legacy-root-planning/` and is not the source of truth for new tasks.
2. Root `task_plan.md`, `progress.md`, and `findings.md` were removed from repo root in AT-2026-05-03-005 to prevent the legacy workflow from re-entering active execution paths.
