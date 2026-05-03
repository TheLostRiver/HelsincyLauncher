# Tauri Rewrite Architecture Blueprint

> Status: local draft v2
> Date: 2026-05-02
> Scope note: this blueprint is self-contained and intended to serve as a standalone architecture baseline for the new project.
> Storage note: this document is intentionally stored under `docs/` and is intended to be included in git commits.
> Companion local drafts: `docs/TauriFabInventoryLoadingDesign.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriEngineVerificationRepairDesign.md`, `docs/TauriIPCAndStateContractsDesign.md`, `docs/TauriStorageAndDatabaseDesign.md`, `docs/TauriRustTsSchemaDesign.md`, `docs/TauriRepositoryPortsAndAdapterDesign.md`, `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`, `docs/TauriFirstCrateApiDrafts.md`, `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriKernelJobsRuntimeDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`, `docs/TauriStartupPipelineDesign.md`, `docs/TauriErrorHandlingAndProjectionDesign.md`, `docs/TauriArchitecturePrinciplesDesign.md`, `docs/TauriSecurityCredentialsAndPermissionsDesign.md`, `docs/TauriDevelopmentEnvironmentBootstrapDesign.md`, `docs/TauriAIDevelopmentTransactionProtocolDesign.md`

---

## 1. Purpose

本蓝图用于定义下一代 Epic 启动器重写方案的总体架构。

新项目不再沿用当前 WinUI 3 + .NET 的宿主实现，而是采用：

- Tauri 2 作为桌面宿主
- Rust 作为后端核心运行时
- Web 前端作为展示层
- 前后端分离、接口优先、模块化、可扩展的工程结构

这次重写的核心目标不是“换一个 UI 技术栈”，而是彻底解决以下系统性问题：

1. 改模块 A 时不应连带破坏模块 C、D
2. 扩展某个模块时不应被迫修改大量无关代码
3. UI 不应承担下载、断点续传、安装、修复等后端职责
4. 平台能力不应渗透到业务核心，避免未来跨平台时再次重构核心逻辑

---

## 2. Architecture Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Cross-platform | Windows / macOS / Linux 共用同一业务核心与大部分前端代码 |
| High cohesion | 每个模块内部独立完成自己的业务闭环 |
| Low coupling | 模块之间只通过稳定契约通信，不共享内部实现与可变对象 |
| Interface-first | 依赖抽象端口与契约，而不是具体实现类 |
| Frontend/backend separation | 前端只做展示与交互，后端负责业务、状态机、IO、网络、持久化 |
| High extensibility | 新增模块、新增提供方、新增平台能力时，不要求回改无关模块内部 |
| High performance | 重任务全部后端化、异步化、可取消、可观察 |
| Local reasoning | 开发者或 AI 可以只读局部模块就理解和修改该模块 |

### 2.2 Explicit Non-goals

| Non-goal | Reason |
|----------|--------|
| 前端直接做下载/校验/安装 | 违反职责分离，且难以测试、难以恢复、难以跨平台落地 |
| Tauri command 里堆业务逻辑 | Tauri 只是桌面壳和传输层，不应成为新的 God Layer |
| 一个超级 store 管全局所有状态 | 会形成新的前端耦合中心 |
| 模块间直接访问别人的数据库表、文件、缓存 | 会把边界重新打穿 |
| 用硬编码 if/else 散落实现 provider、模块、平台差异 | 会让扩展能力退化成修改式接入 |
| 为了扩展功能而引入万能 Service/Manager/Utils | 会回到旧式高耦合架构 |

### 2.3 Architecture Decision Priority

若现实约束迫使做取舍，本项目按以下优先级决策：

1. 先保证低耦合、模块化和接口边界不被打穿
2. 再保证扩展是注册式和数据驱动的，而不是修改式和硬编码的
3. 再保证职责清晰，前端不承接后端职责
4. 再保证性能敏感链路有明确的专项设计
5. 最后才是局部实现便利性

解释：

- 低耦合和可扩展性是长期正确性的根基
- 性能很重要，但不能通过打穿边界来换取短期性能
- Rust 能兜底性能，但不是允许系统设计失控的理由

---

## 3. Core Design Principles

