# Tauri Download Runtime Design

> Status: local draft v1
> Date: 2026-05-02
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Focus: scheduler, bounded concurrency, segmented resume, checkpointing, backpressure

---

## 1. Purpose

本文档把蓝图中的 `12. Download and Long Task Architecture` 与 `14.2 Multi-thread Download Strategy` 细化为专项设计。

目标是把“支持 1-128 线程下载、断点续传、多任务并行”落成一条可实现的 Rust 后端运行时模型，而不是停留在产品口号层。

这里的核心约束是：

1. 前端只表达用户意图和展示聚合状态
2. 后端统一拥有任务生命周期、checkpoint、资源预算和故障恢复
3. `1-128` 表达的是用户可配置的并发槽位，不是 `1-128` 个裸 OS 线程

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Configurable concurrency | 用户可配置 `1-128` 并发槽位 |
| Resume safety | 暂停、崩溃、重启后可恢复 |
| Bounded runtime | 网络、内存、磁盘写入都有上界 |
| Provider isolation | Manifest/CDN 差异留在 adapter 层 |
| Aggregated UI model | 前端只收到收敛后的任务快照 |
| Repair-friendly | 下载产物可被安装/验证/修复流程复用 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 为每个下载槽位创建固定线程 | 资源浪费且不必要 |
| 把安装、解压、验证直接塞进下载 worker | 会打穿模块边界 |
| 让前端自己拼 checkpoint / resume 逻辑 | 违反前后端职责分离 |
| 让一个 worker 从拉 manifest 到写盘全部包办 | 会演化成 God Worker |

---

## 3. Runtime Ownership

| Concern | Owner |
|---------|-------|
| 下载命令入口 | `module-downloads` facade |
| Manifest 拉取与标准化 | provider adapter |
| 分段规划 | planner |
| 段调度与预算控制 | scheduler |
| HTTP 拉取 | fetcher |
| 分段落盘 | writer |
| 校验 / hash / 长度检查 | verifier |
| 断点持久化 | checkpoint repository |
| 安装串接 | `module-installations` 或显式 orchestrator |

固定规则：

1. Downloads 只拥有“获取可安装内容到本地 staging 区”的职责。
2. Installations 拥有“如何落到最终安装目录”的职责。
3. Engines / Installations 的 repair 流程只能复用 Downloads 的公开 command facade，不能直接改 scheduler 内部状态。

---

## 4. Core Components

```text
DownloadCommandFacade
  -> DownloadOrchestrator
     -> ManifestResolver
     -> SegmentPlanner
     -> DownloadScheduler
        -> SegmentFetcherPool
        -> SegmentWriter
        -> SegmentVerifier
     -> CheckpointRepository
     -> DownloadReadModelProjector
```

### 4.1 `DownloadCommandFacade`

对外提供：

- `start_download`
- `pause_download`
- `resume_download`
- `cancel_download`
- `set_download_policy`

只做参数校验、鉴权、路由到 orchestrator，不承接业务核心。

### 4.2 `DownloadOrchestrator`

负责：

1. 为 job 分配稳定 `job_id`
2. 创建或恢复 checkpoint
3. 驱动状态机迁移
4. 决定是否进入安装或修复后续链路

### 4.3 `SegmentPlanner`

把 provider manifest 转为统一段计划：

- 产物文件
- 分段边界
- 预估大小
- 校验信息
- 优先级

### 4.4 `DownloadScheduler`

负责：

1. 管理全局并发槽位
2. 维护多 job 公平性
3. 执行 per-job、per-host、per-disk budget
4. 应用 backpressure 和安全降载

### 4.5 `SegmentFetcherPool`

通过 Tokio task 执行 HTTP range / object fetch，不暴露 provider 特殊细节给上层。

### 4.6 `SegmentWriter`

负责 staging 文件写入、预分配、临时分片落盘、原子 rename。

### 4.7 `SegmentVerifier`

负责分段 hash、文件长度和最终产物一致性校验。

---

## 5. Internal Job Model

### 5.1 Job State

建议内部细粒度状态：

- `Queued`
- `Preparing`
- `FetchingManifest`
- `PlanningSegments`
- `Downloading`
- `Pausing`
- `Paused`
- `Verifying`
- `Completed`
- `Failed`
- `Canceled`

