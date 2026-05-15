# Downloads Module Implementation

> Module: downloads
> Status: draft
> Owner scope: backend implementation guide

---

## 1. Purpose

本文档记录 downloads 模块在当前 Rust/Tauri 后端里的实现落点、切片顺序和验证门槛。

它不替代架构设计文档，也不承载运行日志。开始修改 downloads 模块代码前，必须先读本文档和同目录模块文档，再读相关设计与协作文档。

---

## 2. Required Reading Before Coding

修改 downloads 模块代码前，按这个顺序读文档：

1. `README.md`
2. `CONTRIBUTING.md`
3. `docs/README.md`
4. `docs/modules/downloads/README_ARCH.md`
5. `docs/modules/downloads/README_API.md`
6. `docs/modules/downloads/README_FLOW.md`
7. `docs/modules/downloads/README_IMPL.md`
8. `docs/TauriDownloadRuntimeDesign.md`
9. `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`
10. `docs/TauriFirstCrateApiDrafts.md`
11. `docs/TauriKernelJobsRuntimeDesign.md`
12. `docs/TauriTestingStrategyAndQualityGateDesign.md`
13. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`

如果切片涉及 error code 或 IPC envelope，还要读：

1. `docs/TauriErrorHandlingAndProjectionDesign.md`
2. `docs/TauriIPCAndStateContractsDesign.md`

如果切片涉及 concrete adapter 或 assembly，还要读：

1. `docs/TauriRepositoryPortsAndAdapterDesign.md`
2. `docs/TauriCompositionRootWiringDesign.md`
3. `docs/TauriStorageAndDatabaseDesign.md`

---

## 3. Current Rust Landing Zones

| Area | Current Files | Notes |
|------|---------------|-------|
| public crate entry | `crates/module-downloads/src/lib.rs` | re-exports contracts, facade, and driver boundary |
| command DTOs | `crates/module-downloads/src/contracts/commands.rs` | start, pause, resume, cancel request DTOs |
| query/read DTOs | `crates/module-downloads/src/contracts/*.rs` | public projection shape, still mostly placeholder |
| facade/use-case boundary | `crates/module-downloads/src/facade/mod.rs` | current main implementation surface for start/pause/cancel/resume slices |
| runtime restore driver | `crates/module-downloads/src/driver.rs` | stage-2 restore checkpoint gate |
| SQLite adapter | `crates/adapter-storage-sqlite/src/lib.rs` | concrete job/checkpoint persistence shell |
| composition wiring | `crates/composition-root/src/bootstrap.rs` | concrete assembly owner, not business logic owner |
| host transport | `src-tauri/src/commands/downloads.rs` | command handler surface, should stay thin |

Implementation truth should move through module facade and ports first. Do not put downloads business logic into `src-tauri`, frontend components, or composition root.

---

## 4. Current Implementation State

| Capability | Current State | Validation Shape |
|------------|---------------|------------------|
| `start_download` | persists module job record and enqueues shared runtime job | module facade test |
| `pause_download` | delegates to shared job runtime control port | module facade test |
| `cancel_download` | delegates to shared job runtime control port | module facade test |
| stage-2 restore driver | reads checkpoint and fails unrecoverable when missing | driver unit tests |
| `resume_download` job lookup | returns `DL_JOB_NOT_FOUND` when module job record is missing | module facade test |
| `resume_download` checkpoint lookup | returns `DL_CHECKPOINT_MISSING` when checkpoint is missing | module facade test |
| `resume_download` staging boundary | calls `DownloadStagingObjectStore::ensure_staging_root()` after job and checkpoint exist | module facade test |
| `resume_download` manifest boundary | calls `DownloadManifestProviderPort::fetch_manifest()` after staging is valid | module facade test |
| `resume_download` segment decisions | derives `seal_completed`, `resume_partial`, `queue_remaining`, and `reject_mismatch` from manifest/checkpoint facts | module decision tests |
| `resume_download` runtime enqueue boundary | enqueues the existing job through job-level `JobRuntime::enqueue()` when resume decisions contain runtime candidates and no mismatch rejection | module facade test |
| `resume_download` mismatch error projection | returns `DL_RESUME_SEGMENT_MISMATCH` and skips runtime enqueue when derived decisions contain `reject_mismatch` | module facade test |
| `resume_download` all-sealed completion boundary | documented as a distinct no-enqueue outcome that current `AcceptedJob` semantics cannot safely represent yet | README_IMPL |
| list/get/policy surfaces | not wired yet | future slices |

---

## 5. Resume Slice Order

The current `resume_download` implementation must stay aligned with the design flow:

```text
resume request
  -> load module job record
  -> load job checkpoint
  -> validate staging artifacts still exist
  -> reload or reconstruct manifest plan
  -> mark completed segments as sealed
  -> enqueue remaining segments only
  -> continue scheduler loop
```

Current slice boundary:

1. Job record lookup is implemented.
2. Checkpoint lookup is implemented.
3. Staging boundary is implemented as a minimal port call.
4. Manifest reconstruction is implemented as a minimal provider port call.
5. Segment decision derivation is implemented for completed sealing, partial resume, queue remaining, and mismatch rejection.
6. Runtime enqueue-resume is implemented as the first job-level boundary and must stay job-level until a downloads-owned scheduler/driver payload is explicitly introduced.
7. The all-sealed no-enqueue outcome is documented below as a contract gap; do not fake it as queued `AcceptedJob`.
8. Concrete segment persistence, scheduler execution, host transport, and frontend projection remain later slices.

Do not skip directly from checkpoint to `JobRuntime::resume`. The module owns business checkpoint and resume reconstruction.

---

## 6. Port Status

| Port | Status | Notes |
|------|--------|-------|
| `DownloadJobRepository` | defined in facade module | concrete SQLite shell exists |
| `DownloadCheckpointRepository` | defined in driver module | concrete SQLite shell exists |
| `DownloadStagingObjectStore` | minimal facade port exists | `()` placeholder keeps composition wiring stable |
| `DownloadManifestProviderPort` | minimal facade port exists | currently returns a minimal `DownloadManifestPlan` handle |
| `JobRuntime` | shared kernel-jobs runtime trait exists | resume uses job-level `EnqueueJobRequest<()>`; segment details still stay out of `kernel-jobs` |

When adding a port:

1. define the narrow trait needed by the current test;
2. keep `()` placeholder behavior only when it preserves existing composition wiring;
3. add module facade tests before production code;
4. defer concrete adapter behavior unless the task explicitly scopes it.

---

## 7. Resume Segment Data Shape

Completed-segment sealing needs three separate shapes. Keep them separate so provider details, persisted checkpoint facts, and runtime resume decisions do not collapse into one mutable blob.

### 7.1 Manifest Segment

Manifest segments come from `DownloadManifestProviderPort` or a later planner. They describe what could be downloaded, not what has already been written.

Minimum fields:

| Field | Purpose |
|-------|---------|
| `segment_id` | stable segment identifier inside the manifest plan |
| `file_id` | stable logical file identifier for multi-file downloads |
| `offset` | byte offset inside the logical file |
| `length` | expected segment length in bytes |
| `source_locator` | provider-specific source reference kept behind the module boundary |
| `expected_hash` | optional segment integrity expectation when available |
| `write_target` | staging-relative target path or object key |

### 7.2 Segment Checkpoint

Segment checkpoints are module-owned persisted facts. They should be loaded through downloads checkpoint/repository code, not through `kernel-jobs`, host transport, or frontend state.

Minimum fields:

| Field | Purpose |
|-------|---------|
| `job_id` | stable downloads job identifier |
| `segment_id` | segment identifier that must match the manifest segment |
| `file_id` | logical file identifier used to guard against stale manifest/checkpoint mismatches |
| `offset` | checkpointed offset used to validate the manifest segment boundary |
| `length` | checkpointed length used to validate completion and resume range |
| `downloaded_bytes` | persisted progress inside this segment |
| `status` | module-owned status such as `pending`, `in_progress`, `completed`, or `failed` |
| `partial_path` | staging-relative partial file path when one exists |
| `etag` | provider validator used only when the provider supports safe range resume |
| `hash_state_ref` | reference to incremental hash state when hashing cannot be recomputed cheaply |

### 7.3 Resume Segment Decision

Resume decisions are derived in memory from manifest segments plus segment checkpoints. They should not be stored as the source of truth.

Minimum decision actions:

| Action | Meaning |
|--------|---------|
| `seal_completed` | checkpoint proves the segment is complete and matches the manifest boundary, so it must not be re-enqueued |
| `resume_partial` | checkpoint has partial bytes and provider validators still allow safe range resume |
| `queue_remaining` | segment has no safe completed or partial checkpoint and must be queued from the beginning |
| `reject_mismatch` | manifest/checkpoint boundary or validator mismatch prevents safe automatic resume |

Required invariants:

1. Match manifest and checkpoint segments by `segment_id`, then verify `file_id`, `offset`, and `length`.
2. Only seal when `status` is completed and `downloaded_bytes == length`.
3. Only resume partial bytes when `downloaded_bytes < length` and provider validators such as `etag` still match.
4. Treat stale or mismatched segment facts as a downloads-domain resume failure or attention state, not as a silent full-job restart.
5. Do not expose segment-level decisions through frontend IPC until a separate projection design says they are safe to surface.

Current decision coverage:

1. `seal_completed` is covered and must not be a runtime enqueue candidate.
2. `resume_partial` is covered and must be a runtime enqueue candidate.
3. `queue_remaining` is covered and must be a runtime enqueue candidate.
4. `reject_mismatch` is covered and must not be a runtime enqueue candidate.

### 7.4 Runtime Enqueue Boundary

The first runtime enqueue slice should connect `resume_download` to the existing shared job runtime without moving segment details into `kernel-jobs`.

Minimum job-level runtime request:

| Field | Value |
|-------|-------|
| `job_id` | existing `ResumeDownloadRequestDto.job_id`; resume must not generate a new job id |
| `module` | `downloads` |
| `kind` | `download`, matching the normal downloads job driver route |
| `priority` | priority from the persisted `DownloadJobRecord` |
| `recoverable` | `true` |
| `extension` | `None` while `DownloadFacade` and `SharedJobRuntimeHost` use `JobRuntime<Extension = ()>` |

Decision mapping:

| Decision action | Runtime enqueue behavior |
|-----------------|--------------------------|
| `seal_completed` | do not enqueue this segment |
| `resume_partial` | candidate for the downloads-owned scheduler/driver to continue from checkpoint bytes |
| `queue_remaining` | candidate for the downloads-owned scheduler/driver to start from segment offset |
| `reject_mismatch` | do not enqueue; returns `DL_RESUME_SEGMENT_MISMATCH` until a richer needs-attention projection is designed |

The first runtime-enqueue code slice now proves only the job-level enqueue boundary:

1. `resume_download` loads job, checkpoint, staging, and manifest.
2. It derives segment decisions.
3. If there is at least one runtime enqueue candidate and no `reject_mismatch`, it calls `JobRuntime::enqueue()` with the existing job id and persisted priority.
4. It returns the runtime `AcceptedJob`.

That slice must not:

1. store segment decisions as source of truth;
2. add SQLite segment tables or columns;
3. put segment payloads into `kernel-jobs` extension fields;
4. expose segment decisions through host transport or frontend IPC;
5. implement actual fetch/write/verify scheduler execution.

### 7.5 All-Sealed Completion Boundary

An all-sealed resume is the branch where every manifest segment maps to `seal_completed` and the derived decision set contains no `resume_partial`, no `queue_remaining`, and no `reject_mismatch`.

In that branch, downloads has proven that no segment should be re-enqueued. This is distinct from runtime enqueue:

1. `AcceptedJob` / `AcceptedJobDto` means a long job was accepted into the shared runtime queue.
2. `queued_at` is created by `JobRuntime::enqueue()` while the runtime creates a queued `JobSnapshot`.
3. A fully sealed resume is already complete from the downloads resume planner's point of view, so fabricating an `AcceptedJob` would blur "accepted/queued" with "already complete".

Current safe rule:

1. do not call `JobRuntime::enqueue()` solely to manufacture a queued snapshot for an all-sealed plan;
2. do not create a synthetic `AcceptedJob` inside downloads with `IsoDateTime::now()`;
3. do not put completion/segment payloads into `kernel-jobs` extension fields;
4. do not expose segment-level completion details through host transport or frontend IPC in the same slice.

Next Rust boundary:

1. introduce a narrow module-owned resume outcome before changing behavior, for example an outcome that can distinguish "accepted for runtime work" from "already complete after sealing";
2. add a focused module test that proves the all-sealed branch does not runtime enqueue and does not fall through to `DOWNLOADS_NOT_WIRED`;
3. only then adapt host transport or shared DTO shape in a separately scoped slice if the public command result needs to expose the already-complete outcome.

Until that outcome contract exists, `resume_download` must keep the placeholder branch for the no-runtime-candidate path rather than pretending completion is a queued job.

---

## 8. Error Semantics

Downloads-domain errors use `DL_*` codes when they become stable public classifications.

Current stable resume errors:

| Code | Trigger | Retryable | Severity |
|------|---------|-----------|----------|
| `DL_JOB_NOT_FOUND` | requested job has no downloads module record | false | error |
| `DL_CHECKPOINT_MISSING` | requested job has no module checkpoint | false | error |
| `DL_RESUME_SEGMENT_MISMATCH` | a persisted segment checkpoint matches a manifest `segment_id` but conflicts on `file_id`, `offset`, or `length` | false | error |

`DOWNLOADS_NOT_WIRED` remains a temporary implementation placeholder, not a final domain error.

`DL_RESUME_SEGMENT_MISMATCH` is the current narrow projection for stale or unsafe segment checkpoint facts. It must stop before runtime enqueue, must not silently restart the whole job, and must not expose segment internals through host transport or frontend IPC.

---

## 9. Required Validation

For downloads backend implementation slices, prefer the narrowest proof first:

1. focused module test, usually `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml <test-filter>`;
2. full downloads module test, `cargo test -p launcher-module-downloads --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`;
3. scoped `git diff --check` over touched files;
4. wider composition or transport smoke only when the slice touches wiring or host handlers.

Every behavior change must follow RED/GREEN:

1. write the module test first;
2. observe the expected failure;
3. implement the minimal production behavior;
4. rerun focused and module tests;
5. commit only the current slice.

---

## 10. Non-goals

Do not use this module implementation document to:

1. record temporary task logs;
2. replace `.artifacts/ai` active task records;
3. duplicate the full download runtime design;
4. move frontend UI requirements into backend implementation truth;
5. justify broad refactors outside the current atomic task.
