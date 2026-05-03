# CONTRIBUTING

本文档负责回答一件事：

如何在当前这个仓库里协作，而不把已有的架构边界、AI 事务协议和工作区真实状态打乱。

它不是架构蓝图替身，也不是 `.artifacts/ai/` 的运行日志。

---

## 1. 仓库现状

当前仓库不是“纯前端项目”，也不是“已经完成的桌面应用”。

当前真实形态是：

1. 根目录仍保留 Next.js 前端原型。
2. 根目录已经有 Rust workspace、`src-tauri/` 宿主和第一批 backend crates 骨架。
3. `docs/` 下已经有深度架构、测试、安全、startup、release 等专题设计文档。
4. `.artifacts/ai/` 是当前仓库的活跃任务协议与过程记录面。

因此，协作时最容易犯的错不是“代码写不出来”，而是：

1. 把后端真相重新塞回前端。
2. 把 contributor-facing 入口文档和深度设计文档混成一层。
3. 在 dirty worktree 里顺手带上不属于本次切片的文件。

---

## 2. 先读什么

如果你准备开始改动，先按下面顺序读：

1. `README.md`
2. 本文档 `CONTRIBUTING.md`
3. `docs/TauriRewriteArchitectureBlueprint.md`
4. `docs/TauriArchitecturePrinciplesDesign.md`
5. `docs/TauriBackendSkeletonImplementationDesign.md`
6. `docs/TauriTestingStrategyAndQualityGateDesign.md`
7. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`

职责分层：

1. `README.md`：仓库第一入口和当前状态摘要。
2. `CONTRIBUTING.md`：协作者如何落点、如何验证、哪些地方高风险。
3. 架构与专题设计文档：定义边界、原则和实现约束。
4. `.artifacts/ai/`：当前活跃任务、阶段、进度和发现记录。

---

## 3. 当前目录职责

按目录落点时，优先遵守下面的边界：

1. `app/`：Next.js app router 页面壳层与路由入口。
2. `components/`：前端原型组件和展示层组合。
3. `src-tauri/`：桌面宿主、transport wiring、共享宿主状态与宿主 smoke tests。
4. `crates/kernel-foundation/`：基础错误、ID、分页、时间、结果模型等基础契约。
5. `crates/kernel-jobs/`：长任务公共 runtime 语义。
6. `crates/module-fab/`、`crates/module-downloads/`：模块 facade、contracts 与模块边界。
7. `crates/adapter-storage-sqlite/`、`crates/adapter-provider-fab/`：adapter 边界。
8. `crates/composition-root/`：唯一 assembly owner，不承载业务真相。
9. `docs/`：架构、专题设计、模块文档和治理评估文档。
10. `.artifacts/ai/`：活跃任务协议与执行记录，不是长期架构说明目录。
11. `.github/`：工作流约束、skills、prompts 与仓库级 AI 配置。

不要这样落点：

1. 把 backend-owned truth 放回 `app/` 或 `components/`。
2. 把 wiring 逻辑散落到模块内部。
3. 把流程性运行记录写回 `docs/` 或 `README.md`。

---

## 4. 基本协作规则

1. 先按文档判断边界，再落实现，不要先写再补理由。
2. 一次提交只解决一类问题，避免把前端、宿主、backend、docs 多个边界混在一起。
3. 复杂工作默认先拆原子任务，不要让未验证假设跨多个模块扩散。
4. 优先运行最窄验证，而不是先跑最宽的命令。
5. 如果工作树里已经有不属于本次切片的改动，使用路径限定的 `git status` 和 selective staging。
6. 不要回退自己没产生的改动。

对于 AI 驱动的复杂工作，额外规则以 `docs/TauriAIDevelopmentTransactionProtocolDesign.md` 和 `.github/copilot-instructions.md` 为准。

---

## 5. 常用命令

当前仓库在 Windows PowerShell 5.1 下工作时，优先使用 `npm.cmd`，避免 `npm.ps1` 执行策略问题。

前端原型：

```powershell
npm install
npm.cmd run dev
npm.cmd run build
npm.cmd run lint
```

Rust workspace：

```powershell
cargo check --workspace
```

当前命名 smoke tests：

```powershell
cargo test -p launcher-kernel-foundation foundation_contract_smoke
cargo test -p launcher-composition-root bootstrap_wiring_smoke
cargo test -p my-epic-launcher-desktop transport_wiring_smoke
```

docs-only 或小切片收尾：

```powershell
git diff --check
git status --short -- <paths>
```

---

## 6. 最小验证矩阵

优先跑最窄、最能证伪当前改动的验证。

### 6.1 docs-only 改动

至少做：

1. 路径和文档引用回读
2. `git diff --check`
3. 路径限定的 `git status --short`

### 6.2 前端原型改动

至少做：

1. `npm.cmd run build`
2. `npm.cmd run lint`

### 6.3 foundation / composition / host transport 改动

按改动面选择最窄命名测试：

1. foundation 契约：`cargo test -p launcher-kernel-foundation foundation_contract_smoke`
2. composition root 接线：`cargo test -p launcher-composition-root bootstrap_wiring_smoke`
3. 宿主 transport：`cargo test -p my-epic-launcher-desktop transport_wiring_smoke`
4. 跨 backend 切片：`cargo check --workspace`

### 6.4 入口面或跨层改动

如果改动影响 README、宿主、docs 和 backend 验证入口之间的协作面，至少确认：

1. `cargo check --workspace`
2. 受影响的命名 smoke test
3. `npm.cmd run build`

---

## 7. 改动该落哪儿

遇到需求时，先按下面的落点判断：

1. 新页面或新展示组件：优先进 `app/` 或 `components/`
2. 新宿主命令、状态注入、transport glue：优先进 `src-tauri/`
3. 新业务契约、module facade、module contracts：优先进对应 `crates/module-*`
4. 新 adapter 或 provider 接入：优先进对应 `crates/adapter-*`
5. 新 wiring 或 concrete 组合：优先进 `crates/composition-root/`
6. 新架构规则或专题设计：优先进 `docs/`
7. 新工作流约束、skills、slash commands：优先进 `.github/`
8. 新任务状态、恢复点、进度：优先进 `.artifacts/ai/`

如果一个改动同时横跨多个落点，通常应该先拆任务，而不是一次性混改。

---

## 8. 当前高风险文件

下面这些路径在当前仓库里属于高风险入口，修改时应先判断是否继续拆分：

1. `README.md`：第一入口，最容易因为仓库演进而发生状态漂移。
2. `.github/copilot-instructions.md`：项目级 AI 约束总入口，误改会影响整个协作协议。
3. `.github/skills/strict-doc-driven-development/SKILL.md`：复杂任务的核心工作流入口。
4. `.artifacts/ai/active-task.md`：当前唯一活跃任务边界，不能把已完成任务当成当前任务继续注入。
5. `.artifacts/ai/task-plan.md`：阶段与 AT ledger 总入口，误改会影响恢复路径。
6. `docs/TauriRewriteArchitectureBlueprint.md`：深度总蓝图，不应继续堆执行日志或 changelog 式内容。
7. `src-tauri/src/lib.rs`：宿主公共入口，不应继续堆大段装配和流程逻辑。
8. `src-tauri/src/bootstrap.rs`：宿主 bootstrap 面，改动时要保持可测试 surface 清晰。
9. `crates/composition-root/src/bootstrap.rs`：唯一 assembly owner，不能退化成业务逻辑混合层。
10. `Cargo.toml`：workspace 成员和公共依赖入口，误改容易打穿多个 slice。

这些文件不是“不能改”，而是改动前必须先判断：

1. 是否在继续堆总控逻辑。
2. 是否应该先拆独立模块或独立文档。

---

## 9. 大文件预警阈值

达到下面的规模时，不应默认继续往里堆内容：

1. TypeScript / TSX / JavaScript：超过 500 行开始预警，超过 800 行必须说明为什么不拆。
2. Rust：超过 400 行开始预警，超过 700 行必须说明为什么不拆。
3. YAML / workflow：超过 250 行开始预警，超过 400 行必须说明为什么不拆。
4. Markdown：超过 300 行开始预警，优先考虑拆成更明确的专题文档。

这些阈值不是强制“一到就拆”，而是要求在提交前先做结构判断。

---

## 10. 文档维护分工

当前仓库建议把文档职责分成下面几层：

1. `README.md`：项目简介、当前状态、第一入口。
2. `CONTRIBUTING.md`：协作边界、常用命令、最小验证矩阵、高风险文件和文档分工。
3. `docs/TauriRewriteArchitectureBlueprint.md`：总体架构蓝图。
4. `docs/TauriArchitecturePrinciplesDesign.md` 与其他专题设计文档：原则层与专项边界。
5. `docs/modules/*`：模块级架构、API 和流程文档。
6. `.artifacts/ai/*`：活跃任务、阶段、进度、发现与恢复点。

不要把这些职责重新堆回 README。

---

## 11. 提交规则

1. 一次提交只解决一类问题。
2. 提交标题直接描述结果，不写空话。
3. 如果工作树里混有用户自己的前端改动，使用路径限定 `git status --short -- <paths>` 检查后再暂存。
4. 不要把 lockfile、docs、前端原型和 backend wiring 无差别混进同一提交，除非它们属于同一个最小闭环。
5. 提交前先跑最窄验证；如果没有可执行验证，再退回 docs-only 回读和 `git diff --check`。

---

## 12. 什么时候先拆任务

满足下面任一条件时，建议先拆成更小的原子任务：

1. 同时改 `app/`、`src-tauri/` 和 `crates/`
2. 同时改 deep docs、workflow 协议和实现代码
3. 同时改 workspace manifest、transport wiring 和多个模块边界
4. 无法一句话说清“最便宜的验证动作”

对于 AI 主导执行的复杂任务，拆分规则和恢复语义以 `docs/TauriAIDevelopmentTransactionProtocolDesign.md` 为准。

---

## 13. 最后一条规则

如果你发现自己需要反复解释“这个改动到底该落哪儿”，那通常不是解释问题，而是入口层还不够清晰。

这时优先补边界和落点说明，而不是继续在总控文件里加逻辑。