### Principle 1: Frontend only renders and orchestrates user intent

前端只负责：

1. 页面结构与交互
2. 视图状态与筛选状态
3. 调用类型化 IPC 客户端
4. 渲染后端返回的 read model / summary / progress projection

前端禁止负责：

- 断点续传实现
- 文件系统扫描
- 哈希校验
- 下载调度
- 重试策略
- 安装事务
- 本地数据库写入
- OAuth token 生命周期管理

### Principle 2: Rust backend owns business truth

Rust 后端是系统唯一的业务真相源，负责：

- 业务规则
- 状态机
- 长任务调度
- 数据持久化
- 平台能力封装
- 与 Epic/Fab/本地文件系统/数据库的交互

前端缓存是可丢弃的，后端状态才是权威状态。

### Principle 3: Tauri is transport, not business core

Tauri 层只承担：

- 桌面窗口宿主
- IPC command / event 传输
- 平台集成入口
- 打包、更新、权限声明

Tauri command handler 不得直接承载复杂业务编排。业务逻辑必须下沉到 Rust application service / module facade。

### Principle 4: Modules communicate through contracts only

任何模块对外只暴露：

- command contract
- query contract
- event contract
- read model / summary DTO

模块之间不得：

- import 其他模块内部实现
- 共享可变对象
- 直接访问其他模块的表、文件、缓存

### Principle 5: Data-driven over hard-coded branching

凡是下列可配置能力，优先使用类型化配置、描述符和注册表驱动：

- provider 接入
- 模块入口注册
- capability 开关
- 重试/退避/并发等策略参数
- 启动阶段开关
- 错误映射和用户可见文案策略

但以下内容不得滥用“数据驱动”替代代码：

- 核心领域不变量
- 状态机合法性
- 安装/下载事务一致性规则

换句话说：

- 策略、接线、扩展点可以数据驱动
- 核心业务真相仍然必须由代码和测试守护

### Principle 6: Extensibility comes from registries and ports

新增能力时优先：

1. 新增端口 trait
2. 新增适配器 crate
3. 向 capability registry 注册
4. 向 shell/module manifest 注册前端入口

而不是直接修改大量既有模块内部代码。

### Principle 7: All expensive work is backend job-oriented

下载、校验、解压、扫描、缩略图处理、修复、导入、索引构建等全部走后端 job runtime。

前端只接收聚合后的任务状态，不接收每个底层细粒度噪声事件。

---

## 4. Target Tech Stack

## 4.1 Desktop Host

- Tauri 2
- Rust stable
- Tokio async runtime
- Tauri updater / platform plugins when appropriate

## 4.2 Frontend

- TypeScript
- React + Vite 作为默认参考实现
- TanStack Query 负责服务端数据查询缓存
- Zustand 或等价轻量 store 负责局部 UI 状态
- 虚拟化列表组件负责大列表性能

说明：前端框架可以替换，但必须保持架构约束不变。也就是说，Vue/Svelte 可以替换 React，但不能破坏“前端只负责 UI”和“通过类型化 IPC 契约通信”的核心边界。

## 4.3 Backend

- Rust workspace
- `serde` 负责序列化
- `specta` / `tauri-specta` 负责前后端类型绑定生成
- `rusqlite` 作为默认 SQLite 持久化驱动；仅在后续确有必要时再评估 `sqlx`
- `reqwest` + retry/backoff 封装外部 HTTP
- `tracing` + `tracing-subscriber` 负责结构化日志
- `thiserror` / `anyhow` 仅用于后端错误建模与边界转换

## 4.4 Testing

- Rust: `cargo test`, integration tests, contract tests
- Frontend: Vitest + Testing Library
- E2E: Playwright 或 Tauri driver

---

## 5. System Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                        Frontend UI                         │
│  Shell / Routes / Module Views / UI State / Query Cache    │
└──────────────────────────────┬──────────────────────────────┘
                               │ typed IPC client
┌──────────────────────────────▼──────────────────────────────┐
│                    Tauri Transport Layer                   │
│        commands / events / window / platform glue          │
└──────────────────────────────┬──────────────────────────────┘
                               │ module facades
