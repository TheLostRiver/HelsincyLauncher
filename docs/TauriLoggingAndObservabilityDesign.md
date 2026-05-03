# Tauri Logging and Observability Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriStorageAndDatabaseDesign.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriEngineVerificationRepairDesign.md`, `docs/TauriKernelJobsRuntimeDesign.md`
> Focus: repository-level logging strategy, correlation model, log sinks, observability boundaries, rollout order

---

## 1. Purpose

当前仓库并不是完全没有日志设计。

问题在于：**日志、correlationId、诊断边界和结构化字段要求已经散落在多份文档里，但还没有一份独立的日志/observability 总文档。**

这会带来三个直接问题：

1. 后续实现时，日志字段、采样点和落盘位置容易各写各的。
2. `Diagnostics`、`Downloads`、`Engines`、composition root 和 storage 的日志边界容易互相重叠。
3. 没有统一文档时，最容易退化回“随手 print 一行字符串”的不可查询日志。

这份文档就是为了补这个缺口。

---

## 2. Existing State

当前日志相关内容主要散落在这些文档中：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | `tracing`、`correlationId`、diagnostics 边界 |
| `docs/TauriCompositionRootWiringDesign.md` | logging bootstrap handle、startup 阶段日志点 |
| `docs/TauriStorageAndDatabaseDesign.md` | 日志目录和文件系统落盘位置 |
| `docs/TauriDownloadRuntimeDesign.md` | 下载链路关键日志字段 |
| `docs/TauriEngineVerificationRepairDesign.md` | 验证/修复关键日志字段 |
| `docs/TauriFabInventoryLoadingDesign.md` | 库存同步关键日志字段 |
| `docs/TauriKernelJobsRuntimeDesign.md` | snapshot-first observability 原则 |

结论：

1. 日志设计想法已经存在。
2. 当前真正缺的是**独立的仓库级日志总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 日志由谁写。
2. 日志写到哪里。
3. 日志字段最少要有哪些。
4. 前端、宿主、backend module、job runtime 和 diagnostics 的可观察性边界如何划分。

这份文档不负责：

1. 定义每个模块的完整业务事件表。
2. 取代具体模块文档里的专项日志字段章节。
3. 设计完整监控平台或远程日志服务。

---

## 4. Logging Principles

1. 默认使用结构化日志，不使用不可查询的字符串拼接作为主形态。
2. 每个 command、job 和重要跨层操作都必须挂上 `correlationId`。
3. 日志记录“已发生事实”和边界状态变化，不记录重复 UI 噪声。
4. 前端只记录 UI 壳层事件和用户意图，不记录后端业务真相。
5. 后端记录状态转换、adapter 调用、失败分类和恢复路径。
6. Diagnostics 只读受控日志视图，不让前端直接遍历原始日志目录。
7. 日志系统必须服务于排错、恢复和质量验证，而不是成为新的副作用垃圾桶。

---

## 5. Observability Layers

| Layer | Owns What | Must Not Do |
|------|-----------|-------------|
| frontend shell | 页面切换、用户意图、局部 UI 降级信息 | 不伪造下载/安装/认证业务日志 |
| `src-tauri` host | command ingress、窗口宿主生命周期、state 注入结果 | 不承载复杂业务日志编排 |
| composition root | startup stage、adapter 装配成功/失败、依赖注入边界 | 不替代模块记录业务细节 |
| module facades | command/query 入口、状态转换、聚合失败分类 | 不泄漏 adapter 内部第三方 payload |
| adapters | HTTP/SQLite/filesystem/secure storage 边界调用事实 | 不越权写业务层语义日志 |
| kernel-jobs | job lifecycle、snapshot、retry、cancel、recovery | 不记录页面级 UI 事件 |
| diagnostics | 受控检索、聚合视图、健康摘要 | 不绕过契约直接向前端暴露原始文件系统日志 |

---

## 6. Correlation Model

每个重要动作至少要有以下关联字段：

| Field | Purpose |
|------|---------|
| `correlationId` | 串起一次 command / job / startup action 的主追踪键 |
| `module` | 标识日志属于哪个业务模块 |
| `operation` | 标识动作，例如 `downloads.start`、`fab.sync` |
| `jobId` | 长任务链路的稳定标识；无 job 时可为空 |
| `severity` | 统一错误和警告级别 |
| `retryable` | 标识失败是否可重试 |

固定要求：

1. `correlationId` 在前端发起动作时生成或在后端入口生成，但必须贯穿整条链。
2. adapter 和 job runtime 不能丢掉已有的 `correlationId`。
3. 日志检索和 diagnostics 面板默认按 `correlationId` 聚合。

---

## 7. Log Sinks and Storage

当前架构下，日志默认走本地文件系统。

建议落点：

```text
app-data/
└─ logs/
   ├─ app.log
   ├─ startup.log
   ├─ jobs.log
   └─ diagnostics.log
```

规则：

1. 结构化日志文件由后端或宿主统一落盘。
2. 前端不直接决定日志目录和轮转策略。
3. 不把日志混进 SQLite 主业务表；日志主通道仍然是文件。
4. 若后续引入远程上报，也不能替代本地可诊断日志。

---

## 8. Minimum Log Events

第一版至少要覆盖以下日志事件：

### 8.1 Startup

1. startup stage begin / end
2. configuration load success / failure
3. database init success / failure
4. composition root build success / failure

### 8.2 Command Ingress

1. command received
2. command accepted / rejected
3. facade dispatch success / failure

### 8.3 Job Runtime

1. job accepted
2. job state changed
3. retry scheduled
4. job completed / failed / canceled
5. recovery resumed / recovery failed

### 8.4 Adapter Boundaries

1. provider request start / finish
2. SQLite query or transaction failure
3. filesystem read/write failure
4. secure storage access failure

---

## 9. Diagnostics Boundary

`Diagnostics` 模块不应直接等价于“日志文件浏览器”。

它更适合暴露：

1. 受控检索接口
2. 按 `correlationId` 聚合后的事件视图
3. 最近失败任务摘要
4. startup 健康状态摘要
5. 关键模块的最后错误快照

禁止做法：

1. 前端自己拼日志路径去读文件
2. 让 diagnostics 直接暴露原始第三方 payload
3. 用 diagnostics 替代正常模块 contract

---

## 10. Current-repo Rollout Order

对当前仓库而言，日志文档和实现应按下面顺序推进：

1. 先有这份仓库级日志总文档。
2. 当 `src-tauri` 首次落地时，先把 logging bootstrap surface 放在宿主和 composition root 的边界上。
3. 当 `kernel-jobs` 落地时，再补 job lifecycle 的结构化日志。
4. 当 `downloads`、`engines`、`fab-inventory` 等高风险模块落地时，再补各自专项日志字段。
5. 最后才是 diagnostics 面板和日志检索 UI。

---

## 11. Validation Rules

日志相关实现进入代码阶段后，至少要能回答下面几个问题：

1. 这条日志属于哪一层写出的？
2. 它是否带着 `correlationId`？
3. 它是结构化字段，而不是随手拼接的字符串吗？
4. Diagnostics 是否能通过受控接口检索，而不是直接读原始文件？
5. 这条日志是否记录了边界事实，而不是跨层重复噪声？

---

## 12. Exit Criteria

当满足下面四条时，可以认为“日志文档缺口已经补上”：

1. 仓库里已经存在独立的日志/observability 总文档。
2. `correlationId`、log sinks 和 diagnostics 边界已经被统一定义。
3. 后续模块文档不再需要各自重新发明日志总原则。
4. 实现阶段可以直接引用这份文档，而不是继续从散落章节里拼规则。