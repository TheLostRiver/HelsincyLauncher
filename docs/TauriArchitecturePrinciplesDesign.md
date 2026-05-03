# Tauri Architecture Principles Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriIPCAndStateContractsDesign.md`, `docs/TauriRepositoryPortsAndAdapterDesign.md`, `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`
> Focus: architecture principles, boundary rules, design checklist, implementation constraints

---

## 1. Purpose

当前仓库并不是没有架构原则。

问题在于：**前后端边界、接口优先、模块自治、composition root、typed IPC、job runtime、observability、测试门槛这些原则已经散落在蓝图和多份专题文档里，但还没有一份独立的架构原则总文档。**

这会导致两个直接问题：

1. 新增文档或代码时，很难快速判断某个设计是否已经越界。
2. 后续即使细节文档很多，团队仍可能因为没有统一“原则层”而在实现中回退到旧耦合模式。

---

## 2. Existing State

当前原则层内容主要散落在：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | 前后端分离、模块边界、长任务归属、state ownership |
| `docs/TauriIPCAndStateContractsDesign.md` | typed IPC、error envelope、state contract |
| `docs/TauriRepositoryPortsAndAdapterDesign.md` | ports/adapters、provider 接入边界 |
| `docs/TauriCompositionRootWiringDesign.md` | wiring ownership、bootstrap 组合边界 |
| `docs/TauriTestingStrategyAndQualityGateDesign.md` | 测试必须守住架构边界 |
| `docs/TauriLoggingAndObservabilityDesign.md` | observability 必须服务于架构边界而非破坏边界 |

结论：

1. 架构原则并不缺失。
2. 当前缺的是**独立的仓库级架构原则总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 这个项目的核心边界是什么。
2. 新代码和新文档该用什么标准判断是否越界。
3. 哪些技术方案即使“能工作”，也不符合当前架构原则。

这份文档不负责：

1. 替代蓝图的完整细节设计。
2. 取代每个模块自己的专题文档。
3. 规定具体 crate 名称或目录布局细节。

---

## 4. Principle 1: Frontend Is UI, Backend Owns Truth

前端的职责是：

1. 呈现 UI
2. 收集用户意图
3. 展示 read model / query result / job projection
4. 维护纯 view state

后端的职责是：

1. 持有业务真相
2. 维护 runtime state
3. 执行长任务
4. 负责恢复、校验、重试和一致性

禁止：

1. 前端自己维护第二套下载、安装、认证或库存真相
2. 前端自己实现断点续传、校验、修复、重试等业务逻辑

---

## 5. Principle 2: Contract First

模块之间先定义契约，再落实现。

契约包括：

1. command/query/event
2. DTO schema
3. error envelope
4. job snapshot
5. adapter ports

规则：

1. Rust 是后端契约源头。
2. 前端消费生成或稳定导出的契约，不手写平行 DTO。
3. 新模块上线前，必须先明确对外命令、查询、事件和错误模型。

---

## 6. Principle 3: Modules Communicate Through Boundaries

模块是通过契约协作，不是通过内部实现互相耦合。

允许：

1. 通过 facade / port / event / projection 协作
2. 通过共享基础契约类型协作

禁止：

1. 直接 import 别的模块内部实现
2. 直接读写别的模块持久化数据
3. 共享可变内部状态引用

---

## 7. Principle 4: Composition Root Owns Wiring

依赖拼装属于 composition root，不属于业务模块。

因此：

1. 模块暴露能力，不暴露装配细节。
2. provider、adapter、storage、runtime 的选择与接线放在 composition root。
3. `main.rs` 应保持很薄，真实装配逻辑放到可测试 bootstrap surface。

禁止：

1. 在模块内部偷偷 new 别的模块的具体实现
2. 在 command handler 里做大段手工 wiring

---

## 8. Principle 5: Long-running Work Belongs To Backend Runtime

凡是耗时、可恢复、可取消、需要校验或需要一致性保证的流程，都应进入后端 job runtime。

典型范围：

1. downloads
2. installations
3. engine verification / repair
4. inventory sync
5. indexing and prewarm

禁止：

1. 前端用轮询和本地状态机模拟后端任务引擎
2. 每个模块自己再发明一套 job lifecycle

---

## 9. Principle 6: State Must Be Layered

状态必须清楚区分：

1. backend persistent state
2. backend runtime business state
3. frontend view state
4. frontend query cache
5. shell-level derived status

关键约束：

1. persistent/runtime business truth 在 backend
2. frontend query cache 只缓存 backend 事实
3. view state 才属于 frontend

---

## 10. Principle 7: Projection Over Leakage

无论是任务状态还是错误状态，前端消费的都应是稳定投影，而不是底层实现细节。

这意味着：

1. 任务对前端公开统一 snapshot
2. 错误对前端公开统一 `AppErrorDto`
3. diagnostics 面板展示的是诊断摘要，而不是原始底层日志

---

## 11. Principle 8: Observability And Testing Are Boundary Guards

日志和测试不是后加的附件，而是边界守卫。

规则：

1. observability 必须证明某个 command/job/startup 流程真的按边界运行。
2. 测试必须优先验证 contract、wiring、runtime 和 transport 边界。
3. 若某个设计无法被测试或难以被 observability 验证，它通常已经把职责揉乱了。

---

## 12. Principle 9: Extension Through Ports, Not Through Branch Sprawl

新增 provider、平台差异或供应源时，应该通过显式 port/adapter 扩展。

禁止：

1. 在业务核心里大量散落 `if provider == ...`
2. 让平台差异分支侵入 use-case 核心逻辑
3. 为图方便添加万能 `CommonService` 或 `GlobalManager`

---

## 13. Principle 10: Prefer Stable System Shape Over Short-term Convenience

允许为了保持系统形状而多做一点显式建模。

例如：

1. 多一个 facade，比把流程塞进 command handler 更好
2. 多一个 shared DTO，比前后端各写一份更好
3. 多一个 bootstrap surface，比把初始化压进 `main.rs` 更好

---

## 14. Architecture Review Checklist

每次新增设计或实现前，至少要问：

1. 业务真相是否仍在 backend？
2. 是否先定义了 contract，而不是直接写实现？
3. 是否通过边界协作，而不是偷用内部实现？
4. wiring 是否仍然集中在 composition root？
5. 长任务是否进入统一 runtime？
6. 前端拿到的是稳定投影，而不是技术细节吗？
7. 测试和日志是否能证明这条边界真的成立？

任意一项答不清，就不应继续扩大实现面。

---

## 15. Exit Criteria

当满足下面几条时，可以认为“架构原则总文档缺口已经补上”：

1. 仓库里已经存在独立的架构原则总文档。
2. 前后端边界、contract first、module boundaries、composition root、job runtime、state layering 等核心原则已被统一表达。
3. 后续新增模块或专题文档时，可以先引用这份原则文档，再进入细节设计。
4. 团队不再只能从大蓝图里提炼原则层规则。