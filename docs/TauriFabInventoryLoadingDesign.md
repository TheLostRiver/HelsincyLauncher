# Tauri Fab Inventory Loading Design

> Status: local draft v1
> Date: 2026-05-02
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Focus: Fab owned inventory loading, local projection, incremental sync, summary-first rendering

---

## 1. Purpose

本文档把蓝图中的 `14.1 Fab Inventory Loading Strategy` 细化为可执行的专项设计。

目标不是单纯让列表“快一点”，而是把 Fab 库存加载定义为一条稳定、可扩展、可观测的后端读模型链路：

1. 页面打开时先返回本地稳态 summary
2. 后台再做增量同步和校准
3. 缩略图和预览补全不阻塞主列表可见
4. 搜索、筛选、排序优先命中本地 projection，而不是实时拼远程结果

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Fast first paint | 库存页先显示本地 summary，而不是空白等远程 |
| Incremental sync | 后台同步按 cursor / watermark / page continuation 增量推进 |
| Startup prewarm | 已恢复登录会话时，应用启动后自动预热库存，不必等用户首次点开库存页 |
| Stable contracts | 前端只看分页 summary/read model，不消费提供方原始 DTO |
| Media decoupling | 缩略图、预览元数据、主列表数据分开管线 |
| Restart safety | 中断后能从本地 cursor 和 projection 状态恢复 |
| Provider isolation | Epic/Fab 数据源差异只留在 adapter 层 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 打开页面时实时全量拉远程库存 | 启动慢、卡顿、不可预测 |
| 前端直接拼装 provider 原始数据 | 会把 provider 差异渗到 UI |
| 页面一卸载就把库存同步和缓存一起清空 | 用户来回切页时会重复回到冷启动 |
| 在列表渲染路径里做 WebView / HTML probe | 会把媒体补全成本打到首屏链路上 |
| 用一个巨型 service 负责列表、同步、媒体、详情 | 会形成新的 Fab God Module |

---

## 3. Module Ownership

| Concern | Owner |
|---------|-------|
| Fab 库存同步命令 | `module-fab` |
| Provider API / fallback adapter | `adapter-provider-fab-*` |
| Owned inventory projection | `module-fab` |
| 缩略图缓存与媒体元数据缓存 | `module-media-cache` |
| 列表页 query cache | Frontend |
| 列表页筛选、滚动位置、列宽等视图状态 | Frontend |

固定边界：

1. `module-fab` 负责库存事实、同步生命周期和列表 read model。
2. `module-media-cache` 负责图片、预览元数据、etag、失效策略，不反向拥有列表业务语义。
3. 前端只保存 query cache 和 view state，不自己造第三套库存真相。

---

## 4. Read Model First Architecture

Fab 库存页不直接读取 provider DTO，而是读取本地 projection。

建议最少维护以下三类表或等价存储：

### 4.1 `fab_owned_asset_projection`

主列表 projection，面向库存页 summary 查询。

建议字段：

- `asset_id`
- `provider_asset_key`
- `title`
- `seller_name`
- `category_key`
- `subcategory_key`
- `owned_at`
- `updated_at_remote`
- `last_seen_sync_at`
- `thumbnail_ref`
- `preview_hint_ref`
- `install_state`
- `engine_support_summary`
- `search_vector` 或等价索引字段
- `sort_title_normalized`
- `sort_owned_at`

### 4.2 `fab_sync_cursor`

记录同步游标与同步租约，避免每次从头扫。

建议字段：

- `provider_id`
- `account_id`
- `cursor_kind`
- `cursor_value`
- `watermark`
- `last_success_at`
- `last_attempt_at`
- `status`
- `error_code`

### 4.3 `fab_media_metadata_projection`

只存列表和详情真正需要的轻量媒体元数据。

建议字段：

- `asset_id`
- `thumbnail_url`
- `thumbnail_cache_key`
- `preview_locator_kind`
- `preview_locator_value`
- `preview_resolution_status`
- `preview_resolved_at`
- `etag`

说明：

1. 主列表 projection 不依赖媒体补全完成。
2. `thumbnail_ref` 和 `preview_hint_ref` 只是引用，不把媒体细节塞进主列表表结构。
3. 若 provider 缺图，列表直接展示稳定占位态，不让媒体补全阻塞主链路。

---

## 5. End-to-End Load Flow

### 5.1 已登录启动后的库存预热

```text
App startup
  -> restore auth session
  -> if authenticated and fab capability enabled
  -> schedule `fab.inventory.sync.start(reason=startup_prewarm)` in Stage 3
  -> load last local projection head pages
  -> refresh cursor incrementally in background
```

规则：

