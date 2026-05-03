# Tauri Documentation Benchmark Against Codex-Manager

> Status: local draft v1
> Date: 2026-05-03
> Scope note: this document compares MyEpicLauncher's collaboration-constraint and architecture docs against Codex-Manager's CONTRIBUTING and ARCHITECTURE documents, then records actionable documentation optimizations for this repository.
> Storage note: this document is intentionally stored under `docs/` and is intended to be included in git commits.

---

## 1. Purpose

这份文档不负责替代现有架构文档。

它只回答三个问题：

1. 与 Codex-Manager 相比，我们的协作约束文档处于什么水平。
2. 与 Codex-Manager 相比，我们的架构说明文档处于什么水平。
3. 在不削弱现有 strict-doc 体系的前提下，我们的文档体系还应如何优化。

---

## 2. Comparison Inputs

### 2.1 External Baseline

对照对象来自 Codex-Manager 的两份文档：

1. `docs/zh-CN/CONTRIBUTING.md`
2. `docs/zh-CN/ARCHITECTURE.md`

它们的优势非常明确：

1. 用很少的文档入口，把“改哪里、先跑什么、哪些地方高风险”说清楚。
2. 站在协作者视角组织内容，而不是只站在设计者视角组织内容。

### 2.2 MyEpicLauncher Baseline

本次对比主要参照以下文档：

1. `README.md`
2. `.github/copilot-instructions.md`
3. `.github/skills/strict-doc-driven-development/SKILL.md`
4. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
5. `docs/TauriRewriteArchitectureBlueprint.md`
6. `docs/TauriArchitecturePrinciplesDesign.md`
7. `docs/TauriTestingStrategyAndQualityGateDesign.md`
8. 其他专题设计文档，如 startup、安全、release、logging、environment bootstrap

---

## 3. Executive Conclusion

### 3.1 Collaboration Constraints

结论不是“我们弱”，而是“我们偏深而不够平”。

MyEpicLauncher 当前的协作约束体系：

1. 在 AI 执行纪律、原子任务协议、单活跃任务、文档驱动、窄验证这些方面，**明显比 Codex-Manager 更严格**。
2. 但在“新协作者 5 分钟内知道该改哪层、该跑什么命令、哪些文件是高风险、提交流程怎么约束”这些方面，**明显比 Codex-Manager 更不平铺、更难快速进入**。

换句话说：

- 我们更像“高约束执行协议 + 深度设计库”。
- 对方更像“贡献者操作手册 + 当前仓库作战地图”。

### 3.2 Architecture Docs

结论也不是“我们架构文档不够多”，而是“我们更强在原则层和专题层，对方更强在当前仓库导航层”。

MyEpicLauncher 当前架构文档：

1. 在前后端边界、contract first、composition root、job runtime、状态分层、安全、startup、testing、release 等方面，**深度和约束性明显强于 Codex-Manager**。
2. 但在“当前仓库目录职责、复杂域入口索引、请求链路、风险热点、建议改动落点”这些帮助协作者快速定位落点的内容上，**Codex-Manager 更直接、更像一份操作地图**。

### 3.3 Optimization Direction

最正确的优化方向不是删掉现有深度文档，而是：

1. 保留现有严格协议和深度设计文档。
2. 在它们之上增加一层更扁平的 contributor-facing 文档入口。
3. 让“快速判断改动落点”与“深度架构约束”形成上下两层，而不是互相替代。

---

## 4. Collaboration Constraints Comparison

