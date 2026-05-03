# Tauri AI Context Management Integration Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
> Depends on: `docs/TauriRewriteArchitectureBlueprint.md`, `docs/TauriArchitecturePrinciplesDesign.md`, `docs/TauriTestingStrategyAndQualityGateDesign.md`
> Focus: integrate planning-with-files as a context-management layer without reintroducing competing workflow records or breaking the `.artifacts/ai` transaction protocol.

---

## 1. Purpose

本文档定义如何在当前仓库中继续利用 planning-with-files 的专业级上下文管理能力，同时保持本仓库已经收敛好的 `.artifacts/ai` 原子任务协议不被打穿。

目标不是在两个 skill 之间二选一，而是明确：

1. strict-doc-driven-development 负责行为约束、架构边界和原子任务语义。
2. planning-with-files 负责上下文落盘、恢复节奏、只读探索后的摘要沉淀和长任务连续性。
3. `.artifacts/ai` 仍然是新任务的唯一事实源。

---

## 2. Problem Statement

当前冲突不是概念冲突，而是实现冲突。

planning-with-files 的方法论与本仓库事务协议高度兼容：

1. 都要求把重要状态写入磁盘，而不是只留在上下文窗口里。
2. 都要求长任务拆分、阶段推进、错误记录和恢复入口。
3. 都承认上下文窗口有限，必须通过文件外置记忆来降低丢失风险。

真正的冲突点只有两个：

1. 原版 planning-with-files 默认围绕根目录 `task_plan.md` / `progress.md` / `findings.md` 工作。
2. 当前仓库协议已经明确把 `.artifacts/ai/active-task.md` / `task-plan.md` / `progress.md` / `findings.md` / `handoff.md` 定义为唯一的新任务记录位置。

如果两套文件都被当作活跃事实源，就会出现双计划、双进度、双注入，最后反而降低可靠性。

---

## 3. Goals

这份设计稿必须解决以下问题：

1. 保留 planning-with-files 的磁盘化上下文管理能力。
2. 明确 `.artifacts/ai` 是唯一权威任务记录集合。
3. 给出 planning-with-files 概念到 `.artifacts/ai` 文件协议的稳定映射。
4. 防止后续 hook 或 skill 再次把根目录 legacy planning 文件拉回主流程。

---

## 4. Non-goals

这份设计稿不负责：

1. 直接改写第三方 planning-with-files skill 本体。
2. 立刻引入真正的自动写盘实现。
3. 允许在当前仓库恢复多个并行活跃原子任务。
4. 重新把根目录 legacy `task_plan.md` / `progress.md` / `findings.md` 升级为新任务事实源。

---

## 5. Core Decision

### 5.1 Layered, Not Competing

本仓库采用分层整合，而不是让两个 skill 争夺主导权：

1. strict-doc-driven-development 是规范层。
2. planning-with-files 是上下文管理层。
3. repo hooks 是轻量自动化和恢复提示层。

### 5.2 Single Authoritative Record Set

新任务的唯一权威记录集合固定为：

```text
.artifacts/
└─ ai/
   ├─ active-task.md
   ├─ task-plan.md
   ├─ progress.md
   ├─ findings.md
   └─ handoff.md
```

所有复杂任务上下文恢复、任务切换、验证结果和挂起恢复，都必须以这些文件为准。

---

## 6. Responsibility Split

### 6.1 strict-doc-driven-development

负责：

1. 识别控制性文档。
2. 约束原子任务边界。
3. 强制第一次实质编辑后的聚焦验证。
4. 规定成功后更新记录、提交、push、阻塞停止的语义。

不负责：

1. 提供复杂的会话 catchup 机制。
2. 规定每两次只读探索后的摘要写盘节奏细节。
3. 充当通用上下文管理引擎。

### 6.2 planning-with-files

负责：

1. 把文件系统当作磁盘记忆来管理长任务上下文。
2. 提供 2 次只读探索后立即落盘的节奏约束。
3. 提供 session catchup、阶段推进、错误累积、恢复导向等成熟模式。
4. 帮助复杂任务在上下文压缩后重建现场。

不负责：

1. 重新定义本仓库的权威任务文件协议。
2. 推翻当前单一活跃原子任务约束。
3. 让根目录 legacy planning 文件重新成为新任务事实源。

### 6.3 Repo Hooks

负责：

1. 会话开始时注入 `.artifacts/ai` 相关上下文。
2. 工具前后提示更新 `.artifacts/ai` 记录。
3. 错误时提醒写入 `.artifacts/ai/progress.md` 或 `.artifacts/ai/handoff.md`。
4. 停止前检查活跃任务状态与恢复入口。