┌──────────────────────────────▼──────────────────────────────┐
│                   Rust Application Layer                   │
│      use cases / facades / orchestration / job control     │
└──────────────────────────────┬──────────────────────────────┘
                               │ ports
┌──────────────────────────────▼──────────────────────────────┐
│                   Rust Domain + Kernel                     │
│   entities / rules / state machines / events / contracts   │
└──────────────┬───────────────────────┬──────────────────────┘
               │                       │
   ┌───────────▼──────────┐  ┌────────▼─────────┐
   │ Infra Adapters        │  │ Background Jobs  │
   │ sqlite/http/fs/auth   │  │ download/install │
   │ cache/platform        │  │ scan/repair/etc. │
   └───────────────────────┘  └──────────────────┘
```

关键判断标准：

- 换掉 Tauri，不应迫使重写业务核心
- 换掉前端框架，不应迫使改 Rust 业务核心
- 换掉某个外部 API 适配器，不应迫使改 UI 和其他模块核心

---

## 6. Workspace Structure

建议采用 monorepo + Rust workspace：

```text
epic-launcher-rewrite/
├─ apps/
│  └─ desktop/
│     ├─ src/                    # 前端 UI（React + TS）
│     ├─ public/
│     ├─ package.json
│     └─ src-tauri/
│        ├─ src/
│        │  ├─ main.rs
│        │  ├─ bootstrap.rs
│        │  ├─ ipc/
│        │  └─ windowing/
│        ├─ capabilities/
│        ├─ Cargo.toml
│        └─ tauri.conf.json
│
├─ crates/
│  ├─ kernel/                   # Result, Error, EventBus, JobRuntime, shared traits
│  ├─ ipc-contracts/            # 前后端共享契约与类型导出
│  ├─ module-auth/
│  ├─ module-library/
│  ├─ module-downloads/
│  ├─ module-installations/
│  ├─ module-engines/
│  ├─ module-plugins/
│  ├─ module-settings/
│  ├─ module-diagnostics/
│  ├─ module-updates/
│  ├─ adapter-epic-api/
│  ├─ adapter-fab-api/
│  ├─ adapter-sqlite/
│  ├─ adapter-filesystem/
│  ├─ adapter-platform/
│  └─ adapter-secure-storage/
│
├─ packages/
│  ├─ ui-shared/                # 公共前端 UI 组件
│  ├─ frontend-contracts/       # 生成的 TS 契约绑定
│  └─ eslint-config/            # 前端工程约束
│
├─ config/
│  ├─ providers/                # provider 描述符与路由策略
│  ├─ download-policies/        # 默认并发、重试、限速等策略
│  └─ feature-flags/            # 按环境或平台切换的能力开关
│
├─ tests/
│  ├─ backend-integration/
│  ├─ frontend/
│  ├─ contract/
│  └─ e2e/
│
└─ docs/
```

---

## 7. Layer Responsibilities

## 7.1 Frontend Presentation Layer

职责：

- 路由与页面布局
- 组件树
- 用户输入与交互反馈
- 视图状态
- 调用 query/command hooks
- 呈现后端 read models

禁止：

- 直接写文件
- 直接碰数据库
- 自己维护下载状态机
- 自己决定断点策略
- 直接实现重试/恢复/校验

## 7.2 IPC Client Layer

职责：

- 封装 `invoke` / event subscription
- 把后端契约生成到 TS 类型系统
- 做参数校验与统一错误转换

禁止：

- 混入页面逻辑
- 混入模块业务判断

## 7.3 Rust Application Layer

职责：

- 编排 use case
- 协调多个端口
- 维护 job 生命周期
- 向 IPC 输出稳定的 command/query facade

禁止：

- 直接持有平台 UI 对象
- 直接硬编码 Tauri window 细节

## 7.4 Rust Domain Layer

职责：

- 实体、值对象、状态机、领域规则
- 合法性校验
- 领域事件定义

禁止：

- 依赖 Tauri
- 依赖 SQLite/HTTP/文件系统实现

## 7.5 Adapter Layer

职责：

- 实现端口 trait
- 封装外部 API、数据库、文件系统、密钥存储、平台差异

禁止：

- 越过端口直接写回业务核心状态
- 把第三方数据结构泄漏到业务核心

## 7.6 Job Runtime Layer

职责：

- 长任务执行、取消、恢复、聚合、节流、事件广播

禁止：

- 由前端直接控制底层线程/任务实现

---

## 8. Module Model

每个业务模块必须形成完整闭环，而不是只存在一半前端代码或一半后端代码。

每个模块至少包含：

1. frontend view hooks / components / module route
2. backend facade
3. command/query contract
4. read models / summaries
5. domain rules
6. adapter ports
7. tests

### 8.1 Recommended Design Patterns

新项目建议明确采用以下模式，而不是放任实现自由生长：

| Pattern | Recommended Use |
|---------|-----------------|
| Hexagonal / Ports and Adapters | Rust 后端总架构；把 Tauri、SQLite、HTTP、文件系统降级为适配器 |
| Facade | 每个后端模块对 IPC 暴露稳定入口，避免前端感知内部复杂度 |
| CQRS-lite | command 和 query 分离；写路径和读路径分离，但不追求重型事件溯源 |
| State Machine | 下载、安装、修复、认证会话等状态转换集中建模 |
| Strategy | provider 差异、重试退避、下载调度策略、缓存策略 |
| Registry | 模块注册、capability 注册、provider 注册 |
| Repository | 持久化访问隔离，避免业务层直接写 SQL 或文件系统 |
| Projection / Read Model | 给前端提供稳定 summary，避免暴露内部实体和细粒度状态 |
| Observer / Domain Event | 表达“已发生事实”，而不是把事件系统当成万能调用链 |

使用规则：

1. 模式是为了收敛复杂度，不是为了堆名词
2. 若某个模式不能明显降低耦合或提升扩展性，则不要硬用
3. 同一职责优先一个主模式，不要一个模块里混三四套抽象风格

推荐模块如下：

| Module | Frontend owns | Backend owns |
|--------|---------------|-------------|
| Shell | 导航、布局、通知容器、窗口级 UI 状态 | 窗口命令、模块注册、全局事件转发 |
| Auth | 登录界面、账号切换视图 | OAuth、token 生命周期、secure storage |
| Library | 搜索筛选视图、卡片列表、详情页展示 | 目录查询、聚合、缓存、预览元数据解析 |
| Downloads | 下载列表和进度展示 | 分块下载、断点续传、调度、重试、速率控制 |
| Installations | 安装状态与操作界面 | 解压、校验、修复、卸载、文件布局 |
| Engines | 引擎列表与操作视图 | 引擎版本查询、安装、启动命令、验证触发与修复编排 |
| Plugins | 插件管理与兼容性视图 | 项目扫描、元数据解析、兼容性规则 |
| Settings | 设置页面 | 配置读写、平台能力切换 |
| Diagnostics | 日志、运行状态、排错面板 | 诊断数据汇聚、日志检索、健康检查 |
| Updates | 更新 UI 与提示 | 自更新检查、下载和切换策略 |

---

## 9. Dependency Rules

## 9.1 Frontend Rules

允许：

- `shell` 依赖所有模块暴露的 frontend route manifest
- 模块页面依赖自己模块的 hooks / components / generated contracts
- 共享 UI 组件依赖 design tokens 和无业务通用组件

禁止：

- 模块 A 直接 import 模块 B 的内部 store
- 模块 A 直接修改模块 B 的 view state
- 页面直接调用底层 `window.__TAURI__` 或裸 `invoke`

## 9.2 Backend Rules

允许：

- 模块 A 依赖模块 B 暴露的 contract traits / DTO / events
- adapter crate 依赖 kernel + contract + 第三方库
- Tauri IPC crate 依赖各模块 facade

禁止：

- `module-library` 直接依赖 `module-downloads` 的内部调度器实现
- `module-installations` 直接访问 `module-downloads` 的 checkpoint 表
- `module-auth` 直接 import Tauri window or frontend state

## 9.3 Stable Rule of Change Radius

任何内部实现变更都必须满足：

1. 不改 contract，不影响其他模块编译
2. 不改 read model，不影响前端展示层绑定
3. 不改 event schema，不影响订阅方解析

如果一次改动必须同步改很多无关模块，默认判定为边界设计有问题。

## 9.4 Cross-module Interaction Matrix

为避免模块间交互失控，统一使用下表：

| Need | Allowed Mechanism | Example |
|------|-------------------|---------|
| 读取别的模块稳定状态 | Query contract | Library 读取 Downloads 的任务摘要 |
| 请求别的模块执行动作 | Command contract | Library 请求 Downloads 创建下载任务 |
| 通知系统中某件事已经发生 | Domain/App event | Downloads 发布 `task_completed` |
| 注册新模块/新 provider/新 capability | Registry + descriptor | 新 provider 注册到 provider registry |
| 跨多个模块的显式业务编排 | Orchestrator facade | 安装失败后触发受控 repair flow |

禁止：

1. 模块 A 直接调用模块 B 的内部 service 再联动 C
2. 用事件总线替代正常的 command/query 接口
3. 为了省事让 shell 成为所有模块的业务中介

---

## 10. IPC Contract Strategy

## 10.1 Contract Shape

前后端契约必须：

- 类型化
- 模块化
- 版本化
- 稳定
- 对前端友好

推荐命名：

- commands: `downloads.start`, `downloads.pause`, `library.search`, `auth.login.start`
- queries: `downloads.list`, `library.detail`, `settings.get_all`
- events: `downloads.progress_changed`, `downloads.task_completed`, `auth.session_changed`

## 10.2 Contract Generation

Rust 侧定义契约模型，自动导出 TS 类型：

- Rust 是契约源头
- TS 只消费生成产物
- 禁止手写重复 DTO 导致前后端漂移

## 10.3 Error Envelope

统一响应格式：

```ts
type CommandResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: AppErrorDto };
```

错误 DTO 只暴露：

- `code`
- `message`
- `retryable`
- `severity`
- `correlationId`

不向前端泄露后端技术细节堆栈。

## 10.4 Event Frequency Rules

前端不得接收底层高频噪声事件，例如每个 chunk 的写入进度。后端必须先聚合为 UI 可消费的稳定事件，例如 250ms 或 500ms 一个 progress snapshot。

## 10.5 Standard Long-job IPC Protocol

所有长任务 IPC 必须遵守同一生命周期：

1. `start(command)`
2. 返回 `accepted(jobId)` 或同步失败
3. 前端按 `jobId` 订阅 snapshot/event stream
4. 后端定期发送聚合后的 `snapshot`
5. 前端可发 `pause / resume / cancel`
6. 任务最终进入 `completed / failed / canceled`

统一任务快照建议至少包含：

- `jobId`
- `module`
- `kind`
- `uiState`
- `progress`
- `throughput`
- `eta`
- `retryable`
- `error`
- `updatedAt`

这套协议必须适用于：

- 下载
- 安装
- 修复
- 引擎验证
- 库存同步
- 索引构建

禁止每个模块各发明一套长任务协议。

---

## 11. State Management Rules

状态必须分层，且前后端职责明确。

| State Type | Owner | Examples |
|-----------|-------|----------|
| Persistent state | Backend | 下载断点、安装记录、配置、账号会话、索引缓存 |
| Runtime business state | Backend | 当前任务运行态、网络状态、worker 状态、锁、队列 |
| View state | Frontend | 当前 tab、筛选条件、弹窗开关、滚动位置 |
| Query cache | Frontend | 列表页分页数据缓存、详情页缓存 |
| Global shell state | Frontend + backend event bridge | 当前主题、全局通知、活跃任务计数 |

规则：

1. 后端业务状态是权威状态
2. 前端 query cache 可随时失效和重建
3. 前端不自己制造“第二套业务真相”
4. 滚动位置、筛选器等纯视图状态归前端

写权限矩阵：

| State Type | Frontend can write | Backend can write |
|-----------|--------------------|-------------------|
| Persistent state | No | Yes |
| Runtime business state | No | Yes |
| View state | Yes | No |
| Query cache | Yes, but derived only | Yes, as source provider |
| Global shell state | Partial UI-only | Partial fact-only |

说明：

- 前端可以更新 query cache，但它只能缓存后端事实，不能伪造业务状态
- `Global shell state` 需要拆成“前端 UI 壳状态”和“后端广播事实”，禁止双向随意覆写

---

## 12. Download and Long Task Architecture

## 12.1 Ownership

断点续传、重试、chunk 校验、速率限制、磁盘预检、CDN 切换、安装串接，全部属于后端 `module-downloads` 和 `module-installations` 的责任。

UI 绝不能实现这些逻辑。

## 12.2 Runtime Model

后端提供统一 job runtime：

- job id
- cancellation handle
- persisted checkpoint
- progress snapshot
- lifecycle events
- failure classification

## 12.3 Read Model Projection

对前端公开的下载状态必须是收敛后的 UI 投影，例如：

- `Queued`
- `Downloading`
- `Paused`
- `Installing`
- `Completed`
- `Failed`

内部再复杂的细粒度状态机，也不能直接泄露给前端页面。

## 12.4 Download vs Installation Ownership

为避免下载域和安装域相互越界，职责固定如下：

| Concern | Owner |
|---------|-------|
| 下载任务创建、断点续传、chunk 调度、速率限制、重试 | Downloads |
| 安装计划、解压、文件落盘、修复、卸载、校验 | Installations |
| 下载完成这一事实通知 | Downloads |
| 是否自动进入安装、如何安装 | Installations 或显式 orchestrator |

规则：

1. Downloads 只发布“下载已完成/失败/暂停”等事实，不直接实现安装业务
2. Installations 不能绕过 Downloads 自己改 checkpoint 或调度器内部状态
3. 如果存在跨域长流程，必须通过显式 orchestrator/facade 建模，而不是让两个模块互相偷调内部实现

## 12.5 Engine Verification and Repair Model

产品上需要支持类似官方启动器的“引擎验证”能力，但在新架构中必须明确它的真实语义：

- 不是前端自己检查文件
- 不是简单地“重新下载整个引擎”
- 而是一次受控的 `verify -> diff -> repair plan -> targeted repair` 后端流程

建议实现：

1. `Engines` 模块负责暴露“验证某个已安装引擎”的 command facade
2. `Engines` 模块提供引擎版本对应的组件清单、文件清单或 manifest 解析适配器
3. `Installations` 模块提供通用 verifier / repair executor，用于做文件存在性、大小、hash、组件完整性校验
4. `Downloads` 模块只负责获取缺失或损坏的安装包、chunk 或 repair payload
5. 前端只负责发起验证、展示验证进度、展示差异摘要和修复结果

推荐的后端流程：

1. 读取引擎版本对应的期望安装清单
2. 扫描本地安装目录，生成实际状态摘要
3. 对比得到差异：缺失文件、损坏文件、缺失组件、版本不一致组件
4. 生成 `repair plan`
5. 若用户确认或策略允许，则只下载和修复缺失/损坏部分
6. 修复完成后重新做轻量复验，确认状态闭环

性能与体验要求：

1. 引擎验证必须作为后端长任务运行，纳入统一 job runtime
2. 支持可取消和可恢复，不得卡住前端
3. 默认优先做增量校验，不对大型目录做无意义全量重复工作
4. 验证结果必须投影成稳定的 read model，例如：`Healthy / NeedsRepair / Repairing / Repaired / Failed`

边界规则：

1. `Engines` 不得自己实现底层文件校验引擎，而是复用 `Installations` 的通用验证/修复能力
2. `Installations` 不得自己决定某个引擎版本的业务语义，而是消费 `Engines` 提供的 manifest / component 描述
3. 前端不得直接遍历本地引擎目录做验证逻辑

---

## 13. Extensibility Model

## 13.1 Module Extensibility

新增一个完整模块时，应只需要：

1. 新增 backend module crate
2. 新增 frontend module route bundle
3. 在 shell manifest 中注册导航和 capability
4. 如需平台能力，则新增对应 adapter

不应要求修改其他模块的内部实现文件。

## 13.2 Provider Extensibility

像 Epic/Fab 这类外部提供方，应通过 provider ports 接入：

- `CatalogProviderPort`
- `DownloadManifestProviderPort`
- `AuthProviderPort`

更换或新增提供方时，优先新增 adapter，而不是修改核心模块业务代码。

## 13.3 Platform Extensibility

平台差异统一收口在 `adapter-platform` / `adapter-secure-storage` 中，例如：

- 路径解析
- 文件关联
- 系统通知
- 凭据存储
- 托盘和窗口行为

核心模块不直接写 `cfg(target_os = ...)` 业务分支。

## 13.4 Stable Descriptors for Extension

为保证扩展是注册式而不是修改式，至少定义三类稳定描述符：

1. `ModuleDescriptor`
2. `ProviderDescriptor`
3. `CapabilityDescriptor`

建议字段：

- `id`
- `version`
- `features`
- `contracts`
- `platform_support`
- `default_policy_ref`

新增模块或 provider 时，优先新增描述符和适配器；若必须改大量既有代码，默认说明扩展点设计失败。

---

## 14. Performance Rules

1. 启动优先显示 shell，再后台恢复次要能力
2. 重任务全部走 Tokio + bounded channels + cancellation
3. 前端列表必须虚拟化，避免全量 DOM 渲染
4. IPC 传输只传 summary/read model，不传重实体
5. 下载进度必须聚合后再发 UI
6. 搜索/列表必须分页化与增量化
7. 缩略图与预览解析由后端缓存，前端不做重复解析
8. SQLite 必须按查询路径设计索引，避免列表页扫描全表

建议目标：

| Metric | Target |
|--------|--------|
| Shell 可交互时间 | < 1.5s |
| 页面切换感知延迟 | < 100ms |
| 列表滚动 | 60fps |
| 任务状态刷新 | 不阻塞 UI 主线程 |
| 大列表首屏渲染 | 分页 + 虚拟化，禁止一次全量渲染 |

## 14.1 Fab Inventory Loading Strategy

Fab 库存加载属于首要性能敏感链路，必须单独设计：

1. 以后端本地 projection 为主读取路径，而不是每次打开页面都实时拼远程数据
2. 维护 `owned inventory projection` 表或等价索引，给前端返回稳定 summary
3. 列表页先读本地 projection，后台再做增量同步和 SWR 刷新
4. 搜索、排序、筛选优先命中本地索引，不把远程 API 当主热路径
5. 缩略图、预览元数据和主列表数据解耦，不让图片补全阻塞主列表可见
6. 卡片 DTO 只返回列表必需字段，详情扩展字段按需查询
7. 同步策略必须支持游标/增量戳/分页续拉，避免每次全量重扫
8. 已恢复登录会话时，允许在启动后的后台阶段自动预热 Fab 库存，但不得阻塞 shell 首次可交互
9. 用户切离库存页再返回时，必须优先复用本地 projection 和缓存结果，而不是重新回到冷启动加载

目标：

- 打开库存页时“先稳态可见，再后台校准”，而不是每次空白等待远程全量拼装

## 14.2 Multi-thread Download Strategy

下载是第二条性能敏感链路，且必须支持断点续传与 1-128 的用户可配置并发。

这里的“1-128 线程”在产品层可以表现为线程数/并发数，但在实现层必须解释为：

- `1-128` 个用户可配置的并发下载槽位（parallel chunk slots）
- 不等于为每个槽位都创建一个 OS 线程

推荐实现：

1. 前端只提供 `1-128` 的配置入口和即时说明
2. 后端把该值转成调度器的并发槽位参数
3. 真正执行使用 Tokio async tasks + bounded queues + backpressure，而不是 128 个裸线程乱跑
4. checkpoint 至少做到 chunk/segment 粒度，确保暂停、恢复、崩溃恢复可靠
5. 下载链拆为 planner / scheduler / fetcher / verifier / writer 五段，避免一个 worker 包打天下
6. provider 或网络条件不适合时，允许后端在不违反用户意图的前提下做安全降载
7. 进度与速率对 UI 只输出聚合快照，不输出底层噪声

这条链路的关键不是“无脑把线程开到 128”，而是：

- 可配置
- 可恢复
- 可观测
- 有上界
- 不打穿内存、磁盘和网络背压

---

## 15. Error Handling and Observability

## 15.1 Unified Error Model

后端统一使用模块化错误码：

- `AUTH_*`
- `LIB_*`
- `DL_*`
- `INS_*`
- `ENG_*`

错误必须分层：

- domain error
- application error
- adapter error
- user-facing error projection

## 15.2 Correlation and Logs

每个 command/job 生成 `correlationId`，贯穿：

- 前端发起动作
- Tauri command
- backend application facade
- adapter 调用
- job runtime

日志用结构化 `tracing`，禁止纯字符串拼接为主的不可查询日志。

## 15.3 Diagnostics Boundary

诊断模块读取的是后端暴露出来的受控诊断视图，而不是前端到处拼日志文件路径自己读。

---

## 16. Startup Pipeline

推荐四阶段：

### Stage 0: shell first

- 启动窗口
- 加载前端基础资源
- 挂载 shell 路由

### Stage 1: essential services

- 配置加载
- 数据库连接
- secure storage 初始化
- 基础 IPC 注册

只允许以下能力阻塞 Stage 1：

- 配置
- 数据库
- 凭据存储
- 最小 IPC / registry 初始化

### Stage 2: state restore

- 恢复最近 UI 视图状态
- 恢复运行时任务摘要
- 前端 query cache 可以按需 hydration

### Stage 3: optional background warmup

- 列表预热
- 缩略图缓存预热
- 更新检查
- 非关键诊断采样

明确只能进入 Stage 3 的任务：

- Fab 库存同步
- 已登录会话下的 Fab 库存预热
- 缩略图预热
- 更新检查
- 诊断采样
- provider 健康探测

原则：任何非关键任务不得阻塞 shell 首次可交互。

---

## 17. Testing Strategy

## 17.1 Backend

- domain unit tests
- application use case tests
- adapter integration tests
- contract tests
- job runtime recovery tests

## 17.2 Frontend

- module hook tests
- component rendering tests
- shell navigation tests
- contract mock tests

## 17.3 E2E

- login flow
- library browse / search / filter
- download pause / resume / recover
- installation / repair
- engine management

原则：改模块 A 时，默认先跑模块 A 的本地测试，再跑极少量边界集成测试，而不是每次全系统大扫射。

---

## 18. AI and Team Collaboration Rules

新的 Tauri 项目需要直接采用以下协作约束，并将它们视为本项目的原生工程规则：

1. 单次任务默认只改一个模块
2. 先读模块文档和契约，再改代码
3. 不跨模块引用内部实现
4. 契约变更必须列出影响面
5. 每个模块保持自描述文档

建议每个 backend module 与 frontend module 都提供：

- `README_ARCH.md`
- `README_API.md`
- `README_FLOW.md`

---

## 19. Anti-patterns for the New Architecture

以下行为在新项目中明确禁止：

1. 前端实现断点续传、重试、校验逻辑
2. Tauri command handler 直接写复杂业务逻辑
3. 一个全局 mega store 管所有模块真实状态
4. 模块直接 import 其他模块内部实现
5. 直接共享可变 DTO 或实体引用
6. 模块绕过契约直接读写别人表和文件
7. 为方便开发新建 `CommonService` / `GlobalManager` / `Utils` 垃圾桶
8. 每个底层状态变化都高频推 UI
9. 平台差异分支散落在业务核心
10. 新增模块时需要回改多处无关模块内部逻辑

---

## 20. First Formal Docs to Split Out Later

如果后续把这份蓝图升级为正式文档，建议拆分为：

1. 项目总览（跨平台目标、范围、质量目标）
2. 架构原则（前后端分离、接口优先、模块化）
3. 解决方案结构（monorepo + workspace）
4. 模块依赖规则（frontend/backend/module/adapter 方向）
5. IPC 契约规范
6. 状态管理规范
7. 下载与长任务子系统
8. 错误处理与日志策略
9. 启动流程
10. 扩展与 provider 接入规范
11. AI 协作规则
12. 反模式清单

---

## 21. Final Decision Summary

这套新架构的关键不是“Rust + Web”本身，而是下面三条边界是否被严格执行：

1. 前端只负责 UI，后端负责业务真相
2. 模块只通过契约通信，不依赖彼此内部实现
3. 新增功能优先通过新增模块、端口和适配器扩展，而不是回改无关模块内部

只要这三条边界守住，后续无论你继续换前端框架、换 provider、换平台适配层，系统都不会重新滑回“牵一发动全身”的高耦合状态。