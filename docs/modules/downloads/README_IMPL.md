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
| `resume_download` all-sealed completion boundary | returns module-owned `DownloadResumeOutcome::AlreadyComplete` from `resume_download_outcome` without runtime enqueue | module facade test |
| downloads resume host projection | maps `DownloadResumeOutcome` to `DownloadResumeOutcomeDto`; `RuntimeAccepted` wraps accepted-job projection and `AlreadyComplete` uses a non-accepted completed outcome | host mapper tests |
| resume scheduler/driver payload boundary | documented as downloads-owned work plan derived from `resume_partial` / `queue_remaining`, not a `kernel-jobs` extension or transport payload | README_IMPL |
| resume work plan derivation | derives module-local `DownloadResumeWorkPlan` / `DownloadResumeWorkItem` values from manifest, checkpoints, and resume decisions | module work-plan test |
| resume scheduler/driver consumer boundary | `DownloadResumeWorkScheduler` consumes `DownloadResumeWorkPlan` before job-level runtime enqueue; composition currently uses the no-op placeholder | module facade tests + composition smoke |
| resume scheduler execution shell | module-local `InMemoryDownloadResumeWorkScheduler` records pending `DownloadResumeWorkPlan` values before runtime enqueue; composition now wires this shell instead of the `()` placeholder; no fetch/write/verify or persistence behavior is wired yet | module facade test + composition smoke |
| driver pending-work consumption boundary | `DownloadPendingResumeWorkSource` and job-id-scoped draining on `InMemoryDownloadResumeWorkScheduler` prove future driver consumption can drain one job without discarding unrelated pending work; current Rust still has no `JobDriver::run()` | module source/drain tests |
| driver execution-turn classification | `DownloadJobDriver::prepare_resume_execution_turn(...)` reloads durable checkpoint facts before draining pending work and returns a downloads-owned classification; still no fetch/write/verify, runtime `run()`, snapshot completion, transport, or frontend projection | driver unit tests |
| segment execution request handoff | `DownloadSegmentExecutionRequest` / `DownloadSegmentExecutionPort` exist and `DownloadJobDriver::prepare_segment_execution_requests(...)` converts accepted pending work into ordered job-scoped requests; still no real execution | driver unit tests |
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
7. The all-sealed no-enqueue outcome is represented inside `module-downloads` by `DownloadResumeOutcome::AlreadyComplete` and projected by host transport as `DownloadResumeOutcomeDto::AlreadyComplete`.
8. The scheduler/driver payload, preparation, and execution-shell boundaries are documented below.
9. Driver-side pending-work consumption must respect the current Rust reality that `JobDriver` has `restore()` only; it should start with a module-local source/drain boundary before real execution.
10. Concrete segment persistence, scheduler execution, host transport, and frontend projection remain later slices.

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

Current module-owned and host boundary:

1. `DownloadResumeOutcome::RuntimeAccepted` wraps the existing runtime `AcceptedJob` path.
2. `DownloadResumeOutcome::AlreadyComplete` represents an all-sealed resume plan without runtime enqueue.
3. `resume_download_outcome` returns the module-owned outcome for module tests and host projection.
4. The existing `resume_download -> AppResult<AcceptedJob>` method remains a host-compatibility wrapper for the current accepted-job transport path.
5. `downloads_resume` maps the module outcome into `DownloadResumeOutcomeDto`, where `AlreadyComplete` carries only aggregate job identity and a completed marker.

Next public boundary:

1. decide how frontend-side callers should consume `DownloadResumeOutcomeDto`;
2. keep segment-level completion details inside downloads unless a separate projection design says they are safe to surface.

### 7.6 Resume Scheduler/Driver Payload Boundary

`resume_partial` and `queue_remaining` are runtime-work candidates, but the candidate detail is downloads business data. The shared `JobRuntime` still receives only a job-level enqueue request:

1. existing job id;
2. module `downloads`;
3. kind `download`;
4. persisted job priority;
5. `recoverable = true`;
6. `extension = None` while the shared runtime host uses `JobRuntime<Extension = ()>`.

The segment work plan must stay inside `module-downloads`. The first module-owned work payload should be derived from the manifest, checkpoint, staging root, and resume decisions after safe mismatch/all-sealed handling.

Minimum module-owned resume work item:

| Field | Source | Purpose |
|-------|--------|---------|
| `segment_id` | manifest segment | stable segment identity for scheduler routing |
| `file_id` | manifest segment | guard against cross-file stale checkpoint use |
| `source_locator` | manifest segment | provider fetch reference kept behind downloads boundary |
| `write_target` | manifest segment | staging-relative output target |
| `expected_hash` | manifest segment | optional verifier input |
| `start_offset` | decision + checkpoint | `downloaded_bytes` for `resume_partial`, manifest `offset` for `queue_remaining` |
| `length` | manifest segment | total segment length for scheduler and verifier |
| `resume_mode` | decision action | `partial` or `from_start` |
| `checkpoint_ref` | checkpoint segment when present | module-local reference for later checkpoint update |

