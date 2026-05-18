# AI Findings & Decisions

## Requirements

- Repository-local slash commands should use an `hsy-XXX` prefix to avoid collisions with other command surfaces.
- New or revised code comments should default to simplified Chinese.
- Other developers must be able to request English comments explicitly, ideally through a slash command instead of an implicit convention.
- Keep `.artifacts/ai` as the only authoritative workflow record set.
- Preserve the repo's single-active-task protocol and compatibility with existing hook parsing.
- Normalize the live records so they look intentional and match the repo-local planning-with-files workflow.

## Research Findings

- AT-2026-05-16-178 pre-edit read confirmed `docs/modules/downloads/README_IMPL.md` currently says scheduler port and work-plan derivation exist, but concrete scheduler execution, persistence, host transport, frontend projection, and `kernel-jobs` payload changes remain later slices until a dedicated implementation task scopes them.
- Downloads module ARCH/API/FLOW documents keep frontend and host-facing surfaces on aggregate projections/control intents; segment, checkpoint, scheduler, fetcher, writer, and verifier details stay backend-owned and must not leak into UI state.
- `docs/TauriDownloadRuntimeDesign.md` separates `DownloadScheduler`, `SegmentFetcherPool`, `SegmentWriter`, and `SegmentVerifier`, and the resume flow requires sealed completed segments, partial range resume where safe, and queueing only remaining segments.
- `docs/TauriKernelJobsRuntimeDesign.md` keeps module business checkpoints and segment plans out of `kernel-jobs`; the shared runtime owns job-level lifecycle/snapshot/lease semantics while `module-downloads` owns resume reconstruction and concrete download driver behavior.
- Docs-only validation for AT-178 should use path/reference readback, scoped `git diff --check`, and path-limited `git status --short`; no Rust tests are required unless the task adds behavior or mandatory test gates.
- AT-2026-05-16-179 pre-code read confirmed the next implementation slice should be `module-downloads` only: a pending resume work queue/scheduler shell behind `DownloadResumeWorkScheduler`, with no HTTP fetch, staging writes, hash verification, checkpoint mutation, SQLite schema, host transport, frontend IPC, or `kernel-jobs` segment payload changes.
- AT-2026-05-16-179 implementation confirmed the minimal shell can stay entirely inside `crates/module-downloads/src/facade/mod.rs`: `DownloadPendingResumeWork` stores job id + transient work plan, and `InMemoryDownloadResumeWorkScheduler` implements the existing scheduler port with an in-memory queue.
- AT-2026-05-16-180 pre-code read confirmed composition-root still wires downloads with `resume_scheduler: ()` while README_IMPL identifies composition wiring for `InMemoryDownloadResumeWorkScheduler` as one of the next reassessment options; the safe slice is assembly-only and should not add driver consumption or transport behavior.
- AT-2026-05-16-180 implementation confirmed composition-root can use `InMemoryDownloadResumeWorkScheduler` as the concrete downloads scheduler dependency without changing `DesktopAppServices`, host transport, SQLite schema, driver behavior, or `kernel-jobs`.

- The user explicitly prefers the Windsurf rule surface under `.windsurf/rules` rather than a repo-root `.windsurfrules` file.
- Because `.github/skills/strict-doc-driven-development/SKILL.md` carries Copilot-specific frontmatter such as `name`, `user-invocable`, and `allowed-tools`, the portable move remains to restate its operational rules in plain text instead of trying to reuse that file verbatim in Windsurf.
- Once a folder-based Windsurf rule surface is requested, keeping both `.windsurfrules` and `.windsurf/rules/*` would create avoidable drift, so the safer move is a one-file relocation with root file deletion.
- A single `.windsurf/rules/repo-workflow.md` file is enough for the current repository because the existing Windsurf compatibility content is one cohesive workflow rule set rather than multiple independently scoped rule families.
- The Windsurf translation must preserve `.artifacts/ai` as the only authoritative workflow record set and must not revive root `task_plan.md`, `progress.md`, or `findings.md` as active files.

- `pwf-doctor` originally reported `active plan: missing` and `planning files: missing` even though `.artifacts/ai/active-task.md`, `task-plan.md`, `progress.md`, `findings.md`, and `handoff.md` were present.
- The root cause was a path resolver mismatch: `.codex/hooks/planning_state.py` only recognized `.planning/<plan>/task_plan.md` or legacy root `task_plan.md`, while this repo intentionally keeps the active task plan at `.artifacts/ai/task-plan.md`.
- Moving `.artifacts/ai` files back to the repo root would revive the legacy planning surface and still would not match every repo-specific filename, so the safer fix is to make the repo-local resolver understand `.artifacts/ai`.
- The successful repair is to recognize `.artifacts/ai/task-plan.md` as the active plan directory and map `PlanningPaths.task_plan` to the existing hyphenated filename when root artifacts planning is active.

- After AT-2026-05-14-109, the stop hook correctly reported Phase 23 as the remaining incomplete phase rather than a failure of the PWF doctor repair.
- `crates/kernel-jobs/src/model.rs` remains the right local area for the next backend comment rollout slice because AT-2026-05-08-106 only covered `JobState` and `JobUiState`.
- `JobPriority` plus `JobProgress` is the smallest next public contract cluster in that file; `AcceptedJob`, `EnqueueJobRequest`, `RestoreDisposition`, `JobSnapshot`, and `JobSnapshotDto` should remain separate follow-up slices.
- The current shell cannot run the Rust validation gate because `cargo` is not available through PowerShell, cmd, `rustup`, the PATH, or the checked user/default install locations, so AT-2026-05-14-110 must remain blocked until `cargo` is exposed.
- After the user installed Rust through `rustup-init`, `cargo 1.95.0` and `rustc 1.95.0` became available in PATH, and the `launcher-kernel-jobs` lib check passed for AT-2026-05-14-110.
- After AT-2026-05-14-110, `AcceptedJob` and `EnqueueJobRequest<E>` became the next smallest public contract cluster in `crates/kernel-jobs/src/model.rs` because they share the job acceptance/enqueue boundary and are smaller than the restore/snapshot model cluster.
- The safe AT-2026-05-14-111 move was to add only Chinese declaration comments to those two structs and their fields, preserving `serde(default = "default_recoverable")`, the extension generic, and the runtime enqueue behavior.
- After AT-2026-05-14-111, `RestoreDisposition` became the next smallest public contract surface in `crates/kernel-jobs/src/model.rs` because it is one restore-result enum and can be documented without opening the broader snapshot model.
- The safe AT-2026-05-14-112 move was to add only Chinese declaration comments to `RestoreDisposition` and its variants, preserving restore result semantics and variant payload shape.

- `crates/kernel-jobs/src/model.rs` is the strongest next slice after the published kernel-jobs crate entry because it is the next smallest production file in that crate, and the shared state vocabulary sits at the top of the file as the smallest documented declaration cluster.
- Within this file, `JobState` and `JobUiState` are the safest immediate candidates because they are smaller than `AcceptedJob`, `EnqueueJobRequest`, and `JobSnapshot`, while still directly defining the shared runtime/UI state semantics.
- The safest move is to add only Chinese declaration comments to those two enums and their variants, leaving enum values, serde rename rules, and the adjacent English `JobSnapshotDto` comment unchanged.
- `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-jobs/src/lib.rs` is the strongest next slice after the published foundation ID contract because it is the smallest remaining production file in the adjacent shared jobs crate, and the jobs runtime design explicitly defines the crate-level public export surface.
- The safest move is to add only a Chinese file-entry comment to `crates/kernel-jobs/src/lib.rs`, leaving `model` / `runtime` module declarations and re-export wiring unchanged.
- `crates/kernel-jobs/src/model.rs` and `crates/kernel-jobs/src/runtime.rs` are not the best immediate candidates because they widen the slice into much larger declaration surfaces than this crate entry file.
- `cargo check -p launcher-kernel-jobs --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/ids.rs` is the strongest next slice after the published foundation time contract because it is the last remaining production contract file in `kernel-foundation`, and `JobId` plus the other shared identifiers are part of the documented minimal foundation API.
- The safest move is to add only Chinese file-entry and generated public declaration comments by threading documentation through the macro-generated ID surface, leaving the current string wrapper pattern, UUID generation, serde transparency, and conversion behavior unchanged.
- This macro-generated file is now a valid immediate candidate because there are no smaller remaining production contract files in `kernel-foundation`.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/time.rs` is the strongest next slice after the published foundation clock contract because it is the smallest remaining direct-declaration foundation contract file and `IsoDateTime` is the shared timestamp wrapper re-exported by the foundation crate.
- The safest move is to add only Chinese file-entry and public declaration comments to `IsoDateTime`, `now()`, and `as_datetime()`, leaving the current UTC wrapper shape, serde transparency, display formatting, and conversion behavior unchanged.
- `crates/kernel-foundation/src/ids.rs` is not the best immediate candidate because its public surface is macro-generated and would widen the explanation surface beyond this one time-wrapper boundary.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/clock.rs` is the strongest next slice after the published foundation crate entry because it is the smallest remaining direct-declaration foundation contract file and `Clock` is explicitly listed in the minimal API sketch in `docs/TauriFirstCrateApiDrafts.md`.
- The safest move is to add only Chinese file-entry and public declaration comments to `Clock`, `Clock::now()`, and `SystemClock`, leaving the current timestamp source and system-clock behavior unchanged.
- `crates/kernel-foundation/src/time.rs` is a weaker immediate candidate because it exposes a broader timestamp wrapper surface than this one-trait clock boundary, and `crates/kernel-foundation/src/ids.rs` remains weaker because its public surface is macro-generated.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/lib.rs` is the strongest next slice after the published foundation result alias because it is the smallest remaining crate-entry aggregation surface and the foundation crate/module layout is explicitly defined in `docs/TauriFirstCrateApiDrafts.md`.
- The safest move is to add only a Chinese file-entry comment to `crates/kernel-foundation/src/lib.rs`, leaving module declarations and re-export wiring unchanged.
- `crates/kernel-foundation/src/ids.rs` is not the best immediate candidate because its public surface is macro-generated and would widen the explanation surface beyond this single entry file.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file entry surface.

- `crates/kernel-foundation/src/result.rs` is the strongest next slice after the published foundation paging contract because it is the smallest remaining public foundation contract file and `AppResult<T>` is explicitly listed as required minimal API in both `docs/TauriFirstCrateApiDrafts.md` and `docs/TauriBackendSkeletonImplementationDesign.md`.
- The safest move is to add only Chinese declaration comments to the `AppResult<T>` alias and the file entry, leaving its `Result<T, AppError>` binding unchanged.
- `crates/kernel-foundation/src/ids.rs` and `crates/kernel-foundation/src/lib.rs` are not the best immediate candidates because they expose broader or macro-generated public surfaces than this one-line result alias slice.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/paging.rs` is the strongest next slice after the published foundation error contract because it is the smaller direct-declaration contract file in the same crate and its shared paging request/result semantics are explicitly sketched in `docs/TauriFirstCrateApiDrafts.md`.
- The safest move is to add only Chinese declaration comments to `PageCursor`, `PageRequest`, `PageSlice<T>`, their public fields, and their public constructors/accessors, leaving cursor shape, serde behavior, and pagination semantics unchanged.
- `crates/kernel-foundation/src/ids.rs` is not the best immediate candidate because its public surface is macro-generated and would widen the explanation surface beyond this one direct paging contract file.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `crates/kernel-foundation/src/error.rs` is the strongest next slice after the published SQLite storage adapter chain because it is a one-file public foundation contract that directly owns the stable error projection semantics used across backend modules.
- The safest move is to add only Chinese declaration comments to `AppErrorSeverity`, its public variants, `AppError`, its public fields, and `AppError::new()`, leaving enum shape, serde rename rules, field meanings, and constructor behavior unchanged.
- `crates/kernel-foundation/src/ids.rs` and `crates/kernel-foundation/src/paging.rs` are not the best immediate candidates because they expose broader public surfaces than this error contract slice.
- `cargo check -p launcher-kernel-foundation --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor for this one-file contract surface.

- `SqliteJobSnapshotStore` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published download checkpoint repository shell because it is the last remaining public storage shell in the file and directly owns the shared runtime snapshot persistence boundary defined by the kernel-jobs runtime design.
- The safest move is to add only Chinese declaration comments to `SqliteJobSnapshotStore` and `new()`, leaving snapshot schema initialization, mutex ownership, serialization, and recovery query behavior unchanged.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `SqliteDownloadCheckpointRepository` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published download job repository shell because it is the next smallest public declaration cluster in the same file and directly owns persisted checkpoint facts defined by the download runtime and storage docs.
- The safest move is to add only Chinese declaration comments to `SqliteDownloadCheckpointRepository`, `new()`, `config()`, `load_checkpoint()`, and `save_checkpoint()`, leaving checkpoint persistence behavior, config wiring, and the lower snapshot store unchanged.
- `SqliteJobSnapshotStore` is not the best next candidate for this round because it would widen the slice beyond this one adjacent download-checkpoint repository boundary.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `SqliteDownloadJobRepository` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published Fab media metadata repository shell because it is the next smallest public declaration cluster in the same file and directly owns persisted download job facts defined by the download runtime and storage docs.
- The safest move is to add only Chinese declaration comments to `SqliteDownloadJobRepository`, `new()`, `config()`, and `get_job()`, leaving download job persistence behavior, config wiring, and all lower repository shells unchanged.
- `SqliteDownloadCheckpointRepository` and `SqliteJobSnapshotStore` are not the best next candidates for this round because they would widen the slice beyond this one adjacent download-job repository boundary.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `SqliteFabMediaMetadataRepository` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published Fab sync cursor repository shell because it is the next smallest public declaration cluster in the same file and directly matches the `fab_media_metadata_projection` storage boundary from the Fab loading and storage docs.
- The safest move is to add only Chinese declaration comments to `SqliteFabMediaMetadataRepository`, `new()`, and `config()`, leaving media persistence behavior, config wiring, and all lower repository shells unchanged.
- The lower download/job repository shells are not the best next candidates for this round because they would widen the slice beyond this one adjacent Fab media metadata boundary.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `SqliteFabSyncCursorRepository` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published Fab projection repository shell because it is the next smallest public declaration cluster in the same file and directly matches the `fab_sync_cursor` storage boundary from the Fab loading and storage docs.
- The safest move is to add only Chinese declaration comments to `SqliteFabSyncCursorRepository`, `new()`, and `config()`, leaving cursor persistence behavior, config wiring, and all lower repository shells unchanged.
- `SqliteFabMediaMetadataRepository` and the lower download/job repository shells are not the best next candidates for this round because they would widen the slice beyond this one adjacent Fab sync cursor boundary.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `SqliteFabInventoryProjectionRepository` in `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published shared config cluster because it is the next smallest public declaration cluster in the same file and directly owns the local Fab projection read path.
- The safest move is to add only Chinese declaration comments to `SqliteFabInventoryProjectionRepository`, `new()`, and `config()`, leaving the current empty-page/detail-`None` stub behavior unchanged.
- `SqliteFabSyncCursorRepository`, `SqliteFabMediaMetadataRepository`, and the lower download/job repository shells are not the best next candidates for this round because they would widen the slice beyond this one adjacent Fab projection boundary.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` remains the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `crates/adapter-storage-sqlite/src/lib.rs` is the strongest next slice after the published Fab provider adapter shell because it is the next same-class adapter entry file and its shared `SqliteStorageAdapterConfig` cluster still lacks both a file-entry explanation and declaration comments.
- The smallest safe move here is to add only the file-entry comment plus declaration comments for `SqliteStorageAdapterConfig`, `new()`, and `database_path()`, leaving the lower Fab/download/job repository shells for later slices.
- `SqliteFabInventoryProjectionRepository` and the lower repository adapters are not the best next candidates for this round because they would widen the slice into a much larger multi-declaration annotation pass.
- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` is the narrowest reliable validation gate for this slice because the crate currently exposes no smaller named test anchor and this check compiles the touched public shell surface.

- `crates/adapter-provider-fab/src/lib.rs` is the strongest next slice after the published Fab read-model DTO surface because it is a Fab-adjacent shell-first adapter entry file whose public config and adapter declarations still have no file-entry or declaration comments at all.
- `src-tauri/src/state.rs` and `crates/module-fab/src/facade/mod.rs` are not valid missing-comment-only candidates under the user's current rule because they already carry acceptable English comments; rewriting them would add churn without filling a real gap.
- The safest move is to add only Chinese file-entry and declaration comments to `EpicFabCatalogProviderConfig`, its constructor/getters, `EpicFabCatalogProviderAdapter`, and its constructor/getter, leaving remote auth, HTTP wiring, and payload mapping deferred.
- `cargo check -p launcher-adapter-provider-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml --lib` is the narrowest reliable validation gate for this slice because the crate currently exposes only a small public shell surface and has no narrower named test anchor.

