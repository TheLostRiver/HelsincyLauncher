# Tauri Rust/TS Schema Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Focus: Rust-owned DTO schemas, TS generation, shared envelopes, long-job snapshots, Fab inventory prewarm/cache models

---

## 1. Purpose

本文档把“契约优先”继续往下压一层，直接落到 Rust / TypeScript 的 schema 设计上。

它要解决的问题不是“模块之间原则上怎么通信”，而是：

1. Rust 侧具体要定义哪些 DTO
2. 这些 DTO 如何稳定导出为 TypeScript 类型
3. 哪些结构属于共享 envelope，哪些属于模块专用 read model
4. Fab 库存预热、缓存复用、后台刷新这些状态到底如何表达，避免前端自己猜

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Rust-owned schemas | 所有跨 IPC DTO 都以后端 Rust 为源头 |
| TS generation only | 前端只消费生成类型，不手写重复结构 |
| Stable string enums | 所有前端可见枚举必须是稳定字符串值 |
| Shared envelopes first | command/query/error/job snapshot 先统一，再谈模块特化 |
| Fab warm-cache semantics | Fab 库存预热、route return、background refresh 必须有显式 schema |
| UI-ready read models | 页面直接消费 read model，不自己二次猜状态 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 直接暴露 domain entity | 会把内部规则渗到前端 |
| 用匿名 JSON 结构临时拼接口 | 无法长期演进 |
| 把每个模块都设计成完全不同的响应风格 | 会让前端集成失控 |
| 在 schema 里掺入数据库表结构细节 | DTO 不是 persistence model |

---

## 3. Source-of-Truth Rule

所有跨前后端 schema 的权威源头都在 Rust：

1. Rust 定义 `struct` / `enum`
2. 统一使用 `serde` 序列化
3. 统一使用 `specta` / `tauri-specta` 导出 TS 类型
4. 前端只引用生成产物

推荐基础约束：

```rust
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDto {
    pub id: String,
}
```

固定规则：

1. 前端可见字段统一导出为 `camelCase`。
2. 前端可见 `enum` 必须是字符串枚举，不使用数字枚举。
3. 任何需要长期兼容的字段都不能依赖“字段缺失即语义变化”的隐式协议。

---

## 4. Shared Scalar and Identity Types

不要在每个 DTO 里重复发明 loosely typed 字符串。

建议至少统一以下基础类型：

```rust
pub type JobId = String;
pub type CorrelationId = String;
pub type AccountId = String;
pub type AssetId = String;
pub type EngineInstallationId = String;
pub type IsoDateTime = String;
```

如果后续需要更强约束，可以升级为 newtype，但前端导出保持为稳定字符串。

---

## 5. Shared Envelope Schemas

### 5.1 `AppErrorDto`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub severity: ErrorSeverityDto,
    pub correlation_id: CorrelationId,
    pub details: Option<ErrorDetailsDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverityDto {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetailsDto {
    pub user_hint: Option<String>,
    pub support_hint: Option<String>,
}
```

导出后前端只看：

```ts
type AppErrorDto = {
  code: string;
  message: string;
  retryable: boolean;
  severity: "info" | "warning" | "error" | "fatal";
  correlationId: string;
  details?: {
    userHint?: string;
    supportHint?: string;
  };
};
```

### 5.2 `CommandResultDto<T>`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "ok", rename_all = "camelCase")]
pub enum CommandResultDto<T> {
    #[serde(rename = "true")]
    Success { data: T },
    #[serde(rename = "false")]
    Failure { error: AppErrorDto },
}
```

### 5.3 `QueryResultDto<T>`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "ok", rename_all = "camelCase")]
pub enum QueryResultDto<T> {
    #[serde(rename = "true")]
    Success { data: T, as_of: Option<IsoDateTime> },
    #[serde(rename = "false")]
    Failure { error: AppErrorDto },
}
```

### 5.4 `AcceptedJobDto`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedJobDto {
    pub accepted: bool,
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub queued_at: IsoDateTime,
}
```

固定规则：

1. 长任务 start command 成功时优先返回 `AcceptedJobDto`。
2. 前端收到后走 snapshot/query，不把“已接受”误当成“已完成”。

---

## 6. Shared Long-job Snapshot Schema

### 6.1 `JobSnapshotDto`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct JobSnapshotDto {
    pub job_id: JobId,
    pub module: String,
    pub kind: String,
    pub title: String,
    pub ui_state: JobUiStateDto,
    pub progress: JobProgressDto,
    pub throughput: Option<ThroughputDto>,
    pub eta_seconds: Option<u64>,
    pub retryable: bool,
    pub error: Option<AppErrorDto>,
    pub updated_at: IsoDateTime,
    pub extension: Option<JobExtensionDto>,
}
```

### 6.2 Supporting DTOs

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum JobUiStateDto {
    Queued,
    Running,
    Paused,
    AwaitingUser,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct JobProgressDto {
    pub phase: String,
    pub current: u64,
    pub total: Option<u64>,
    pub percentage: Option<f64>,
    pub unit: ProgressUnitDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ProgressUnitDto {
    Bytes,
    Files,
    Segments,
    Items,
    Pages,
    Components,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputDto {
    pub value: f64,
    pub unit: String,
}
```

