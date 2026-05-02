# Tauri AI Development Transaction Protocol Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`
> Scope note: this document defines the new project's self-contained AI development protocol for complex tasks, context recovery, and atomic task execution.
> Storage note: this document is intentionally stored under `.artifacts/` and must not be included in git commits.

---

## 1. Purpose

本文档定义新项目的 AI 开发规范，目标不是“教 AI 如何回答”，而是约束 AI 如何在长任务中稳定执行、稳定恢复、稳定验证，并把上下文溢出风险降到可控范围。

核心问题不是模型偶尔犯错，而是复杂任务一旦跨越过多步骤、过多文件、过多未验证假设，就会出现以下退化：

1. 忘记最初目标和边界
2. 在错误假设上继续推进
3. 未完成任务被压缩后缺少可靠恢复点
4. 为了追求连续性而跨模块、跨主题扩写，最终引入幻觉型 bug

因此，本项目采用事务式任务协议：

- 大任务必须先拆成原子任务
- 任一时刻只允许一个活跃原子任务
- 每个原子任务必须有显式的开始、执行、验证、提交、挂起语义
- 重要状态必须写入本地文件，而不是只留在对话上下文里

---

## 2. Non-goals

以下内容不属于本协议的目标：

1. 不试图替代工程架构文档、模块 README 或接口文档
2. 不试图让所有简单任务都走完整事务流程
3. 不要求每个任务都引入复杂的多代理编排
4. 不要求依赖额外交接文档才能运行本协议

---

## 3. Core Decisions

### 3.1 Single Active Atomic Task

任一时刻只允许一个 `active task` 处于 `in_progress` 或 `validating` 状态。

原因：

1. 并行执行多个复杂任务会稀释注意力窗口
2. 一个任务未验证完就切到另一个任务，最容易产生“半完成状态被误判为已完成”
3. 对恢复流程来说，单活跃任务比多任务并行更容易重建现场

### 3.2 Complex Work Must Be Split First

满足以下任一条件时，任务必须先拆分，不能直接开始实现：

1. 预计超过 5 次工具调用
2. 会跨越多个模块或多个职责边界
3. 需要两个及以上不同验证动作
4. 需要创建或修改 4 个以上文件
5. 当前任务无法用一句话描述“最便宜的验证动作”

### 3.3 Atomic Task Means a Verifiable Local Slice

原子任务不是“功能大目标”，而是一个局部、可验证、可挂起、可恢复的最小闭环。

一个合格的原子任务必须满足：

1. 只围绕一个局部目标展开
2. 只依赖少量必要上下文
3. 能写出一个明确假设
4. 能写出一个最便宜的验证动作
5. 完成后能留下清晰的下一步入口

反例：

- “完成下载系统”不是原子任务
- “定义下载 planner 的 port 和 facade，并让对应契约测试通过”才是原子任务

---

## 4. Atomic Task Model

### 4.1 Required Fields

每个原子任务至少包含以下字段：

| Field | Meaning |
|------|---------|
| `id` | 稳定任务编号，例如 `AT-003` |
| `title` | 简洁任务标题 |
| `goal` | 该任务完成后的局部结果 |
| `scope` | 限定修改范围和不在范围内的内容 |
| `allowed_files` | 允许编辑的文件或目录 |
| `required_context` | 开始前必须读取的文档/文件 |
| `hypothesis` | 当前任务的主假设 |
| `cheap_check` | 最便宜的验证动作 |
| `status` | `ready` / `in_progress` / `validating` / `committed` / `blocked` / `aborted` |
| `next_step` | 如果被中断，恢复时第一步该做什么 |

### 4.2 Optional Fields

以下字段建议支持：

| Field | Meaning |
|------|---------|
| `dependencies` | 前置原子任务 |
| `notes` | 只保留高价值备注 |
| `risks` | 本任务已知风险 |
| `validation_result` | 最近一次验证结果摘要 |

### 4.3 State Machine

