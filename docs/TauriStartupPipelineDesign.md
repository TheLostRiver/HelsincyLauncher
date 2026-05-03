# Tauri Startup Pipeline Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriBackendSkeletonImplementationDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`
> Focus: startup stages, blocking rules, restore policy, warmup boundaries, failure handling

---

## 1. Purpose

当前仓库并不是完全没有启动流程设计。

问题在于：**启动阶段、阻塞规则、恢复顺序和 warmup 边界已经散落在蓝图和 composition root 文档里，但还没有一份独立的 startup pipeline 总文档。**

这会带来几个直接问题：

1. “什么可以阻塞首屏，什么只能后台进行”容易被实现时再次打穿。
2. startup、state restore、optional warmup 的职责很容易混在一起。
3. 后续如果 diagnostics、auth、fab prewarm、downloads resume 同时接入，没有统一文档就容易互相抢启动时机。

---

## 2. Existing State

当前启动相关内容主要散落在：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | Stage 0-3 的高层定义和阻塞原则 |
| `docs/TauriCompositionRootWiringDesign.md` | `StartupPipelineFacade`、wiring 顺序、startup handle |
| `docs/TauriBackendSkeletonImplementationDesign.md` | 当前仓库骨架的 startup 接线位置 |
| `docs/TauriLoggingAndObservabilityDesign.md` | startup 日志事件要求 |

结论：

1. 启动流程原则已经存在。
2. 当前缺的是**独立的 startup pipeline 总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 启动分成哪些阶段。
2. 每个阶段哪些任务允许阻塞，哪些必须后置。
3. state restore、task resume、warmup 和 diagnostics 的顺序如何安排。
4. 阶段失败时系统如何降级，而不是直接让启动流程失控。

这份文档不负责：

1. 定义每个模块的完整业务逻辑。
2. 取代 composition root 的具体 wiring 细节。
3. 取代各模块自己的 prewarm 或恢复策略细节。

---

## 4. Startup Principles

1. shell first：用户应先看到可交互壳层，而不是等待次要能力全部恢复。
2. blocking is explicit：只有明确列为 blocking 的任务才能卡住启动阶段。
3. restore before warmup：先恢复已有状态，再启动可选预热。
4. no hidden boot side effects：模块构造函数不应偷偷启动任务。
5. startup is observable：每个阶段开始、结束、失败都必须可观测。
6. degradation beats deadlock：非关键能力失败时，应降级并继续进入下一个允许阶段。

---

## 5. Stage Model

### Stage 0: Shell First

目标：

1. 拉起窗口和前端壳层。
2. 让用户尽快获得可交互 UI。

允许内容：

1. 窗口启动
2. 基础静态资源加载
3. shell 路由挂载

禁止内容：

1. 等待 Fab 同步
2. 等待更新检查
3. 等待缩略图预热
4. 等待 diagnostics 采样

### Stage 1: Essential Services

目标：

1. 建立最小可工作的 backend host 环境。
2. 注册最小 IPC / registry / storage 依赖。

允许阻塞：

1. configuration load
2. database init
3. secure storage init
4. minimum IPC registration

不允许阻塞：

1. provider health scan
2. inventory sync
3. update check
4. 非关键 diagnostics warmup

### Stage 2: State Restore

目标：

1. 恢复最近 UI 视图状态。
2. 恢复运行时任务摘要。
3. 接回必要的 read model 缓存或投影快照。

允许内容：

1. shell view restore
2. job summary restore
3. 已存在任务的最小 resume decision

禁止内容：

1. 借 restore 名义启动新的大规模后台任务
2. 在没有用户价值的情况下重跑全量同步

### Stage 3: Optional Background Warmup

目标：

1. 在不阻塞首屏的前提下恢复次要能力。
2. 提升后续页面命中率和感知速度。

适合放在此阶段的任务：

1. Fab inventory prewarm
2. update check
3. thumbnail prewarm
4. diagnostics sampling
5. provider health probe

