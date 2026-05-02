# Tauri Storage and Database Design

> Status: local draft v1
> Date: 2026-05-02
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Focus: SQLite vs PostgreSQL decision, storage layering, `rusqlite` recommendation, data placement rules

---

## 1. Purpose

本文档把新架构中的“存储到底怎么选、数据到底放哪”明确为专项设计，而不是停留在泛泛的技术栈描述。

它要解决四个容易漂移的问题：

1. 本地桌面应用的主数据库到底选 SQLite 还是 PostgreSQL
2. Rust 侧 SQLite 访问到底默认用 `rusqlite` 还是 `sqlx`
3. 哪些数据应该进数据库，哪些应该留在文件系统、内存或安全存储
4. 如何避免随着模块增加，又长出一个混杂的“万能存储层”

---

## 2. Decision Summary

本项目的默认持久化决策如下：

1. **主数据库使用 SQLite**
2. **不把 PostgreSQL 作为当前桌面架构的默认依赖**
3. **Rust 侧默认使用 `rusqlite` 作为 SQLite 驱动**
4. **用户配置默认走 JSON/配置系统，而不是强塞进数据库**
5. **认证凭据默认走系统安全存储，而不是 SQLite**
6. **大体积 manifest、下载 staging、缩略图缓存、日志默认走文件系统**

一句话总结：

**SQLite 是本地结构化事实存储，文件系统是大对象和缓存存储，内存 store 是运行时状态，安全存储负责凭据。**

---

## 3. Why SQLite, Not PostgreSQL

### 3.1 Why SQLite fits this architecture

当前系统本质上仍然是：

1. 单机桌面应用
2. 本地 Rust 后端为权威业务核心
3. 本地持有下载 checkpoint、安装记录、Fab projection、引擎健康摘要、任务恢复信息
4. 需要离线可用、零额外部署、零数据库服务依赖

这些特征都天然更适合嵌入式数据库，而不是独立数据库服务。

SQLite 的直接收益：

1. 无需安装和维护数据库服务
2. 易于备份、迁移、恢复
3. 支持事务、索引、WAL、FTS 等客户端真正需要的能力
4. 与本地文件系统和本地 job runtime 的边界自然贴合

### 3.2 Why PostgreSQL is not the default here

如果当前就引入 PostgreSQL，会平白增加：

1. 本地数据库服务生命周期管理
2. 安装、升级、端口、权限、恢复、失败诊断复杂度
3. 用户环境依赖和分发成本
4. “本来是本地桌面应用，却需要外部 DB 服务才能正常工作”的失败面

对当前目标来说，这些成本没有对应收益。

### 3.3 When PostgreSQL becomes reasonable

只有当架构边界变成“有真正独立的远程服务端”时，才值得重新评估 PostgreSQL，例如：

1. 多设备同步同一账号的下载/安装/库存状态
2. 团队或多用户共享索引与任务编排
3. 远程代理下载、服务端统一缓存 provider 数据
4. 云端审计、运营分析、远程控制面板

即便未来有这些能力，也应理解为：

**新增一个服务端存储层**，而不是“用 PostgreSQL 替换本地 SQLite”。

---

## 4. Why `rusqlite`, Not `sqlx`, as the Default

当前默认推荐 `rusqlite`，原因不是个人偏好，而是架构匹配度更高。

### 4.1 Core reasoning

1. SQLite 本身就是本地同步型数据库，不存在真正的网络异步 IO 场景。
2. 对 SQLite 做“看起来 async”的封装，不会改变它本质上仍是本地阻塞访问的事实。
3. 我们更需要的是**明确的存储执行边界**和**受控的读写调度**，而不是为了统一风格引入额外抽象。

### 4.2 Why `rusqlite` fits better initially

1. 更贴近 SQLite 原生能力，控制 PRAGMA、事务、备份、FTS、WAL 更直接。
2. 适合与“专用存储执行器 / blocking executor”模型结合。
3. 对本地桌面应用更透明，不会给团队造成“数据库天然是 async service”的错觉。
4. 更符合“SQLite 是本地嵌入式事实存储”的定位。

