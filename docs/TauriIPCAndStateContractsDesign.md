# Tauri IPC and State Contracts Design

> Status: local draft v1
> Date: 2026-05-02
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Focus: command/query/event catalog, long-job schema, error envelope, state ownership, frontend read models

---

## 1. Purpose

本文档把蓝图中的 IPC 和状态管理原则细化成一份真正可用于实现的契约专项稿。

目标是把以下四件事钉死：

1. 模块之间到底通过哪些 command / query / event 通信
2. 长任务 snapshot、错误 envelope、diff summary、repair plan 这些跨模块 DTO 怎么长
3. 后端权威状态、前端 query cache、前端 view state 的边界和写权限如何划分
4. 前端页面真正消费哪些 read model，而不是直接碰 provider DTO 或内部运行态

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Contract-first | 前后端和跨模块都围绕显式契约协作，不靠隐式约定 |
| Single source of truth | Rust 后端是契约源头，前端消费生成类型 |
| Stable naming | command / query / event 命名可预测、可检索、可扩展 |
| Read model isolation | 前端只消费 UI 友好的投影对象，不消费内部状态机或 provider 原始 DTO |
| Strict state ownership | 每类状态只有一个权威 owner |
| Shared long-job schema | 下载、安装、验证、同步统一使用一套长任务协议 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 前端手写一套和 Rust 重复的 DTO | 容易漂移 |
| 每个模块各自发明命名和错误格式 | 会导致集成成本失控 |
| 把 query cache 当业务真相 | 会形成第二套状态源 |
| 直接把内部运行态或 provider JSON 暴露给页面 | 会打穿边界 |

---

## 3. Contract Source of Truth

契约源头必须固定在后端 Rust：

1. Rust 定义 command input、query input、read model、event payload、error DTO
2. 使用 `serde` + `specta` 或等价方案导出 TS 类型
3. 前端只消费生成结果，不手写重复结构

固定规则：

1. 任何跨 IPC 的 DTO 都必须可版本化。
2. 任何公开给前端的结构都必须是稳定 read model，而不是内部 domain entity。
3. 若一个页面需要大量字段转换，优先说明 read model 设计失败，而不是在前端堆 mapping 层。

---

## 4. Naming and Versioning Rules

### 4.1 Command Naming

统一格式：

```text
<module>.<resource>.<action>
```

示例：

- `fab.inventory.sync.start`
- `downloads.job.pause`
- `engines.verify.start`
- `engines.repair.confirm`
- `settings.download_policy.update`

### 4.2 Query Naming

统一格式：

```text
<module>.<resource>.get|list|status|facets
```

示例：

- `fab.inventory.list`
- `fab.inventory.facets`
- `downloads.job.get`
- `downloads.job.list`
- `engines.installation.health.get`

### 4.3 Event Naming

统一格式：

```text
<module>.<resource>.<fact_past_tense>
```

示例：

- `fab.inventory.changed`
- `downloads.job.completed`
- `downloads.job.failed`
- `engines.verification.completed`
- `engines.repair.plan_ready`

### 4.4 Versioning

推荐规则：

1. schema 破坏性变更才升 major
2. query / command 新增可选字段不视为 breaking change
3. event payload 只能追加兼容字段，不能随意改字段语义

---

## 5. Contract Taxonomy

跨边界契约只允许分成五类：

1. Command
2. Query
3. Event
4. Snapshot
5. Read Model

说明：

1. Command 表示改变后端权威状态或启动长任务的意图。
2. Query 表示获取某个稳定视图，不产生副作用。
3. Event 表示已经发生的事实广播。
4. Snapshot 表示长任务或运行态的周期性聚合视图。
5. Read Model 表示前端页面直接消费的数据模型。

禁止额外发明 `ManagerMessage`、`ShellPayload`、`MiscDto` 这种无边界命名。

---

## 6. Module Contract Catalog

本节先覆盖当前最核心的三条链路：Fab、Downloads、Engines。

### 6.1 Fab Module

#### Commands

- `fab.inventory.sync.start`
- `fab.inventory.sync.cancel`
- `fab.asset.media.enrich.retry`

#### Queries

- `fab.inventory.list`
- `fab.inventory.facets`
- `fab.inventory.sync.status`
- `fab.asset.details.get`

#### Events

- `fab.inventory.changed`
- `fab.inventory.sync.completed`
- `fab.asset.media.changed`

#### Read Models

- `FabInventoryListItemDto`
- `FabInventoryFacetDto`
- `FabInventorySyncStatusDto`
- `FabAssetDetailsDto`

### 6.2 Downloads Module

#### Commands

- `downloads.job.start`
- `downloads.job.pause`
- `downloads.job.resume`
- `downloads.job.cancel`
- `downloads.policy.update`

#### Queries

- `downloads.job.list`
- `downloads.job.get`
- `downloads.policy.get`

#### Events

- `downloads.job.completed`
- `downloads.job.failed`
- `downloads.job.canceled`

#### Snapshots / Read Models

- `JobSnapshotDto`
- `DownloadJobListItemDto`
- `DownloadPolicyDto`