- `crates/module-fab/src/contracts/dto.rs` is the strongest next slice after the published query contracts because it is the remaining adjacent Fab contracts file in this local area and still exposes uncommented public read-model declarations.
- The safest move is to add only Chinese declaration comments to `FabInventoryItemDto`, `FabInventoryPageDto`, and `FabAssetDetailDto`, leaving their field shape and alias wiring unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` remains the narrowest current executable validation gate for this Fab read-model contract slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-fab/src/contracts/queries.rs` is the strongest next slice after the published event contracts because it is the next adjacent Fab contracts file and still remains smaller than the read-model DTO file while exposing two uncommented public query DTOs.
- `crates/module-fab/src/contracts/dto.rs` is not the best next candidate for this round because it would widen the slice into a broader read-model annotation pass.
- The safest move is to add only Chinese declaration comments to `FabInventoryListQueryDto` and `FabAssetDetailQueryDto`, leaving their field shape and serde wiring unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` remains the narrowest current executable validation gate for this Fab query-contract slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-fab/src/contracts/events.rs` is the strongest next slice after the published command contracts because it is the smallest remaining adjacent Fab contracts file and exposes only one public enum with two public variants.
- `crates/module-fab/src/contracts/queries.rs` and `crates/module-fab/src/contracts/dto.rs` are not the best next candidates for this round because they would widen the slice into larger query/read-model annotation passes.
- The safest move is to add only Chinese declaration comments to `FabInventoryEventDto` and its two public variants, leaving the event payload shape and serde tagging unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` remains the narrowest current executable validation gate for this Fab event-contract slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-fab/src/contracts/commands.rs` is the strongest next slice after the published contracts entry because it is the next adjacent Fab contracts file and currently exposes only two uncommented public command DTOs.
- `crates/module-fab/src/contracts/dto.rs` is not the best next candidate for this round because it would widen the slice into a broader read-model annotation pass with more public types.
- The safest move is to add only Chinese declaration comments to `FabInventoryPrewarmRequestDto` and `FabInventorySyncRequestDto`, leaving their field shape and serde wiring unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` remains the narrowest current executable validation gate for this Fab command-contract slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-fab/src/contracts/mod.rs` is the strongest next slice after the published fab crate entry because it is the next adjacent public aggregation boundary inside the same module and still has no file-entry explanation.
- `crates/module-fab/src/contracts/commands.rs` and `crates/module-fab/src/contracts/dto.rs` also expose uncommented public DTO declarations, but they are larger multi-declaration candidates and therefore weaker than the smaller same-class `contracts/mod.rs` entry slice.
- The safest move is to add only a Chinese file-entry comment to `crates/module-fab/src/contracts/mod.rs`, leaving its `pub mod` and `pub use` export wiring unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` remains the narrowest current executable validation gate for this Fab contracts-entry slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-fab/src/lib.rs` is the strongest next slice after the published downloads crate entry because it is the remaining same-class backend crate-entry boundary that still has no file-entry explanation.
- `src-tauri/src/commands/downloads.rs` and `crates/module-downloads/src/contracts/queries.rs` are not valid missing-comment-only candidates under the user's current rule because their public/module-level comments are already acceptable and should remain untouched.
- The safest move is to add only a Chinese file-entry comment to `crates/module-fab/src/lib.rs`, leaving its `pub mod` and `pub use` export wiring unchanged.
- `cargo test -p launcher-module-fab --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` is the narrowest current executable validation gate for this adjacent crate-entry slice because the fab crate has a small package-local test surface but no narrower named test anchor was identified during the local scan.

- `crates/module-downloads/src/lib.rs` is the strongest next slice after the published engines crate entry because it is the next small backend crate-entry boundary that still has no file-entry explanation.
- `crates/module-downloads/src/contracts/queries.rs` is not a valid missing-comment-only candidate under the user's current rule because its module-entry and public DTO declarations already carry acceptable English comments that should remain untouched.
- The safest move is to add only a Chinese file-entry comment to `crates/module-downloads/src/lib.rs`, leaving its `pub mod` and `pub use` export wiring unchanged.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` is the narrowest executable validation gate for this adjacent crate-entry slice because it compiles the public downloads surface through the currently wired facade intake path.

- `crates/module-engines/src/lib.rs` is the strongest next slice after the published driver entry because it is the adjacent crate-entry boundary and still has no file-entry explanation even though its submodule surfaces are now individually documented.
- The safest move is to add only a Chinese file-entry comment that explains the engines module's public export surface, while leaving the current `pub mod` and `pub use` lines untouched.
- The named engine verification unit test remains blocked by the same pre-existing missing `JobPriority` import in `crates/module-engines/src/facade/mod.rs` test code, so `cargo check --manifest-path q:\DEV\MyEpicLauncher\crates\module-engines\Cargo.toml --lib` remains the narrowest reliable validation gate for this adjacent comment-only slice.

- `crates/module-engines/src/driver.rs` is the strongest next slice after the published facade boundary because it is the adjacent driver entry file and still lacks a file-entry comment, while its public `EngineJobDriver` declaration already has a correct English comment that should remain untouched.
- The safest move is to add only a Chinese file-entry comment that explains the driver boundary and current stub restore ownership, without rewriting the existing English struct comment or the inline TODO/body comments.
- The named engine verification unit test is still blocked by the same pre-existing missing `JobPriority` import in `crates/module-engines/src/facade/mod.rs` test code, so `cargo check --manifest-path q:\DEV\MyEpicLauncher\crates\module-engines\Cargo.toml --lib` remains the narrowest reliable validation gate for this adjacent comment-only slice.

- `crates/module-engines/src/facade/mod.rs` is the strongest next slice after the contracts file because it is the adjacent public engines boundary and its dependency bundle, facade type, and public methods still expose no declaration comments.
- The safest change is to add Chinese comments only to the public dependency bundle, facade type, public fields, and public methods, while leaving the short private `not_wired` helper and the currently blocked test code untouched.
- The named `run_verification_returns_backend_owned_accepted_job` unit test is still blocked by the same pre-existing missing `JobPriority` import in the test module, so `cargo check --manifest-path q:\DEV\MyEpicLauncher\crates\module-engines\Cargo.toml --lib` remains the narrowest reliable validation gate for this adjacent comment-only slice.

- `src-tauri/src/commands/downloads.rs` is no longer a valid missing-comment-only candidate under the user's current rule because its public handler declarations already carry acceptable comments; rewriting those English comments would now violate the stated preference.
- `crates/module-engines/src/contracts/mod.rs` is the strongest next slice because it is a one-file backend contracts boundary whose three public DTO declarations currently have no declaration comments at all.
- The safest change is to add Chinese comments only to `ListEnginesRequestDto`, `GetEngineStatusRequestDto`, and `RunEngineVerificationRequestDto`, leaving the field and payload shape unchanged.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-engines\Cargo.toml run_verification_returns_backend_owned_accepted_job` would normally be the narrowest executable validation for this slice, but it is currently blocked by a pre-existing missing `JobPriority` import in `crates/module-engines/src/facade/mod.rs` test code.
- `cargo check --manifest-path q:\DEV\MyEpicLauncher\crates\module-engines\Cargo.toml --lib` is therefore the narrowest reliable validation gate for this comment-only slice because it compiles the touched contracts file without widening scope into unrelated test repair.

- `crates/module-downloads/src/facade/mod.rs` remains the strongest next missing-comment slice because the file already has high-level boundary comments, but `DownloadJobRecordState` still exposes uncommented enum variants that carry persisted-state meaning.
- The smallest safe move here is to add Chinese comments only to those enum variants and leave the surrounding acceptable comments untouched.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` is the narrowest executable validation for this slice because it compiles the touched facade surface through the only currently wired facade intake path.

- The user clarified that already-correct English comments do not need to be deleted or rewritten; the rollout should now target only missing declaration comments, while new comments still default to simplified Chinese.
- `crates/module-downloads/src/driver.rs` is the strongest next slice under that rule because its checkpoint record and repository declarations still lack comments, while the main restore-driver comment block is already acceptable and does not need rewriting.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml restore_returns_failed_when_checkpoint_is_missing` is the narrowest executable validation for this slice because it directly compiles and exercises the touched checkpoint/restore boundary.

- `crates/module-downloads/src/contracts/events.rs` is the strongest next rewrite slice after `dto.rs` because it is the next adjacent contracts file, still carries fully English declaration comments, and can validate through the same narrow downloads module test.
- The change here should rewrite the event-union and lifecycle-payload comments into Chinese without changing the event variants, payload fields, or transport-safe semantics.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` remains the narrowest executable validation for this slice because it compiles the touched event contracts through the public downloads module.

- `crates/module-downloads/src/contracts/dto.rs` is the strongest next rewrite slice after `commands.rs` because it is the next adjacent contracts file, still carries fully English declaration comments, and can validate through the same narrow downloads module test.
- The change here should rewrite the read-model DTO, projection-field, and policy-snapshot comments into Chinese without changing the DTO field names, projected transport shape, or state semantics.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` remains the narrowest executable validation for this slice because it compiles the touched read-model contracts through the public downloads module.

- `crates/module-downloads/src/contracts/commands.rs` is the strongest next rewrite slice after `contracts/mod.rs` because it is the next adjacent contracts file, still carries fully English declaration comments, and can validate through the same narrow downloads module test.
- The change here should rewrite the command request and policy-field comments into Chinese without changing any DTO field names, accepted payload shape, or scheduling semantics.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` remains the narrowest executable validation for this slice because it compiles the touched command contracts through the public downloads module.

- `crates/module-downloads/src/contracts/mod.rs` is the strongest next rewrite slice after `driver.rs` because it is a tiny adjacent module-entry file with fully English declaration comments and no behavior logic to reopen.
- The change here should rewrite the contracts aggregation comments into Chinese without changing what the file re-exports or how callers import the downloads contract surface.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` is the narrowest executable validation for this slice because it compiles the touched contracts boundary through the public downloads module.

- `crates/module-downloads/src/driver.rs` is the strongest first rewrite slice for old English backend comments because it has one compact restore-driver comment block, sits adjacent to the just-finished downloads facade slice, and can validate through a driver-specific unit test.
- The change here should rewrite existing restore/checkpoint comments into Chinese without changing the driver's current checkpoint-gated restore semantics.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml restore_returns_failed_when_checkpoint_is_missing` is the narrowest executable validation for this slice because it compiles the touched file and directly exercises the commented restore boundary.