1. 这条预热链只在“已恢复登录会话 + Fab 能力可用”时启动。
2. 它属于启动 Stage 3 的后台预热任务，不得阻塞 shell 首次可交互。
3. 预热优先做“projection 头部页 + 增量 cursor 校准”，而不是启动即全量重扫。
4. 如果用户之后进入 Fab 库存页，应直接复用已经存在的 projection、query cache 和同步任务快照，而不是重新开一条平行同步。

### 5.2 页面首次打开

```text
Frontend route enter
  -> query `fab.inventory.list`
  -> backend reads local projection
  -> return paged summaries immediately
  -> frontend renders virtualized list
  -> backend schedules background freshness check
```

规则：

1. 列表 query 绝不等待远程同步完成后才返回。
2. 若本地已有数据，优先返回“可能略旧但稳定”的 summary。
3. 若本地为空，返回空列表 + `cold_start_syncing=true`，并立即发起 bootstrap sync。

### 5.3 后台增量同步

```text
Background sync trigger
  -> acquire sync lease
  -> load last cursor
  -> fetch next provider page
  -> normalize provider DTO
  -> upsert projection rows
  -> update sync cursor
  -> publish inventory changed event
```

同步完成后不把整个页面强制重刷，而是：

1. 推送轻量事件 `fab.inventory.changed`
2. 前端按当前 query key 选择 `invalidate` 或增量合并
3. 只刷新受影响页，不全量重建页面状态

### 5.4 详情页打开

详情扩展字段单独按需读取，不把详情字段塞进列表 summary：

```text
open asset details
  -> query `fab.asset.details(assetId)`
  -> backend joins detail projection / provider on-demand fetch
  -> cache normalized detail snapshot
```

---

## 6. Sync Modes

### 6.1 Bootstrap Sync

适用场景：

1. 首次登录某账号
2. 本地 projection 不存在
3. projection schema 升级且需要重建

策略：

1. 分页拉取并分页落盘
2. 每页落盘后立即可见，不要求全量完成后再开放读取
3. 每页结束更新 cursor 与 `last_success_at`

### 6.2 Incremental Sync

适用场景：

1. 页面打开后的后台校准
2. 定时刷新
3. 用户手动刷新

策略：

1. 优先使用 provider cursor / continuation token
2. 若 provider 只支持时间戳增量，则用 watermark 模式
3. 若 provider 缺乏可靠增量语义，使用分页扫描 + `last_seen_sync_at` 去重，而不是直接删表重建

### 6.3 Repair Sync

适用场景：

1. projection 校验失败
2. cursor 状态不可信
3. provider 返回结构变化导致部分标准化失败

策略：

1. 标记本次 repair 原因
2. 对受影响账号或分区做局部 rebuild
3. 保留旧 projection 直到新 projection 达到可切换阈值

### 6.4 Route Return and Session Warm Cache

适用场景：

1. 用户离开 Fab 库存页后切到其他页面
2. 用户短时间内再切回 Fab 库存页
3. 后台库存同步仍在进行或刚完成

策略：

1. 离开页面只销毁纯 view state，不销毁后端 projection。
2. 前端 query cache 不因 route unmount 立即清空，必须保留一个明确的 session retention 窗口。
3. 正在运行的 inventory sync job 不因页面卸载自动取消，除非用户显式取消或全局策略要求停机。
4. 用户返回库存页时，优先复用最近一次 query cache 和本地 projection，再按 freshness 策略决定是否做局部刷新。
5. route return 的目标是“秒开最近稳态 + 后台校准”，而不是回到 `cold_start_syncing=true`。

---

## 7. IPC Contracts

### 7.1 Commands

```text
fab.inventory.sync.start({ accountId, reason, scope })
fab.inventory.sync.cancel({ jobId })
fab.inventory.cache.rebuild({ accountId, reason })
```

### 7.2 Queries

```text
fab.inventory.list({ page, pageSize, sort, filters, search })
fab.inventory.facets({ filters, search })
fab.inventory.sync.status({ accountId })
fab.asset.details({ assetId })
```

### 7.3 Events / Snapshots

```text
fab.inventory.changed
fab.inventory.sync.snapshot
fab.asset.media.changed
```

分页 query 返回的 summary 建议字段：

- `assetId`
- `title`
- `sellerName`
- `category`
- `ownedAt`
- `thumbnailState`
- `thumbnailUrl`
- `installState`
- `engineSupportSummary`
- `badges`

禁止直接把 provider 原始 JSON 透给前端列表。

---

## 8. Search / Sort / Filter Strategy

### 8.1 Search

1. 本地搜索优先命中 projection 索引。
2. 搜索输入采用前端 debounce，但搜索执行和排名逻辑以后端为准。
3. 若后端支持 SQLite FTS，则优先构建 FTS 表；否则维护归一化 token 索引。

### 8.2 Sort

允许的稳定排序键建议限制为：

