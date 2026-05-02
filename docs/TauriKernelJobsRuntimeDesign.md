# Tauri Kernel Jobs Runtime Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `.artifacts/docs/TauriDownloadRuntimeDesign.md`, `.artifacts/docs/TauriRustTsSchemaDesign.md`, `.artifacts/docs/TauriFirstCrateApiDrafts.md`, `.artifacts/docs/TauriCompositionRootWiringDesign.md`
> Focus: shared job runtime host, queue policy, snapshot store, lease model, recovery lifecycle, module integration boundary

---

## 1. Purpose

前面的文档已经把以下几层分别定义清楚了：

1. 下载运行时如何做 segment 调度、checkpoint 和 backpressure
2. 共享 job snapshot / accepted job / progress schema 如何表达
3. `launcher-kernel-jobs` 在 crate 层应该暴露哪些公开面
4. `composition-root` 如何装配 job runtime 并把它接进模块 facade

当前还缺的不是“下载还能细化什么”，而是：

**共享的 job runtime 内核本身到底是什么，它拥有哪些状态，如何调度，如何持久化快照，如何在重启后恢复。**

这份文档专门定义 `launcher-kernel-jobs` 的运行时设计，目标是让它成为真正可复用的共享内核，而不是变成另一个下载专用实现或新的 God Layer。

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Shared runtime kernel | Downloads、Fab 预热同步、后续 Engines/Installations 都能共用同一 runtime host |
| Snapshot-first observability | 前端和宿主只观察聚合 snapshot，不看底层噪声事件 |
| Safe recovery | 崩溃、重启、阶段恢复后能重新发现并恢复 resumable jobs |
| Explicit queue policy | 并发、公平性、优先级和 backpressure 由统一策略控制 |
| Module-owned business checkpoints | `kernel-jobs` 不吞掉下载 checkpoint、Fab cursor 等业务状态 |
| Lease-based execution safety | 同一时刻同一 job 只能被一个 runtime host 认领执行 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 让 `kernel-jobs` 直接理解 segment、cursor、repair plan | 这些属于模块语义 |
| 让 runtime 直接访问 provider API | provider 仍属于模块 adapter |
| 让前端直接驱动 runtime 内部状态机 | 前端只能发 command / 读 snapshot |
| 用一个 snapshot store 代替全部模块 repository | 通用快照和模块业务 checkpoint 不是一回事 |

---

## 3. Core Responsibility Split

`launcher-kernel-jobs` 只拥有以下四类东西：

1. 通用 job lifecycle state
2. 通用 runtime host / queue policy / lease 协议
3. 通用 snapshot store 和控制接口
4. 通用事件收敛与恢复入口

模块继续拥有：

1. 业务 checkpoint
2. 业务输入和执行逻辑
3. 业务扩展 snapshot 字段
4. 业务完成条件和补偿逻辑

### 3.1 Example Ownership

| Concern | Owner |
|---------|-------|
| 下载 segment offset checkpoint | `module-downloads` |
| Fab sync cursor | `module-fab` |
| job queued/running/paused/completed state | `launcher-kernel-jobs` |
| 统一 `AcceptedJob` / `JobSnapshot` | `launcher-kernel-jobs` |
| 下载 resume 的业务重建逻辑 | `module-downloads` |
| Fab startup prewarm 的业务重建逻辑 | `module-fab` |

---

## 4. Runtime Mental Model

建议把共享运行时理解为四层：

```text
module facade / use case
  -> JobRuntimePort
     -> RuntimeHost
        -> QueuePolicy
        -> LeaseCoordinator
        -> SnapshotStore
        -> JobDriverRegistry
```

### 4.1 `JobRuntimePort`

面向模块用例的公共入口：

1. enqueue job
2. query snapshot
3. control pause/resume/cancel

### 4.2 `RuntimeHost`

负责：

1. 调度执行循环
2. 认领 job lease
3. 驱动 job runner
4. 更新 snapshot store

### 4.3 `QueuePolicy`

负责：

1. 全局并发预算
2. 模块级权重与优先级
3. backpressure 降载策略
4. job eligibility 选择规则

### 4.4 `JobDriverRegistry`

负责把 `module + kind` 路由到具体模块提供的执行器，但 runtime 不知道内部业务语义。

---

## 5. Canonical Job Lifecycle

### 5.1 Internal Lifecycle