```text
drafted -> ready -> in_progress -> validating -> committed
                    |              |
                    |              -> blocked
                    |
                    -> aborted
```

状态定义：

1. `drafted`: 仅完成草拟，尚未具备开工条件
2. `ready`: 必读上下文、假设和验证动作均已明确
3. `in_progress`: 已进入实现或写作
4. `validating`: 已完成本轮修改，正在执行聚焦验证
5. `committed`: 本任务闭环完成，结果已写回进度文件
6. `blocked`: 因依赖缺失、环境问题或决策未决而暂停
7. `aborted`: 当前方案被否定，需要新建后继任务处理

---

## 5. Local File Protocol

### 5.1 Storage Location

本项目建议把 AI 任务状态统一放在本地目录：

```text
.artifacts/
└─ ai/
   ├─ active-task.md
   ├─ task-plan.md
   ├─ progress.md
   ├─ findings.md
   └─ handoff.md
```

理由：

1. 属于本地工作记忆，不应污染源码目录主结构
2. 与 `.artifacts/docs/` 一样天然适合非提交草稿和会话状态
3. 方便新项目直接复制同一套目录约定

### 5.2 File Responsibilities

| File | Role | Update Rule |
|------|------|-------------|
| `active-task.md` | 当前唯一活跃原子任务 | 每次任务切换、挂起、验证、提交时更新 |
| `task-plan.md` | 更高层阶段和原子任务队列 | 创建新任务或完成阶段时更新 |
| `progress.md` | 追加式执行日志 | 每个关键动作后追加 |
| `findings.md` | 研究、外部资料、长摘要 | 每完成 2 次只读探索后落盘 |
| `handoff.md` | 会话压缩前的恢复入口 | 会话结束、即将压缩、出现阻塞时更新 |

### 5.3 Trust Boundary

为了避免不可信内容反复污染上下文，文件必须分层：

1. `active-task.md` 和 `task-plan.md` 只写可信、结构化、短文本
2. 外部网页、搜索结果、长研究摘录只能进 `findings.md`
3. `handoff.md` 只写恢复执行需要的信息，不复制大段外部材料

---

## 6. Transaction Lifecycle

### 6.1 Begin

开始复杂任务前必须执行：

1. 读取 `active-task.md`、`task-plan.md`、`progress.md` 的尾部
2. 若存在未完成活跃任务，先判断继续、阻塞还是中止
3. 写入新的原子任务记录或显式复用当前任务
4. 明确 `required_context`、`hypothesis`、`cheap_check`

没有 `cheap_check` 的任务，不得进入 `in_progress`。

### 6.2 Execute

执行阶段必须遵守：

1. 只围绕当前原子任务的范围行动
2. 每完成 2 次只读探索，必须把关键发现写入 `findings.md` 或 `progress.md`
3. 一旦发现任务开始跨出边界，立即拆分为新原子任务，而不是继续扩写当前任务

### 6.3 Validate

第一次实质编辑后，必须立刻做一次聚焦验证，优先级如下：

1. 当前假设最便宜的行为级检查
2. 受影响切片的窄测试
3. 受影响切片的编译、lint 或类型检查
4. 若没有可执行验证，再退回结构化回读

### 6.4 Commit

只有满足以下条件，任务才能标记为 `committed`：

1. 聚焦验证已完成且结果明确
2. `progress.md` 已记录本轮动作和结果
3. `active-task.md` 已写入下一步入口或标记本任务完成

### 6.5 Suspend

出现以下情况必须挂起并写 `handoff.md`：

1. 上下文接近饱和
2. 当前假设被证伪但还未形成替代方案
3. 环境、权限、依赖导致无法继续
4. 用户中断当前任务并切换方向

挂起时必须至少记录：

1. 已完成内容
2. 未完成内容
3. 最后一条可靠事实
4. 恢复时的第一步动作

---

## 7. Protocol Layers

本规范不应只靠一个 skill 承担全部责任，而应拆成三层：