### 5.2 UI State Projection

对前端公开的状态收敛为：

- `Queued`
- `Downloading`
- `Paused`
- `Installing`
- `Completed`
- `Failed`
- `Canceled`

禁止把内部状态机细节直接泄露给页面组件。

### 5.3 Stable Identity

每个下载 job 至少包含：

- `job_id`
- `download_target_id`
- `provider`
- `account_id`
- `content_version`
- `destination_policy`
- `created_at`
- `policy_snapshot`

---

## 6. Concurrency and Budget Model

### 6.1 User-facing Setting

前端只暴露：

- `downloadConcurrencySlots: 1..128`

可配说明必须明确写成“并发下载槽位”，避免误导成真实线程数。

### 6.2 Global Budget

后端把用户设置转为全局槽位预算：

```text
global_slots = clamp(user_setting, 1, 128)
```

但实际调度还要再受三类约束：

1. `per_job_slot_cap`
2. `per_host_slot_cap`
3. `writer_backpressure_cap`

所以“用户设为 128”不代表任何时刻都一定同时跑满 128 个活动 segment。

### 6.3 Fair Scheduling

推荐调度策略：

1. job 级队列先做 weighted fair scheduling
2. job 内 segment 再做优先级调度
3. 小文件或收尾阶段可获得适度优先权，避免大任务长期霸占资源

推荐优先级因子：

- 用户主动前台任务
- 修复任务
- 普通后台预下载任务
- 低优先级缓存预热任务

### 6.4 Dynamic Backpressure

必须至少检测以下信号：

- writer 队列堆积
- hash/verifier 堵塞
- 磁盘可用空间逼近阈值
- CDN / host 返回限流
- 运行时内存占用超过安全阈值

触发时允许：

1. 暂时减少活跃 segment 数
2. 放缓新 segment 派发频率
3. 降低单 job 同时活跃度

这不算违背用户设置，而是资源保护策略。

---

## 7. Segmentation Strategy

### 7.1 Logical Segment

建议统一使用“segment”抽象，而不是暴露 provider 原始 chunk 语义。

一个 segment 至少包含：

- `segment_id`
- `file_id`
- `offset`
- `length`
- `source_locator`
- `expected_hash`
- `write_target`

### 7.2 Segment Sizing

默认段大小不应写死在 UI 设置里，而由 planner 决定：

1. 小文件可直接单段
2. 大文件按阈值切段
3. provider manifest 若已有天然 chunk，则优先复用其边界

### 7.3 Sparse and Partial Writes

建议优先使用 staging 区分段写入：

1. 大文件先建立目标临时文件
2. segment 按 offset 写入
3. 完整校验通过后再原子切换为完成态

避免让半成品直接污染最终安装目录。

---

## 8. Checkpoint and Resume Design

### 8.1 Persistence Model

建议至少维护：

#### `download_job_checkpoint`

- `job_id`
- `state`
- `manifest_ref`
- `policy_snapshot_json`
- `updated_at`
- `resume_token`

#### `download_segment_checkpoint`

- `job_id`
- `segment_id`
- `file_id`
- `offset`
- `length`
- `downloaded_bytes`
- `status`
- `partial_path`
- `etag`
- `hash_state_ref`

### 8.2 Save Rules

必须在以下时机持久化 checkpoint：

1. job 创建成功后
2. manifest / segment plan 确认后
3. segment 完成后
4. pause 完成前
5. 固定时间窗口批量刷盘
6. 进入 failed / canceled / completed 终态前

### 8.3 Resume Flow

```text
resume request
  -> load job checkpoint
  -> validate staging artifacts still exist
  -> reload or reconstruct manifest plan
  -> mark completed segments as sealed
  -> enqueue remaining segments only
  -> continue scheduler loop
```

恢复原则：

1. 已完成 segment 不重下。
2. 部分完成 segment 若支持 range resume，则从中断位置继续。
3. 若 provider 不支持安全续传，则只重下受影响 segment，不回滚整个 job。

### 8.4 Crash Recovery

应用重启时：

1. 扫描未终态 job checkpoint
2. 做 staging 文件一致性检查
3. 把可恢复任务标成 `Paused` 或 `NeedsAttention`
4. 由策略决定自动恢复还是等待用户确认