- `owned_at_desc`
- `owned_at_asc`
- `title_asc`
- `title_desc`
- `updated_remote_desc`

排序字段必须有对应索引或可接受的查询计划，禁止临时全表排序。

### 8.3 Filter

筛选维度建议只暴露稳定字段：

- 类别
- 是否已安装
- 引擎支持范围
- 是否有缩略图

Facet 统计单独查询，避免把 facet 计算塞进主列表 SQL。

---

## 9. Media and Thumbnail Rules

媒体是附属链路，不是主链路。

固定规则：

1. 列表 summary 先返回 `thumbnailState`，再决定显示真实图或占位图。
2. `thumbnailState` 至少区分：`Ready / Missing / Deferred / Failed`。
3. 列表渲染路径不触发高成本预览解析。
4. 若后续仍需要 provider-specific 预览恢复，只能在后台 enrichment job 或详情链路里做。
5. 相同资源的媒体解析结果必须可缓存，不能在滚动过程中反复求值。

推荐后台 enrichment job：

```text
inventory row inserted/updated
  -> enqueue low-priority media enrichment
  -> resolve thumbnail metadata if cheap
  -> update media projection
  -> publish `fab.asset.media.changed`
```

该 job 必须是低优先级、可取消、可批处理的，不得与主列表抢首屏资源。

---

## 10. Frontend Integration Rules

前端推荐组合：

1. React route loader 或页面 effect 触发 query
2. TanStack Query 存 query cache
3. 虚拟化列表组件只消费 summary DTO
4. Zustand 只保存纯视图状态，例如筛选器草稿、滚动位置、布局模式

### 10.1 Route Switch and Cache Retention Rules

前端必须把“页面切换”与“库存真相失效”区分开：

1. route unmount 只释放组件实例和纯页面态，不自动清空 inventory query cache。
2. inventory query key 必须稳定，避免因为页面重挂载就生成另一套缓存分区。
3. query cache 的 `stale` 与 `garbage collect` 策略必须显式配置，不能依赖默认值碰运气。
4. 页面重新进入时，如果 query cache 命中且本地 projection 可读，应先渲染缓存结果，再决定是否后台刷新。
5. 前端不得因为“用户切过一次页面”就再次触发整页冷启动骨架和全量远程加载。

禁止事项：

1. 前端自己保留一份可变 inventory map 当业务真相
2. 前端在列表组件里直接串发补图请求风暴
3. 远程刷新后重置滚动位置和筛选状态
4. 用页面生命周期直接绑定后台 sync job 的生死

---

## 11. Failure Handling

| Failure | Expected handling |
|---------|-------------------|
| Provider API 超时 | 保留旧 projection，sync job 标记失败，可重试 |
| Provider challenge / fallback 切换 | 仅 adapter 层处理，列表 query 继续读本地 projection |
| 媒体解析失败 | `thumbnailState=Failed`，不影响主列表分页 |
| Cursor 损坏 | 进入 repair sync，而不是清空前端页面 |
| Schema 升级 | 后台 rebuild projection，旧 projection 未替换前继续可读 |

---

## 12. Observability

建议至少记录以下指标：

- `fab_inventory_query_latency_ms`
- `fab_inventory_projection_row_count`
- `fab_inventory_sync_page_latency_ms`
- `fab_inventory_sync_items_upserted`
- `fab_inventory_sync_cursor_lag_seconds`
- `fab_media_enrichment_queue_depth`
- `fab_media_ready_ratio`

建议日志字段：

- `accountId`
- `provider`
- `cursorKind`
- `pageSize`
- `itemsReceived`
- `itemsChanged`
- `reason`
- `durationMs`

---

## 13. Acceptance Criteria

满足以下条件，才算这条链路设计达标：

1. 打开库存页时可以先返回本地分页 summary，而不是等待远程全量完成。
2. 已恢复登录会话时，应用启动后可以在后台自动预热 Fab 库存。
3. 用户离开库存页再返回时，能够复用 projection 和 query cache，而不是重新回到冷启动。
4. 搜索、排序、筛选都能在本地 projection 上完成主路径查询。
5. 媒体补全失败不会阻塞主列表可见。
6. 同步过程支持 cursor 恢复、route return 复用和 repair sync。
7. 前端不会形成第二套库存业务真相。

---

## 14. Anti-patterns

以下做法视为违规：

1. 把 Fab provider DTO 直接暴露给前端组件。
2. 列表页每次打开都实时全量扫远程库存。
3. 列表滚动进入视口时再触发高成本预览解析。
4. 媒体缓存层反向拥有库存业务状态。
5. 用一个 `FabManager` 同时承担同步、查询、媒体补全、详情拼装和 UI 事件。
6. 把后台库存同步的生命周期绑死在页面是否挂载上。