### 4.3 When `sqlx` may still be acceptable

如果未来出现以下条件，再评估 `sqlx`：

1. 团队强依赖 compile-time query checking 宏工作流
2. 需要在同一数据访问抽象上同时支持多种数据库方言
3. 已经形成稳定的 repository ports，不会因为换 driver 打穿架构

但这不是当前默认方案。

---

## 5. Storage Layering

所有存储严格分成五层：

| Layer | Role | Default Medium |
|------|------|----------------|
| Structured persistent facts | 可索引、可事务、可恢复的结构化业务数据 | SQLite |
| Large objects and caches | 大文件、原始 manifest、下载 staging、缩略图缓存 | File system |
| User-editable configuration | 下载目录、主题、并发槽位、开关策略 | JSON / config system |
| Secrets | Token、refresh token、敏感凭据 | OS secure storage |
| Runtime state | 活跃任务、队列、瞬时资源状态 | Memory stores |

固定规则：

1. 不把所有东西都塞进 SQLite。
2. 不把需要事务和索引的结构化事实丢给零散 JSON。
3. 不把凭据明文塞进数据库或普通配置文件。
4. 不把运行时业务真相长期只留在内存里。

---

## 6. What Goes into SQLite

SQLite 只存“结构化、可索引、可恢复、可事务”的本地事实。

### 6.1 Fab and Catalog

适合进 SQLite：

- `fab_owned_asset_projection`
- `fab_sync_cursor`
- `fab_media_metadata_projection`
- 搜索索引 / FTS 表
- facets 预计算摘要或等价投影

### 6.2 Downloads

适合进 SQLite：

- `download_job_summary`
- `download_job_checkpoint`
- `download_segment_checkpoint`
- `download_policy_snapshot`
- 下载历史摘要

### 6.3 Installations

适合进 SQLite：

- `installation_record`
- `installation_manifest_index`
- `installation_health_summary`
- repair 历史摘要

### 6.4 Engines

适合进 SQLite：

- `engine_installation_record`
- `engine_verification_summary`
- `engine_repair_plan_projection`
- `engine_component_index`

### 6.5 Shared Operational Metadata

适合进 SQLite：

- schema migrations 版本
- background job 恢复元数据
- 去重 token / cursor / watermark
- 关键索引构建状态

---

## 7. What Stays out of SQLite

### 7.1 File System Objects

以下内容默认不直接塞进 SQLite：

1. 原始大 manifest 文件
2. 下载 staging 文件和临时分片
3. 缩略图缓存文件
4. 安装包、repair payload、临时解压目录
5. 结构化日志文件

这些内容更适合：

- 文件系统路径 + SQLite 中的引用 / 索引摘要

### 7.2 User Configuration

以下内容默认放配置系统：

1. 主题
2. 默认下载目录
3. 引擎安装根目录策略
4. 下载并发槽位
5. 自动恢复、自动安装等开关

### 7.3 Secrets

以下内容默认不放 SQLite：

1. access token
2. refresh token
3. 任何需要系统级保护的敏感凭据

### 7.4 Runtime-only State

以下内容默认只放内存 runtime store：

1. 当前活跃 job 的瞬时进度
2. writer queue 深度
3. network availability
4. scheduler 当前预算

但其中需要恢复的关键断点摘要，必须定期回写 SQLite。

---

## 8. SQLite Access Model

### 8.1 Repository-only Access

模块不能直接随手拿连接执行 SQL。

固定规则：

1. 只通过 repository ports 访问 SQLite
2. SQL 和表结构收口在 `adapter-storage-sqlite` 或等价基础设施层
3. 模块看到的是业务语义方法，而不是裸 SQL

### 8.2 Execution Model

建议采用受控执行模型：

1. 单独的 storage executor / dedicated blocking runtime
2. 写事务串行化或显式有界并发
3. 读操作允许受控并发，但不允许无限制散打

### 8.3 Connection Strategy

建议至少区分：

1. 一个主写连接或等价写执行路径
2. 少量只读连接或受控读执行路径