Decision mapping:

| Decision action | Work-plan behavior |
|-----------------|--------------------|
| `seal_completed` | no work item; the segment is already complete |
| `resume_partial` | create work item with `resume_mode = partial` and `start_offset = downloaded_bytes` |
| `queue_remaining` | create work item with `resume_mode = from_start` and `start_offset = manifest.offset` |
| `reject_mismatch` | no work item; return `DL_RESUME_SEGMENT_MISMATCH` before enqueue |

This boundary must not:

1. put segment plans into `kernel-jobs` `extension`;
2. store resume work items as source-of-truth persistence;
3. expose segment work items through host transport or frontend IPC;
4. add SQLite tables or columns;
5. implement fetch/write/verify scheduler execution.

Current Rust slice:

1. `crates/module-downloads/src/facade/mod.rs` defines module-local `DownloadResumeWorkPlan`, `DownloadResumeWorkItem`, and `DownloadResumeWorkMode`.
2. `build_resume_work_plan()` derives work items from manifest, checkpoint, and decision facts after decision derivation has classified each segment.
3. `resume_partial` creates a `partial` work item with `start_offset = downloaded_bytes` and a module-local checkpoint reference.
4. `queue_remaining` creates a `from_start` work item with `start_offset = manifest.offset` and no checkpoint reference.
5. `seal_completed` and `reject_mismatch` create no work item; mismatch projection still happens before enqueue in `resume_download_outcome()`.
6. Runtime enqueue, concrete scheduler execution, persistence, host transport, frontend, and `kernel-jobs` payloads remain unchanged.

Later Rust slices should wire this work plan into a downloads-owned scheduler/driver behind the module boundary before adding concrete persistence or host projections.

### 7.7 Resume Scheduler/Driver Consumer Boundary

The scheduler/driver consumer of `DownloadResumeWorkPlan` is a downloads-owned port, not a shared runtime extension. The first Rust slice should introduce a module-local boundary shaped like:

1. trait name: `DownloadResumeWorkScheduler`;
2. method: `schedule_resume_work(&self, job_id: &JobId, plan: &DownloadResumeWorkPlan) -> AppResult<()>`;
3. default placeholder: `impl DownloadResumeWorkScheduler for ()` returning `Ok(())` so current composition wiring can stay minimal until a real scheduler lands;
4. dependency owner: `DownloadModuleDeps`, alongside repositories, manifest provider, staging store, and shared job runtime;
5. public export: re-export through `crates/module-downloads/src/lib.rs` with the other facade boundary types.

The resume facade call order should be:

1. load the module job record;
2. load checkpoint facts;
3. validate staging root;
4. fetch or reconstruct manifest;
5. derive segment decisions;
6. return `DL_RESUME_SEGMENT_MISMATCH` before any scheduler or runtime call when a decision is `reject_mismatch`;
7. return `AlreadyComplete` before any scheduler or runtime call when all decisions are `seal_completed`;
8. build `DownloadResumeWorkPlan` when decisions contain runtime candidates;
9. call `DownloadResumeWorkScheduler::schedule_resume_work()` with the job id and work plan;
10. enqueue the existing job id through shared `JobRuntime` using the existing job-level request shape.

Failure behavior:

1. if the scheduler port returns an error, `resume_download_outcome()` must return that error and must not enqueue the shared runtime job;
2. the first scheduler-port slice should not invent public IPC fields or transport DTOs for scheduler failures;
3. scheduler preparation failure is a downloads-domain failure, not a `kernel-jobs` lifecycle state by itself.

This boundary still must not:

1. implement fetch/write/verify execution;
2. add SQLite tables or columns for work items;
3. expose work items through host transport or frontend IPC;
4. put segment payloads into `kernel-jobs` `extension`;
5. add job completion or checkpoint mutation APIs to `kernel-jobs`.

Current Rust slice:

1. `DownloadResumeWorkScheduler` exists as the downloads-owned scheduler/driver port.
2. `()` implements the port as a no-op placeholder so current composition wiring stays minimal.
3. `DownloadModuleDeps` owns the scheduler dependency next to the repositories, manifest provider, staging store, and shared job runtime.
4. `resume_download_outcome()` builds `DownloadResumeWorkPlan`, schedules it through the downloads-owned port, then enqueues the existing job id through shared runtime.
5. The focused module test proves scheduler scheduling happens before runtime enqueue.
6. A focused module guard proves scheduler errors return before shared runtime enqueue.
7. A focused all-sealed guard proves `AlreadyComplete` resumes do not touch scheduler or runtime work.
8. The composition smoke uses the placeholder scheduler and remains assembly-only.