- `crates/module-downloads/src/facade/mod.rs` is the strongest next backend comment slice because it is the downloads module's public intake boundary and still exposes the accepted-job record model, dependency bundle, and multiple `DOWNLOADS_NOT_WIRED` facade methods without declaration-level comments.
- The comments here should focus on the facade boundary, dependency bundle, persisted intake record semantics, and the ownership of the still-stubbed `DOWNLOADS_NOT_WIRED` operations, not on obvious test helpers or every one-line delegation.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority` is the narrowest executable validation for this slice because it compiles the touched facade surface and exercises the only currently wired facade command path.

- Before AT-2026-05-05-070, all six current workspace prompt commands still used unprefixed names; the normalized surface is now `hsy-plan-atomic-task`, `hsy-plan-backend-skeleton`, `hsy-plan-doc-review`, `hsy-resume-from-handoff`, `hsy-comment-zh`, and `hsy-comment-en`.
- The live references to these command names are narrow and local: the prompt files themselves, the comment standard's comment-language section, and the current `.artifacts/ai` task records.
- Renaming both the prompt filenames and frontmatter `name` fields to `hsy-XXX` is sufficient for a consistent repo-local command surface; hook scripts do not depend on these names.
- The repository already has `.artifacts/ai/language-mode.txt` plus `MYEPIC_WORKFLOW_LANG`, but those surfaces currently control workflow and hook language, not source-code comment language.
- The repository already exposes named slash commands through `.github/prompts/*.prompt.md`, so the least disruptive collision-avoidance move is to keep that surface and normalize the command names under an `hsy-` prefix.
- `docs/TauriCodeCommentStandard.md` currently defines comment coverage, syntax, and concurrency expectations, but it does not yet define the language of comment text or how to switch that language per request.
- `src-tauri/src/lib.rs` is the strongest next slice after the host command handlers because it is the desktop host crate entry boundary that re-exports bootstrap, commands, and shared state, while `src-tauri/src/main.rs` is only a trivial one-line binary handoff.
- The comments here should focus on crate-entry ownership and the meaning of the public re-export surface, not on restating the obvious `run_desktop_host()` call in `main.rs`.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke` remains the narrowest executable validation for this slice because the smoke test builds and uses the crate-level public host surface re-exported from `lib.rs`.

- `src-tauri/src/commands/fab.rs` is the strongest next slice after engines because it is the remaining host transport boundary in the same folder that still lacks declaration-level comments and still owns multiple temporary `FAB_NOT_WIRED` fallback projections.
- The comments here should focus on list/detail stub ownership and the accepted-job projection for sync/prewarm commands, not on restating the obvious mapper calls.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke` remains the narrowest executable validation for this slice and directly exercises `fab_list_inventory` today.

- `src-tauri/src/commands/engines.rs` is the strongest next slice after downloads because it is a host transport boundary still missing declaration-level comments, while the adjacent `jobs.rs` file already documents its cross-module snapshot query semantics.
- The comments here should focus on verification-intent forwarding and accepted-job envelope projection, not on restating the obvious `map_accepted_job_result` one-line wrapper.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke` is the narrowest executable validation for this slice and is stronger than the downloads-host case because the smoke test directly calls `engines_run_verification` today.

- `src-tauri/src/commands/downloads.rs` is the strongest next slice because it is the host-owned downloads transport boundary that forwards IPC DTOs into the downloads facade while still owning temporary `DOWNLOADS_NOT_WIRED` fallback projections for list/detail/policy queries.
- The comments here should focus on handler responsibility and stub fallback ownership, not on restating the obvious `map_command_result` and `map_accepted_job_result` one-line wrappers.
- `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke` is the narrowest practical executable validation for this slice: it does not call the downloads handlers directly today, but it does compile the same host transport boundary and remains the documented host transport gate.

- `crates/module-downloads/src/contracts/dto.rs` and `crates/module-downloads/src/contracts/events.rs` are a good paired slice because together they finish the public downloads contracts surface with projected state snapshots and event payloads while staying inside one narrow module boundary.
- The comments here should emphasize which backend facts are stable read models versus transient emitted events, plus the few non-obvious projection fields like throughput, progress labels, and retryability.
- The same named downloads unit test remains the cheapest executable validation for this slice because the module compiles these contracts through the public facade without widening to transport or host startup.

- `crates/module-downloads/src/contracts/commands.rs` and `crates/module-downloads/src/contracts/queries.rs` are a good paired slice because they are adjacent public input-contract files, together define user intents and read filters, and can be validated by the same narrow module-downloads compile/test path.
- The existing `start_download_persists_request_metadata_and_enqueue_priority` unit test is still the cheapest executable validation for this paired slice because it compiles both input-contract files through the downloads module without widening to host transport.
- The comments here should focus on request/query semantics and policy fields, not on serde derives or the bare presence of wrapper IDs.

- `crates/module-downloads/src/contracts/mod.rs` is a valid small comment slice because it is the module's public contract aggregation boundary, yet it currently exposes commands/dto/events/queries re-exports without any declaration-level explanation of that role.
- The narrowest executable validation for this slice is `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\module-downloads\Cargo.toml start_download_persists_request_metadata_and_enqueue_priority`, because that named unit test compiles the downloads module and exercises one of the public contract paths without reopening broader host wiring.
- This slice should stay focused on the aggregation boundary itself rather than spilling into the underlying downloads contract submodules or facade implementation.

- `src-tauri/src/state.rs` is the strongest next slice because it is the host-owned state handle that projects composition-root services into the desktop command layer, yet it currently exposes its wrapper struct and methods without declaration-level comments.
- The narrowest executable validation for this host-state slice is `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke`, because that smoke test exercises the state handle indirectly through the host bootstrap and command path.
- This slice should stay small and focused on the wrapper boundary itself; the concrete command handlers can remain later slices.

- `crates/composition-root/src/startup.rs` is the strongest next slice because it is the startup-stage boundary exposed by composition-root, yet it currently lacks declaration-level comments on the prewarm port, startup facade, and stage methods even though those methods encode restore-vs-warmup policy.
- `docs/TauriStartupPipelineDesign.md` is the controlling task-specific document for this slice because it defines shell-first, restore-before-warmup, and explicit blocking rules that the comments should reflect.
- A focused validation for this slice can use `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\composition-root\Cargo.toml run_stage3_background_prewarm_triggers_fab_prewarm_when_enabled`, because it compiles the startup pipeline file and executes one of the key stage-3 boundary tests without widening back to the full bootstrap smoke matrix.

- `crates/composition-root/src/bootstrap.rs` is the strongest next slice because it is the concrete assembly owner promised by the wiring design, yet it currently exposes the public bootstrap config, desktop service aggregation, and many builder helpers without declaration-level comments.
- The narrowest executable validation for this composition-root slice is `cargo test --manifest-path q:\DEV\MyEpicLauncher\crates\composition-root\Cargo.toml bootstrap_wiring_smoke`, because that smoke test exercises `build_desktop_services()` and the startup/runtime wiring assembled in the same file.
- The adjacent `crates/composition-root/tests/bootstrap_wiring_smoke.rs` already contains several intent-revealing comments; for now it is better kept out of scope so the slice remains focused on the production assembly owner.

- `src-tauri/src/bootstrap.rs` is a strong second backend comment slice because it is the desktop host assembly entry point exposed by `my_epic_launcher_desktop`, yet it currently lacks comments on the bootstrap struct and the two public functions that define host startup semantics.
- `src-tauri/src/commands/mod.rs` is the shared transport mapping surface for registered command names, error envelopes, accepted-job projection, and result wrappers; it currently exposes multiple public DTOs and helpers without declaration-level comments.
- The narrowest executable validation for this host-facing slice is `cargo test --manifest-path q:\DEV\MyEpicLauncher\src-tauri\Cargo.toml transport_wiring_smoke`, because that test exercises both bootstrap assembly and command invocation through the desktop transport boundary.

- `crates/module-fab/src/facade/mod.rs` is a strong first backend comment slice because it is the public module boundary for projection reads, cold-start placeholders, and accepted-job handoff, yet it currently lacks declaration-level comments on most public types and traits.
- `crates/module-fab/src/driver.rs` is a good adjacent second file because it is small, backend-only, and its restore semantics are simple enough to annotate without reopening behavior changes.
- For the first rollout slice, comments should stay concentrated on module entry, public types/traits/functions, and a few non-obvious cold-start helpers; test code in the same file should remain mostly uncommented unless a test-specific invariant is non-obvious.

- The repository already has `docs/ModuleDocumentationStandard.md`, but it still lacks a repository-level code comment standard that separates declaration comments from selective function-body comments.
- `docs/README.md` is the minimal correct routing surface for a new standalone comment standard; this slice does not need to widen into root README churn if the docs map is updated.
- The comment standard must reconcile two constraints at once: obvious code should remain uncommented, but high-risk concurrency and multi-threading logic should have stricter mandatory comments.
- The language split should be explicit instead of pretending one syntax fits all cases: TypeScript/TSX/JavaScript can prefer Doxygen-style block comments for declarations, while Rust should keep Rustdoc mainly for module entry points and public APIs and use ordinary comments for private internals.

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
- AT-039 confirmed that the narrowest useful post-E2 backend slice is no longer another host-wiring task; it is a module-specific behavior slice on an already-validated transport chain.
- The first real Fab behavior can stay very small: `FabFacade::list_inventory()` can delegate to a projection repository while the current SQLite adapter still returns a cold-start empty page, which upgrades the path from `FAB_NOT_WIRED` without pulling in provider sync.
- AT-040 confirmed that the next narrow Fab slice after `list_inventory` is `get_asset_detail`, not startup prewarm; moving placeholder ownership into the backend facade is still local enough to avoid cursor/provider/runtime work.
- The current SQLite detail stub can safely return `None` while `FabFacade::get_asset_detail()` owns the cold-start placeholder, which removes another transport-owned fallback without forcing real provider detail hydration yet.
- AT-041 confirmed that after `get_asset_detail`, the next narrow Fab command slice is `sync_inventory`, not startup prewarm, because startup prewarm is still explicitly tied to startup stage-3 orchestration and a later runtime bundle.
- The current Fab `job_runtime` placeholder can be wrapped behind a narrow sync-job acceptance boundary so `FabFacade::sync_inventory()` returns a backend-owned `AcceptedJob` now, while real enqueue/runtime wiring remains deferred to a later slice.
- AT-042 confirmed that a facade-level startup-prewarm acceptance slice is still possible before stage-3 orchestration lands: `FabFacade::run_startup_prewarm()` can return a backend-owned `AcceptedJob` locally while the startup pipeline remains a no-op.
- The current doc set still implies that real startup prewarm execution belongs to the later startup facade and runtime bundle, so the safe narrow move is to stop at facade acceptance and leave `StartupPipelineFacade::run_stage3_background_prewarm()` untouched for now.
- AT-043 confirmed that the next narrow startup slice after facade acceptance is the composition-root stage-3 orchestration hook itself: `StartupPipelineFacade::run_stage3_background_prewarm()` can call the already-wired Fab prewarm facade path without opening real runtime execution.
- The current startup surface can safely gate stage-3 prewarm on the existing config capability flag, but richer session-availability gating and true job-runtime execution still remain later startup/runtime slices.
- AT-044 confirmed that the next narrow post-startup slice is a shared runtime bundle, not another module-specific Fab task: a minimal `SharedJobRuntimeHost` plus composition-root injection is enough to replace the `()` runtime placeholder without opening persistence, recovery, or driver execution.
- Rust coherence rules reject blanket Fab acceptance impls for every `J: JobRuntime<Extension = ()>` while the placeholder `()` acceptance impls still exist; the safe narrow bridge is to implement those acceptance traits only for the concrete `SharedJobRuntimeHost` currently injected by composition-root.
- The composition-root smoke can now verify real runtime ownership directly by calling `FabFacade::run_startup_prewarm()` and asserting the injected shared runtime host stores a queued snapshot for the accepted job.

## Technical Decisions

| Decision | Rationale |
|----------|-----------|
| Prefix all repository-local workspace prompts with `hsy-` | The user wants a collision-resistant repo command surface, and the prompt system only needs filename plus frontmatter-name consistency to expose the new slash commands |
| Use prompt-based `/hsy-comment-zh` and `/hsy-comment-en` switches for comment authoring instead of reusing `.artifacts/ai/language-mode.txt` | Workflow copy language and source-comment language are related but not identical concerns; a separate prompt surface avoids surprising hook-language changes when a developer only wants English code comments |
| `.artifacts/ai` remains the only authoritative task record set | Prevents dual planning surfaces and stale recovery paths |
| The task-plan keeps a numbered AT ledger | `check-complete` and stop hooks already parse that shape |
| The repo should use one hybrid schema across live records, templates, and bootstrap output | planning-with-files readability is useful, but strict-doc semantics must stay explicit |
| Backend skeleton kickoff begins with a workspace plus a minimal `src-tauri` lib stub | Cargo metadata requires one real target-bearing member before the documented gate can pass |
| Preserve the deep design docs and add a flatter contributor-facing entry layer above them | Lowers onboarding friction without weakening the repo's strict architecture and AI transaction constraints |
| Bridge Fab runtime-backed acceptance through `SharedJobRuntimeHost` directly instead of a blanket `JobRuntime` impl | Avoids Rust coherence conflicts with the retained `()` placeholder acceptance path while still letting composition-root move onto a real shared runtime bundle now |

## Issues Encountered

| Issue | Resolution |
|-------|------------|
| Generic planning templates and repo live files drifted into different shapes | Normalize both the live records and the template/bootstrap sources together |
| Sync terminal commands did not preserve the intended repo cwd for verification and commit flow | Use async terminal commands with explicit repo paths when git output matters |
| The backend skeleton doc's A1 assumption conflicted with actual Cargo behavior | Surface the gap in findings/progress and bridge the workspace root to the smallest valid `src-tauri` stub |
| A blanket Fab acceptance impl for all `JobRuntime` types conflicted with the existing `()` placeholder impls under Rust coherence rules | Narrow the new acceptance bridge to the concrete `SharedJobRuntimeHost` injected by composition-root and rerun the same composition-root smoke |
| Sync terminal Git publication for AT-056 produced misleading no-op behavior | After five silent `run_in_terminal` variants, switched to an async terminal session plus `send_to_terminal`, verified `git status --short` there, and used that session as the working Git publish path |

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

## Phase 28 Backend Recovery Findings

- AT-2026-05-15-150 selected a verification-only backend host slice after AT-149: `downloads_start`, `downloads_pause`, and `downloads_cancel` should be covered through `src-tauri/tests/transport_wiring_smoke.rs` because README lists `cargo test -p my-epic-launcher-desktop transport_wiring_smoke` as a backend baseline, and CONTRIBUTING marks `src-tauri/` as the desktop host/transport boundary.
- `docs/TauriDownloadRuntimeDesign.md` keeps pause/resume/cancel as downloads facade commands while assigning task lifecycle and checkpoint ownership to the backend, so this smoke test should call the host handler path and avoid putting any lifecycle truth in frontend code.
- `docs/TauriKernelJobsRuntimeDesign.md` defines pause/resume/cancel as shared job runtime control-port behavior; because `resume_download` still returns `AcceptedJob`, AT-150 should not change resume semantics and should stop at start/pause/cancel coverage.
- Resume remains a separate design-sensitive backend slice: `docs/TauriFirstCrateApiDrafts.md` says `resume_download` must explicitly read checkpoint, and `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` sketches a multi-port resume use case rather than a simple runtime-control call.

## Phase 29 Resume Download Design Findings

- README/CONTRIBUTING/docs map all point to a docs-driven backend skeleton: backend truth and long-running work stay in Rust backend, while dirty frontend/Cargo/sqlite/.codex/src files must remain untouched unless explicitly scoped.
- `docs/TauriArchitecturePrinciplesDesign.md` requires contract-first module boundaries, composition-root-only wiring, and backend-owned long-running runtime state; this rules out frontend-owned checkpoint/resume logic and composition-root-owned resume business logic.
- `docs/TauriTestingStrategyAndQualityGateDesign.md` says module facade tests should verify facade output and use-case boundaries with the narrowest cheap test first; for `resume_download`, the right first proof is a module-downloads RED test, not a host smoke test.
- `docs/TauriDownloadRuntimeDesign.md` says downloads owns command entry, checkpoint persistence, and resume safety; it also says provider/staging/scheduler details are lower-level concerns, so the first current-repo slice should not invent full segment scheduling.
- `docs/TauriKernelJobsRuntimeDesign.md` separates shared runtime snapshots from module business checkpoints and says recovery can be triggered by explicit user `resume` command. It also states kernel-jobs must not swallow module checkpoints.
- `docs/TauriFirstCrateApiDrafts.md` explicitly requires `resume_download` to read checkpoint rather than letting runtime guess; its test matrix calls out a `resume_download` checkpoint-read unit test for `launcher-module-downloads`.
- `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` sketches full resume as job repo + checkpoint repo + staging store + manifest provider + runtime orchestration. Current repo ports only expose concrete behavior for job repo, checkpoint repo, and shared runtime; manifest/staging are still `()` placeholders, so full enqueue-resume execution needs a later design slice.
- Current `crates/module-downloads/src/facade/mod.rs` has `resume_download` returning `DOWNLOADS_NOT_WIRED`; current `DownloadCheckpointRepository` already supports `load`, and sqlite adapter already implements `load_checkpoint`, so the smallest safe next task is to prove facade-level checkpoint loading before deciding full accepted-job resume semantics.

## Phase 31 Resume Missing Checkpoint Findings

- AT-2026-05-15-155 is committed locally as current HEAD and already proves `resume_download` explicitly reads `DownloadCheckpointRepository`.
- `docs/TauriDownloadRuntimeDesign.md` orders resume as: load job checkpoint, validate staging artifacts, reconstruct manifest plan, then enqueue remaining segments. That makes missing checkpoint the first behavior branch after AT-155.
- `docs/TauriKernelJobsRuntimeDesign.md` says downloads restore reads checkpoint and treats module business checkpoints as module-owned, not shared runtime snapshot state.
- `crates/module-downloads/src/driver.rs` already treats missing checkpoint during stage-2 restore as `FailedAsUnrecoverable`, which supports giving the explicit user `resume` command a stable module-level failure instead of keeping a generic not-wired placeholder for that branch.
- `docs/TauriErrorHandlingAndProjectionDesign.md` requires facade/application errors to converge into stable `AppError` fields, and module-level tests should verify key error `code`, `retryable`, and `severity` classifications.
- `docs/TauriIPCAndStateContractsDesign.md` recommends `DL_*` for downloads-domain error codes, so a missing checkpoint branch should use a stable downloads code instead of `DOWNLOADS_NOT_WIRED`.
- The smallest document-backed next slice is to make `resume_download` return a stable missing-checkpoint `AppError` when `checkpoint_repo.load()` returns `None`; full job lookup, staging validation, manifest reconstruction, and runtime enqueue remain later slices.

## Phase 32 Resume Job Lookup Findings

- AT-2026-05-15-156 is committed locally as current HEAD and gives `resume_download` a stable `DL_CHECKPOINT_MISSING` branch.
- `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` sketches `ResumeDownloadUseCase` as `jobs.get_job(...)` before checkpoint, staging, manifest, and runtime enqueue.
- `docs/TauriFirstCrateApiDrafts.md` keeps `DownloadJobRepository` in the fixed internal port set for `module-downloads`, so explicit user resume should not proceed from checkpoint alone.
- The current facade still reads checkpoint before checking `DownloadJobRepository`, so a requested job with no module job record can incorrectly reach later checkpoint/not-wired branches.
- The next smallest document-backed slice is to make `resume_download` read `DownloadJobRepository` first and return stable `DL_JOB_NOT_FOUND` without touching checkpoint when no module record exists.
- Full staging validation, manifest reconstruction, and runtime enqueue remain out of scope for this slice.

## Phase 33 Resume Staging Validation Findings

- AT-2026-05-15-157 is committed locally as current HEAD and gives `resume_download` stable job lookup semantics before checkpoint loading.
- `docs/TauriDownloadRuntimeDesign.md` orders the next resume step after checkpoint as validating that staging artifacts still exist.
- `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` sketches `self.staging.ensure_staging_root(&request.job_id)?` before manifest reconstruction and runtime enqueue.
- `docs/TauriFirstCrateApiDrafts.md` names `DownloadStagingObjectStore` as part of the fixed `module-downloads` internal port set.
- Current code only stores a generic `staging_store: S` dependency without a trait, so the next narrow slice should define the minimal staging object-store port and prove `resume_download` calls it after job and checkpoint are present.
- Real filesystem validation, staging cleanup, manifest reconstruction, and runtime enqueue remain out of scope for this slice.

## Phase 34 Module Implementation Documentation Findings

- The existing module docs standard required `README_ARCH.md`, `README_API.md`, and `README_FLOW.md`, but did not define where backend implementation state and slice order should live.
- `docs/modules/downloads/README_ARCH.md`, `README_API.md`, and `README_FLOW.md` describe module responsibility, public surface, and flows, but they intentionally do not track Rust facade/port implementation state.
- Downloads backend implementation currently needs a single module-local implementation guide that points contributors to module docs first, then runtime/use-case/API/testing/collaboration docs before coding.
- `README_IMPL.md` should not replace design docs or `.artifacts/ai`; it should summarize current landing zones, implemented behavior, next implementation slices, port status, and validation gates for module code work.

## Phase 35 Resume Manifest Boundary Findings

- `docs/modules/downloads/README_IMPL.md` now identifies `DownloadManifestProviderPort` as not yet defined and records manifest reconstruction as the next likely backend resume slice after staging validation.
- `docs/TauriDownloadRuntimeDesign.md` orders explicit resume as checkpoint load, staging validation, manifest reconstruction, completed-segment sealing, then remaining-segment enqueue; therefore this slice must stop before runtime enqueue.
- `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` sketches resume use-case collaboration as job repo, checkpoint repo, staging store, manifest provider, then runtime, and explicitly says provider/repository/object store should not互调.
- `docs/TauriFirstCrateApiDrafts.md` names `DownloadManifestProviderPort` as a fixed internal port for `module-downloads`, but the current code still keeps `manifest_provider: M` as an unconstrained placeholder.
- `docs/TauriTestingStrategyAndQualityGateDesign.md` makes module facade tests the right cheap validation layer for this slice.
- The smallest document-backed change is to define a minimal manifest plan/port, keep `()` placeholder compatibility for current wiring, and prove `resume_download` calls the provider only after job, checkpoint, and staging are present.

## Phase 36 Resume Segment Shape Findings

- AT-2026-05-15-160 is committed as `0d9689a` and now defines `DownloadManifestProviderPort`, but `DownloadManifestPlan` still only carries `target_id`.
- `docs/TauriDownloadRuntimeDesign.md` defines logical segment fields (`segment_id`, `file_id`, `offset`, `length`, `source_locator`, `expected_hash`, `write_target`) and checkpoint fields (`downloaded_bytes`, `status`, `partial_path`, `etag`, `hash_state_ref`), but README_IMPL has not yet converted them into module-local implementation guidance.
- `crates/module-downloads/src/driver.rs` currently exposes `DownloadCheckpointRecord { job_id }`, so completed-segment sealing cannot be coded safely without deciding whether segment checkpoint data lives inside that record, beside it, or behind a new repository method.
- `docs/TauriKernelJobsRuntimeDesign.md` keeps segment offset checkpoint and download resume reconstruction inside `module-downloads`; therefore the next data-shape guidance must not move segment details into `kernel-jobs`, host transport, or frontend state.
- The narrow next step is documentation, not production code: define the minimal segment/checkpoint/resume-decision shape and invariants in `docs/modules/downloads/README_IMPL.md`, then code can follow with a focused RED test.

## Phase 37 Resume Sealed Segment Decision Findings

- AT-2026-05-15-161 is committed as `5e08cd2` and now makes the completed-segment sealing contract explicit in README_IMPL.
- Current `DownloadCheckpointRecord` still only stores `job_id`, so the first code slice needs a module-owned segment checkpoint type and a compatibility path for current adapters.
- The SQLite checkpoint adapter currently stores only `download_job_checkpoints(job_id)`; this slice can preserve compile-time compatibility by returning empty segment lists without adding tables or columns.
- The first behavior should be a pure module decision: matching `segment_id`, `file_id`, `offset`, `length`, `status=completed`, and `downloaded_bytes == length` produces `seal_completed` and not a runtime enqueue candidate.
- Partial resume, stale manifest/checkpoint mismatch handling, concrete segment persistence, and runtime enqueue remain later slices.

## Phase 38 Resume Partial Segment Decision Findings

- AT-2026-05-15-162 is committed as `f7afcd2` and intentionally leaves partial checkpoints falling through to `QueueRemaining`.
- README_IMPL and `TauriDownloadRuntimeDesign.md` both require partial segment checkpoints to resume from the interrupted byte range when provider validators allow it.
- The current in-memory decision shape already has `DownloadResumeSegmentAction::ResumePartial` and treats it as a runtime enqueue candidate; the missing behavior is the derivation branch.
- The narrow next change is a single focused test plus the branch for matching `0 < downloaded_bytes < length` checkpoints, without persistence, runtime enqueue, or mismatch error projection.

## Phase 39 Resume Mismatch Rejection Coverage Findings

- AT-2026-05-15-163 is committed as `07ed4aa` and added the partial checkpoint -> `ResumePartial` decision branch.
- `docs/modules/downloads/README_IMPL.md` requires manifest and checkpoint segments to match by `segment_id` and then validate `file_id`, `offset`, and `length`; stale or mismatched segment facts must not silently restart a whole job.
- `docs/TauriDownloadRuntimeDesign.md` keeps resume reconstruction inside downloads and says completed segments should not re-download, partial segments may resume safely, and unsafe provider conditions should only requeue affected segments rather than collapse the whole job.
- `docs/TauriKernelJobsRuntimeDesign.md` says `kernel-jobs` must not directly understand download segments, so mismatch safety belongs in `module-downloads`, not shared runtime.
- `docs/modules/downloads/README_ARCH.md`, `README_API.md`, and `README_FLOW.md` all preserve the front-end boundary: UI consumes aggregate projections and must not own checkpoint or segment resume logic.
- Current `build_resume_segment_decisions` already has a `RejectMismatch` branch for matching `segment_id` with mismatched `file_id`, `offset`, or `length`, but there is no focused test proving it remains non-enqueueable.
- The smallest safe next slice is coverage-only in `crates/module-downloads/src/facade/mod.rs`, plus PWF records; public error projection, runtime enqueue, concrete persistence, host transport, and frontend remain out of scope.

## Phase 40 Resume Queue Remaining Coverage Findings

- AT-2026-05-15-164 is committed as `ba06e7c` and added focused `RejectMismatch` coverage.
- `docs/modules/downloads/README_IMPL.md` defines `queue_remaining` as the decision for a segment with no safe completed or partial checkpoint; runtime enqueue-resume must wait until sealed and remaining segment decisions are explicit enough to test.
- `docs/TauriDownloadRuntimeDesign.md` orders resume reconstruction as manifest plan -> seal completed segments -> enqueue remaining segments only, and says unsafe resume conditions should requeue affected segments rather than restart the whole job.
- `docs/TauriKernelJobsRuntimeDesign.md` keeps segment checkpoint and resume reconstruction in `module-downloads`, not `kernel-jobs`, so this coverage belongs in the module facade tests.
- `docs/modules/downloads/README_ARCH.md`, `README_API.md`, and `README_FLOW.md` keep frontend at aggregate projection and command intent level; AT-165 must not expose segment decisions through IPC or UI.
- Current `build_resume_segment_decisions` already uses `QueueRemaining` as the fallback action and marks it as a runtime enqueue candidate, but there is no focused test for a manifest segment without a checkpoint.
- The next implementation step is coverage-only in `crates/module-downloads/src/facade/mod.rs`; runtime enqueue, concrete persistence, host transport, and frontend stay out of scope.

## Phase 41 Resume Runtime Enqueue Boundary Findings

- AT-2026-05-15-165 is committed as `491add7` and completes focused coverage for all four segment decision actions: `SealCompleted`, `ResumePartial`, `QueueRemaining`, and `RejectMismatch`.
- `docs/modules/downloads/README_IMPL.md` still has stale wording that says completed-segment sealing is the next code slice; this must be refreshed before runtime enqueue code begins.
- Current `JobRuntime` accepts job-level `EnqueueJobRequest<Extension>`; the current `SharedJobRuntimeHost` and `DownloadFacade` use `Extension = ()`, so the first resume enqueue slice should not invent a segment payload inside `kernel-jobs`.
- `docs/TauriKernelJobsRuntimeDesign.md` explicitly keeps download segment plans/checkpoints out of `kernel-jobs`; runtime snapshot state is not a replacement for downloads business checkpoint state.
- `docs/TauriDownloadRuntimeDesign.md` orders resume as: load checkpoint, validate staging, reconstruct manifest, seal completed segments, enqueue remaining segments only, then continue scheduler loop.
- `docs/modules/downloads/README_ARCH.md`, `README_API.md`, and `README_FLOW.md` keep UI at aggregate projection/intent level; AT-166 must not expose segment decisions or runtime enqueue details through frontend IPC.
- The smallest safe next document update is to define that `resume_download` should enqueue the existing downloads job id with module `downloads`, kind `download`, original priority, `recoverable = true`, and `extension = None` only after decision derivation finds runtime enqueue candidates and no mismatch rejection.

## Phase 42 Resume Runtime Enqueue Boundary Findings

- AT-2026-05-15-166 documented the job-level runtime enqueue boundary and current HEAD starts AT-167 from that implementation guide.
- `docs/modules/downloads/README_IMPL.md` requires the first code slice to load job, checkpoint, staging, and manifest, derive segment decisions, enqueue only when at least one runtime candidate exists and no `reject_mismatch` exists, and return the runtime `AcceptedJob`.
- `docs/TauriDownloadRuntimeDesign.md` keeps checkpoint, segment planning, and resume reconstruction in downloads, while the frontend only consumes aggregate task projections.
- `docs/TauriKernelJobsRuntimeDesign.md` says `kernel-jobs` must not understand segment plans/checkpoints; AT-167 must use job-level `EnqueueJobRequest<()>` only.
- `crates/module-downloads/src/facade/mod.rs` already derives segment decisions in `resume_download` but still returns `DOWNLOADS_NOT_WIRED`, making a focused RED test possible.
- Existing test helpers already record job lookups, checkpoint loads, staging validations, manifest fetches, and runtime enqueue requests, so the narrow test can assert the existing job id, module, kind, persisted priority, recoverable flag, and empty extension.
- AT-167 implemented the first job-level enqueue boundary and kept mismatch/no-candidate branches on the existing placeholder path, so the next slice needs an explicit design choice before coding deeper scheduler or error projection behavior.

## Phase 43 Resume Mismatch Error Projection Findings

- `docs/modules/downloads/README_IMPL.md` and `TauriDownloadRuntimeDesign.md` both require stale or mismatched segment facts to stop safe automatic resume rather than silently restart the whole job.
- `docs/TauriErrorHandlingAndProjectionDesign.md` requires stable codes over message matching; downloads-domain errors should use the `DL_*` prefix and expose retryable/severity consistently.
- `docs/TauriIPCAndStateContractsDesign.md` keeps frontend-facing failures behind `AppErrorDto`; AT-168 should not add IPC fields or expose segment details.
- `docs/TauriKernelJobsRuntimeDesign.md` keeps segment checkpoints out of `kernel-jobs`; mismatch classification belongs in `module-downloads`.
- Current `resume_download` already detects `RejectMismatch` decisions but falls through to `DOWNLOADS_NOT_WIRED`, making a focused RED test possible.

## Phase 44 Resume All-Sealed Completion Boundary Findings

- `docs/TauriDownloadRuntimeDesign.md` says resume marks completed segments as sealed and enqueues remaining segments only; if every manifest segment seals, there are no remaining segment candidates.
- `docs/TauriIPCAndStateContractsDesign.md` says `AcceptedJobDto` means a long job was accepted/queued and explicitly says start success is not task completion.
- `crates/kernel-jobs/src/model.rs` mirrors that contract: `AcceptedJob` only carries `job_id`, `module`, `kind`, and `queued_at`, with no completed/no-op outcome.
- `crates/kernel-jobs/src/runtime.rs` constructs `AcceptedJob` while creating a queued `JobSnapshot`, so fabricating `AcceptedJob` in downloads would blur an already-complete all-sealed resume with runtime queue acceptance.
- `docs/TauriKernelJobsRuntimeDesign.md` says `kernel-jobs` must not store download segment checkpoint details or interpret module-specific resume plans; all-sealed classification stays in `module-downloads`.
- The next safe slice is documentation-first: define the all-sealed outcome and current return-contract gap in `docs/modules/downloads/README_IMPL.md` before adding a Rust test or changing facade return semantics.

## Phase 45 Module-Owned Resume Outcome Findings

- AT-2026-05-16-169 documented the all-sealed contract gap and recommends introducing a narrow module-owned resume outcome before public transport/DTO adaptation.
- Current `src-tauri/src/commands/downloads.rs` still maps `services.downloads.resume_download(request)` through `map_accepted_job_result`, so changing the existing public method return type would immediately widen into host transport.
- A safer first code slice is to add `resume_download_outcome` beside the existing compatibility method, then keep `resume_download -> AppResult<AcceptedJob>` available for current host wiring.
- The focused module test can use existing in-memory repositories/runtime helpers and assert that an all-sealed manifest/checkpoint pair returns `AlreadyComplete` and leaves runtime enqueue requests empty.

## Phase 46 Resume Outcome Host Projection Findings

- `src-tauri/src/commands/mod.rs` currently has a shared `AcceptedJobDto` and `map_accepted_job_result` for accepted queued work.
- `src-tauri/src/commands/downloads.rs` still returns `CommandResultDto<AcceptedJobDto>` from `downloads_resume`, which cannot distinguish all-sealed already-complete outcomes from accepted queued work.
- `docs/TauriIPCAndStateContractsDesign.md` says accepted long-job results mean the backend accepted work for async processing; therefore `AlreadyComplete` must use a separate downloads resume outcome DTO rather than `accepted: true`.
- The next smallest host slice is a mapper-level TDD change: add `DownloadResumeOutcomeDto`, map `RuntimeAccepted` through existing accepted-job semantics, map `AlreadyComplete` without segment details, and switch only `downloads_resume` to the new mapper.

## Phase 47 Resume Scheduler Driver Payload Boundary Findings

- `docs/TauriDownloadRuntimeDesign.md` defines resume as loading checkpoint, validating staging, reconstructing manifest, sealing completed segments, enqueueing remaining segments only, then continuing the scheduler loop.
- The same runtime design says `resume_partial` should continue from checkpoint bytes when validators allow it, while `queue_remaining` should start the affected segment from its manifest offset.
- `docs/TauriKernelJobsRuntimeDesign.md` says `kernel-jobs` owns shared job state and queueing, but not module business checkpoints or download segment plans.
- `kernel-jobs` extension data is only for snapshot summaries, not complete business checkpoint or segment payloads.
- Downloads integration says the concrete download driver should call planner/scheduler/writer/verifier in `run()`, with business checkpoint writes going back through `DownloadCheckpointRepository`.
- The next safe implementation slice is module-local: define a downloads-owned resume work plan/payload derived from segment decisions, without touching shared runtime payload, host transport, frontend, SQLite schema, or concrete execution.

## Phase 48 Resume Work Plan Derivation Findings

- Required docs were read in scoped snippets before Rust edits: `README.md`, `CONTRIBUTING.md`, `docs/README.md`, downloads `README_ARCH.md` / `README_API.md` / `README_FLOW.md` / `README_IMPL.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `docs/TauriFirstCrateApiDrafts.md`, `docs/TauriKernelJobsRuntimeDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, `docs/TauriAIDevelopmentTransactionProtocolDesign.md`, and `docs/TauriCodeCommentStandard.md`.
- `module-downloads` owns resume reconstruction and checkpoint-aware work planning; `kernel-jobs` owns only generic job lifecycle/snapshot/runtime control and must not receive segment plans.
- AT-173 should add only a pure module-local derivation in `crates/module-downloads/src/facade/mod.rs`: `resume_partial` / `queue_remaining` become work items, while `seal_completed` / `reject_mismatch` produce no work item.
- User override for comments: preserve existing English comments and add Chinese comments alongside them; new declarations in this slice should use concise bilingual comments.

## Phase 49 Resume Scheduler Boundary Findings

- Required docs were read in scoped snippets before README_IMPL edits: `README.md`, `CONTRIBUTING.md`, `docs/README.md`, downloads `README_ARCH.md` / `README_API.md` / `README_FLOW.md` / `README_IMPL.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `docs/TauriFirstCrateApiDrafts.md`, `docs/TauriKernelJobsRuntimeDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, and `docs/TauriAIDevelopmentTransactionProtocolDesign.md`.
- The scheduler/driver consumer is still downloads-owned. It should consume `DownloadResumeWorkPlan` inside `module-downloads`, not through `kernel-jobs`, host IPC, frontend state, or SQLite schema in this slice.
- Safe call order for the next Rust slice should be: derive decisions -> reject mismatch -> classify all-sealed -> build work plan for runtime candidates -> hand work plan to downloads scheduler/driver port -> enqueue existing job id through shared job runtime.
- If scheduler/driver port preparation fails, `resume_download_outcome()` should return that downloads-domain error and skip runtime enqueue, preventing a queued job with no module-owned work plan.

## Phase 50 Resume Scheduler Port Findings

- AT-175 code spec comes from README_IMPL section 7.7: add `DownloadResumeWorkScheduler`, method `schedule_resume_work(&self, job_id, plan)`, no-op `()` implementation, `DownloadModuleDeps` ownership, and crate-entry export.
- Composition root may receive a placeholder scheduler dependency, but it must remain assembly-only; it must not execute download business orchestration.
- User comment override remains active: preserve existing English comments and add concise Chinese comments alongside new declaration comments.

## Phase 56 Driver Pending Work Consumption Boundary Findings

- AT-2026-05-16-180 is already committed as `d3b1b7d`; the lingering "ready for local commit" wording in handoff/task-plan was stale recovery text.
- Required docs were read in scoped snippets before editing: root README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/README_IMPL, download runtime, kernel-jobs runtime, composition-root wiring, first crate API draft, testing strategy, and AI transaction protocol.
- `docs/TauriDownloadRuntimeDesign.md` says resume continues by loading checkpoints, validating staging, reconstructing manifest, sealing completed segments, enqueueing remaining segments, then continuing the scheduler loop; frontend must not own segment/checkpoint state.
- `docs/TauriKernelJobsRuntimeDesign.md` says `kernel-jobs` owns generic lifecycle/snapshot/routing, while downloads owns segment checkpoints, resume reconstruction, planner/scheduler/writer/verifier internals, and business checkpoint writes.
- Current Rust reality differs from future design sketches: `crates/kernel-jobs/src/runtime.rs` defines `JobDriver` with `module()`, `kind()`, and `restore()` only; there is no `run()` method or execution context yet.
- `crates/module-downloads/src/facade/mod.rs` already has `InMemoryDownloadResumeWorkScheduler::pending_work()` and `drain_pending_work()`, but directly coupling `DownloadJobDriver` to that concrete type would make tests pass while hardening an internal implementation detail.
- The next implementation document should prefer a narrow module-local pending-work source/drain boundary that the driver can consume when a documented execution turn exists, while keeping fetch/write/verify, checkpoint mutation, SQLite schema, host transport, frontend, and `kernel-jobs` payloads out of scope.

## Phase 57 Pending Resume Work Source Drain Findings

- AT-2026-05-16-181 committed as `ccb0eac` and defined the AT-182 code slice as source/drain semantics only.
- The RED test failed for the intended reason: `DownloadPendingResumeWorkSource` and `drain_pending_resume_work()` did not exist.
- `InMemoryDownloadResumeWorkScheduler` already stores `DownloadPendingResumeWork` in a shared `Arc<Mutex<Vec<_>>>`, so the minimal source implementation can filter the in-memory vector by `JobId` without introducing persistence or driver execution.
- The job-scoped drain must preserve unrelated job pending work; draining all work would make a future driver execution turn for one job accidentally erase another queued job's work plan.
- Empty drain is a valid source result for the source boundary but must not be treated as download completion; future driver integration still needs an explicit documented execution behavior.
- AT-182 intentionally leaves `DownloadJobDriver`, `kernel-jobs`, composition-root, host transport, frontend, SQLite schema, fetch/write/verify, and checkpoint mutation unchanged.

## Phase 58 DownloadJobDriver Local Consumer Boundary Findings

- AT-2026-05-16-182 committed as `bb35c6f` and left `DownloadJobDriver` untouched.
- Current `DownloadJobDriver` has only a checkpoint repository field and `new(checkpoint_repo)`, and current `kernel-jobs::JobDriver` still exposes only `module()`, `kind()`, and `restore()`.
- Because there is no runtime `run()` callback yet, the next safe Rust slice should not modify shared runtime execution semantics.
- README_IMPL now defines a local driver consumer method boundary: keep `restore()` unchanged, add `with_pending_resume_work_source(...)`, and add a local `drain_pending_resume_work(&JobId)` delegating to `DownloadPendingResumeWorkSource`.
- The default constructor must stay compatible by using a no-op source that returns an empty vector, so current composition and restore tests do not need to wire the real scheduler yet.
- AT-184 can code this local driver method with focused driver tests while still deferring composition sharing, fetch/write/verify, checkpoint mutation, SQLite schema, host transport, frontend, and `kernel-jobs` API changes.

## Phase 59 DownloadJobDriver Local Consumer Findings

- AT-2026-05-16-183 committed as `17402bc` and documented the local driver consumer boundary.
- The RED driver test failed on missing `with_pending_resume_work_source(...)` and `drain_pending_resume_work(...)`, proving the test exercised new API surface.
- A no-op `DownloadPendingResumeWorkSource` implementation for `()` lets `DownloadJobDriver::new(checkpoint_repo)` remain compatible with existing composition and restore tests.
- The injected constructor lets tests and later composition pass the real in-memory scheduler source without changing shared runtime execution semantics in this slice.
- The local driver drain method delegates to the source only; it does not fetch, write, verify, read or mutate checkpoints, mutate snapshots, complete jobs, publish events, or alter `restore()`.
- Full downloads module tests passed with 26 unit tests after formatting, so existing restore/facade behavior stayed intact.
- The next likely slice is composition-level shared scheduler/source wiring, but it should be reassessed from README_IMPL before coding.

## Phase 60 Downloads Composition Shared Scheduler Source Wiring Findings

- AT-2026-05-16-184 is already committed as `a710cfc`; the lingering "ready for local commit" wording in older PWF files is stale recovery text.
- Required docs were read in scoped snippets before this docs-first slice: `README.md`, `CONTRIBUTING.md`, `docs/README.md`, downloads ARCH/API/FLOW/README_IMPL, `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriKernelJobsRuntimeDesign.md`, and `docs/TauriDownloadRuntimeDesign.md`.
- Composition-root is the only concrete assembly owner, but it must not execute download resume business logic, fetch, write, verify, mutate checkpoints, or expose driver internals through its public API.
- Current Rust wiring creates the downloads facade scheduler in the facade builder while the driver registry still uses `DownloadJobDriver::new(...)`, so the driver source is currently the no-op `()` implementation rather than the real in-memory scheduler.
- The next implementation boundary must create one shared `InMemoryDownloadResumeWorkScheduler` in composition assembly and pass it to both the facade dependency graph and `DownloadJobDriver::with_pending_resume_work_source(...)`.
- The current public composition-root service graph should not grow a driver-registry accessor just for testing; the next Rust slice should prefer a private builder/helper or narrowly scoped composition test surface.

## Phase 61 Downloads Composition Shared Scheduler Source Wiring Findings

- AT-2026-05-16-185 committed locally as `cb991f3`.
- Push to `origin/main` was attempted after AT-185 and rejected by safety review because direct external default-branch push was not sufficiently authorized; per user rule, continue without a push and do not work around the rejection.
- Required docs and code surfaces were read before test edits: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/README_IMPL 7.11, composition-root wiring design, kernel-jobs runtime design, download runtime design, TDD skill, `crates/composition-root/src/bootstrap.rs`, composition smoke tests, module-downloads facade/driver, and kernel-jobs registry API.
- `JobDriverRegistry::resolve()` returns `&dyn JobDriver<()>`, so a test that needs `DownloadJobDriver::drain_pending_resume_work()` should use a private composition helper rather than exposing the registry or widening `JobDriver`.
- The focused RED test should live in `bootstrap.rs` under `#[cfg(test)]`, where it can exercise private builders without changing `launcher-composition-root` public API.
- The RED test failed on the missing shared scheduler argument and missing private driver builder, proving the test exercised the intended composition seam.
- The implementation keeps `DesktopAppServices` facade-only and only changes private composition builders.
- The same scheduler handle is now cloned into the facade deps and injected into `DownloadJobDriver`, so work registered through the facade scheduler can be drained by the driver source.
- Startup stage-2 restore behavior remains checkpoint-only and does not drain in-memory pending work.

## Phase 62 Downloads Checkpoint Mutation Boundary Findings

- AT-2026-05-16-186 committed locally as `6a721af`.
- README_IMPL 7.11 now closes the shared scheduler/source wiring prerequisite.
- `docs/TauriDownloadRuntimeDesign.md` requires checkpoint persistence after job creation, manifest/segment plan confirmation, segment completion, pause completion, time-window flush, and terminal failed/canceled/completed transitions.
- `docs/TauriStorageAndDatabaseDesign.md` classifies `download_job_checkpoint` and `download_segment_checkpoint` as SQLite facts, while staging files and large manifests stay in the filesystem with references.
- `docs/TauriRepositoryPortsAndAdapterDesign.md` says modules access SQLite only through repository ports, SQLite adapters own SQL/transactions/mapping, and cross-medium consistency relies on state machine + checkpoint + retry/compensation rather than distributed transactions.
- Current Rust still has a richer `DownloadCheckpointRecord` / `DownloadSegmentCheckpointRecord` shape than the SQLite adapter persists; the adapter currently only stores enough job-level checkpoint presence for restore.
- The next Rust slice should therefore be segment-checkpoint persistence in the repository/adapter boundary, not driver fetch/write/verify execution.

## Phase 63 Downloads SQLite Segment Checkpoint Persistence Findings

- AT-2026-05-16-187 committed locally as `95cf6fa`.
- README_IMPL 7.12 defines the next Rust slice as focused tests proving `SqliteDownloadCheckpointRepository::save(...)` and `load(...)` round-trip segment checkpoint facts.
- Current `SqliteDownloadCheckpointRepository::ensure_table()` creates only `download_job_checkpoints(job_id)`.
- Current `save_checkpoint()` inserts only job id; current `load_checkpoint()` returns `DownloadCheckpointRecord::empty(job_id)`, discarding segment facts.
- The first RED test should assert full round-trip of `DownloadSegmentCheckpointRecord`, including status and nullable `partial_path`, `etag`, and `hash_state_ref`.
- RED failed exactly on segment loss: load returned the saved job id with an empty segment list.
- The implementation stores segment records in a job-scoped `download_segment_checkpoints` table and replaces one job's segment rows in the same transaction as the job checkpoint upsert.
- Offset, length, and downloaded byte counts are stored as text for now to preserve the Rust `u64` contract without forcing signed SQLite narrowing.
- This slice remains persistence-only; no driver execution, runtime completion, transport, frontend, composition public API, fetch/write/verify, or DTO shape changed.

## Phase 64 Downloads Driver Execution Boundary Findings

- AT-2026-05-16-188 committed locally as `4e3e5ac`.
- Required docs were reread in scoped snippets before editing: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/README_IMPL 7.12, kernel-jobs runtime design, download runtime design, and testing strategy.
- Current Rust `kernel-jobs::JobDriver` still exposes only `module()`, `kind()`, and `restore()`; the broader design's `run()` callback is not implemented yet.
- Downloads now has all prerequisites for a future same-process execution handoff: pending work source/drain, composition-shared scheduler/source wiring, and durable segment checkpoint facts.
- The next safe Rust slice should still avoid concrete HTTP/disk/hash work; it should either introduce a local downloads driver execution-turn method or define a `kernel-jobs` runtime `run()` boundary first.

## Phase 65 Downloads Driver Local Execution-Turn Classification Findings

- AT-2026-05-17-189 committed locally as `3117914`; push remains skipped because an earlier push was rejected by safety review.
- Required docs were reread in scoped snippets before coding: README, CONTRIBUTING, docs map, downloads ARCH/API/FLOW/README_IMPL 7.12 and 7.13, kernel-jobs runtime design, download runtime design, testing strategy, AI transaction protocol, and code comment standard.
- Current `DownloadJobDriver` already owns both `DownloadCheckpointRepository` and `DownloadPendingResumeWorkSource`, and already exposes `drain_pending_resume_work(&JobId)`.
- Current `kernel-jobs::JobDriver` still has no `run()` in Rust, so the new behavior must stay a local downloads method and must not mutate runtime snapshots or shared job completion state.
- `README_IMPL.md` 7.13 makes the smaller next slice explicit: add a downloads-owned execution classification such as pending work accepted, no pending work, checkpoint missing, or checkpoint present.
- The RED test should prove ordering, especially that checkpoint-missing execution does not drain the in-memory pending-work source.

## Phase 66 Downloads Segment Execution Ports Boundary Findings

- AT-2026-05-17-190 committed locally as `f5d950d`; path-limited AT-190 file status was clean after commit.
- `README_IMPL.md` 7.13 still described the module-local execution-turn method as a future option, so the implementation document must be updated before more backend coding.
- The next code task should not jump directly to HTTP range requests, staging writes, hash verification, runtime snapshot completion, or host/frontend projection.
- The safe next design boundary is a module-local segment execution ports boundary: define how `DownloadDriverExecutionTurn::PendingWorkAccepted` can be translated into future fetch/write/verify/checkpoint operations while keeping concrete IO and runtime `run()` out of scope until separately tested.

## Phase 67 Downloads Segment Execution Request Handoff Findings

- AT-2026-05-17-191 committed locally as `3d7f246`; its file set was clean after commit.
- README_IMPL 7.14 defines the first Rust slice as request/result/port shell plus local driver helper only.
- Existing `DownloadResumeWorkItem` already carries most segment execution input fields; the missing stable request context is the owning `JobId`.
- The focused RED test should assert stable ordering across pending work entries and plan items so future execution ports do not reorder segments accidentally.
- Non-`PendingWorkAccepted` turns should not produce segment execution requests; `NoPendingWork` remains a classification, not completion.

## Phase 68 Downloads Fake Segment Execution Acceptance Findings

- AT-2026-05-17-192 committed locally as `5ab0bec`; the AT-192 file set was clean after commit.
- README_IMPL 7.14 now permits picking one concrete execution concern after request/port handoff, but each concern must remain its own atomic task.
- The next smallest concern is fake/local execution port acceptance, not real HTTP fetch: use `DownloadSegmentExecutionPort` as the boundary and test with a recording fake.
- The driver helper should preserve request order and collect `DownloadSegmentExecutionResult` values, while leaving retry, checkpoint mutation, snapshot updates, and concrete IO to later slices.
- RED/GREEN confirmed that a module-local helper can delegate prepared requests through `DownloadSegmentExecutionPort` in stable order without widening into HTTP fetch, staging writes, verification, checkpoint mutation, runtime completion, transport, or frontend work.

## Phase 69 Downloads Fake Segment Completion Result Findings

- AT-2026-05-17-193 committed locally as `7e8d6bd`; its file set was clean after commit.
- README_IMPL 7.15 now needs to be brought forward from "helper does not exist" to "helper exists and preserves ordered results".
- Jumping directly to checkpoint mutation would force decisions about save semantics before the result payload exists.
- The smaller next slice is therefore a fake completed result contract on `DownloadSegmentExecutionResult`, collected through the existing port helper.
- The result may carry request facts plus optional fake persistence tokens, but it must not perform HTTP fetch, staging writes, hash verification, checkpoint save, runtime completion, transport, or frontend projection.
- RED/GREEN confirmed that the existing port helper can carry a fake completed result payload unchanged, so a later checkpoint-mutation slice can consume a typed result instead of inventing payload semantics inline.

## Phase 70 Downloads Fake Completed-result Checkpoint Mutation Findings

- AT-2026-05-17-194 committed locally as `218e70c`; the AT-194 file set is clean.
- README_IMPL 7.12 says driver/execution turns must reload or validate durable checkpoint facts before mutating them.
- `TauriDownloadRuntimeDesign.md` requires checkpoint persistence after segment completion, but verifier/hash and concrete writer behavior remain separate responsibilities.
- `TauriRepositoryPortsAndAdapterDesign.md` keeps `DownloadCheckpointRepository` as the downloads-owned repository port; adapters own SQL/transactions, not business decisions.
- `TauriStorageAndDatabaseDesign.md` classifies `download_job_checkpoint` and `download_segment_checkpoint` as SQLite facts, but this slice can stay inside the existing repository port and should not change schema.
- Current Rust has `DownloadSegmentExecutionResult::Completed`, but no driver helper consumes it into `DownloadSegmentCheckpointRecord`.
- The next RED test should prove completed results replace or append segment checkpoint facts and save through `DownloadCheckpointRepository`, while non-completed results and concrete IO remain out of scope.
- RED/GREEN confirmed the driver can reload checkpoint facts, replace a matching completed segment in place, preserve unrelated segment order, and save through the repository port without touching concrete IO, SQLite adapter/schema, runtime completion, transport, or frontend code.

## Phase 71 Downloads Fake Local Resume Execution Orchestration Findings

- AT-2026-05-17-195 committed locally as `227458a`; its path-limited file set was clean after commit.
- The current branch is `main` and is ahead of `origin/main` by 69 commits; `origin` already points to the user-provided `https://github.com/TheLostRiver/HelsincyLauncher.git`.
- README_IMPL 7.17 confirms completed fake results can already be turned into saved checkpoint facts through `DownloadJobDriver::record_completed_segment_checkpoints(...)`.
- `TauriKernelJobsRuntimeDesign.md` still describes broader runtime ownership for snapshots, leases, and lifecycle; current Rust still does not expose a `JobDriver::run()` callback, so this slice must stay module-local.
- `TauriDownloadRuntimeDesign.md` keeps scheduler/fetch/write/verify/checkpoint responsibilities separate, so local orchestration must not collapse into concrete HTTP, staging writes, hash verification, or runtime completion.
- README_IMPL 7.18 now defines the AT-196 boundary: add a narrow driver helper that chains existing helper steps for one fake/local resume turn and returns `AppResult<Option<DownloadCheckpointRecord>>`.
- The focused RED test should use an existing checkpoint, one pending work item, and a recording fake completed execution port to prove ordered request handoff, pending-work drain, and saved completed segment checkpoint facts.
- RED failed on the missing `DownloadJobDriver::execute_local_resume_turn(...)` method, confirming the test targeted the new orchestration helper.
- GREEN confirmed `execute_local_resume_turn(...)` can chain the existing local helpers and persist completed segment checkpoint facts without adding runtime `run()`, concrete IO, SQLite adapter/schema, composition wiring, transport, frontend, or public execution errors.
- Full downloads module tests passed with 35 tests after rustfmt, so existing facade, driver restore, execution-turn, request handoff, fake acceptance, and checkpoint mutation behavior stayed intact.

## Phase 72 Downloads Fake Segment Failure Result Boundary Findings

- AT-2026-05-17-196 final commit is `9294f9d` and was pushed to `origin/main`; older PWF mentions of initial hash `3d6f4f7` are pre-amend history.
- README_IMPL 7.18 closes fake/local successful orchestration but still explicitly leaves the concrete execution failure surface undesigned.
- TauriDownloadRuntimeDesign says failed/canceled/completed terminal transitions require checkpointing, retryable errors may retry locally per segment, and verification failures should redownload affected segments rather than restarting the whole job.
- TauriErrorHandlingAndProjectionDesign says public errors must use stable codes, retryable is a hint rather than a retry engine, and public projection belongs at application/transport/job snapshot boundaries.
- Current Rust has `DownloadSegmentExecutionResult::Accepted` and `Completed`; it can propagate an `AppResult` from the execution port, but it has no module-local result value for a fake executor to report a handled segment failure.
- The next safe boundary should define a local failed segment result contract for the next TDD slice, not public `DL_*` codes, checkpoint mutation, runtime completion, concrete fetch/write/verify, transport, frontend, or composition wiring.
- README_IMPL 7.19 now pins the first Rust slice: add a focused test whose fake port returns `DownloadSegmentExecutionResult::Failed`, carrying request facts, a local failure reason string, a retryable hint, and downloaded bytes known at failure time.

## Phase 73 Downloads Fake Segment Failure Result Contract Findings

- AT-2026-05-17-197 final commit is `af6ca27` and was pushed to `origin/main`.
- Current `DownloadSegmentExecutionResult` has only `Accepted` and `Completed`, while `DownloadSegmentExecutionPort` already returns `AppResult<DownloadSegmentExecutionResult>` and `accept_segment_execution_requests(...)` preserves collected result values.
- README_IMPL 7.19 makes the next RED target precise: a fake port should return a local failed result value in-band without changing the port signature or adding checkpoint mutation, retry/backoff, public `DL_*` projection, concrete IO, runtime completion, transport, frontend, or composition wiring.
- RED/GREEN confirmed `DownloadSegmentExecutionResult::Failed` can carry request facts, downloaded bytes known at failure time, a local reason string, and a retryable hint through the existing execution port helper without changing the helper or widening scope.
- Full downloads module tests passed with 36 tests after adding the failed result contract.

## Phase 74 Downloads Fake Failed-result Checkpoint Mutation Boundary Findings

- AT-2026-05-17-198 final commit is `89f5a06` and was pushed to `origin/main`.
- Current checkpoint records can already store `DownloadSegmentCheckpointStatus::Failed` plus downloaded bytes, offset, length, and optional persistence tokens, but they do not store a public failure code, local reason string, or retryable flag.
- TauriDownloadRuntimeDesign says failed/canceled/completed terminal transitions need checkpointing and retryable errors may retry locally per segment, but retry policy and terminal job state are separate concerns from segment checkpoint mutation.
- The next safe boundary should record failed segment status/progress through `DownloadCheckpointRepository` while preserving existing partial metadata on replacement, and defer retry/backoff, public `DL_*` projection, terminal runtime state, concrete IO, SQLite adapter/schema, transport, composition-root, and frontend behavior.
- README_IMPL 7.20 now pins the first Rust slice: a local helper should apply only same-job failed results, replace or append segment checkpoint facts, set status to `Failed`, copy `downloaded_bytes`, preserve existing offset/partial persistence tokens on replacement, and leave reason/retryable outside the persisted checkpoint shape for now.

## Phase 75 Downloads Fake Failed-result Checkpoint Mutation Findings

- AT-2026-05-17-199 final commit is `59db102` and was pushed to `origin/main`.
- Current `record_completed_segment_checkpoints(...)` provides the local mutation shape to mirror: reload checkpoint, filter same-job result values, replace or append segment facts, save only when a matching result was applied, and return the checkpoint option.
- For failed results, the helper should preserve existing offset/partial persistence tokens on replacement because `DownloadSegmentExecutionResult::Failed` does not carry those tokens; appended failed facts should use request start offset and `None` optional persistence tokens.
- RED/GREEN confirmed `record_failed_segment_checkpoints(...)` can reload checkpoint facts, ignore accepted/completed results, replace a matching failed segment in place, preserve existing offset and optional persistence tokens, and save through the repository port without adding retry/backoff, public error projection, terminal runtime state, concrete IO, transport, composition-root, SQLite adapter/schema, or frontend behavior.

## Phase 76 Downloads Fake Local Mixed-result Checkpoint Orchestration Boundary Findings

- AT-2026-05-17-200 final commit is `c973da9` and was pushed to `origin/main`.
- `execute_local_resume_turn(...)` currently prepares a turn, builds requests, delegates to the execution port, and records completed segment checkpoints only.
- Since failed-result checkpoint mutation now exists, the next orchestration boundary should define how the local helper records both completed and failed fake result facts while still avoiding retry/backoff, public error projection, terminal runtime state, concrete IO, runtime `run()`, transport, composition-root, SQLite adapter/schema, and frontend behavior.
- README_IMPL 7.21 now pins the first Rust slice: update `execute_local_resume_turn(...)` to delegate to both existing checkpoint mutation helpers after collecting execution results, keeping mutation logic inside those helpers.

## Phase 77 Downloads Fake Local Mixed-result Checkpoint Orchestration Findings

- AT-2026-05-17-201 final commit is `8790f8d` and was pushed to `origin/main`.
- The current orchestration helper calls `record_completed_segment_checkpoints(...)` only, so fake failed results collected from `DownloadSegmentExecutionPort` are not persisted during a local resume turn.
- The next test should use the existing `FailedSegmentExecutionPort` and scheduler path to prove the orchestration helper records a failed checkpoint fact end to end.
- RED/GREEN confirmed `execute_local_resume_turn(...)` now records failed fake results through the existing failed checkpoint helper while preserving the existing completed-result orchestration path.
- Full downloads module tests passed with 38 tests after the mixed-result orchestration update.
- RED/GREEN confirmed `record_failed_segment_checkpoints(...)` can replace a matching failed segment checkpoint, preserve existing partial persistence tokens, ignore accepted/completed results, and save through the repository port without widening into retry/backoff, public error projection, runtime terminal state, concrete IO, transport, composition-root, SQLite adapter/schema, or frontend behavior.

## Phase 78 Downloads Get-job Snapshot Query Boundary Findings

- AT-2026-05-17-202 final commit is `043f3f7` and was pushed to `origin/main`.
- README_IMPL listed `list/get/policy surfaces` as the only remaining broad unwired row, but it did not define an executable slice.
- Current contracts already define `GetDownloadJobQueryDto`, `DownloadJobSnapshotDto`, and related list/policy DTOs.
- Current facade methods still return `DOWNLOADS_NOT_WIRED` for `list_jobs`, `get_job_snapshot`, `get_policy`, and `update_policy`.
- Current `JobRuntime` exposes `snapshot(job_id)` but no list query, so `list_jobs` should not be the first code slice without a separate runtime/read-source design.
- Current policy DTOs exist, but there is no policy source of truth or policy repository in module deps, so `get_policy` / `update_policy` also need a separate boundary.
- The smallest next Rust slice is `get_job_snapshot(...)`: verify the downloads module record exists, read the shared runtime snapshot, and project a `DownloadJobSnapshotDto` with conservative module extension facts.

## Phase 79 Downloads Get-job Snapshot Query Implementation Findings

- AT-2026-05-17-203 final commit is `fb5a94e` and was pushed to `origin/main`.
- `DownloadJobRepository::get_job(...)` already records looked-up job ids in tests and can prove the module-record precondition.
- `RecordingJobRuntime::snapshot(...)` currently always returns a queued downloads snapshot, so it can support the success RED test with minimal fixture changes.
- A missing-runtime-snapshot RED test will need the recording runtime to optionally return `None`; this remains test fixture behavior, not a production runtime change.
- The production slice can stay inside `crates/module-downloads/src/facade/mod.rs` by projecting `JobSnapshot<()>` into `DownloadJobSnapshotDto` with `DownloadJobExtensionDto` built from `DownloadJobRecord`.
- RED confirmed all three focused tests currently fail because `DownloadsFacade::get_job_snapshot(...)` returns `DOWNLOADS_NOT_WIRED` before touching module job lookup or runtime snapshot lookup.
- GREEN confirmed `get_job_snapshot(...)` now composes module job facts and runtime snapshot facts, reuses `DL_JOB_NOT_FOUND` for missing module records, and returns `DL_JOB_SNAPSHOT_MISSING` when the runtime snapshot is absent after module ownership is confirmed.

## Phase 80 Downloads List-jobs Query Boundary Findings

- AT-2026-05-17-204 final commit is `2ccc436` and was pushed to `origin/main`.
- The backend skeleton use-case table says `ListDownloadJobsQuery` depends on `DownloadJobRepository`.
- The current repository-port design lists `DownloadJobRepository` with create/get/update only; it does not yet define pagination.
- Current Rust `JobRuntime` exposes `snapshot(job_id)` but no list method, even though broader design docs mention a future `list_active(...)`.
- `SqliteDownloadJobRepository` already stores the module job record fields needed for a conservative list row and can add a read method without schema changes.
- The safest first `list_jobs(...)` slice is a repository-backed module record page, with live runtime joins and policy data left for later boundaries.

## Phase 81 Downloads List-jobs Query Implementation Findings

- AT-2026-05-17-205 final commit is `17e0bb4` and was pushed to `origin/main`.
- `PageRequest` exposes `limit` and optional string cursor; the first facade tests can ignore cursor behavior and prove page projection/filtering.
- The module test repository already stores `created_jobs`, so it can implement a simple in-memory repository page for RED/GREEN tests.
- Because `DownloadJobRepository` is implemented by `SqliteDownloadJobRepository`, the Rust slice must also add SQLite adapter compile support even if runtime list/live snapshot joins remain out of scope.
- RED/GREEN confirmed `list_jobs(...)` now projects module repository records to conservative list DTO rows and applies optional `ui_state` filtering without adding runtime list APIs, live snapshot joins, policy storage, transport, frontend, or concrete IO.

## Phase 82 Downloads Policy Source Boundary Findings

- AT-2026-05-17-206 final commit is `d0ad61a` and was pushed to `origin/main`.
- `DownloadPolicyDto`, `GetDownloadPolicyQueryDto`, and `UpdateDownloadPolicyRequestDto` exist, but the facade policy methods still return `DOWNLOADS_NOT_WIRED`.
- `RuntimeQueuePolicy` is currently a kernel-jobs host setting with `max_concurrent_jobs`, not a downloads user policy store.
- `JobRuntime` exposes no policy read/write API, so policy facade methods should not mutate runtime queue policy directly.
- Storage docs mention `download_policy_snapshot`, but no SQLite schema/adapter exists yet.
- The next safe Rust slice should introduce a downloads-owned policy store/port, clamp user-facing concurrency slots to `1..=128`, and keep runtime policy application separate.

## Phase 83 Downloads Policy Store Implementation Findings

- AT-2026-05-17-207 final commit is `1d9a04c` and was pushed to `origin/main`.
- Required docs confirm the first policy implementation must stay backend-owned and module-local: `DownloadPolicyDto` is a user-facing downloads policy snapshot, while `RuntimeQueuePolicy` remains a lower-level shared runtime queue budget.
- The safe Rust boundary is a downloads-owned policy store/port plus in-memory implementation for module tests and composition wiring.
- `concurrency_slots` must clamp to `1..=128`; `bandwidth_limit_bytes_per_sec` and `auto_resume` should be stored/read back but must not drive runtime behavior in this slice.
- SQLite `download_policy_snapshot`, runtime queue-budget application, host transport, frontend settings wiring, concrete IO, retry/backoff, and terminal runtime completion remain later slices.
- RED/GREEN confirmed `get_policy(...)` reads `DownloadPolicyStore`, `update_policy(...)` stores a normalized snapshot, and composition-root initializes the in-memory store from `default_download_slots` without changing shared runtime queue policy.

## Phase 84 Downloads Policy SQLite Persistence Boundary Findings

- AT-2026-05-17-208 final commit is `6d8c022` and was pushed to `origin/main`.
- Storage docs list `download_policy_snapshot` as a downloads persistence fact, but also say broad user-editable configuration belongs to the settings/config system by default.
- The next safe boundary is therefore a downloads-owned policy snapshot adapter (`SqliteDownloadPolicyStore`) for the existing `DownloadPolicyStore` port, not a global settings implementation.
- Current `adapter-storage-sqlite` has job, checkpoint, and shared snapshot stores but no policy store or policy table.
- The first Rust slice should use project-local test database paths under `D:\DEV\MyEpicLauncher` to respect the user safety boundary and avoid deleting temp files outside the repo.
- Runtime queue-policy application, host transport, frontend settings, global settings/config-system sync, concrete IO, retry/backoff, and terminal runtime completion remain later tasks.

## Phase 85 Downloads SQLite Policy Store Implementation Findings

- AT-2026-05-17-209 final commit is `41f0b8c` and is already pushed to `origin/main`.
- Required docs confirm this Rust slice belongs in `adapter-storage-sqlite` plus composition-root wiring: SQLite stores structured downloads facts, the downloads module owns the `DownloadPolicyStore` port, and composition-root is the only place that should know the concrete adapter type.
- The implementation must not turn `download_policy_snapshot` into a global settings system, mutate `RuntimeQueuePolicy`, touch active jobs/leases/snapshots/pending segment work, change host transport/frontend behavior, or perform concrete download IO.
- Focused adapter tests must create any SQLite database under `D:\DEV\MyEpicLauncher`, not `std::env::temp_dir()`, so verification does not create or delete files outside the project boundary.
- RED/GREEN confirmed `SqliteDownloadPolicyStore` can load a normalized default policy when no row exists, upsert a normalized policy snapshot, and round-trip bandwidth-limit plus auto-resume facts without applying them to runtime behavior.
- Composition-root now uses the SQLite policy store for the desktop downloads facade while leaving `RuntimeQueuePolicy` and shared job runtime construction unchanged.
- Verification passed for focused adapter policy tests, affected composition check, existing downloads module policy tests, rustfmt check, and scoped `git diff --check` with CRLF warnings only.

## Phase 86 Downloads Runtime Policy Application Boundary Findings

- AT-2026-05-17-210 final commit is `2f9e828` and was pushed to `origin/main`.
- `TauriDownloadRuntimeDesign.md` maps the user-facing `downloadConcurrencySlots: 1..128` to a backend `global_slots = clamp(user_setting, 1, 128)` budget, but also says real scheduling remains constrained by per-job, per-host, and writer-backpressure caps.
- `TauriKernelJobsRuntimeDesign.md` treats queue policy as shared runtime input, while current Rust only has `RuntimeQueuePolicy { max_concurrent_jobs }` and a read-only `SharedJobRuntimeHost::policy()` snapshot.
- Current `JobRuntime` exposes no policy mutation API, so applying persisted downloads policy live would require a separate kernel-jobs boundary.
- The first safe integration slice should be startup seeding: composition-root can load the persisted downloads policy before constructing the shared runtime and pass the loaded `concurrency_slots` to `RuntimeQueuePolicy::new(...)`.
- README_IMPL 7.26 now pins the first Rust slice to focused composition-root coverage for persisted-policy startup seeding and empty-store fallback, with live `update_policy(...)` runtime mutation explicitly deferred.

## Phase 87 Downloads Runtime Policy Startup Seeding Findings

- AT-2026-05-17-211 final commit is `1d31f56` and was pushed to `origin/main`.
- Current composition-root constructs `SharedJobRuntimeHost` before constructing the downloads facade policy store, so persisted policy cannot yet influence runtime startup budget.
- A focused RED test can call the private `build_job_runtime(...)` from the composition-root unit-test module after seeding `SqliteDownloadPolicyStore` on a project-local SQLite path.
- The implementation should pass the same `SqliteDownloadPolicyStore` through composition-root into both `build_job_runtime(...)` and `DownloadModuleDeps.policy_store`.
- RED/GREEN confirmed persisted `DownloadPolicyDto.concurrency_slots` now seeds the initial `RuntimeQueuePolicy.max_concurrent_jobs`, while an empty policy table falls back to `DesktopBootstrapConfig.default_download_slots`.
- The same `SqliteDownloadPolicyStore` object is constructed before runtime assembly and then moved into the downloads facade; live `update_policy(...)` runtime mutation remains absent.
- Full composition-root integration tests were not run because existing integration tests still create/delete sqlite files under system temp/default package paths; focused lib tests used project-local `.artifacts/tmp` paths instead.

## Phase 88 Downloads Live Runtime Policy Update Boundary Findings

- AT-2026-05-17-212 final commit is `ed27996` and was pushed to `origin/main`.
- Host transport already has `downloads_get_policy` and `downloads_update_policy` handlers that call the downloads facade, so the immediate missing boundary is not command existence.
- Current `SharedJobRuntimeHost` exposes `policy()` but has no update method; current `DownloadsFacade::update_policy(...)` persists policy only.
- The next safe design boundary is a `kernel-jobs` runtime policy control surface first, followed later by downloads facade/composition wiring.
- README_IMPL 7.27 now pins the first Rust slice to focused kernel-jobs tests for `SharedJobRuntimeHost` policy update/readback, with downloads facade wiring and transport/frontend work deferred.

## Phase 89 Kernel Jobs Runtime Policy Control Findings

- AT-2026-05-17-213 final commit is `38c32b2` and was pushed to `origin/main`.
- Current `SharedJobRuntimeHost` stores `RuntimeQueuePolicy` by value, so clones cannot observe a later policy replacement without changing the internal storage shape.
- The narrow RED target is a cloned-host readback test: update through one host handle, then assert both the original and clone return the new `RuntimeQueuePolicy` snapshot.
- RED/GREEN confirmed `SharedJobRuntimeHost::update_policy(...)` replaces a shared runtime policy snapshot while `policy()` keeps returning by-value `RuntimeQueuePolicy` copies.
- The implementation stores policy behind `Arc<Mutex<RuntimeQueuePolicy>>`; cloned hosts now observe the same updated snapshot without changing the public `JobRuntime` trait or downloads facade behavior.
- Package-level `cargo fmt -p launcher-kernel-jobs --check` is still blocked by pre-existing out-of-scope formatting diffs in `crates/kernel-jobs/src/lib.rs` and `crates/kernel-jobs/src/model.rs`; AT-214 formatted and checked only `crates/kernel-jobs/src/runtime.rs`.

## Phase 90 Downloads Runtime Policy Applier Boundary Findings

- AT-2026-05-17-214 final commit is `c92be25` and was pushed to `origin/main`.
- README/docs routing confirms concrete module work must read module docs plus related architecture, testing, composition, and collaboration documents before coding.
- `TauriDownloadRuntimeDesign.md` defines the user-facing concurrency setting as `downloadConcurrencySlots: 1..128` and maps it to a clamped backend global slot budget.
- `TauriKernelJobsRuntimeDesign.md` keeps runtime queue policy in `kernel-jobs`, while module business checkpoints and policy snapshots remain module-owned.
- `TauriCompositionRootWiringDesign.md` keeps concrete dependency assembly in `launcher-composition-root`; host/transport layers should not see concrete adapter/runtime details.
- Current `DownloadsFacade::update_policy(...)` normalizes and persists `DownloadPolicyDto` through `DownloadPolicyStore`, but has no dedicated runtime policy applier dependency.
- The next safe Rust slice should introduce a downloads-owned policy applier port that receives the normalized persisted `DownloadPolicyDto`; composition-root concrete wiring to `SharedJobRuntimeHost::update_policy(...)` should remain a later slice.
- README_IMPL 7.28 now pins the first Rust slice to `module-downloads`: add a narrow runtime policy applier port, call it with normalized `DownloadPolicyDto` after persistence, and keep concrete runtime wiring in composition-root for a later task.

## Phase 91 Downloads Runtime Policy Applier Port Findings

- AT-2026-05-17-215 final commit is `4ef3f10` and was pushed to `origin/main`.
- Current `DownloadModuleDeps` is constructed directly in many module tests, so adding another required generic field would create wide test churn.
- A smaller module-local approach is to store a runtime policy applier inside `DownloadFacade` itself: `new(...)` can install a no-op default, and a new opt-in constructor can accept a test/later composition applier.
- The applier should receive `DownloadPolicyDto`, not `RuntimeQueuePolicy`, so downloads code stays independent from concrete runtime policy construction and `SharedJobRuntimeHost`.
- RED/GREEN confirmed `DownloadsFacade::update_policy(...)` saves the normalized policy and passes that same normalized `DownloadPolicyDto` to a dedicated `DownloadRuntimePolicyApplier`.
- `NoopDownloadRuntimePolicyApplier` keeps existing `DownloadFacade::new(...)` behavior stable; the opt-in `with_runtime_policy_applier(...)` constructor supports focused tests and later composition-root wiring without changing `DownloadModuleDeps`.

## Phase 92 Composition-root Downloads Runtime Policy Applier Wiring Findings

- AT-2026-05-17-216 final commit is `1094c10` and was pushed to `origin/main`.
- README_IMPL 7.28 leaves composition-root as the owner of concrete mapping from `DownloadPolicyDto.concurrency_slots` to `RuntimeQueuePolicy::new(...)` and `SharedJobRuntimeHost::update_policy(...)`.
- Current `build_downloads_module(...)` receives a `SharedJobRuntimeHost` clone and constructs `DownloadFacade::new(...)`, so it can create a private applier from the same cloned runtime without changing public service types.
- Focused composition-root tests already use project-local SQLite paths under `.artifacts/tmp`; AT-217 should keep that safety boundary.
- RED/GREEN confirmed composition-root now wires downloads policy updates to the shared runtime policy snapshot: `downloads.update_policy(...)` changes the cloned `SharedJobRuntimeHost::policy().max_concurrent_jobs`.
- The concrete mapping remains private to composition-root: `DownloadPolicyDto.concurrency_slots` becomes `RuntimeQueuePolicy::new(policy.concurrency_slots as usize)`.

## Phase 93 Documentation Budget Rules Findings

- AT-2026-05-17-217 final commit is `37765ef` and was pushed to `origin/main`.
- The user confirmed the repository has started drifting toward documentation overload, especially when `README_IMPL.md` records every AT as long-form history.
- The least disruptive rule change is not to delete existing docs immediately, but to add a forward rule: `.artifacts/ai` owns task logs/validation/commit facts; `docs/` owns durable boundaries and contracts.
- Central rule surfaces to update are `docs/ModuleDocumentationStandard.md`, `docs/README.md`, `.github/copilot-instructions.md`, `.github/skills/strict-doc-driven-development/SKILL.md`, and `.windsurf/rules/repo-workflow.md`.
- AT-218 validation passed for the intended rule surfaces: central docs now define a documentation budget, task execution records stay in `.artifacts/ai`, and long per-AT `README_IMPL.md` completion logs are discouraged.

## Phase 94 Downloads Policy Host Transport Smoke Findings

- AT-2026-05-17-218 final commit is `5aae7f1` and was pushed to `origin/main`.
- README/docs routing, module docs, README_IMPL 7.28, IPC contracts, composition-root wiring, testing strategy, and task protocol docs were read in focused chunks before coding.
- README_IMPL 7.28 explicitly leaves the next later slice as host transport tests proving `downloads_update_policy` persists policy and applies the runtime snapshot.
- Current `src-tauri/src/commands/downloads.rs` already forwards `downloads_update_policy(...)` to `services.downloads.update_policy(...)`.
- Current `DesktopAppServices.downloads.deps().job_runtime` is visible to the transport smoke test, so the smallest proof can assert `policy().max_concurrent_jobs` after the command succeeds without adding public host APIs.
- The slice should not update README_IMPL because it validates an already-documented durable boundary rather than changing one.
- The new host smoke assertion passed immediately, confirming no production code change was needed after AT-217; this task adds regression coverage for the existing host/composition path.

## Phase 95 Shared Runtime Execution-Turn Boundary Findings

- AT-2026-05-17-219 final commit is `f618718` and was pushed to `origin/main`.
- README_IMPL 7.13 says `JobDriver::run(...)` is a future boundary and current `kernel-jobs::JobDriver` only exposes `module()`, `kind()`, and `restore()`.
- `TauriKernelJobsRuntimeDesign.md` defines the desired future driver shape with `restore(...)`, `run(...)`, a runtime context, queue policy, lease ownership, and snapshot writer boundaries.
- Current `kernel-jobs` Rust has `JobDriverRegistry::resolve(...)`, `SharedJobRuntimeHost` enqueue/snapshot/pause/resume/cancel/policy methods, and no runtime-owned execution turn or lease API.
- The next code task needs a narrow `kernel-jobs` contract first; concrete downloads fetch/write/verify, retry/backoff, terminal completion, host transport, and frontend work must remain deferred.
- README_IMPL 7.29 now pins that next durable boundary: add a module-neutral execution-turn contract in `kernel-jobs` first, validated with fake-driver tests, before downloads integrates concrete execution.

## Phase 96 Kernel Jobs Execution-Turn Contract Findings

- AT-2026-05-17-220 final commit is `aa8d6e3` and was pushed to `origin/main`.
- Required context was read in focused chunks: README_IMPL 7.29, kernel-jobs runtime design driver/queue/lease/recovery/runtime-context sections, downloads scheduler/budget notes, current `kernel-jobs` runtime/lib/model code, and current module driver implementations.
- The smallest code slice is inside `kernel-jobs`: add a read-only execution context and explicit run disposition, then prove a fake driver can override `run(...)`.
- To preserve existing modules without claiming real execution, the default `JobDriver::run(...)` should return an explicit deferred disposition until a module overrides it.
- RED/GREEN confirmed the contract: missing `JobExecutionContext`, `JobRunDisposition`, and `JobDriver::run(...)` failed first; after implementation, focused and full `launcher-kernel-jobs` lib tests passed.
- `cargo check -p launcher-composition-root` also passed, proving current Fab/downloads/engines drivers compile through the default deferred run path.

## Phase 97 Shared Runtime Execution Dispatch Boundary Findings

- AT-2026-05-17-221 final commit is `89d3a19` and was pushed to `origin/main`.
- Required context was read in focused chunks: README/docs routing, downloads module ARCH/API/FLOW/README_IMPL 7.29, kernel-jobs runtime driver/runtime-host/runtime-context sections, download runtime scheduler/budget notes, current `kernel-jobs` runtime/lib/model code, and composition-root driver-registry wiring.
- The next Rust slice should stay in `kernel-jobs` and compose existing facts rather than jumping into downloads execution: `SharedJobRuntimeHost` can read snapshots, `JobDriverRegistry` can resolve drivers, and `JobDriver::run(...)` can accept one read-only context.
- The first dispatch boundary should be one-shot and explicit: missing snapshot or missing driver should return `JobRunDisposition::Deferred`, not silently succeed.
- Scheduler loops, durable leases, snapshot writer/cancellation context, terminal state mutation, downloads driver overrides, concrete IO, retry/backoff, host transport, frontend, and SQLite schema remain later boundaries.
- Scoped docs-only validation passed with `git diff --check` for README_IMPL and PWF files; warnings were CRLF normalization only.

## Phase 98 Kernel Jobs One-shot Execution Dispatch Findings

- AT-2026-05-17-222 final commit is `feddcfc` and was pushed to `origin/main`.
- Required context was read in focused chunks: README/docs routing, downloads module ARCH/API/FLOW/README_IMPL 7.29-7.30, kernel-jobs runtime design driver/runtime-host/runtime-context sections, download runtime scheduler/budget notes, testing strategy job-runtime guidance, current `kernel-jobs` runtime/lib/model code, and composition-root driver-registry wiring.
- README_IMPL 7.30 pins the next code slice to `SharedJobRuntimeHost`: load one snapshot, resolve one driver, and call one `driver.run(...)` through a read-only context.
- The dispatch method should not mutate state yet. Missing snapshots or drivers should be explicit deferred dispositions so callers can observe that execution was not started.
- RED/GREEN confirmed the one-shot dispatch boundary: the initial focused tests failed on missing `run_one_execution_turn(...)`; after implementation, focused dispatch tests passed.
- Full `launcher-kernel-jobs` lib tests passed with 7 tests, and `cargo check -p launcher-composition-root` passed, confirming existing module drivers still compile through the default deferred run path.
- `run_one_execution_turn(...)` leaves the queued snapshot lifecycle state unchanged, keeping scheduler loops, leases, terminal state mutation, and downloads concrete execution out of scope.

## Phase 99 Downloads Driver Runtime-run Boundary Findings

- AT-2026-05-17-223 final commit is `f87df03` and was pushed to `origin/main`.
- Required context was read in focused chunks: downloads README_IMPL execution sections, current downloads driver helpers, kernel-jobs runtime design, and current one-shot runtime dispatch code.
- A direct downloads `run(...) -> prepare_resume_execution_turn(...)` override would be unsafe because `prepare_resume_execution_turn(...)` drains pending work after checkpoint reload; if no segment execution port is available, the prepared work could be consumed without execution.
- The next safe code slice should therefore make the downloads run override explicitly defer unless a driver-owned segment execution port (or equivalent execution strategy) is present.
- README_IMPL 7.31 now pins the next Rust slice to an optional downloads-owned segment execution port/deferred default path before implementing `DownloadJobDriver::run(...)`.
- Scoped docs-only validation passed with `git diff --check`; warnings were CRLF normalization only.

## Phase 100 Downloads Driver Guarded Run Override Findings

- AT-2026-05-17-224 final commit is `597c0e5` and was pushed to `origin/main`.
- Required context was read in focused chunks: README_IMPL 7.31, current downloads driver helpers/tests, kernel-jobs runtime driver/context design, and download runtime ownership/checkpoint sections.
- The default `DownloadJobDriver::new(...)` and `with_pending_resume_work_source(...)` paths must not drain pending work from `run(...)` because no execution port is present.
- The opt-in fake execution path can reuse existing `execute_local_resume_turn(...)` with `CompletedSegmentExecutionPort` test coverage to prove checkpoint mutation without concrete IO.
- RED/GREEN confirmed the guarded run boundary: source-only drivers defer before draining, while opt-in fake completed execution persists checkpoint mutation and returns `JobRunDisposition::Accepted`.
- Full `launcher-module-downloads` lib tests passed with 49 tests, and `cargo check -p launcher-composition-root` passed.

## Phase 101 Downloads Driver Run Deferred Branch Coverage Findings

- AT-2026-05-17-225 final commit is `c5b0695` and was pushed to `origin/main`.
- README_IMPL 7.31 requires missing checkpoint, no pending work, and no checkpoint mutation to return deferred rather than pretend completion.
- The implementation already returns `Deferred` when `execute_local_resume_turn(...)` returns `None`; focused branch tests should confirm the missing-checkpoint path also keeps pending work intact.
- RED/GREEN showed the missing-checkpoint branch already deferred and preserved pending work, but no-pending-work and Accepted-only/no-mutation initially returned `Accepted`.
- Root cause: `record_completed_segment_checkpoints(...)` and `record_failed_segment_checkpoints(...)` returned `Some(checkpoint)` even when no completed/failed result was recorded, so `DownloadJobDriver::run(...)` misclassified a non-mutating local turn as accepted.
- The minimal behavior fix is for those helpers to return `None` when they do not record a matching result; checkpoint-mutating completed/failed paths still return `Some(checkpoint)` after saving.
- Validation passed for focused `driver_run` tests, full `launcher-module-downloads` lib tests, `launcher-composition-root` check, and scoped rustfmt on `driver.rs`.
- AT-2026-05-17-226 final commit is `d2d5405` and was pushed to `origin/main`.

## Phase 102 Accepted Execution State Projection Boundary Findings

- Required context was read in focused chunks: README/CONTRIBUTING routing, README_IMPL 7.29-7.31, kernel-jobs lifecycle/driver/context/snapshot sections, composition-root runtime/driver wiring, testing strategy backend test placement, current runtime dispatch code, current downloads driver run behavior, and composition-root driver registry wiring.
- Current `SharedJobRuntimeHost::run_one_execution_turn(...)` dispatches to the driver but intentionally leaves the snapshot lifecycle unchanged from AT-223.
- `JobRunDisposition::Accepted` explicitly means the driver accepted a non-terminal execution turn; the next safe shared-runtime projection is therefore `Queued -> Running` / UI `Running`, not `Completed` or `Failed`.
- `Deferred` must remain non-mutating because downloads production wiring still uses a no-execution-port driver and should not consume pending work or claim execution.
- Durable lease acquisition, background scheduler loops, cancellation/snapshot-writer context, downloads concrete IO, retry/backoff, terminal completion/failure, host transport, frontend, and SQLite schema remain later boundaries.
- README_IMPL 7.32 now defines the next Rust slice and scoped validation; docs-only `git diff --check` passed with CRLF normalization warnings only.
- AT-2026-05-17-227 final commit is `fc615db` and was pushed to `origin/main`.

## Phase 103 Accepted Execution State Projection Findings

- Required context was read in focused chunks: README_IMPL 7.32, kernel-jobs lifecycle/driver/context/snapshot rules, testing strategy kernel-jobs guidance, current runtime dispatch tests, and current `JobRunDisposition` / `JobSnapshotStore` contracts.
- Current accepted dispatch test still expects the snapshot to remain `Queued`; that assertion should flip to `Running` and fail before implementation.
- Deferred dispatch with a missing driver already returns `Deferred`; AT-228 should add/adjust coverage to assert the queued snapshot remains unchanged.
- RED/GREEN confirmed accepted dispatch now projects the stored snapshot to `JobState::Running` and `JobUiState::Running`, while missing-driver deferred dispatch keeps the queued snapshot unchanged.
- The runtime update is deliberately non-terminal: it does not acquire leases, start loops, create snapshot-writer/cancellation context, or project `Completed` / `Failed`.
- Validation passed for focused dispatch tests, full `launcher-kernel-jobs` lib tests, `launcher-composition-root` check, and scoped rustfmt on `runtime.rs`.
- AT-2026-05-17-228 final commit is `fb9fb57` and was pushed to `origin/main`.

## Phase 104 One-shot Queued Execution Selection Boundary Findings

- Required context was read in focused chunks: README_IMPL 7.32/current state, kernel-jobs queue policy and eligible-job selection notes, testing strategy kernel-jobs guidance, current `run_one_execution_turn(...)`, and current `JobSnapshotStore::list_resumable(...)` implementations.
- `SqliteJobSnapshotStore::list_resumable(...)` currently returns all resumable states with no explicit `ORDER BY`, and the in-memory store is HashMap-backed; a one-shot selector must impose deterministic ordering itself or first document a store ordering contract.
- The safest next Rust slice is a runtime-owned one-shot selector that filters `JobState::Queued`, orders selected candidates deterministically, and calls the existing `run_one_execution_turn(...)` for exactly one job.
- Scheduler loops, active-slot accounting, per-module fairness, lease acquisition, cancellation/snapshot-writer context, terminal projection, downloads concrete IO, host transport, frontend, and SQLite schema remain later boundaries.
- README_IMPL 7.33 now defines the next Rust slice and scoped validation; docs-only `git diff --check` passed with CRLF normalization warnings only.

## Phase 105 One-shot Queued Execution Selector Findings

- Required context was read in focused chunks: README.md, docs/README.md, ModuleDocumentationStandard documentation-budget rules, README_IMPL 7.33, kernel-jobs queue/eligible selection notes, testing strategy kernel-jobs guidance, current runtime dispatch code, `JobId`/`IsoDateTime` contracts, and current memory/SQLite `list_resumable(...)` implementations.
- AT-2026-05-17-229 is already published at `d339db7` on `origin/main`; stale PWF notes that said commit/push pending were corrected while opening AT-230.
- `JobId` does not implement `Ord`, but it implements `Display`, so selector tie-breaking should use `job_id.to_string()` rather than changing the shared ID type.
- `IsoDateTime` derives `Ord`, so `(updated_at, job_id)` ordering can be expressed in the runtime selector without adding store ordering or schema changes.
- Plain git commands are blocked by repository ownership protection in this shell; use temporary `git -c safe.directory=D:/DEV/MyEpicLauncher ...` so no global config or project-external files are modified.
- RED/GREEN confirmed `SharedJobRuntimeHost::run_next_execution_turn(...)` filters only `Queued` snapshots, orders ties by job id after `updated_at`, delegates one selected job to existing dispatch, and returns `Deferred` when no queued snapshot exists.
- Validation passed for focused queued-selector tests, full `launcher-kernel-jobs` lib tests, `launcher-composition-root` check, scoped rustfmt, and scoped diff-check.
- AT-2026-05-17-230 final commit `8db4900` was pushed to `origin/main`.

## Phase 106 One-shot Queue Policy Slot Gate Boundary Findings

- Required context was read in focused chunks: README_IMPL 7.33, kernel-jobs queue policy design, downloads concurrency/budget notes, current task plan, and AT-230 pushed state.
- `run_next_execution_turn(...)` now selects one queued snapshot deterministically, but it does not yet use `RuntimeQueuePolicy::max_concurrent_jobs`.
- The safe next boundary is not a scheduler loop or lease model; it is a minimal snapshot-observed gate that counts current `JobState::Running` snapshots from `list_resumable()` and defers selection when that count is greater than or equal to `max_concurrent_jobs`.
- `max_concurrent_jobs == 0` should mean no runtime capacity for this selector, even though downloads user-facing policy is normally clamped before it reaches `RuntimeQueuePolicy`.
- README_IMPL 7.34 now defines the next Rust slice and validation expectations; scoped docs/PWF `git diff --check` passed with CRLF normalization warnings only.
- AT-2026-05-17-231 final commit `6f5bd32` was pushed to `origin/main`.

## Phase 107 One-shot Queue Policy Slot Gate Findings

- Required context was read in focused chunks: README_IMPL 7.34, kernel-jobs queue policy notes, downloads concurrency/budget notes, current `run_next_execution_turn(...)`, current selector tests, and AT-231 pushed state.
- Current `run_next_execution_turn(...)` performs one `list_resumable()` read already, so the policy gate can reuse that snapshot list rather than adding a new store API or relying on adapter ordering.
- The implementation should check capacity before queued candidate selection; if capacity is exhausted, the queued snapshot should remain `Queued` and the driver must not be called.
- Existing no-queued coverage should avoid mixing with capacity exhaustion by using a policy that has room for the running fixture or by using no running fixture.
- RED/GREEN confirmed `run_next_execution_turn(...)` now defers when `running_count >= max_concurrent_jobs`, treats zero capacity as exhausted, and still dispatches the deterministic queued candidate when capacity remains.
- Full `launcher-kernel-jobs` lib tests, `launcher-composition-root` check, scoped rustfmt, and scoped diff-check passed.
- AT-2026-05-17-232 final commit `d2fa1d9` was pushed to `origin/main`.

## Phase 108 Composition One-shot Runtime Execution Helper Boundary Findings

- Required context was read in focused chunks: composition-root wiring docs, startup pipeline restore/warmup rules, README_IMPL 7.34, current bootstrap runtime/registry assembly, current startup facade registry usage, and AT-232 pushed state.
- Composition-root already builds one `SharedJobRuntimeHost` and one `JobDriverRegistry<()>`, but no public composition-owned helper currently combines them to call `run_next_execution_turn(...)`.
- `StartupPipelineFacade` already owns staged restore/prewarm entry points and has driver registry context; any execution helper must stay explicit and must not be automatically invoked by `build_desktop_services()`, stage 2 restore, or stage 3 prewarm.
- The next Rust slice should remain one-shot and testable; scheduler loops, background tasks, leases, terminal projection, downloads concrete IO, transport, frontend, and schema changes remain separate.
- `docs/TauriCompositionRootWiringDesign.md` 9.4 now defines the helper boundary and validation expectations; scoped docs/PWF diff-check passed with CRLF normalization warnings only.
- AT-2026-05-17-233 final commit `01c206d` was pushed to `origin/main`.

## Phase 109 Composition One-shot Runtime Execution Helper Findings

- Required context was read in focused chunks: composition docs 9.4, startup pipeline restore/warmup rules, current `StartupPipelineFacade`, current bootstrap runtime/registry wiring, and existing startup/bootstrap tests.
- `StartupPipelineFacade::new(...)` is used in multiple tests; preserving that constructor and adding an opt-in runtime wiring method should minimize churn.
- `build_desktop_services(...)` currently consumes `job_runtime` into the engines module before building startup, so implementation will need to clone the runtime for engines/startup wiring rather than moving it once.
- Bootstrap smoke can distinguish wired vs absent execution helper by expecting a no-queued deferred result from a fresh project-local SQLite store.
- RED/GREEN confirmed `StartupPipelineFacade::run_one_runtime_execution_turn(...)` defers when runtime wiring is absent and delegates exactly one queued fake-driver job when runtime plus registry are wired.
- Bootstrap wiring now clones the shared runtime into the startup surface and proves a fresh project-local store returns the runtime's no-queued deferred result.
- Existing startup restore/prewarm tests remained green; no execution helper is invoked automatically by stage 2 or stage 3.
- AT-2026-05-17-234 final commit `256f89b` was pushed to `origin/main`.

## Phase 110 Host Runtime Execution Command Boundary Findings

- Required context was read in focused chunks: README/docs routing, composition-root helper docs 9.4, composition Tauri integration rules, IPC command/query envelopes and implementation guidance, startup stage ownership rules, downloads runtime execution sections 7.29-7.34, current host command modules, bootstrap/state wrappers, and transport smoke tests.
- The next boundary should be a command, not a query, because one runtime execution turn can mutate backend-owned job snapshot state from `Queued` to `Running`.
- Tauri command handlers are allowed to depend on `DesktopAppServices` and IPC DTOs only; they must not take `SharedJobRuntimeHost`, repositories, or driver registries directly.
- `StartupPipelineFacade::run_one_runtime_execution_turn(...)` is now the right composition-owned entry point because it already composes the shared runtime with the driver registry and preserves explicit invocation.
- The host DTO should expose only a coarse runtime turn disposition (`accepted`, `deferred`, `failed`) and optional reason text; downloads segment checkpoint/work details must stay module-local.
- Deferred or failed execution-turn dispositions are non-terminal runtime outcomes for this first host command and should remain successful command envelopes unless the composition helper itself returns `AppError`.
- AT-235 validation passed with scoped docs/PWF diff-check; warnings were CRLF normalization only.
- AT-2026-05-17-235 final commit `18ea7d7` was pushed to `origin/main`.

## Phase 111 Host Runtime Execution Command Findings

- Required context was read in focused chunks: IPC section 7.4, composition helper docs, startup ownership rules, downloads execution sections, current command registry, jobs command module, desktop bootstrap/state wrappers, and transport smoke test.
- `DesktopAppServices.startup` is public and exposes the already-wired `StartupPipelineFacade`, so the host command can remain a thin transport call without taking `SharedJobRuntimeHost` or repositories directly.
- Current `src-tauri` command tests call handler functions directly through `bootstrap.services.services()` and use `REGISTERED_COMMANDS` for the registration surface; the RED test should follow that pattern.
- Existing command DTOs are Rust-side `Debug/Clone/PartialEq/Eq` projections rather than serde-enabled public TS generation, so the first DTO can follow the local enum/struct style and leave TS generation out of scope.
- RED/GREEN confirmed the host command boundary: the initial transport smoke failed on the missing DTO/handler, then passed after adding `RuntimeExecutionTurnDto`, `RuntimeExecutionTurnDispositionDto`, `map_runtime_execution_turn_result(...)`, command registration, and `jobs_run_next_execution_turn(...)`.
- The command calls only `services.startup.run_one_runtime_execution_turn()` and maps the resulting disposition; no runtime/composition/downloads internals changed.
- `Deferred` and `Failed` execution-turn dispositions are mapped into successful command DTOs, while `AppError` remains the only path into `CommandResultDto::Failure`.
- Full desktop package tests and compile gate passed; scoped rustfmt used `--config skip_children=true` to avoid pre-existing out-of-scope `fab.rs` formatting churn.
- AT-2026-05-17-236 final commit `f720d9c` was pushed to `origin/main`.

## Phase 112 Host Runtime Command Downloads Deferred Coverage Findings

- Required context was read in focused chunks: IPC 7.4, downloads README_IMPL 7.31-7.34, `DownloadJobDriver::run(...)`, `SharedJobRuntimeHost::run_next_execution_turn(...)`, and the current transport smoke helper.
- Production composition-root still wires `DownloadJobDriver` without a segment execution port, so dispatching a queued downloads job should return a deferred disposition mentioning the missing execution port.
- `run_one_execution_turn(...)` keeps deferred dispatch non-mutating, so the host smoke can assert the downloads job snapshot remains `Queued` / `Queued`.
- The existing isolated sqlite helper in `transport_wiring_smoke` is the right place to avoid default smoke database pollution.
- Validation confirmed the host command reaches the queued production downloads job and returns `Deferred` with `execution port not wired`.
- The snapshot remains `Queued` / `Queued`, preserving the documented deferred non-mutation rule.
- AT-237 touched only the transport smoke test plus PWF records; no production Rust behavior changed.
- AT-2026-05-17-237 final commit `a8e3492` was pushed to `origin/main`.

## Phase 113 Downloads Concrete Segment Execution Boundary Findings

- Required context was read in focused chunks: downloads module docs routing/current-state table, README_IMPL runtime execution sections, Tauri download runtime fetcher/writer/verifier/staging references, kernel-jobs driver/runtime context references, and current segment execution request/result/port shape.
- Current `DownloadSegmentExecutionPort` is intentionally broad enough for fake/future executors, but real IO still needs explicit fetch/write/verify responsibility boundaries before production wiring.
- `DownloadSegmentExecutionRequest` already carries source locator, staging write target, optional hash expectation, range offsets, resume mode, and checkpoint reference, so the next boundary can avoid changing public host transport or `kernel-jobs` payloads.
- Real execution must not be introduced in the same slice as retry/backoff, terminal snapshot projection, scheduler loops, leases, or frontend state.
- README_IMPL 7.35 now defines the next safe Rust slice as a module-owned executor adapter shell behind `DownloadSegmentExecutionPort`, starting with fake/in-memory sub-ports or adapter-shell tests rather than real IO.
- AT-238 scoped docs/PWF diff-check passed with CRLF normalization warnings only.
- AT-2026-05-17-238 final commit `d5af454` was pushed to `origin/main`.

## Phase 114 Downloads Segment Executor Adapter Shell Findings

- Required context was read in focused chunks: downloads module ARCH/API/FLOW snippets, README_IMPL 7.35, Tauri download runtime fetch/write/verify/staging references, kernel-jobs driver/runtime references, current driver request/result/port code, and existing driver tests.
- Existing `DownloadSegmentExecutionRequest` has enough facts for an adapter-shell test: source locator, staging target, expected hash, length/range, resume mode, and checkpoint reference.
- The first code slice should not wire composition-root production execution. It should prove only that an adapter behind `DownloadSegmentExecutionPort` can pass request facts through fake fetch/write/verify sub-ports and return the existing `Completed` result shape.
- Public exports in `crates/module-downloads/src/lib.rs` should include the new adapter shell and sub-port contracts if they are intended as module-owned extension points.
- RED/GREEN confirmed `DownloadSegmentExecutor` composes fake in-memory `DownloadSegmentFetchPort`, `DownloadSegmentWritePort`, and `DownloadSegmentVerifyPort` implementations behind the existing driver-facing `DownloadSegmentExecutionPort`.
- The adapter returns the existing `DownloadSegmentExecutionResult::Completed` shape from sub-port facts and does not change driver request/result shape, checkpoint mutation helpers, `kernel-jobs`, composition-root production wiring, host transport, frontend, SQLite schema, retry/backoff, or real HTTP/disk/hash IO.
- Validation passed for the focused adapter test, existing `driver_run` tests, full `launcher-module-downloads` lib tests, `launcher-composition-root` compile gate, and scoped rustfmt.

## Phase 115 Downloads Segment Executor Failure Mapping Boundary Findings

- Required context was read in focused chunks: README/docs routing, ModuleDocumentationStandard documentation-budget rules, downloads ARCH/API/FLOW boundaries, README_IMPL 7.35 plus error semantics, Tauri error handling/projection rules, and download runtime error taxonomy notes.
- README_IMPL 7.35 already says fake sub-ports can produce an existing `Completed` or `Failed` result, but AT-239 only implemented the successful `Completed` adapter path.
- Existing docs separate module-local `DownloadSegmentExecutionResult::Failed` from stable public `DL_*` execution projection; public execution errors must still wait for a later projection boundary.
- The next safe code target should not invent retry/backoff or terminal runtime behavior. It should first define when a sub-port failure is a handled segment execution failure versus when an infrastructure/configuration error remains an `AppError`.
- README_IMPL 7.36 now defines that handled fetch/write/verify segment failures become in-band `DownloadSegmentExecutionResult::Failed`, while infrastructure/configuration errors that prevent a segment decision may still propagate as `AppError`.
- The next Rust slice is now concrete enough: add fake sub-port coverage for a handled write or verify failure, map it to `Failed`, preserve `AppError` propagation for true infrastructure failures, and rerun focused adapter plus existing failed-checkpoint tests.

## Phase 116 Downloads Segment Executor Handled Failure Mapping Findings

- Required context was read in focused chunks: README/docs routing, ModuleDocumentationStandard documentation-budget rules, README_IMPL 7.35/7.36, error handling/projection rules, `AppError` shape, and current executor/sub-port traits/tests.
- `DownloadSegmentExecutor` currently maps only the successful fake path into `Completed`; sub-port failures can only be represented as `AppError` because fetch/write/verify trait methods return `AppResult<...>` or `AppResult<()>`.
- To keep public `DL_*` projection out of scope, the code slice should add module-local handled-failure outcomes for sub-ports and have the adapter translate those to `DownloadSegmentExecutionResult::Failed`.
- `AppError` should remain available for true infrastructure/configuration failures, e.g. the sub-port cannot make a segment decision at all.
- RED/GREEN confirmed the adapter now distinguishes handled segment failures from infrastructure errors: `DownloadSegmentWriteOutcome::Failed` maps to `DownloadSegmentExecutionResult::Failed`, while a verifier `AppError` still propagates unchanged.
- New outcome types remain module-local/public Rust extension points only; no public `DL_*` execution projection, transport DTO, retry/backoff, terminal runtime state, production wiring, real IO, or frontend work was added.
- Validation passed for focused adapter tests, the existing failed-result checkpoint mutation test, full `launcher-module-downloads` lib tests, `launcher-composition-root` check, and scoped rustfmt.

## Phase 117 Downloads Segment Staging Target Guard Boundary Findings

- Required context was read in focused chunks: README/docs routing and documentation-budget rules from this session, README_IMPL 7.35/7.36, Tauri download runtime SegmentWriter/staging/failure notes, storage placement notes for download staging files, and current `write_target`/writer sub-port shape.
- Real writer IO is still premature because staging root ownership, path normalization, partial/temp naming, and artifact moves must not be guessed inside the executor adapter.
- The next safe backend slice can still make progress without disk IO: validate `DownloadSegmentExecutionRequest.write_target` as a staging-relative target before a writer would touch the file system.
- Unsafe `write_target` values are a handled module-local segment failure for now, not a stable public `DL_*` error or host transport projection.
- README_IMPL 7.37 now defines accepted/rejected staging target cases and keeps the first Rust guard pure: no directory creation, file opening, host canonicalization, artifact movement, hash verification, provider inspection, production wiring, or public error projection.
- The next Rust slice is now concrete: add focused RED tests for safe and unsafe staging-relative targets, implement a module-owned guard, and prove unsafe targets can become the existing handled failure path without disk IO.

## Phase 118 Downloads Staging Target Guard Findings

- Required context was read in focused chunks: README/docs routing and documentation-budget rules from this session, README_IMPL 7.37, Tauri download runtime SegmentWriter/staging sections, storage design staging file ownership notes, and current handled-failure/writer sub-port contracts.
- The guard should be pure path-component inspection: it must not canonicalize through the host file system because future staging targets may not exist yet.
- Unsafe target rejection should reuse `DownloadSegmentHandledFailure` with zero downloaded bytes and `retryable = false`.
- Re-exporting the guard type is appropriate if future writer sub-ports need it as a module-owned extension point.
- RED/GREEN confirmed `DownloadSegmentStagingTarget::parse(...)` accepts a normal relative target and rejects empty, current-dir, parent-dir, drive-prefixed, UNC/rooted, and absolute-looking targets as handled failures.
- The implementation uses path component inspection only and does not create directories, open files, canonicalize, move artifacts, inspect provider URLs, or introduce public error projection.
- Validation passed for focused guard tests, focused adapter tests, full `launcher-module-downloads` lib tests, `launcher-composition-root` check, and scoped rustfmt.

## Phase 115 Downloads Guarded Writer Boundary Findings

- Required context was read in focused chunks: README/docs routing, docs map, documentation-budget rules, downloads ARCH/API/FLOW notes, README_IMPL 7.35-7.37, download runtime fetch/write/verify/staging notes, storage staging-file ownership notes, and current driver writer/handled-failure contracts.
- AT-243 added the pure `DownloadSegmentStagingTarget` guard, but `DownloadSegmentWritePort` still receives the raw request; integrating the guard directly into a future disk writer would mix path-safety semantics with IO too early.
- The next durable boundary should be a module-owned guarded writer wrapper behind `DownloadSegmentWritePort`: unsafe targets become `DownloadSegmentWriteOutcome::Failed`, safe targets delegate unchanged to an injected writer.
- Real directories, file writes, temp naming, staging-root canonicalization, artifact moves, hash checks, public `DL_*` projection, retry/backoff, terminal runtime state, production wiring, transport, frontend, and schema remain out of scope.

## Phase 116 Downloads Guarded Writer Port Findings

- Required context for AT-245 is already fresh in this session: README/docs routing, module docs, README_IMPL 7.38, runtime writer/staging notes, storage staging ownership, and current writer/guard code.
- The wrapper should be module-owned and test-only/future-port friendly: it should hold `Arc<dyn DownloadSegmentWritePort>` and implement the same trait.
- Unsafe target handling can reuse `DownloadSegmentStagingTarget::parse(...)` directly, returning `Ok(DownloadSegmentWriteOutcome::Failed(failure))` so the existing executor mapping turns it into a failed segment result later.
- Safe target delegation should not rewrite the request yet because no concrete writer accepts a typed `DownloadSegmentStagingTarget` value in this boundary.
- RED/GREEN confirmed the wrapper rejects unsafe targets without calling the inner writer, delegates safe targets exactly once, and propagates inner writer `AppError` values unchanged.

## Phase 117 Downloads Guarded Writer Executor Coverage Findings

- Required context was read in focused chunks: README/docs routing, downloads module docs, README_IMPL 7.36-7.38, download runtime writer/staging notes, documentation budget/testing rules, and current executor/guarded-writer tests.
- The next useful slice is coverage, not a new abstraction: `DownloadSegmentExecutor` already maps `DownloadSegmentWriteOutcome::Failed` to `DownloadSegmentExecutionResult::Failed`, but no test proves the new guarded writer can feed that path.
- The test should assert three observable facts: failed segment result, wrapped writer not called, verifier not reached.
- No README_IMPL update is needed because the stable boundary was already documented in 7.38; task details belong in `.artifacts/ai`.
- Mutation RED confirmed the test catches guard regressions: temporarily bypassing `DownloadSegmentStagingTarget::parse(...)` made the test fail because the wrapped writer was called; restoring the guard made it pass.
- Final validation passed for focused executor adapter tests, full downloads module tests, composition-root check, scoped rustfmt, and scoped diff-check.

## Phase 118 Downloads Completion Roadmap Findings

- Required context was read in focused chunks: README/docs map, downloads module ARCH/API/FLOW/README_IMPL, download runtime design, storage/data placement rules, error projection rules, testing gates, contribution rules, and composition-root boundaries.
- Current downloads backend has strong resume/checkpoint/policy/query scaffolding plus a module-local segment executor shell, but production concrete execution is still intentionally absent.
- The safe remaining order is: filesystem staging writer, verifier shell, fetcher boundary, composition wiring, runtime terminal projection, retry/backoff/public execution errors, then host/frontend projection.
- The roadmap belongs in README_IMPL because it is durable implementation ordering, while individual AT evidence stays in `.artifacts/ai`.

## Phase 119 Filesystem Staging Writer Boundary Findings

- Required context was read in focused chunks: README_IMPL roadmap and segment execution sections, download runtime SegmentWriter/sparse-write/staging-first notes, storage staging-file ownership notes, and current `DownloadSegmentStagingTarget`/writer contracts.
- The first concrete writer should be job-scoped: a configured `.downloads/staging` root plus `request.job_id` plus a validated target.
- `partial_path` should remain the validated target relative to the job staging root, matching existing fake writer/checkpoint behavior and avoiding a new checkpoint shape.
- Filesystem errors should stay on the `AppError` infrastructure path for now; public `DL_WRITE_FAILED` projection and retry/backoff need a later classification boundary.

## Phase 120 Downloads Filesystem Staging Writer Findings

- Required code context was read in focused chunks: module-downloads Cargo manifest, driver imports, writer trait/guarded writer code, test helpers, and crate re-export surface.
- The crate currently has no temp-file test dependency, so focused writer tests should use a unique path under the workspace `target/` directory instead of writing outside `D:\DEV\MyEpicLauncher`.
- The writer should validate `write_target` itself even when it can also be wrapped by `DownloadSegmentGuardedWritePort`, keeping the concrete writer safe if used directly in tests or later wiring.
- New code comments/doc comments should be Chinese-first per the user's latest preference.
- RED/GREEN confirmed `DownloadSegmentFilesystemWritePort` creates job-scoped parent directories, writes from-start bytes under the validated staging target, and preserves an existing prefix when partial writes seek to `start_offset`.
- The concrete writer returns existing write facts only: `downloaded_bytes`, `partial_path`, and `hash_state_ref = None`; provider fetch, hash verification, final artifact moves, production wiring, retry/backoff, public `DL_*` projection, host transport, frontend, and schema work remain separate later slices.
- Filesystem IO failures currently stay on the infrastructure `AppError` path, matching the documented split before the later public execution-error classification boundary.
- README_IMPL 6.1 and 7.39 now reflect the implemented writer status and identify the verifier shell as the next backend-owned slice.

## Phase 121 Downloads Segment Length Verifier Boundary Findings

- Required context was read in focused chunks: repository README, docs map, CONTRIBUTING, downloads module docs, README_IMPL port status/7.39, download runtime verifier/staging-first/verification-stage notes, error projection rules, testing guidance, and code comment rules.
- The first verifier should stay module-local behind `DownloadSegmentVerifyPort`; it should not become a host DTO, scheduler policy, retry engine, or composition-root wiring task.
- The smallest useful verifier is byte-length only: compare `written.downloaded_bytes` with `request.length` after the writer returns facts.
- A length mismatch can reuse the existing handled segment failure path through `DownloadSegmentVerifyOutcome::Failed`, preserving `AppError` for infrastructure/configuration failures and avoiding public `DL_VERIFY_FAILED` projection for now.
- Hash verification, file-level verification, job-level manifest sealing, retry/backoff, runtime terminal state, host transport, frontend, and schema work remain later slices.

## Phase 122 Downloads Segment Length Verifier Findings

- Required code context was read in focused chunks: verifier outcomes, `DownloadSegmentVerifyPort`, `DownloadSegmentExecutor`, fake verifier test helpers, and crate re-export surface.
- The existing executor already maps `DownloadSegmentVerifyOutcome::Failed` into `DownloadSegmentExecutionResult::Failed`, so the code slice can focus on a concrete verifier plus coverage instead of changing executor result shapes.
- A direct verifier test should prove the success contract without fetch/write/executor noise.
- An executor-routed mismatch test should prove the handled failure path uses `written.downloaded_bytes`, preserves the original request, and remains retryable without introducing public `DL_VERIFY_FAILED`.
- RED/GREEN confirmed `DownloadSegmentLengthVerifyPort` verifies only `written.downloaded_bytes == request.length` and reports a retryable handled failure on mismatch.
- The implementation does not read files, compute hashes, change checkpoint shapes, introduce public `DL_*` projection, or wire production composition-root execution.

## Phase 123 Downloads Length Verifier Partial Resume Findings

- Fetcher prerequisite reading exposed a verifier semantic gap: partial resume work carries `start_offset = checkpoint.downloaded_bytes` but keeps `length = segment.length`.
- `DownloadSegmentWriteResult.downloaded_bytes` is the current write outcome, not the total segment completion count.
- Therefore from-start verification can compare `written.downloaded_bytes == request.length`, but partial verification must compare `request.start_offset + written.downloaded_bytes == request.length`.
- This is a local verifier semantics fix and should not introduce fetcher, hash, retry/backoff, public error projection, production wiring, transport, frontend, or schema work.
- RED/GREEN confirmed the corrected semantics: partial completion with `start_offset = 6`, current write bytes `4`, and total length `10` now verifies successfully.

## Phase 124 Downloads Static Segment Fetcher Boundary Findings

- Required context was read in focused chunks: downloads module fetch/provider/source-locator notes, download runtime fetcher/provider/range/resume notes, provider adapter ownership rules, retryable/public projection rules, and `DownloadResumeWorkItem` / `DownloadSegmentExecutionRequest` offset semantics.
- The first fetcher should stay deterministic and module-local: exact `source_locator` lookup into configured static bytes, preserving optional etag.
- `FromStart` should return the configured segment bytes unchanged because this boundary treats the locator as already segment-scoped.
- `Partial` should return bytes after `request.start_offset`, matching the corrected length verifier and partial writer semantics.
- Missing locators or impossible partial offsets can be handled fetch failures for now; real network/provider errors and stable public `DL_NETWORK_*` / `DL_PROVIDER_*` projection remain later.

## Phase 125 Downloads Static Segment Fetcher Findings

- Required code context was read in focused chunks: README_IMPL 7.41, `DownloadSegmentFetchPort`, fetch outcome/result types, resume mode semantics, existing executor test helpers, and crate re-export surface.
- The first static fetcher can live entirely in `driver.rs` beside the fetch port because it is module-local execution infrastructure, not a provider adapter.
- Tests should cover direct fetcher behavior and one executor completion path with `RecordingWritePort` plus `DownloadSegmentLengthVerifyPort`, proving the static fetcher composes with existing sub-ports.
- RED/GREEN confirmed the static fetcher returns configured from-start bytes and etag, returns partial remaining bytes after `start_offset`, reports missing locators and impossible offsets as handled failures, and composes through `DownloadSegmentExecutor`.
- The implementation remains deterministic and in-memory; it does not introduce HTTP, provider auth, streaming workers, public network/provider error projection, composition-root production wiring, host transport, frontend, or schema work.

## Phase 126 Downloads Composition-root Segment Executor Wiring Boundary Findings

- Required context was read in focused chunks: root README, docs map, downloads ARCH/API/FLOW/README_IMPL 6.1/7.41, composition-root wiring design, download runtime fetch/write/verify and staging notes, repository/error/testing/comment docs, composition-root bootstrap code, and downloads driver/executor surfaces.
- `DownloadJobDriver::with_pending_resume_work_source_and_execution_port(...)` exists, but default composition-root driver registration still omits the execution port so production one-shot dispatch defers.
- `DownloadSegmentExecutor` can now compose static fetch, filesystem write, and length verify ports, but no real provider fetcher or explicit production local-source config exists.
- Wiring an empty `DownloadSegmentStaticFetchPort` into default desktop production would change behavior from `Deferred` to handled segment failure for normal provider locators, so the next code slice should keep default `build_desktop_services(...)` deferred.
- The safe first composition-root proof should use an explicit private static-source helper/test path, derive staging from `app_data_dir/.downloads/staging`, and avoid host transport, frontend, schema, retry/backoff, public `DL_*` execution projection, or real HTTP behavior.