### 6.3 Engines Module

#### Commands

- `engines.verify.start`
- `engines.verify.cancel`
- `engines.repair.plan.generate`
- `engines.repair.confirm`
- `engines.repair.cancel`

#### Queries

- `engines.installation.list`
- `engines.installation.health.get`
- `engines.repair.plan.get`
- `engines.version.list`

#### Events

- `engines.verification.completed`
- `engines.installation.health_changed`
- `engines.repair.plan_ready`
- `engines.repair.completed`

#### Read Models

- `EngineInstallationListItemDto`
- `EngineHealthSummaryDto`
- `VerificationDiffSummaryDto`
- `RepairPlanSummaryDto`

### 6.4 Installations Module

虽然当前专项重点不在安装域，但它必须为 Engines / Downloads 暴露稳定合同：

#### Commands

- `installations.job.start`
- `installations.job.cancel`
- `installations.repair.apply`

#### Queries

- `installations.record.get`
- `installations.record.list`
- `installations.health.get`

#### Events

- `installations.job.completed`
- `installations.repair.completed`

---

## 7. Standard Command / Query Envelope

### 7.1 Command Result

统一建议：

```ts
type CommandResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: AppErrorDto };
```

### 7.2 Accepted Result for Long Jobs

启动长任务的 command 应优先返回：

```ts
type AcceptedJobDto = {
  accepted: true;
  jobId: string;
  module: string;
  kind: string;
  queuedAt: string;
};
```

说明：

1. “开始成功”不等于“任务完成”。
2. 前端收到 `jobId` 后，再转入 snapshot/query 路径。

### 7.3 Query Envelope

query 建议统一：

```ts
type QueryResult<T> =
  | { ok: true; data: T; asOf?: string }
  | { ok: false; error: AppErrorDto };
```

---

## 8. Unified Error Model

### 8.1 AppErrorDto

前端可见的错误 envelope 只允许包含：

- `code`
- `message`
- `retryable`
- `severity`
- `correlationId`
- `details?` 仅限 UI 友好字段

### 8.2 Error Code Domains

建议前缀：

- `AUTH_*`
- `FAB_*`
- `DL_*`
- `INS_*`
- `ENG_*`
- `CFG_*`
- `SYS_*`

### 8.3 Severity

推荐稳定枚举：

- `info`
- `warning`
- `error`
- `fatal`

### 8.4 Retryability

`retryable` 只表达“用户或系统是否可重试”，不表达具体重试策略。

真正的重试控制仍由后端 runtime 决定。

---

## 9. Standard Long-job Snapshot Schema

所有长任务统一使用 `JobSnapshotDto` 基础模型。

### 9.1 Required Fields

- `jobId`
- `module`
- `kind`
- `uiState`
- `title`
- `progress`
- `throughput`
- `etaSeconds`
- `retryable`
- `error`
- `updatedAt`

### 9.2 `progress` Shape

建议统一字段：

- `phase`
- `current`
- `total`
- `percentage`
- `unit`

说明：

1. 下载可以是 bytes / segments。
2. 验证可以是 files / components。
3. 库存同步可以是 pages / items。

### 9.3 UI State Rules

所有模块都只能投影到稳定 UI 状态集合，例如：

- `Queued`
- `Running`
- `Paused`
- `AwaitingUser`
- `Completed`
- `Failed`
- `Canceled`

模块可以在 read model 中附带更具体的 `phase`，但不能自行发明不兼容的主状态。

### 9.4 Frequency Rules

前端只能收到聚合 snapshot，不能收到底层高频噪声：

1. 默认 250ms - 500ms 一次
2. 大量 segment / file 级事件必须先在后端聚合
3. 任务完成、失败、等待用户确认等关键状态可即时推送

---

## 10. Diff Summary and Repair Plan Schema

这是当前跨 Downloads / Engines / Installations 最容易漂移的部分，必须统一。

### 10.1 `VerificationDiffSummaryDto`

建议字段：

- `subjectId`
- `subjectType`
- `healthy`
- `missingFileCount`
- `corruptedFileCount`
- `missingComponentCount`
- `unexpectedFileCount`
- `estimatedRepairBytes`
- `topIssues[]`

### 10.2 `VerificationDiffItemDto`

建议字段：

- `id`
- `type`
- `componentId`
- `relativePath`
- `severity`
- `repairStrategy`
- `message`

### 10.3 `RepairPlanSummaryDto`

建议字段：

- `planId`
- `subjectId`
- `actionCount`
- `estimatedDownloadBytes`
- `estimatedTouchedFiles`
- `requiresUserConfirmation`
- `actions[]`
- `generatedAt`

### 10.4 `RepairActionDto`

建议稳定枚举：

- `ReDownloadFile`
- `RestoreComponent`
- `DeleteUnexpectedFile`
- `IgnoreOptionalIssue`
- `ManualActionRequired`

禁止前端自己根据 diff item 临时推导 repair action。

---

## 11. State Ownership Matrix

### 11.1 State Categories

状态固定分成五类：