---

## 6. Blocking Policy

统一规则：

| Task Type | Can Block Startup? | Notes |
|-----------|--------------------|-------|
| config load | Yes | 属于最小工作前提 |
| database open | Yes | 属于最小持久化前提 |
| secure storage init | Yes | 仅当当前阶段需要认证上下文时 |
| IPC registration | Yes | 否则前后端边界无法工作 |
| task restore summary | Partial | 允许恢复摘要，不允许借机做大规模 warmup |
| Fab sync | No | 只能后台进行 |
| update check | No | 只能后台进行 |
| diagnostics sampling | No | 只能后台进行 |
| thumbnail prewarm | No | 只能后台进行 |

判断标准：

1. 若任务失败后用户仍能安全看到并操作 shell，则它默认不应阻塞。
2. 若任务是系统最小可运行前提，则可以进入 blocking 集合。

---

## 7. Restore Policy

startup pipeline 中的 restore 只恢复三类内容：

1. 最近 UI 壳层状态
2. 运行中或可恢复任务的摘要
3. 必要的只读投影缓存

restore 不应负责：

1. 重新做完整 provider 同步
2. 重建大型索引
3. 全量扫描文件系统
4. 启动新的可选 warmup 任务

这些都应该留给 Stage 3。

---

## 8. Warmup Boundary

Warmup 的存在意义是“提升后续命中率”，不是“补齐系统基本可用性”。

因此它必须满足：

1. 可取消
2. 可延后
3. 失败可降级
4. 不影响 shell 首次可交互

若某项能力必须完成后系统才能正常工作，它就不应被叫作 warmup。

---

## 9. Failure Handling

### 9.1 Stage 0 Failure

若 Stage 0 失败，系统无法建立最小 UI 壳层，应直接视为启动失败。

### 9.2 Stage 1 Failure

若 configuration、database、secure storage 或最低 IPC 注册失败：

1. 返回结构化错误
2. 写入 startup 日志
3. 阻止继续进入后续阶段

### 9.3 Stage 2 Failure

若 restore 某个局部能力失败：

1. 保留可恢复部分
2. 记录失败摘要
3. 尽量继续进入 Stage 3

### 9.4 Stage 3 Failure

若 warmup 失败：

1. 不应回滚前面阶段
2. 记录日志和 diagnostics 摘要
3. 必要时提示用户，但不阻塞当前 shell

---

## 10. Ownership

| Concern | Owner |
|---------|-------|
| startup stage orchestration | composition-root / startup facade |
| shell visibility and interaction | frontend shell |
| state restore truth | backend |
| warmup scheduling | backend modules via controlled startup facade |
| startup observability | logging / diagnostics boundary |

禁止：

1. 让前端自己决定 Stage 1 或 Stage 2 的业务恢复顺序
2. 让某个模块在构造函数里偷偷启动 background work
3. 让宿主层绕过 startup facade 直接拼接多模块恢复逻辑

---

## 11. Current-repo Rollout Order

对当前仓库而言，startup 文档与实现应按下面顺序推进：

1. 先有这份 startup pipeline 总文档。
2. 当 `src-tauri` 首次落地时，先建立最薄 `main.rs` 和可测试 bootstrap surface。
3. 当 composition root 落地时，再接出 `StartupPipelineFacade`。
4. 当 restore 和 warmup 真正进入代码时，再分别为任务恢复和预热能力补专项文档。

---

## 12. Exit Criteria

当满足下面四条时，可以认为“startup 文档缺口已经补上”：

1. 仓库里已经存在独立的 startup pipeline 总文档。
2. Stage 0-3、blocking policy、restore policy 和 warmup boundary 已被统一定义。
3. 后续实现可以直接引用这份文档，不再从蓝图和 wiring 文档里拼启动规则。
4. diagnostics、logging 和 startup 不再互相重复定义阶段职责。