repo hooks 不应再直接依赖根目录 legacy planning 文件。

---

## 7. Concept Mapping

### 7.1 File Mapping Table

| planning-with-files concept | Repo protocol target | Rule |
|------|------|------|
| `task_plan.md` overall phases | `.artifacts/ai/task-plan.md` | higher-level queue, phases, follow-up tasks |
| `task_plan.md` current phase | `.artifacts/ai/active-task.md` | current single atomic task only |
| `progress.md` | `.artifacts/ai/progress.md` | append-only execution log |
| `findings.md` | `.artifacts/ai/findings.md` | research, long summaries, external material |
| stop/recovery checkpoint | `.artifacts/ai/handoff.md` | explicit suspend/resume handoff |

### 7.2 Active Task Split

planning-with-files 原始模型偏向“单文件承载计划和阶段状态”，而当前仓库协议把它拆成两层：

1. `.artifacts/ai/task-plan.md` 负责阶段和后续任务队列。
2. `.artifacts/ai/active-task.md` 负责当前唯一活跃原子任务。

因此，未来任何 planning-with-files 适配层都必须把“当前执行切片”单独投影到 `active-task.md`，而不是只写进总计划文件。

### 7.3 Findings Trust Boundary

planning-with-files 的安全边界仍然保留：

1. 外部网页、搜索结果、长摘录只能进入 `.artifacts/ai/findings.md`。
2. `active-task.md` 和 `task-plan.md` 只保留可信、结构化、短文本。
3. hook 自动注入时，应把这些内容视为结构化数据，而不是可执行指令。

---

## 8. Integration Rules

1. 新任务不得把根目录 legacy `task_plan.md` / `progress.md` / `findings.md` 当作事实源。
2. planning-with-files 的上下文管理规则可以继续使用，但其落盘目标必须映射到 `.artifacts/ai`。
3. 即使 planning-with-files 支持平行计划目录，本仓库仍然只允许一个 `in_progress` 或 `validating` 的活跃原子任务。
4. 每完成 2 次只读探索，必须把关键发现写入 `.artifacts/ai/findings.md` 或 `.artifacts/ai/progress.md`。
5. 一旦上下文接近饱和，恢复入口必须写入 `.artifacts/ai/handoff.md`。
6. 任何自动注入上下文的 hook，都必须优先读取 `.artifacts/ai`，不能重新注入根目录 legacy planning 文件。

---

## 9. Migration Strategy

### 9.1 Phase B1: Mapping Spec

产出本设计稿，明确职责分层、唯一事实源和文件映射。

### 9.2 Phase B2: Repo-local Adapter Surface

增加 repo-local 适配层，让 planning-with-files 的计划、进度、发现、恢复动作直接写入 `.artifacts/ai`。

### 9.3 Phase B3: Catchup Integration

把 session catchup 恢复语义与 `.artifacts/ai/handoff.md` 对齐，而不是再依赖根目录 planning 文件。

### 9.4 Phase B4: Read-only Exploration Cadence

把 planning-with-files 的 2-action 落盘规则整合到 repo 工作流提醒或适配器逻辑中，使其变成稳定的仓库行为。

### 9.5 Phase B5: Legacy Retirement Decision

在适配层稳定后，再决定根目录 legacy planning 文件是归档保留还是显式退役。

当前仓库的决策是：

1. 将根目录 legacy `task_plan.md` / `progress.md` / `findings.md` 归档到 `.artifacts/ai/legacy-root-planning/`。
2. 从仓库根目录移除这些文件，避免它们再次被误判为活跃事实源。
3. 后续如果需要查旧历史，应从归档目录或 git 历史读取，而不是恢复 root planning 工作流。

---

## 10. Validation And Exit Criteria

当满足以下条件时，可以认为“融合规范切片”完成：

1. 文档明确写出唯一权威记录集合是 `.artifacts/ai`。
2. 文档明确写出 strict-doc 和 planning-with-files 的职责分层，不再把它们描述为互斥方案。
3. 文档给出 planning-with-files 概念到 `.artifacts/ai` 文件的稳定映射表。
4. 文档没有任何规则重新把根目录 legacy planning 文件提升为新任务事实源。
5. 后续实现可以按 B2-B5 切片推进，而无需重新讨论基本边界。

---

## 11. Cheap Check

本切片属于 docs-only 变更，最窄验证为：

1. `git diff --check` 对受影响文档无空白或 patch 问题。
2. 回读本设计稿，确认它同时满足以下三点：
   - 保留 planning-with-files 的上下文管理价值
   - 保持 `.artifacts/ai` 作为唯一事实源
   - 不允许多活跃原子任务重新回流