| Dimension | Codex-Manager | MyEpicLauncher | Assessment |
|----------|---------------|----------------|------------|
| 协作文档目标 | 直接约束日常协作、验证、提交、发版 | 直接约束 AI 长任务执行、文档驱动和原子任务 | 我方更严格，但受众更窄 |
| 目标读者 | 新协作者、人类开发者、提交者 | AI 主导执行者，兼顾用户指令约束 | 对方更适合快速 onboarding |
| 必读入口 | README / ARCHITECTURE / TESTING / SECURITY / docs README | README + project instructions + strict-doc skill + 多份设计文档 | 我方入口更多，切换成本更高 |
| 任务拆分规则 | 大改动建议先拆任务再提交 | 复杂任务必须先拆原子任务，且单活跃任务 | 我方更强 |
| 验证门槛 | 按前端 / Rust / 桌面链路列最小检查清单 | 文档定义 named smoke tests、narrow validation gate | 我方更严谨 |
| 环境与常用命令 | CONTRIBUTING 直接给出工具链和命令 | 环境信息主要在独立设计文档和 README | 对方更平铺 |
| 文件归属与边界 | 明确路径级职责边界 | 主要靠架构原则和模块文档推导 | 对方更适合快速定点 |
| 高风险文件提醒 | 有 | 目前无统一“高风险文件”入口 | 对方明显更好 |
| 大文件阈值 | 有明确预警阈值 | 当前无统一阈值 | 对方明显更好 |
| 提交/PR 约束 | 有提交说明、PR 最低要求 | 当前更多依赖会话协议和局部约束 | 对方更成熟 |
| 发版前检查 | 有独立章节 | 规则分散在 release 与其他设计文档 | 对方更平铺 |
| 文档维护职责 | README / CHANGELOG / ARCHITECTURE / CONTRIBUTING 分工明确 | 目前缺少统一 docs map | 对方更清晰 |

### 4.1 Our Strong Areas

MyEpicLauncher 当前更强的点：

1. 把复杂任务执行协议写到了可恢复、可验证、可压缩恢复的粒度。
2. 明确限制 AI 不能跨模块猜实现，不能跳过文档，不能在第一次实质编辑后继续盲改。
3. 把测试门槛和架构边界绑定得更紧，而不是停留在“跑一下 package 级命令”。

这些能力是 Codex-Manager 的 CONTRIBUTING 文档没有覆盖到的深度。

### 4.2 Our Main Gaps

MyEpicLauncher 当前更弱的点：

1. 缺少一份面向“协作者快速判断”的仓库级 CONTRIBUTING 入口。
2. 缺少路径级、高风险文件级、规模阈值级的显式提示。
3. 缺少 README / docs / release / transaction protocol 之间的职责总图。

这意味着：

1. 对熟悉本仓库协议的人来说，我们的规范更强。
2. 对第一次接触仓库的人来说，Codex-Manager 的进入成本更低。

---

## 5. Architecture Docs Comparison

| Dimension | Codex-Manager | MyEpicLauncher | Assessment |
|----------|---------------|----------------|------------|
| 架构总目标 | 有，偏产品与运行模式描述 | 有，且原则和边界更完整 | 我方更强 |
| 原则层深度 | 有，但更偏目录和职责说明 | blueprint + principles + 多专题设计 | 我方明显更强 |
| 当前仓库目录职责 | 很清楚 | 局部存在，但缺单独 current-repo overview | 对方更好 |
| 复杂域入口索引 | 有明确入口文件索引 | 目前分散在各专题文档 | 对方更好 |
| 运行关系 | 有桌面模式 / service 模式说明 | 有高层系统图，但缺当前 repo 操作视角 | 对方略好 |
| 请求链路 | 有典型请求链路概览 | 有 contract / facade / transport 原则，但缺一页总览 | 对方更直观 |
| 数据与配置边界 | 有 | 有且更深，已拆出 storage / security / startup | 我方更强 |
| testing / startup / security / release 专题 | 相对集中在协作文档与架构文档中 | 已拆成独立专题文档 | 我方明显更强 |
| 当前结构风险 | 有明确热点 | 风险散落在文档与任务记录中 | 对方更适合作战地图 |
| 建议改动落点 | 有独立章节 | 主要靠架构原则推导 | 对方更适合协作者 |

### 5.1 Our Strong Areas

MyEpicLauncher 的架构文档体系已经具备这些优势：

1. 目标架构不是一页目录树，而是边界、职责、状态、runtime、测试、安全、release 的完整设计基线。
2. 许多最容易在实现时退化的点，已经先被文档钉死，例如 frontend 不能拥有业务真相、Tauri 只做 transport、composition root 才拥有 wiring。
3. 我们已经把“为什么不能这么做”写出来了，而不只是写“目录大概长什么样”。

### 5.2 Our Main Gaps

MyEpicLauncher 当前仍缺少一份更接近 Codex-Manager ARCHITECTURE 定位的文档：

1. 不是再讲一次原则。
2. 而是帮助协作者快速判断“当前仓库改动应该落在哪一层”。

缺口主要体现在：

1. 缺少 current-repo 视角的目录职责总览。
2. 缺少关键入口文件索引。
3. 缺少典型请求链路或命令链路的单页视图。
4. 缺少当前高风险热点和建议改动落点。