---

## 9. SQLite Operational Settings

建议默认启用或评估以下设置：

1. `journal_mode = WAL`
2. `foreign_keys = ON`
3. `busy_timeout` 设为明确非零值
4. `synchronous = NORMAL` 作为默认平衡点
5. 视查询路径建立普通索引、复合索引和 FTS

---

## 10. Schema Design Rules

### 10.1 Projection-first

列表页和页面主路径优先围绕 projection 设计表，而不是围绕 provider 原始 DTO 设计表。

### 10.2 No Cross-module Table Coupling

模块间不能直接依赖彼此表结构。

### 10.3 Raw Data vs Indexed Data

如果某类原始数据很大但仍需保留，建议：

1. 文件系统存原文
2. SQLite 存索引、摘要、路径引用、版本戳

### 10.4 Migrations

必须有显式 schema migration 机制：

1. 启动时检查 schema version
2. 顺序执行 migration
3. migration 必须幂等或可恢复
4. 大 rebuild 需要与旧 projection 切换策略配合，避免直接把页面打空

---

## 11. Data Placement Matrix

| Data | Medium | Why |
|------|--------|-----|
| Fab inventory summaries | SQLite | 需要分页、搜索、索引 |
| Fab raw remote payload snapshots | File system + SQLite ref | 体积大，主路径不直接读 |
| Download checkpoints | SQLite | 崩溃恢复必需 |
| Download staging files | File system | 大对象、顺序写入 |
| Installation records | SQLite | 结构化查询和恢复 |
| Install manifests raw | File system + SQLite index | 体积可能大，但要保留摘要检索 |
| Engine verification summaries | SQLite | 页面稳定读模型 |
| Repair payloads | File system | 大对象 |
| User settings | JSON/config | 策略配置型 |
| Auth tokens | Secure storage | 敏感凭据 |
| Active queue depth / temp progress | Memory store | 高频运行态 |

---

## 12. Failure and Recovery Implications

### 12.1 SQLite Failure Strategy

如果 SQLite 文件损坏或 schema 不一致：

1. 优先保住用户配置和凭据，因为它们不在 SQLite
2. 可重建 projection 的数据优先走 repair / rebuild
3. 不允许因为 projection 损坏就把下载 staging 和安装文件直接一并清掉

### 12.2 File Cache Failure Strategy

如果缩略图缓存、manifest 缓存或日志目录损坏：

1. 可以局部重建
2. 不应影响 SQLite 中的核心结构化事实

### 12.3 Runtime Crash Strategy

应用崩溃后：

1. 高频瞬时进度可以少量丢失
2. checkpoint、安装记录、repair plan 等关键事实必须能从 SQLite 恢复

---

## 13. Recommended Initial Stack

当前建议的初始组合是：

1. **SQLite + `rusqlite`** 负责结构化本地事实
2. **配置系统 / JSON** 负责用户策略配置
3. **OS secure storage adapter** 负责凭据
4. **File system object stores** 负责 manifest、staging、thumbnail、logs
5. **Runtime stores** 负责瞬时业务状态

---

## 14. Acceptance Criteria

满足以下条件，才算这份专项稿达标：

1. 已明确 SQLite 是当前本地桌面架构的主数据库，PostgreSQL 不是默认方案。
2. 已明确 `rusqlite` 是当前默认 SQLite 驱动。
3. 已明确哪些数据进 SQLite，哪些进文件系统、配置系统、安全存储和内存 store。
4. 已明确模块不能直接耦合彼此表结构，必须通过 repository/contracts 交互。
5. 已明确 migration、恢复和缓存重建的基本边界。

---

## 15. Anti-patterns

以下做法视为违规：

1. 为了“显得专业”在本地桌面应用里默认引入 PostgreSQL。
2. 把 access token 或 refresh token 存进 SQLite。
3. 把大体积 manifest、下载分片、缩略图二进制直接硬塞进主查询表。
4. 模块直接访问别人的表而不是通过 repository/contracts。
5. 把所有状态都混成一个“万能数据库”或“万能配置文件”。