### 6.3 `JobExtensionDto`

共享 snapshot 必须允许模块加少量扩展字段，但方式要统一：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JobExtensionDto {
    Download(DownloadJobExtensionDto),
    FabInventory(FabInventoryJobExtensionDto),
    EngineVerify(EngineVerifyJobExtensionDto),
}
```

规则：

1. 页面公共任务面板只依赖 `JobSnapshotDto` 通用字段。
2. 模块专页可以再读对应 `extension`。
3. 禁止每个模块完全另起一套 snapshot 顶层结构。

---

## 7. Fab Inventory Schema Set

这一组 schema 是当前优先级最高的，因为它同时承载：

1. 启动后自动预热
2. route return 缓存复用
3. 背景增量刷新
4. 页面真正需要的热/冷状态表达

### 7.1 Sync Commands

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventorySyncStartInputDto {
    pub account_id: AccountId,
    pub reason: FabInventorySyncReasonDto,
    pub scope: FabInventorySyncScopeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventorySyncReasonDto {
    StartupPrewarm,
    RouteEnterRefresh,
    ManualRefresh,
    RepairRebuild,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventorySyncScopeDto {
    HeadPagesOnly,
    IncrementalCursor,
    FullRebuild,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventorySyncCancelInputDto {
    pub job_id: JobId,
}
```

### 7.2 List Query

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventoryListQueryDto {
    pub page: u32,
    pub page_size: u32,
    pub sort: FabInventorySortDto,
    pub filters: FabInventoryFiltersDto,
    pub search: Option<String>,
}
```

### 7.3 List Response

不要只返回 `items[]`，必须把 warm/cache/freshness 元数据一起返回：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventoryListPageDto {
    pub items: Vec<FabInventoryListItemDto>,
    pub page: PageInfoDto,
    pub freshness: DataFreshnessDto,
    pub warm_state: FabInventoryWarmStateDto,
    pub sync: FabInventorySyncStatusDto,
    pub source: FabInventoryDataSourceDto,
}
```

关键点：

1. 前端不需要猜“这页数据是不是来自本地 projection”。
2. 前端不需要猜“现在是不是后台正在刷新”。
3. 前端不需要猜“切页回来是不是应该重新冷启动”。

### 7.4 Warm-state DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventoryWarmStateDto {
    ColdEmpty,
    WarmProjection,
    WarmRefreshing,
    StaleUsable,
    SyncFailedUsable,
}
```

语义：

- `cold_empty`: 本地 projection 暂无可用结果
- `warm_projection`: 已有可用本地结果，当前无需强调刷新
- `warm_refreshing`: 已有可用结果，后台正在刷新
- `stale_usable`: 有结果但 freshness 超阈值
- `sync_failed_usable`: 最近刷新失败，但旧结果仍可展示

### 7.5 Freshness DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DataFreshnessDto {
    pub as_of: Option<IsoDateTime>,
    pub last_success_at: Option<IsoDateTime>,
    pub stale: bool,
    pub stale_reason: Option<DataStaleReasonDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum DataStaleReasonDto {
    NeverSynced,
    ThresholdExceeded,
    LastRefreshFailed,
    ProjectionRebuildPending,
}
```

### 7.6 Sync Status DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventorySyncStatusDto {
    pub active_job_id: Option<JobId>,
    pub state: FabInventorySyncStateDto,
    pub reason: Option<FabInventorySyncReasonDto>,
    pub progress: Option<JobProgressDto>,
    pub last_started_at: Option<IsoDateTime>,
    pub last_completed_at: Option<IsoDateTime>,
    pub last_error: Option<AppErrorDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventorySyncStateDto {
    Idle,
    Running,
    Failed,
}
```

### 7.7 Data Source DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventoryDataSourceDto {
    ProjectionOnly,
    ProjectionWithBackgroundRefresh,
    ColdBootstrap,
}
```