### 7.1 Always-on Project Instructions

负责项目级硬规则，例如：

1. 开始编码前必须先读取当前活跃任务和必要上下文
2. 不允许跨模块猜实现
3. 第一次实质编辑后必须立即执行聚焦验证
4. 复杂任务必须先拆分为原子任务

### 7.2 On-demand Skill

负责复杂任务工作流，例如：

1. 初始化 `.artifacts/ai/` 目录与模板
2. 生成原子任务记录
3. 在挂起或恢复时输出所需文件模板
4. 把复杂任务压成小任务队列

### 7.3 Hooks

负责轻量自动化提醒或校验，例如：

1. 会话开始时提示读取 `active-task.md`
2. 编辑后提醒更新 `progress.md`
3. 停止前检查是否仍有 `in_progress` 任务未写入 `handoff.md`

---

## 8. Minimal Templates

### 8.1 active-task.md

```markdown
# Active Task

- id: AT-001
- title: 定义 AI 事务协议文件模型
- status: in_progress
- goal: 固化 active-task/task-plan/progress/findings/handoff 的职责分工
- scope: 只修改 AI 规范文档，不改业务代码
- allowed_files: .artifacts/docs/TauriAIDevelopmentTransactionProtocolDesign.md
- required_context:
  - .artifacts/docs/TauriRewriteArchitectureBlueprint.md
- hypothesis: 通过单活跃任务 + 文件分层可显著降低复杂任务的恢复成本
- cheap_check: 回读文档并确认规则完整、无外部依赖引用
- next_step: 补全原子任务状态机与 hooks 落点
```

### 8.2 handoff.md

```markdown
# Handoff

- active_task: AT-001
- state: blocked
- completed:
  - 已完成文件协议和状态机草案
- remaining:
  - 尚未细化 hooks 与模板初始化脚本
- last_reliable_fact:
  - 文档主结构已稳定，当前缺实现切片
- resume_first_step:
  - 读取 active-task.md，然后补 9.1 的切片 A4
```

---

## 9. Implementation Slices

本协议自身的落地也必须拆成小原子任务，建议顺序如下：

| Slice | Goal | Output |
|------|------|--------|
| A1 | 定义协议目标、边界与非目标 | 本文档第 1-2 节 |
| A2 | 定义原子任务模型和状态机 | 本文档第 4 节 |
| A3 | 定义本地文件协议和信任边界 | 本文档第 5 节 |
| A4 | 定义 begin/execute/validate/commit/suspend 生命周期 | 本文档第 6 节 |
| A5 | 定义 instructions / skill / hooks 的职责切分 | 本文档第 7 节 |
| A6 | 产出最小模板和恢复样例 | 本文档第 8 节 |
| A7 | 设计真正的 skill 文件结构与命令入口 | 后续专项稿 |
| A8 | 设计 hooks 脚本与自动检查策略 | 后续专项稿 |

每个切片应单独完成、单独验证、单独记录进度，而不是在一个会话里把 A1-A8 全部混成一个大任务。

---

## 10. Acceptance Criteria

当以下条件满足时，可以认为这份协议设计稿达到可落地状态：

1. 已定义“何时必须拆分任务”的明确触发条件
2. 已定义原子任务最小字段集合
3. 已定义单活跃任务约束
4. 已定义文件职责与信任边界
5. 已定义挂起与恢复时必须记录的信息
6. 已把真正的实现工作拆成后续小切片

---

## 11. Next Recommended Drafts

在本稿之后，最合适继续补两份小稿：

1. `TauriAIDevelopmentSkillFileLayoutDesign.md`
   - 只定义 skill 文件、模板、脚本、hooks 的目录布局
2. `TauriAIDevelopmentHooksAndRecoveryDesign.md`
   - 只定义会话开始、编辑后、停止前的 hook 行为与恢复策略

这样可以继续保持每个后续任务都足够小，不把“协议设计”和“文件布局/脚本行为”压进同一个上下文切片。