---

## 6. Concrete Findings In This Repository

这次对比不是抽象结论，已经能指出当前仓库的具体漂移点：

1. `README.md` 在本次 slice 开始前仍保留“仓库还没有 `Cargo.toml` / `src-tauri/`”之类旧状态描述。
2. `README.md` 在本次 slice 开始前仍把任务记录写成根目录 `task_plan.md` / `progress.md` / `findings.md`，与 `.artifacts/ai/` 的真实协议不一致。
3. 当前仓库已经有很强的深度设计文档，但仍缺少一份 contributor-facing 的对比总结或快速入口。

这三个点一起说明：

1. 我们的问题不是“缺少更多深文档”。
2. 我们的问题是“缺少更平的入口层，并且入口层偶尔会发生状态漂移”。

---

## 7. Recommended Documentation Optimizations

### 7.1 Priority P0: Keep README Aligned With Repo Reality

README 应该首先回答：

1. 当前仓库已经落了什么。
2. 当前仓库还没落什么。
3. 读者接下来该看哪份文档。

因此 README 至少要持续保持：

1. 当前 repo layout 准确。
2. `.artifacts/ai` 路径准确。
3. 后端 baseline 验证入口准确。

### 7.2 Priority P1: Add A Real Contributor-facing Collaboration Guide

建议新增一个仓库级 CONTRIBUTING 文档，职责不要与 AI transaction protocol 冲突，而是补它没有覆盖的平面信息。

建议内容：

1. 当前仓库边界总览
2. 常用命令
3. 按目录的职责落点
4. 最小验证矩阵
5. 高风险文件列表
6. 大文件预警阈值
7. 文档维护分工

注意：

1. CONTRIBUTING 负责“协作者如何改”。
2. `docs/TauriAIDevelopmentTransactionProtocolDesign.md` 负责“复杂 AI 任务如何稳定执行”。

两者不是替代关系。

### 7.3 Priority P2: Add A Current-repo Architecture Overview

建议新增一份 current-repo 视角的架构总览文档，例如：

1. 当前目录职责
2. 关键入口文件索引
3. 前端原型、`src-tauri`、`crates/` 的关系
4. 典型 command / query / startup 链路
5. 当前结构热点与建议改动落点

这份文档的定位应该更接近 Codex-Manager 的 ARCHITECTURE，而不是重复 blueprint。

### 7.4 Priority P3: Add A Docs Map

建议新增 `docs/README.md`，明确回答：

1. 哪份文档管什么。
2. 哪些内容不应该继续堆回 README。
3. 哪些是原则层，哪些是专题层，哪些是 current-repo 操作层。

这能避免“文档越来越多，但入口越来越乱”。

### 7.5 Priority P4: Surface Risk Hotspots Explicitly

建议把下面这类信息显式写到协作文档里：

1. 当前厚文件或总控文件
2. 容易被继续堆逻辑的入口文件
3. 需要克制追加分支的 transport / composition / workflow 路径
4. 文档体量过大时的拆分阈值

这正是 Codex-Manager 对协作者非常友好的地方。

---

## 8. Suggested Rollout Order

推荐按下面顺序推进，而不是一口气重写所有文档：

1. 先修 README 的当前状态漂移。
2. 再新增 CONTRIBUTING 文档，补平协作者入口。
3. 再新增 current-repo architecture overview，补平落点导航。
4. 最后新增 `docs/README.md`，把原则层、专题层、操作层串起来。

原因很简单：

1. README 是第一入口。
2. CONTRIBUTING 解决“怎么协作”。
3. current-repo architecture overview 解决“改动该落哪层”。
4. docs map 解决“文档太多时怎么找”。

---

## 9. Final Assessment

最终判断如下：

1. MyEpicLauncher 的文档体系**并不比 Codex-Manager 弱**。
2. 如果只看深度设计、边界约束和 AI 执行协议，MyEpicLauncher **明显更强**。
3. 如果只看新人快速上手、当前仓库导航、提交前检查、风险热点提示，Codex-Manager **明显更平、更适合作为第一入口**。

因此，本仓库最值得做的优化不是“少写设计文档”，而是：

1. 保留深度设计文档。
2. 增加平面导航文档。
3. 保持入口文档与 repo 真实状态同步。

只要这三件事做到位，我们的文档体系会比 Codex-Manager 更完整，也更适合长期演进。