建议共享内核统一使用以下内部状态：

1. `Queued`
2. `ClaimingLease`
3. `Restoring`
4. `Running`
5. `PauseRequested`
6. `Paused`
7. `ResumeRequested`
8. `Completing`
9. `Completed`
10. `Failed`
11. `Canceled`

### 5.2 UI Projection Mapping

映射到前端只保留：

1. `Queued`
2. `Running`
3. `Paused`
4. `AwaitingUser`
5. `Completed`
6. `Failed`
7. `Canceled`

`Restoring`、`ClaimingLease`、`Completing` 等状态只存在于内核，不直接暴露给页面。

### 5.3 Lifecycle Rule

固定规则：

1. 所有状态迁移都先更新 snapshot store，再对外发收敛事件。
2. 模块执行器不直接篡改共享 snapshot store，只通过 runtime context 报告进度与结束结果。

---

## 6. Shared Runtime Contracts

### 6.1 Public Modules

建议 `launcher-kernel-jobs` 公开这些模块：

```text
src/
  lib.rs
  contracts/
    accepted.rs
    snapshot.rs
    progress.rs
    definition.rs
  runtime/
    ports.rs
    queue_policy.rs
    lease.rs
    recovery.rs
    registry.rs
  store/
    snapshot_store.rs
  state/
    job_state.rs
```

### 6.2 `JobDefinition`

```rust
pub struct JobDefinition<E> {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub title: String,
    pub priority: JobPriority,
    pub recoverable: bool,
    pub extension: Option<E>,
}
```

说明：

1. 这是 runtime-level 定义，不是模块业务 DTO。
2. `extension` 只用于给 snapshot 带模块扩展摘要，不用于承载完整业务 checkpoint。

### 6.3 `JobRuntimePort`

```rust
pub trait JobRuntimePort: Send + Sync {
    type Extension;

    fn enqueue(&self, request: EnqueueJobRequest<Self::Extension>) -> AppResult<AcceptedJob>;
    fn snapshot(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<Self::Extension>>>;
    fn list_active(&self, filter: JobListFilter) -> AppResult<Vec<JobSnapshot<Self::Extension>>>;
}
```

### 6.4 `JobRuntimeControlPort`

```rust
pub trait JobRuntimeControlPort: Send + Sync {
    fn pause(&self, job_id: &JobId) -> AppResult<()>;
    fn resume(&self, job_id: &JobId) -> AppResult<()>;
    fn cancel(&self, job_id: &JobId) -> AppResult<()>;
}
```

### 6.5 `JobDriver` Registry

```rust
pub trait JobDriver: Send + Sync {
    type Extension;

    fn module(&self) -> &'static str;
    fn kind(&self) -> &'static str;
    fn restore(&self, job_id: &JobId, context: &JobRuntimeContext<Self::Extension>) -> AppResult<RestoreDisposition>;
    fn run(&self, job_id: &JobId, context: &JobRuntimeContext<Self::Extension>) -> AppResult<JobRunDisposition>;
}
```

解释：

1. `restore()` 允许模块从自己的 checkpoint repository / cursor store 重建运行条件。
2. `run()` 才是真正执行。
3. runtime 只按 `module + kind` 调度 driver，不理解其业务内部实现。

---

## 7. Queue Policy Design

### 7.1 `RuntimeQueuePolicy`

建议统一策略结构：

```rust
pub struct RuntimeQueuePolicy {
    pub global_slots: u16,
    pub per_module_caps: HashMap<String, u16>,
    pub module_weights: HashMap<String, u16>,
    pub allow_low_priority_prewarm: bool,
    pub writer_backpressure_threshold: u32,
    pub memory_pressure_threshold_mb: u32,
}
```

### 7.2 Scheduling Rule

运行时每个调度周期建议按以下顺序：

1. 读取所有 eligible jobs
2. 过滤掉无 lease、暂停中、取消中、不可恢复的 job
3. 应用 `global_slots`
4. 应用 `per_module_caps`
5. 按 `module_weights + priority` 做 weighted fair selection
6. 在 backpressure 触发时主动降载

### 7.3 Priority Model

建议共享优先级枚举：

1. `UserBlocking`
2. `UserInitiated`
3. `BackgroundRefresh`
4. `Prewarm`

Fab startup prewarm 默认 `Prewarm`。

用户主动下载默认 `UserInitiated`。

---

## 8. Lease Model