---

## 9. File Writing and Integrity

### 9.1 Staging First

所有下载内容先落 staging 区：

- `.downloads/staging/{jobId}/...`

完成并校验通过后，才交给安装域或产物归档域处理。

### 9.2 Verification Stages

最少三层校验：

1. segment 完成时校验段级 hash 或长度
2. 文件完成时校验文件级 hash / size
3. job 完成时校验 manifest 级完整性

### 9.3 Cleanup Rules

1. Completed 后保留最小必要元数据，清理临时碎片
2. Failed / Canceled 默认保留可恢复 staging 和 checkpoint
3. 用户显式清理后才彻底删除恢复资料

---

## 10. IPC and Frontend Contracts

### 10.1 Commands

```text
downloads.start({ targetId, kind, installIntent, priority })
downloads.pause({ jobId })
downloads.resume({ jobId })
downloads.cancel({ jobId })
downloads.configure({ concurrencySlots, bandwidthLimit, autoResume })
```

### 10.2 Queries

```text
downloads.list({ state, page })
downloads.get({ jobId })
downloads.policy.get()
```

### 10.3 Job Snapshot

必须兼容蓝图中的标准 long-job 协议，建议字段：

- `jobId`
- `module`
- `kind`
- `uiState`
- `title`
- `progress.completedBytes`
- `progress.totalBytes`
- `progress.completedSegments`
- `progress.totalSegments`
- `throughput.bytesPerSec`
- `etaSeconds`
- `retryable`
- `error`
- `policy.concurrencySlots`
- `updatedAt`

前端只能消费聚合快照，不能订阅每个 segment 的高频明细流。

---

## 11. Interaction with Installations and Repair

### 11.1 Download Completed Event

Downloads 只发布事实：

```text
downloads.completed(jobId, artifactRef)
```

是否自动进入安装，由安装域或显式 orchestrator 决定。

### 11.2 Engine Repair

引擎验证/修复场景下：

1. `Engines` 产出 repair plan
2. `Installations` 决定需要哪些 payload
3. `Downloads` 负责获取缺件
4. 下载完成后再回到 `Installations` 执行 repair

Downloads 不自己推断引擎业务语义。

---

## 12. Failure Classification

建议统一分成以下类别：

- `DL_NETWORK_TRANSIENT`
- `DL_NETWORK_FATAL`
- `DL_PROVIDER_MANIFEST_INVALID`
- `DL_DISK_NO_SPACE`
- `DL_WRITE_FAILED`
- `DL_VERIFY_FAILED`
- `DL_POLICY_BLOCKED`

处理原则：

1. 可重试错误允许局部 segment 重试。
2. 不可重试错误终止 job，但保留恢复信息。
3. 校验失败优先重下受影响 segment，而不是盲目清空整个 job。

---

## 13. Observability

建议至少采集：

- `download_active_jobs`
- `download_active_segments`
- `download_scheduler_queue_depth`
- `download_writer_queue_depth`
- `download_checkpoint_flush_ms`
- `download_resume_success_rate`
- `download_segment_retry_count`
- `download_verify_failure_rate`

关键日志字段：

- `jobId`
- `targetId`
- `provider`
- `segmentId`
- `host`
- `state`
- `reason`
- `durationMs`
- `bytes`

---

## 14. Acceptance Criteria

满足以下条件，才算下载运行时设计达标：

1. 用户可配置 `1-128` 并发槽位，但后端仍保持有界调度。
2. 暂停、恢复、崩溃重启后都能基于 checkpoint 继续执行。
3. 前端只依赖统一 job snapshot，不关心 segment 内部实现。
4. 下载、安装、修复之间边界清晰，互不偷写对方内部状态。
5. 高并发时出现磁盘或网络背压，调度器会主动降载而不是打穿系统资源。

---

## 15. Anti-patterns

以下做法视为违规：

1. 把 `1-128` 直接实现成 `1-128` 个永久线程。
2. 让下载 worker 同时承担 manifest 解析、调度、落盘、安装、修复所有职责。
3. 前端自己缓存 segment 级运行态并尝试驱动 resume。
4. 失败后整任务一刀切重下，不区分 segment 级恢复。
5. 把下载调度器做成任何模块都能随意写入的全局共享可变对象。