Next Rust slice:

1. reassess the next downloads backend slice from the implementation map before coding;
2. keep concrete scheduler execution and persistence unchanged until a dedicated scheduler implementation task exists;
3. avoid host transport, frontend, SQLite schema, and `kernel-jobs` payload changes unless a later implementation document explicitly scopes them.

### 7.8 Concrete Scheduler Execution Boundary

`DownloadResumeWorkScheduler` is a preparation boundary. It consumes a module-owned `DownloadResumeWorkPlan` before job-level runtime enqueue, but it must not turn the command path into the place where segment fetch, file write, or verification work runs.

The implementation split should be:

| Layer | Owns | Must Not Own |
|-------|------|--------------|
| `resume_download_outcome()` facade path | checkpoint/manifest validation, decision derivation, work-plan creation, scheduler preparation call, job-level runtime enqueue | network fetch, staging writes, verifier loops, SQLite schema migration |
| downloads-owned scheduler preparation | accepting a validated `DownloadResumeWorkPlan`, registering pending module work, rejecting invalid scheduler preparation | shared job lifecycle state, host IPC shape, frontend projection |
| downloads driver / scheduler loop | draining prepared segment work after the shared runtime owns the job execution turn | Tauri command routing, direct UI state, cross-module installation/repair decisions |
| shared `JobRuntime` | job id, queue/lease/snapshot lifecycle, pause/cancel control, driver routing by module/kind | segment plan, checkpoint bytes, provider source locator, work item persistence |
| checkpoint repository | persisted business checkpoint facts that allow resume reconstruction | transient scheduler queue state as source-of-truth |

The first concrete scheduler shell therefore starts with a module-local pending-work boundary, not with real HTTP or disk IO.

Minimum future Rust shape:

1. a downloads-owned pending resume work queue/store boundary exists behind `DownloadResumeWorkScheduler`;
2. `DownloadResumeWorkPlan` remains transient derived work, not authoritative persistence;
3. the scheduler implementation registers the plan for the existing job id and returns before shared runtime enqueue proceeds;
4. the future downloads driver/scheduler loop can consume the registered plan during `run()` or an equivalent module-owned execution turn;
5. `()` remains the composition placeholder until a separate wiring task explicitly swaps in the real scheduler implementation.

Current Rust slice:

1. `DownloadPendingResumeWork` records an existing `JobId` plus its derived `DownloadResumeWorkPlan`.
2. `InMemoryDownloadResumeWorkScheduler` implements `DownloadResumeWorkScheduler`.
3. The shell stores pending work in memory only and exposes `pending_work()` plus `drain_pending_work()` for later module-owned driver use.
4. The focused module test proves shared runtime enqueue observes pending work already registered for the job id.
5. Composition now wires `InMemoryDownloadResumeWorkScheduler` into the desktop downloads facade instead of `()`.
6. Fetcher, writer, verifier, checkpoint mutation, SQLite schema, host transport, frontend IPC, driver-side consumption, and `kernel-jobs` payloads remain unchanged.

This execution boundary still must not:

1. perform HTTP range requests or provider object fetches;
2. write staging files, sparse ranges, temp fragments, or final artifacts;
3. compute or verify hashes;
4. mutate segment checkpoints or add checkpoint repository methods;
5. add SQLite tables/columns for work items;
6. expose segment work through host transport or frontend IPC;
7. put segment payloads into `kernel-jobs` `extension`;
8. add `kernel-jobs` job-completion APIs just to satisfy downloads internals.

Failure behavior remains layered:

1. scheduler preparation failure is returned synchronously by `resume_download_outcome()` and must skip shared runtime enqueue;
2. later execution failures belong to the downloads driver/scheduler loop and should update module checkpoint/runtime snapshot through documented ports, not through the resume command return path;
3. stable public error codes for execution failures should be introduced only when the concrete fetch/write/verify slice defines the failing boundary.

Next Rust slice:

1. define driver-side pending-work consumption as a job-scoped source/drain boundary before touching concrete execution;
2. keep concrete fetch/write/verify and checkpoint mutation deferred;
3. keep host transport, frontend, SQLite schema, and `kernel-jobs` payloads unchanged unless that slice explicitly scopes them.

### 7.9 Driver Pending-Work Consumption Boundary

The future downloads driver/scheduler loop must consume prepared resume work after the shared runtime has accepted the job-level execution turn. Current Rust code is not there yet: `kernel-jobs::JobDriver` currently exposes `module()`, `kind()`, and `restore()` only. The `run()` method described in the broader runtime design is a future execution boundary, not a current API.

Current Rust reality:

1. `DownloadJobDriver::restore()` is a stage-2 recovery gate that checks whether durable checkpoint facts still exist.
2. `restore()` must not drain in-memory pending resume work. Stage-2 recovery may happen after process restart, when the in-memory scheduler queue is necessarily gone.
3. Explicit `resume_download_outcome()` already registers pending work before job-level runtime enqueue.
4. A future execution turn may consume that pending work, but AT-181 must not pretend that the current shared runtime can already execute segment work.

The driver dependency should be a narrow downloads-owned source boundary, not the concrete scheduler type. The next Rust slice should shape it like:

```rust
pub trait DownloadPendingResumeWorkSource: Send + Sync {
    fn drain_pending_resume_work(
        &self,
        job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>>;
}
```

Dependency rules:

1. `DownloadJobDriver` should depend on the source trait when a driver-side consumer is added.
2. `InMemoryDownloadResumeWorkScheduler` may implement both `DownloadResumeWorkScheduler` and `DownloadPendingResumeWorkSource`.
3. Composition-root must pass the same scheduler/source instance to the facade preparation path and the future driver consumer path; creating two independent in-memory queues would make the driver see no work.
4. The source trait should drain by job id and preserve pending work for unrelated job ids.
5. The trait must stay inside `module-downloads`; it must not move segment payloads into `kernel-jobs`, host transport, frontend state, or SQLite schema.

Drain timing:

1. The command path schedules pending work before `JobRuntime::enqueue()`.
2. The driver-side source is drained only during a documented downloads-owned execution turn.
3. Until `kernel-jobs` exposes that execution turn, a Rust slice may add and test the source/drain boundary, but must not claim that real download execution has landed.
4. Empty drain for a job id is a valid source result, but it is not proof that a download completed. A future driver execution implementation must decide whether to reconstruct from durable checkpoint/manifest facts or return a module execution failure; it must not silently succeed after finding no work.

Failure behavior:

1. Preparation failures from `DownloadResumeWorkScheduler::schedule_resume_work()` still belong to the resume command path and skip shared runtime enqueue.
2. Source/drain failures during a future execution turn belong to the driver/scheduler loop and should be reported through the documented runtime/driver failure surface, not by changing the already-returned resume command result.
3. No new public `DL_*` execution code should be added until the concrete fetch/write/verify slice defines the failing boundary.

Current Rust slice:

1. `DownloadPendingResumeWorkSource` defines the future driver-facing pending-work source boundary.
2. `InMemoryDownloadResumeWorkScheduler` implements a job-id-scoped `drain_pending_resume_work(&JobId)` source method.
3. Focused module tests prove that draining one job preserves unrelated pending work and that draining an empty job returns an empty vector.
4. keep `DownloadJobDriver`, `kernel-jobs`, composition wiring, host transport, frontend, SQLite schema, fetch/write/verify, and checkpoint mutation unchanged unless a later task explicitly scopes driver integration.

Next Rust slice:

1. reassess whether to integrate the source into `DownloadJobDriver` through an explicit local consumer method, or first document the shared runtime execution turn that would call such a consumer;
2. do not add fetch/write/verify execution until driver consumption and checkpoint mutation boundaries are separately documented;
3. keep `kernel-jobs`, host transport, frontend, SQLite schema, and composition wiring unchanged unless the next implementation document explicitly scopes them.

### 7.10 `DownloadJobDriver` Local Consumer Boundary

The next safe driver-facing code slice can add a local consumer method to `DownloadJobDriver` without changing `kernel-jobs::JobDriver`. This keeps current runtime semantics honest: `restore()` remains the only trait callback, while the downloads crate prepares a method that a later documented execution turn can call.

Recommended shape:

```rust
impl DownloadJobDriver {
    pub fn with_pending_resume_work_source(
        checkpoint_repo: Arc<dyn DownloadCheckpointRepository>,
        pending_work_source: Arc<dyn DownloadPendingResumeWorkSource>,
    ) -> Self;

    pub fn drain_pending_resume_work(
        &self,
        job_id: &JobId,
    ) -> AppResult<Vec<DownloadPendingResumeWork>>;
}
```

Constructor rules:

1. Preserve `DownloadJobDriver::new(checkpoint_repo)` for existing composition and restore tests.
2. Back `new()` with a module-local no-op pending-work source that returns `Ok(Vec::new())`.
3. Add `with_pending_resume_work_source(...)` for focused driver tests and later composition wiring.
4. Do not require composition-root to share the real in-memory scheduler in the first driver-consumer code slice; shared composition wiring should be a later task.

Method rules:

1. `drain_pending_resume_work(&JobId)` delegates to `DownloadPendingResumeWorkSource`.
2. The method does not call `restore()` and does not read or mutate checkpoint records.
3. The method does not fetch, write, verify, update snapshots, complete jobs, or publish events.
4. Empty drain returns an empty vector and is not a completion signal.
5. Source errors propagate as `AppResult` failures for the future execution turn; they do not change the already-returned resume command result.

`restore()` must remain unchanged:

1. stage-2 restore still reads durable `DownloadCheckpointRepository`;
2. restore must not drain in-memory pending work because stage-2 may run after process restart;
3. restore returning `RestoreDisposition::Resumed` only means checkpoint facts are recoverable, not that segment work has executed.

Current Rust slice:

1. `()` implements `DownloadPendingResumeWorkSource` as an empty no-op source.
2. `DownloadJobDriver` owns a pending-work source field.
3. `DownloadJobDriver::new(checkpoint_repo)` stays compatible by wiring the no-op source.
4. `DownloadJobDriver::with_pending_resume_work_source(...)` allows focused tests and future composition wiring to inject a real source.
5. `DownloadJobDriver::drain_pending_resume_work(&JobId)` delegates to the source and does not execute download IO.
6. Focused driver tests prove the injected source drains work and the default constructor returns an empty drain.
7. keep `kernel-jobs`, composition-root, host transport, frontend, SQLite schema, fetch/write/verify, snapshot mutation, and checkpoint mutation unchanged.

Next Rust slice:

1. reassess whether the next step is composition wiring that shares the same `InMemoryDownloadResumeWorkScheduler` between facade and driver, or a docs-first boundary for that wiring;
2. do not start fetch/write/verify execution until the shared scheduler/source wiring and checkpoint mutation boundary are explicit;
3. keep `kernel-jobs`, host transport, frontend, SQLite schema, snapshot mutation, and checkpoint mutation unchanged unless a later task explicitly scopes them.

### 7.11 Composition Shared Scheduler/Source Wiring Boundary

The next composition-root slice should wire one shared in-memory resume scheduler instance through both sides of the downloads runtime preparation path:

1. as `DownloadResumeWorkScheduler` for `DownloadsFacade::resume_download_outcome()` preparation;
2. as `DownloadPendingResumeWorkSource` for the `DownloadJobDriver` local consumer method.

This matters because the scheduler shell is currently in-memory. If composition creates two separate `InMemoryDownloadResumeWorkScheduler` values, the facade can successfully register pending work while the driver drains from an empty queue. That would make wiring tests pass superficially while the future execution turn has no prepared work to consume.

Current Rust reality:

1. composition-root already wires `InMemoryDownloadResumeWorkScheduler` into the downloads facade dependency graph;
2. the job driver registry still constructs downloads drivers through `DownloadJobDriver::new(checkpoint_repo)`;
3. `DownloadJobDriver::new(...)` deliberately uses the no-op `()` pending-work source for compatibility;
4. therefore the real scheduler and the driver source are not shared yet.

Required composition shape:

1. create one `InMemoryDownloadResumeWorkScheduler` in the composition assembly path that already owns concrete adapters and module dependencies;
2. pass a clone of that scheduler to the downloads facade dependencies as `DownloadResumeWorkScheduler`;
3. pass the same shared scheduler instance to `DownloadJobDriver::with_pending_resume_work_source(...)` as `DownloadPendingResumeWorkSource`;
4. keep `DesktopAppServices` facade-only; do not expose the driver registry or pending-work source through the public composition API just to make tests easier;
5. keep startup stage-2 restore unchanged: restore may read durable checkpoints, but it must not drain the in-memory pending-work queue.

Implementation boundaries for the next Rust slice:

1. allowed: composition-root private builder/helper signature changes needed to pass the shared scheduler into both the facade builder and driver registry builder;
2. allowed: focused composition-root test proving work scheduled through the shared scheduler can be drained by a driver built with the same source;
3. allowed: existing smoke test adjustments only if construction signatures change;
4. not allowed: `kernel-jobs::JobDriver` trait changes, runtime `run()` semantics, job completion APIs, snapshot mutation, or host transport changes;
5. not allowed: concrete HTTP fetch, staging writes, hash/length verification, checkpoint mutation, SQLite work-item persistence, frontend IPC, or UI projection.

Preferred validation for the next Rust slice:

1. first add a focused RED composition-root test around the private/shared wiring helper or equivalent narrow construction seam;
2. implement the minimal shared scheduler/source wiring;
3. run the focused composition-root test;
4. run `cargo test -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml`;
5. run `cargo fmt -p launcher-composition-root --manifest-path D:\DEV\MyEpicLauncher\Cargo.toml --check`;
6. run scoped `git diff --check` and path-limited status before committing.

The slice still does not mean downloads can execute segment work. It only ensures that once a documented execution turn exists, the driver can see the pending resume work that the facade prepared before shared runtime enqueue.