| State Type | Owner | Example |
|-----------|-------|---------|
| Persistent state | Backend | 下载 checkpoint、安装记录、引擎健康摘要、设置 |
| Runtime business state | Backend | 活跃 job、worker 状态、队列、锁、network flags |
| Query cache | Frontend | TanStack Query 中的分页数据和详情缓存 |
| View state | Frontend | tab、筛选器草稿、滚动位置、输入框文本 |
| Global shell UI state | Frontend + backend fact bridge | 主题、全局通知、活跃任务计数 |

### 11.2 Write Permissions

| State Type | Frontend can write | Backend can write |
|-----------|--------------------|-------------------|
| Persistent state | No | Yes |
| Runtime business state | No | Yes |
| Query cache | Derived only | Yes, as source provider |
| View state | Yes | No |
| Global shell UI state | UI-only subset | Fact-only subset |

### 11.3 Guardrails

1. Query cache 失效后可以重建，所以它不能承载独占业务信息。
2. View state 不能被后端直接改写。
3. 后端广播事实时，只能更新 shell 的 fact slice，例如活跃任务计数、登录状态、全局错误横幅。
4. 前端 UI 行为状态，例如 modal 开关、侧边栏宽度，不得写回后端。

---

## 12. Frontend Read Model Catalog

前端页面只允许消费 read model，不允许直接消费 provider DTO 或 domain entity。

### 12.1 Fab Inventory Page

列表项建议只包含：

- `assetId`
- `title`
- `sellerName`
- `category`
- `ownedAt`
- `thumbnailState`
- `thumbnailUrl`
- `installState`
- `badges`

### 12.2 Downloads Page

列表项建议只包含：

- `jobId`
- `title`
- `kind`
- `uiState`
- `progressLabel`
- `percentage`
- `throughputLabel`
- `etaLabel`
- `retryable`

### 12.3 Engines Page

安装引擎列表项建议只包含：

- `engineInstallationId`
- `versionLabel`
- `installPath`
- `healthState`
- `lastVerifiedAt`
- `needsRepair`
- `estimatedRepairBytes`

### 12.4 Rule

如果页面必须先拿原始 DTO 再二次拼装很多字段，优先回改 read model 设计，而不是让前端越来越像后端。

---

## 13. Query Cache Strategy

### 13.1 What Frontend May Cache

允许缓存：

1. 分页列表结果
2. 详情页结果
3. facets / summary 查询结果
4. 长任务 snapshot 的最近一次稳定值

### 13.2 What Frontend Must Not Cache as Truth

禁止把以下内容作为前端权威状态：

1. 下载 checkpoint
2. 引擎 verification diff 明细的唯一版本
3. 安装记录真相
4. 当前任务真实可恢复性

### 13.3 Invalidation Triggers

建议由 event 触发 query invalidation：

- `fab.inventory.changed` -> invalidate inventory list/facets
- `downloads.job.completed` -> invalidate downloads list/history
- `engines.installation.health_changed` -> invalidate engine health summaries

---

## 14. Event Consumption Rules

前端消费 event 的唯一合法用途：

1. 让 query cache 失效或局部刷新
2. 更新 shell fact slice
3. 触发 toast / notification

前端不得用 event 流自己重建一整套业务真相状态树。

后端消费 event 的规则：

1. event 只表达事实，不表达命令
2. 跨模块动作需要显式 orchestrator 时，优先由应用层编排，不靠“听事件后偷偷做大事”蔓延

---

## 15. Implementation Guidance

### 15.1 Rust Side

建议分层：

1. `contracts` crate 或等价模块定义公开 DTO
2. `commands` 模块定义 command input / result
3. `queries` 模块定义 query input / read model
4. `events` 模块定义 event payload

### 15.2 Frontend Side

建议分层：

1. `generated/contracts` 存生成的 TS 类型
2. `services/ipc` 只做 transport binding
3. `queries/*` 封装 TanStack Query hooks
4. `stores/*` 只放 view state 和 shell UI state

### 15.3 Review Checklist

实现前每个新契约至少过四个检查：

1. 它是 command 还是 query，是否混了副作用？
2. 它返回的是 read model 还是内部 entity？
3. 它有没有和已有长任务 schema / error envelope 重复发明？
4. 它会不会让前端形成第二套业务真相？

---

## 16. Acceptance Criteria

满足以下条件，才算这份专项稿达标：

1. 核心模块的 command / query / event 目录已经有统一命名和最小覆盖面。
2. 长任务 snapshot、错误 envelope、diff summary、repair plan schema 已统一。
3. 状态 ownership 和写权限矩阵已明确，前后端边界清晰。
4. 页面消费 read model，而不是 provider DTO 或内部运行态。
5. 新模块加入时可以沿用同一套契约分类，不必重新发明协议。

---

## 17. Anti-patterns

以下做法视为违规：

1. 前端手写和 Rust 重复但不受控的 DTO。
2. 用 query 触发副作用，或者用 command 伪装读取。
3. 把 provider 原始 JSON 直接透给页面。
4. 用 event 流在前端重建完整业务状态树。
5. 每个模块定义自己独特的 job snapshot 和 error payload 结构。
