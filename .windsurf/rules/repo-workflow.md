# MyEpicLauncher Windsurf Rules

本文件是当前仓库面向 Windsurf 的仓库级规则入口。
它把现有的严格文档驱动流程映射成 Windsurf 可直接消费的纯文本规则，避免依赖 Copilot 专用的 skill frontmatter。

## Source Of Truth

以下文件仍然是规范来源，本文件只是兼容性投影，不是新的事实源：

1. `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
2. `docs/TauriAIContextManagementIntegrationDesign.md`
3. `.github/copilot-instructions.md`
4. `.github/skills/strict-doc-driven-development/SKILL.md`
5. `.artifacts/ai/README.md`

如果本文件与以上文件冲突，以以上文件为准，并应先修正规范缺口，而不是即兴发挥实现。

## Required Working Mode

1. 把仓库文档当成有约束力的工程规范，而不是可选参考。
2. 非琐碎任务开始前，先识别控制性文档，再决定实现路径。
3. 大多数实现任务至少先读以下基线文档：
   - `docs/TauriRewriteArchitectureBlueprint.md`
   - `docs/TauriArchitecturePrinciplesDesign.md`
   - `docs/TauriAIDevelopmentTransactionProtocolDesign.md`
   - `docs/TauriTestingStrategyAndQualityGateDesign.md`
4. 如果任务涉及 AI 工作流、自定义 agent、上下文恢复或计划文件，还要补读：
   - `docs/TauriAIContextManagementIntegrationDesign.md`
   - `.github/copilot-instructions.md`
   - `.github/skills/strict-doc-driven-development/SKILL.md`
5. 如果文档缺失、过期或彼此冲突，不要自行发明架构；先暴露缺口，或先更新文档。

## Single Source Of Truth

复杂任务的唯一权威记录集合固定为：

- `.artifacts/ai/active-task.md`
- `.artifacts/ai/task-plan.md`
- `.artifacts/ai/progress.md`
- `.artifacts/ai/findings.md`
- `.artifacts/ai/handoff.md`

规则：

1. 不要把根目录 legacy `task_plan.md`、`progress.md`、`findings.md` 当成活跃事实源。
2. 不要创建第二套计划系统来替代 `.artifacts/ai`。
3. 即使 Windsurf 提供额外 memory、notes 或 planning 能力，也只能作为辅助，不得覆盖 `.artifacts/ai`。

## Atomic Task Protocol

1. 任一时刻只允许一个活跃原子任务处于 `in_progress` 或 `validating` 状态。
2. 满足以下任一条件时，必须先拆分原子任务，不能直接开写：
   - 预计超过 5 次工具调用
   - 会跨越多个模块或职责边界
   - 需要两个及以上不同验证动作
   - 需要修改 4 个以上文件
   - 当前任务说不清最便宜的验证动作
3. 每个原子任务都要写清：
   - goal
   - scope 和 non-goals
   - allowed files
   - controlling docs
   - falsifiable hypothesis
   - cheap check
4. 没有 cheap check 的任务，不得进入实现阶段。
5. 每完成 2 次只读探索，要把关键结论写入 `.artifacts/ai/progress.md` 或 `.artifacts/ai/findings.md`。

## Validation Order

第一次实质编辑后，下一步必须立刻做聚焦验证，优先级如下：

1. 当前假设最便宜的行为级检查
2. 受影响切片的窄测试
3. 受影响切片的编译、lint 或类型检查
4. 若没有可执行验证，再退回结构化回读和 scoped `git diff --check`

禁止：

1. 用更大的全量构建替代本该运行的窄验证。
2. 在第一次实质编辑后继续扩写多个切片而不先验证。
3. 用新的 planning 文件来规避 `.artifacts/ai` 的记录要求。

## Completion And Failure Handling

1. 验证通过后，必须更新 `.artifacts/ai` 记录，再提交 git commit，并尝试 push。
2. 如果 push 失败，不要丢弃 commit；把 pending push 状态写回 `.artifacts/ai`。
3. 如果同一阻塞修复 5 次仍失败，记录阻塞详情后停止，等待用户决策。
4. 如果上下文接近饱和或需要暂停，先刷新 `.artifacts/ai/handoff.md`。

## Architecture Guardrails

1. 前端只负责 UI 和用户意图编排；后端负责业务真相、状态机、IO、长任务和持久化。
2. 不要把 backend-owned truth 搬回前端。
3. Tauri 和宿主 transport 不是业务核心；复杂逻辑必须下沉到后端模块或 facade。
4. 模块之间只通过稳定契约协作，不要直接耦合别的模块内部实现或存储。
5. wiring 属于 composition root，不要在 command handler 或模块内部偷偷 new 具体实现。

## Comment And Language Rules

1. 默认使用中文输出和中文新增注释。
2. 已有正确英文注释应保留，不要为了统一语言而无差别重写。
3. 注释工作优先补缺失声明注释，不要顺手改行为。

## Windsurf-specific Notes

1. `.github/skills/strict-doc-driven-development/SKILL.md` 中的 `name`、`user-invocable`、`allowed-tools` 属于 Copilot skill 元数据，不应直接当成 Windsurf 的原生配置格式。
2. 在 Windsurf 中，应把本文件视为对该 skill 的操作性翻译，而不是要求 Windsurf 原生执行 `.github/skills`。
3. `.windsurf/rules/` 是当前仓库唯一的 Windsurf 规则落点；不要同时维护根 `.windsurfrules`，以免形成双入口漂移。