虽然当前桌面应用默认只有单进程宿主，但仍建议共享内核显式建模 lease，避免恢复链路混乱。

### 8.1 Lease Purpose

lease 只解决一件事：

**确保单个 job 在任一时刻只被一个 runtime host 认领执行。**

### 8.2 `JobLease`

```rust
pub struct JobLease {
    pub job_id: JobId,
    pub host_id: String,
    pub lease_epoch: u64,
    pub acquired_at: IsoDateTime,
    pub expires_at: IsoDateTime,
}
```

### 8.3 Lease Rules

1. `build_desktop_services()` 不认领任何 job。
2. 只有 `run_stage2_restore_runtime_state()` 或显式 `enqueue()` 后的 runtime loop 才尝试认领 lease。
3. 崩溃恢复后，新 host 可以认领已过期 lease 的 job。
4. `Completed`、`Failed`、`Canceled` job 必须释放 lease。

### 8.4 Why still needed on desktop

原因不是多节点，而是：

1. 阶段恢复可能与用户命令同时到来
2. 未来可能引入多个窗口或多个 runtime loop
3. lease 模型让恢复语义和测试更清晰

---

## 9. Snapshot Store Design

### 9.1 Purpose

`SnapshotStore` 只存共享 runtime 可见状态，不存模块业务 checkpoint。

### 9.2 Stored Data

建议每条 snapshot 至少包含：

1. `job_id`
2. `module`
3. `kind`
4. `title`
5. `internal_state`
6. `ui_state`
7. `priority`
8. `recoverable`
9. `progress`
10. `error`
11. `updated_at`
12. `extension_summary`

### 9.3 `SnapshotStorePort`

```rust
pub trait SnapshotStorePort<E>: Send + Sync {
    fn create(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    fn update(&self, snapshot: &JobSnapshot<E>) -> AppResult<()>;
    fn get(&self, job_id: &JobId) -> AppResult<Option<JobSnapshot<E>>>;
    fn list_recoverable(&self) -> AppResult<Vec<JobSnapshot<E>>>;
    fn acquire_lease(&self, job_id: &JobId, host_id: &str) -> AppResult<Option<JobLease>>;
    fn release_lease(&self, job_id: &JobId, host_id: &str) -> AppResult<()>;
}
```

### 9.4 Boundary Rule

下载 segment checkpoint 不进这里。

Fab sync cursor 不进这里。

这些仍然在各自模块 repository 中。

`SnapshotStore` 只负责“runtime 当前怎么看这份 job”。

---

## 10. Recovery Lifecycle

### 10.1 Recovery Trigger

恢复只允许在两个入口发生：

1. `StartupPipelineFacade.run_stage2_restore_runtime_state()`
2. 明确的用户 `resume` command

禁止：

1. module facade 构造时偷偷恢复
2. 第一次读列表时懒恢复

### 10.2 Stage 2 Recovery Flow

```text
run_stage2_restore_runtime_state()
  -> snapshot_store.list_recoverable()
  -> for each job
       -> try acquire lease
       -> resolve driver by module/kind
       -> driver.restore(job_id, context)
       -> if resumable then enqueue to runtime loop
       -> else mark failed_or_canceled with reason
```

### 10.3 Module-specific Recovery Responsibility

Downloads 在 `restore()` 里：

1. 读取 `DownloadCheckpointRepository`
2. 校验 staging 区是否仍可用
3. 准备 manifest / planner 重建条件

Fab 在 `restore()` 里：

1. 读取 `FabSyncCursorRepository`
2. 判断是否还有未完成 prewarm / sync job
3. 重新进入增量 sync 链路

### 10.4 Recovery Outcomes

建议统一三类恢复结果：

1. `Resumed`
2. `SkippedAsTerminal`
3. `FailedAsUnrecoverable`

---

## 11. Runtime Context Given to Modules

模块 driver 不应该直接拿到整个 runtime host，而是拿到一个窄上下文：

```rust
pub struct JobRuntimeContext<E> {
    pub clock: Arc<dyn Clock>,
    pub snapshot_writer: Arc<dyn SnapshotWriter<E>>,
    pub cancellation: CancellationToken,
    pub lease: JobLease,
}
```

其中：

1. `snapshot_writer` 只允许模块报告进度和状态变化
2. `cancellation` 允许模块 cooperatively cancel
3. 模块拿不到 queue internals 或其他 job 信息

