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
| `resume_download` manifest/runtime orchestration | not wired yet, still returns `DOWNLOADS_NOT_WIRED` after staging | future slices |
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
4. Manifest reconstruction is the next likely backend slice.
5. Runtime enqueue-resume must wait until manifest/staging inputs are explicit enough to test.

Do not skip directly from checkpoint to `JobRuntime::resume`. The module owns business checkpoint and resume reconstruction.

---

## 6. Port Status

| Port | Status | Notes |
|------|--------|-------|
| `DownloadJobRepository` | defined in facade module | concrete SQLite shell exists |
| `DownloadCheckpointRepository` | defined in driver module | concrete SQLite shell exists |
| `DownloadStagingObjectStore` | minimal facade port exists | `()` placeholder keeps composition wiring stable |
| `DownloadManifestProviderPort` | not defined in code yet | likely next resume/start implementation boundary |
| `JobRuntime` | shared kernel-jobs runtime trait exists | current resume does not enqueue yet |

When adding a port:

1. define the narrow trait needed by the current test;
2. keep `()` placeholder behavior only when it preserves existing composition wiring;
3. add module facade tests before production code;
4. defer concrete adapter behavior unless the task explicitly scopes it.

---

## 7. Error Semantics

Downloads-domain errors use `DL_*` codes when they become stable public classifications.

Current stable resume errors:

| Code | Trigger | Retryable | Severity |
|------|---------|-----------|----------|
| `DL_JOB_NOT_FOUND` | requested job has no downloads module record | false | error |
| `DL_CHECKPOINT_MISSING` | requested job has no module checkpoint | false | error |

`DOWNLOADS_NOT_WIRED` remains a temporary implementation placeholder, not a final domain error.

---

## 8. Required Validation

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

## 9. Non-goals

Do not use this module implementation document to:

1. record temporary task logs;
2. replace `.artifacts/ai` active task records;
3. duplicate the full download runtime design;
4. move frontend UI requirements into backend implementation truth;
5. justify broad refactors outside the current atomic task.
