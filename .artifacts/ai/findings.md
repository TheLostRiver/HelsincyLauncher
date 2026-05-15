# AI Findings & Decisions

## Requirements

- Repository-local slash commands should use an `hsy-XXX` prefix to avoid collisions with other command surfaces.
- New or revised code comments should default to simplified Chinese.
- Other developers must be able to request English comments explicitly, ideally through a slash command instead of an implicit convention.
- Keep `.artifacts/ai` as the only authoritative workflow record set.
- Preserve the repo's single-active-task protocol and compatibility with existing hook parsing.
- Normalize the live records so they look intentional and match the repo-local planning-with-files workflow.

## Research Findings

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
