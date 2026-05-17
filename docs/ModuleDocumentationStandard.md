# Module Documentation Standard

> Status: working draft
> Purpose: define which modules in this repository need dedicated documentation, what each module document must contain, and where those documents should live.

---

## 1. Short Answer

有必要，但不是给每个小组件都机械写一套文档。

应当优先为“承担边界、状态、交互流、业务语义”的模块写文档；纯展示型叶子组件只在复杂度上升时再补充。

---

## 2. What Counts as a Module Here

在当前仓库里，模块应优先按用户可感知的业务切片和壳层边界划分，而不是按单个 React 组件文件划分。

当前建议的模块集合：

| Module | Current Surface |
|--------|-----------------|
| shell | app/page.tsx, components/Sidebar.tsx, components/TopBar.tsx |
| home | components/HomeContent.tsx, components/HeroProject.tsx |
| projects | components/MyProjectsContent.tsx |
| fab-inventory | components/FabInventoryContent.tsx, components/FabInventory.tsx |
| downloads | components/Downloads.tsx |
| engines | components/EngineManagement.tsx |
| settings | components/Settings.tsx |
| account-auth | components/AccountLogin.tsx |

不建议单独建模块文档的对象：

- 纯视觉包装组件
- 无独立状态和契约的简单局部子组件
- 只被一个模块内部消费、且职责非常直白的渲染片段

---

## 3. Minimum Required Docs Per Module

每个正式模块至少维护三份文档：

1. README_ARCH.md
2. README_API.md
3. README_FLOW.md

如果模块已经接入或即将接入后端实现、IPC、adapter、runtime 或持久化边界，还必须补充：

4. README_IMPL.md

职责如下：

| File | Must Describe |
|------|---------------|
| README_ARCH.md | 模块职责、边界、拥有的状态、依赖方向、禁止事项 |
| README_API.md | 对外暴露的 props、hooks、commands、events、数据契约 |
| README_FLOW.md | 关键交互流、状态迁移、异常分支、加载与空态处理 |
| README_IMPL.md | 代码落点、实现切片顺序、port/adapter 状态、验证门槛 |

建议位置：

```text
docs/
└─ modules/
   └─ <module-name>/
      ├─ README_ARCH.md
      ├─ README_API.md
      ├─ README_FLOW.md
      └─ README_IMPL.md   # backend/IPC/adapter/runtime 模块必须有
```

---

## 4. Documentation Budget

模块文档有预算。它们默认只记录长期有效的边界、契约和实现顺序，不记录每个原子任务的完整过程。

1. `.artifacts/ai/` 记录任务过程、验证命令、提交号、handoff 和临时发现。
2. `docs/` 只记录会影响后续实现者决策的稳定事实：模块边界、公开契约、数据模型、错误语义、wiring 规则、验证门槛。
3. 普通实现任务不强制更新模块文档；只有改变长期边界、接口、数据模型、错误码、依赖方向或验证规则时才更新。
4. `README_IMPL.md` 不再为每个 AT 追加长篇 `Completed by ...` 流水账；需要记录时优先压缩为短表或 3-5 行“当前状态 / 下一边界”。
5. `out of scope` 不要在多份文档里反复复制大段清单；完整任务排除项放在 `.artifacts/ai/active-task.md`，模块文档只保留长期禁止事项。
6. 新增文档章节前，先判断能否更新既有章节、短表或 `.artifacts/ai` 记录；如果只是执行日志，就不要写进 `docs/`。

---

## 5. Coding Constraints That Should Be Written Down

每个模块文档都应该明确写清这些约束，而不是默认大家“口头理解”：

1. 模块负责什么，不负责什么
2. 允许依赖哪些外部模块或共享层
3. 禁止跨模块直接读写哪些状态
4. 哪些数据是权威源，哪些只是派生视图
5. 异步请求、缓存、错误处理由哪一层负责
6. 允许暴露哪些公共入口，哪些实现细节不得被外部引用
7. UI 组件命名、props 设计、状态提升和副作用放置规则
8. 最低测试要求和回归检查清单

如果这些约束不写出来，模块会很快退化成“文件分开了，但边界仍然是糊的”。

---

## 6. Documentation Threshold

满足任一条件，就应升级为正式模块文档：

1. 有独立路由或独立导航入口
2. 有自己的加载、空态、错误态、交互流
3. 有跨文件状态或数据流
4. 后续会接后端契约或 IPC
5. 可能被多人或 AI 单独修改

若暂时不满足这些条件，可以只保留简短文件头注释或在所属模块文档中登记，不需要独立成册。

---

## 7. Rollout Order for This Repo

优先级建议如下：

1. shell
2. fab-inventory
3. downloads
4. engines
5. projects
6. account-auth
7. settings
8. home

排序依据不是页面先后，而是边界复杂度、未来演进风险、以及最可能先接入真实数据和后端契约的程度。

---

## 8. Practical Rule

结论可以压缩成一句话：

给每个业务模块写文档是必要的，给每个小组件写文档则没有必要。

当前仓库最合适的做法是：

1. 先建立统一模板和约束
2. 再按模块逐步补 README_ARCH / README_API / README_FLOW
3. 对已经接入后端或即将写后端代码的模块补 README_IMPL
4. 不把纯展示组件也硬拉进同一套重量级文档流程