Current Rust slice:

1. composition-root creates one `InMemoryDownloadResumeWorkScheduler` during desktop service assembly;
2. the downloads facade receives a clone of that scheduler as its `DownloadResumeWorkScheduler`;
3. the downloads job driver registry receives the same shared scheduler as `DownloadPendingResumeWorkSource`;
4. a private `build_download_job_driver(...)` helper keeps the concrete driver construction testable without exposing driver internals through `DesktopAppServices`;
5. the focused composition-root test proves work registered through the facade dependency graph can be drained by the driver source;
6. `kernel-jobs`, host transport, frontend, SQLite schema, checkpoint mutation, snapshot mutation, startup stage-2 restore, and concrete fetch/write/verify execution remain unchanged.

### 7.12 Checkpoint Mutation Boundary

Checkpoint mutation is the last boundary that must be explicit before the downloads driver can start real segment execution. The prepared in-memory work queue is useful for a same-process execution turn, but it is not the durable source of truth. Durable resume facts remain downloads-owned checkpoint records written through `DownloadCheckpointRepository`.

Ownership rules:

1. `module-downloads` owns `DownloadCheckpointRecord` and `DownloadSegmentCheckpointRecord` semantics.
2. `adapter-storage-sqlite` owns SQL tables, transactions, and row mapping for persisted checkpoint facts.
3. `kernel-jobs` owns generic job snapshots only; it must not store segment checkpoint details in `extension`.
4. composition-root only wires the repository and scheduler/source dependencies; it must not inspect or mutate checkpoint contents.
5. host transport and frontend only see stable command results and job/read-model projections, never segment checkpoint internals.

Command path rules:

1. `resume_download_outcome()` may load checkpoint facts to validate resume safety.
2. It may derive segment decisions and schedule pending work before job-level runtime enqueue.
3. It must not treat the in-memory pending queue as persistence.
4. It must not rewrite segment checkpoint facts while merely accepting a resume command.
5. Scheduler preparation failure still returns synchronously and skips runtime enqueue.

Driver/execution-turn rules:

1. A future driver execution turn may drain pending work for the accepted job id.
2. Before mutating durable facts, it should reload or otherwise validate the current checkpoint record so stale in-memory work cannot overwrite newer persisted state.
3. Segment checkpoint writes happen after a concrete boundary has produced a durable fact, such as manifest/segment plan confirmation, partial byte progress eligible for flush, verified segment completion, pause completion, or terminal failure/cancel/completion.
4. Checkpoint save failure belongs to the driver/execution failure surface; it must not change the already-returned resume command result.
5. Runtime snapshot completion must not be reported before the corresponding downloads checkpoint transition has been persisted or deliberately classified as unrecoverable.

Persistence rules:

1. `download_job_checkpoint` and `download_segment_checkpoint` belong in SQLite as structured, recoverable facts.
2. Staging files, partial fragments, raw manifests, and large provider payloads stay in the filesystem/object-store layer, with references stored in checkpoint records when needed.
3. Cross-medium consistency uses a state machine plus checkpoint/retry/compensation, not a fake distributed transaction between SQLite, filesystem, and provider IO.
4. If multiple checkpoint rows for one job need atomic mutation, the adapter may introduce a downloads-local transaction boundary such as `DownloadCheckpointWriteTx`; it must not become a global unit of work.
5. The first persistence implementation may stay job-scoped and segment-scoped, but it must round-trip the current `DownloadSegmentCheckpointRecord` facts instead of only preserving checkpoint presence.

Next Rust slice:

1. add focused tests proving `SqliteDownloadCheckpointRepository::save(...)` and `load(...)` round-trip segment checkpoint facts in `DownloadCheckpointRecord`;
2. extend only the repository/adapter persistence boundary required for those tests;
3. avoid changing driver execution, runtime `JobDriver`, host transport, frontend, composition public API, concrete fetch/write/verify, snapshot completion, or public error DTOs;
4. run adapter/module-level tests plus scoped diff checks before committing.

Current Rust slice:

1. `SqliteDownloadCheckpointRepository` now creates `download_segment_checkpoints` alongside the job-level checkpoint table.
2. `save_checkpoint(...)` replaces one job's segment facts in a transaction after upserting the job checkpoint row.
3. `load_checkpoint(...)` returns the persisted `DownloadSegmentCheckpointRecord` list in saved order instead of always returning an empty checkpoint.
4. Segment status and nullable `partial_path` / `etag` / `hash_state_ref` round-trip through adapter-local row mapping.
5. `u64` offset/length/downloaded byte facts are stored as text in this first slice so SQLite's signed integer limit does not narrow the Rust contract.
6. Focused adapter coverage proves segment facts round-trip; driver execution, runtime completion, host transport, frontend, composition public API, fetch/write/verify, and public DTOs remain unchanged.