---

## 12. Event Model

### 12.1 Internal Runtime Events

共享内核可以有更细的内部事件：

1. `JobQueued`
2. `JobLeaseAcquired`
3. `JobRestored`
4. `JobProgressUpdated`
5. `JobPauseRequested`
6. `JobCompleted`
7. `JobFailed`

### 12.2 External Event Rule

对前端或桌面宿主公开时，必须先收敛成：

1. snapshot 变更
2. accepted job
3. terminal state notifications

不能把 segment、cursor、页面级 sync 分页事件直接冒到公共事件层。

---

## 13. Integration with First-slice Modules

### 13.1 Downloads Integration

`module-downloads` 使用 `kernel-jobs` 的方式应当是：

1. `start_download()` 构造 `JobDefinition<DownloadJobExtensionSummary>`
2. enqueue 到 runtime
3. 具体 download driver 在 `run()` 中调用 planner / scheduler / writer / verifier
4. 业务 checkpoint 仍写回 `DownloadCheckpointRepository`

### 13.2 Fab Integration

`module-fab` 使用 `kernel-jobs` 的方式应当是：

1. `run_startup_prewarm()` 构造 `JobDefinition<FabInventoryJobExtensionSummary>`
2. enqueue 到 runtime
3. 具体 Fab sync driver 在 `run()` 中调用 provider + projection repo + cursor repo
4. 业务 cursor 仍写回 `FabSyncCursorRepository`

### 13.3 What stays out of kernel

以下都不进入 `launcher-kernel-jobs`：

1. segment plan
2. Fab page continuation token
3. engine repair manifest diff
4. install artifact replacement strategy

---

## 14. First-slice Concrete Shape

第一版建议只实现一个共享 runtime host，但预留 module-specific driver registry：

```text
SharedJobRuntimeHost
  -> SnapshotStorePort
  -> RuntimeQueuePolicy
  -> DriverRegistry
     -> DownloadsJobDriver
     -> FabInventoryJobDriver
```

这样可以最快验证：

1. queue policy 是否真的可共享
2. snapshot store 是否足够表达多模块任务
3. 恢复语义是否不需要把全部业务塞进 kernel

---

## 15. Testing Strategy

### 15.1 Queue Policy Tests

验证：

1. `global_slots` 生效
2. `per_module_caps` 生效
3. 用户主动下载优先于 prewarm
4. backpressure 触发时会降载

### 15.2 Lease Tests

验证：

1. 同一 job 不会被双重认领
2. 过期 lease 可被新 host 夺回
3. terminal 状态会释放 lease

### 15.3 Snapshot Store Tests

验证：

1. snapshot create/update/get 正常
2. `list_recoverable()` 只返回可恢复 job
3. 模块扩展摘要不会破坏通用字段读取

### 15.4 Recovery Tests

验证：

1. Stage 2 才会触发恢复
2. Downloads restore 会读取 checkpoint repository
3. Fab restore 会读取 sync cursor repository
4. 不可恢复 job 会进入明确 terminal outcome

### 15.5 Handler Boundary Tests

验证：

1. 前端 query/command 只读 snapshot / accepted job
2. handler 无需知道 driver、queue、lease 的 concrete 类型

---

## 16. Anti-patterns

以下做法视为偏离本稿：

1. 把 `launcher-kernel-jobs` 做成“下载模块的另一个名字”。
2. 让 `SnapshotStore` 存完整 segment checkpoint、完整 cursor payload、完整 provider 响应体。
3. 让模块 driver 直接操作 queue internals 或其他 job 的状态。
4. 在 `build_desktop_services()` 阶段就自动恢复并启动所有 job。
5. 让前端事件流直接看到 segment/page/chunk 级细节。

---

## 17. Acceptance Criteria

满足以下条件，才算这份 `kernel-jobs` 稿可进入实现前阶段：

1. 已明确共享 runtime host、queue policy、snapshot store、lease/recovery 的边界。
2. 已明确 `kernel-jobs` 不拥有模块业务 checkpoint，只拥有共享 snapshot。
3. 已明确 Stage 2 恢复是唯一启动恢复入口之一。
4. 已明确 Downloads 和 Fab 如何接入共享 runtime，而不把业务细节塞进内核。
5. 已明确第一批测试矩阵，能够验证调度、公平性、lease、快照和恢复语义。