### 7.8 List Item DTO

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventoryListItemDto {
    pub asset_id: AssetId,
    pub title: String,
    pub seller_name: String,
    pub category: String,
    pub owned_at: Option<IsoDateTime>,
    pub thumbnail_state: ThumbnailStateDto,
    pub thumbnail_url: Option<String>,
    pub install_state: InstallStateBadgeDto,
    pub engine_support_summary: Option<String>,
    pub badges: Vec<String>,
}
```

### 7.9 Events

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FabInventoryChangedEventDto {
    pub account_id: AccountId,
    pub changed_item_count: u32,
    pub source: FabInventoryChangeSourceDto,
    pub occurred_at: IsoDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum FabInventoryChangeSourceDto {
    IncrementalSync,
    Rebuild,
    MediaEnrichment,
}
```

### 7.10 TS Consumer Shape

前端最终只需要消费类似下面的生成类型：

```ts
type FabInventoryListPageDto = {
  items: FabInventoryListItemDto[];
  page: PageInfoDto;
  freshness: DataFreshnessDto;
  warmState:
    | "cold_empty"
    | "warm_projection"
    | "warm_refreshing"
    | "stale_usable"
    | "sync_failed_usable";
  sync: FabInventorySyncStatusDto;
  source:
    | "projection_only"
    | "projection_with_background_refresh"
    | "cold_bootstrap";
};
```

这个 schema 已经足够让页面决定：

1. 显示真实列表还是空态骨架
2. 显示“后台正在刷新”还是“结果略旧”提示
3. 路由返回时直接复用缓存，不再重新回冷启动

---

## 8. Download and Engine Schema Extensions

本轮重点是 Fab，但另外两条链路也需要稳定 extension schema，避免下轮再推倒。

### 8.1 `DownloadJobExtensionDto`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadJobExtensionDto {
    pub completed_bytes: u64,
    pub total_bytes: Option<u64>,
    pub completed_segments: u32,
    pub total_segments: Option<u32>,
    pub concurrency_slots: u16,
}
```

### 8.2 `EngineVerifyJobExtensionDto`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EngineVerifyJobExtensionDto {
    pub engine_installation_id: EngineInstallationId,
    pub scanned_files: u64,
    pub total_files: Option<u64>,
    pub diff_summary: Option<VerificationDiffSummaryDto>,
    pub estimated_repair_bytes: Option<u64>,
}
```

---

## 9. Repository-facing vs IPC-facing Models

同一个概念必须允许存在两层模型：

1. persistence model
2. IPC/read model

例如 Fab：

- SQLite 里是 `fab_owned_asset_projection`
- IPC 上是 `FabInventoryListItemDto` / `FabInventoryListPageDto`

两者不能直接等同。

规则：

1. repository model 可以有内部列、索引、cursor 字段
2. IPC model 只保留页面和 shell 需要的语义字段
3. 若 IPC model 长得像数据库表，通常说明映射层偷懒了

---

## 10. Generation and Packaging Layout

推荐目录：

```text
backend/contracts/
  common/
  fab/
  downloads/
  engines/
frontend/src/generated/contracts/
```

Rust 侧建议至少拆成：

1. `common_dto.rs`
2. `fab_inventory_dto.rs`
3. `download_dto.rs`
4. `engine_verify_dto.rs`

前端只从 `generated/contracts` 入口 re-export 引用。

---

## 11. Review Checklist

新增任何一个 schema 前，至少问五个问题：

1. 它是不是应该复用现有共享 envelope，而不是另起一套？
2. 它是不是把内部 entity 或 persistence model 直接暴露了？
3. 它是不是已经足够表达 warm/cache/freshness，而不用前端猜？
4. 它是不是使用了稳定字符串枚举？
5. 它会不会逼前端手写额外 mapping 才能用？

---

## 12. Acceptance Criteria

满足以下条件，才算这份 schema 稿达标：

1. 共享 `AppErrorDto`、`CommandResultDto`、`QueryResultDto`、`AcceptedJobDto`、`JobSnapshotDto` 已有明确 shape。
2. Fab 库存链的预热、缓存复用、后台刷新状态已经有显式 DTO，而不是靠前端猜。
3. 下载和引擎验证链的 snapshot extension 已有统一挂点。
4. Rust 是唯一 schema 源头，TS 只消费生成类型。
5. IPC model 与 persistence model 的边界已经明确。

---

## 13. Anti-patterns

以下做法视为违规：

1. 前端自己手写一份与 Rust 不受控同步的接口类型。
2. 把 `fab_owned_asset_projection` 直接导出给前端当 DTO。
3. 用布尔值拼出一堆隐式状态，让前端猜“当前到底是冷、热、刷新中还是失败可用”。
4. 每个模块都设计自己风格完全不同的结果 envelope。
5. 用匿名 `HashMap<String, Value>` 一类结构临时穿透 IPC。