Only after segment checkpoint facts are durable should a later slice start consuming pending work to perform concrete fetch/write/verify behavior.

### 7.13 Driver Execution Boundary

The downloads backend now has the three prerequisites for a future same-process execution turn:

1. `DownloadPendingResumeWorkSource` can drain prepared work by job id;
2. composition-root shares the same scheduler/source instance between facade preparation and driver construction;
3. `SqliteDownloadCheckpointRepository` can persist and reload segment checkpoint facts.

That still does not mean real download execution exists in current Rust. The broader runtime design describes `JobDriver::run(...)`, but the current `kernel-jobs` crate only exposes `module()`, `kind()`, and `restore()`. Until a `run()` callback or equivalent execution turn exists, downloads code must not pretend that shared runtime can fetch, write, verify, or complete segment work.

Current Rust reality:

1. `resume_download_outcome()` reconstructs resume decisions, schedules pending work, then enqueues a job-level runtime request.
2. `DownloadJobDriver::restore()` only checks whether durable checkpoint facts exist.
3. `DownloadJobDriver::drain_pending_resume_work(&JobId)` is a local helper, not a shared runtime callback.
4. `DownloadJobDriver::prepare_resume_execution_turn(&JobId)` is also a local helper. It reloads checkpoint facts, avoids draining pending work when checkpoint facts are missing, and returns an explicit module-owned classification.
5. `JobDriverRegistry::resolve(...)` returns trait objects that only support current `JobDriver` trait methods.
6. There is no runtime-owned lease/execution loop that calls downloads-specific segment execution.

Execution-turn ownership rules:

1. A future downloads execution turn may drain prepared pending work for one accepted job id.
2. It must reload or validate the current durable checkpoint before treating in-memory work as executable.
3. It may map each `DownloadResumeWorkItem` into later fetch/write/verify operations, but those operations need their own ports and tests.
4. It must write checkpoint facts before reporting runtime-visible completion or terminal progress.
5. It must report progress and terminal state through the future runtime execution context, not by directly editing `JobSnapshotStore`.
6. Empty pending work is not success. It must become an explicit execution classification such as reconstruct-from-durable-facts, no-op/already-complete, or module execution failure.

Current Rust slice:

1. `DownloadDriverExecutionTurn` exists as a downloads-owned classification with `CheckpointMissing`, `NoPendingWork`, and `PendingWorkAccepted`.
2. `DownloadJobDriver::prepare_resume_execution_turn(&JobId)` reloads `DownloadCheckpointRepository` before draining pending work.
3. A missing checkpoint returns `CheckpointMissing` and preserves pending work in the source so a later durable recovery path can decide what to do.
4. An existing checkpoint with no pending work returns `NoPendingWork`; this is explicitly not runtime completion.
5. An existing checkpoint with pending work returns `PendingWorkAccepted` with the reloaded checkpoint and drained job-scoped work.
6. Focused driver tests cover all three classifications.
7. `kernel-jobs`, composition-root, host transport, frontend, SQLite schema, runtime snapshot mutation, completion APIs, and concrete fetch/write/verify execution remain unchanged.

Next boundary options:

1. segment-execution-ports option: define the downloads-owned request, port, and result boundary that would later consume `PendingWorkAccepted` work items without implementing real HTTP/disk/hash behavior;
2. runtime-first option: document and then implement the minimal `kernel-jobs` `run()` boundary before any runtime-owned loop can call the downloads local execution turn;
3. do not combine both options in one atomic task.

The segment-execution-ports option is the smaller next slice if the goal is to keep momentum inside downloads:

1. define a module-local segment execution request shape derived from `DownloadResumeWorkItem` plus the owning `JobId`;
2. define narrow fetch/write/verify or executor port boundaries only as far as a focused test needs;
3. keep all concrete IO implementations out of the first port-boundary slice;
4. keep `kernel-jobs::JobDriver`, runtime snapshots, host transport, frontend projection, SQLite schema, and completion APIs unchanged.

The runtime-first option is the better next slice if the goal is to make shared scheduling honest before more module-local driver methods:

1. document the current `kernel-jobs` gap against the broader runtime design;
2. define the minimal `run()`/execution context API and how it preserves module-owned business checkpoints;
3. add kernel-jobs tests before any downloads behavior depends on it.

Out of scope until segment execution ports and a runtime execution boundary land:

1. HTTP range requests or provider object fetches;
2. staging file writes, sparse range writes, temp fragments, or final artifact moves;
3. hash/length verification;
4. runtime snapshot completion or terminal events for downloads;
5. host transport or frontend projection of segment execution details.

---

### 7.14 Segment Execution Ports Boundary

AT-190 proves that the driver can classify a future execution turn, not that it can execute segments. The next downloads-owned code boundary must translate `DownloadDriverExecutionTurn::PendingWorkAccepted` into a narrow segment execution handoff while preserving the same ownership split:

1. `module-downloads` owns request/result semantics, checkpoint ordering, and error classification.
2. provider adapters own provider-specific fetch details.
3. staging/object-store adapters own disk writes and final artifact movement.
4. verifier/hash logic owns integrity checks.
5. `kernel-jobs` owns generic runtime snapshots and leases only after a runtime `run()` boundary exists.

Minimum segment execution request requirements:

1. include the owning `JobId`;
2. preserve `segment_id`, `file_id`, `source_locator`, `write_target`, `expected_hash`, `start_offset`, `length`, `resume_mode`, and `checkpoint_ref` from `DownloadResumeWorkItem`;
3. carry enough context for later checkpoint mutation to know which durable facts are being advanced;
4. stay inside `module-downloads` and not become a host transport DTO or `kernel-jobs` extension.

Port boundary rules:

1. The first Rust slice may introduce request/result types and narrow trait boundaries with focused fake implementations.
2. The first Rust slice may prove ordering from `PendingWorkAccepted` to request handoff.
3. The first Rust slice must not perform real HTTP range requests, file writes, hash verification, checkpoint saves, snapshot updates, job completion, or event publication.
4. Fetch/write/verify may be separate ports or an executor facade over separate internal ports, but the design must not collapse manifest parsing, scheduling, disk writes, verification, checkpoint mutation, and runtime completion into one long-lived worker object.
5. Errors from a fake execution port must remain module-local until the concrete failure surface is designed.

First Rust slice after this document:

1. add focused RED tests in `launcher-module-downloads` proving `PendingWorkAccepted` can be converted into job-scoped segment execution requests in stable order;
2. add only the request/result/port shell and local driver helper needed to pass those tests;
3. keep `kernel-jobs`, composition-root, host transport, frontend, SQLite schema, real fetch/write/verify, checkpoint mutation, and runtime completion unchanged;
4. run focused module tests, full module tests, rustfmt check, scoped `git diff --check`, and path-limited status before commit.

Current Rust slice:

1. `DownloadSegmentExecutionRequest` captures the owning `JobId` plus segment execution facts derived from `DownloadResumeWorkItem`.
2. `DownloadSegmentExecutionResult` is a module-local result shell with an `Accepted` variant.
3. `DownloadSegmentExecutionPort` is a module-local port shell for handing requests to later fake or concrete execution boundaries.
4. `DownloadJobDriver::prepare_segment_execution_requests(...)` converts `PendingWorkAccepted` turns into stable ordered segment requests.
5. Non-pending turns produce no segment execution requests.
6. `kernel-jobs`, composition-root, host transport, frontend, SQLite schema, real fetch/write/verify, checkpoint mutation, and runtime completion remain unchanged.

Only after that request/port handoff exists should a later slice pick one concrete execution concern, such as fake execution acceptance, staging write contract, verifier contract, or checkpoint mutation after a fake segment result. Each of those must remain its own atomic task.

---

### 7.15 Fake Segment Execution Acceptance Boundary

The next safe code slice is fake/local execution acceptance through `DownloadSegmentExecutionPort`. This is not HTTP fetching. It only proves that prepared segment execution requests can cross a narrow module-owned port in stable order and return module-local results.

Current Rust reality:

1. `DownloadSegmentExecutionRequest` values can already be prepared from `PendingWorkAccepted`.
2. `DownloadSegmentExecutionPort` exists but no driver helper delegates a batch of requests to it.
3. There is no concrete fetcher, writer, verifier, checkpoint mutation, runtime completion, host transport, or frontend projection.

Boundary rules:

1. The driver helper may accept a `&dyn DownloadSegmentExecutionPort` plus a slice of `DownloadSegmentExecutionRequest`.
2. It must call the port once per request and preserve request order in the returned `DownloadSegmentExecutionResult` list.
3. It may propagate the first `AppResult` error returned by the port, but it must not invent public `DL_*` execution errors yet.
4. The test fake may record accepted requests and return `DownloadSegmentExecutionResult::Accepted`.
5. The production slice must not perform HTTP range requests, file writes, hash verification, checkpoint saves, snapshot updates, job completion, event publication, or retry/backoff behavior.

First Rust slice:

1. add focused RED tests proving ordered request handoff and result collection through a recording fake port;
2. add only the local `DownloadJobDriver` helper required for those tests;
3. keep request/result/port shapes stable unless the tests reveal a missing field;
4. run focused module tests, full module tests, rustfmt check, scoped `git diff --check`, and path-limited status before commit.

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
