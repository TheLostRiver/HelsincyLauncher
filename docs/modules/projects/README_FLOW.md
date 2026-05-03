# Projects Module Flow

> Module: projects
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 查看工程库 | 用户切换到 my-projects 页面 | 看到工程列表、统计摘要和选中工程详情 | 若未来无工程，应显示空库态 |
| 搜索或筛选工程意图 | 用户点击搜索或筛选控件 | 进入后续真实筛选流程入口 | 当前未接线，只停留在 UI 层 |
| 打开选中工程 | 用户点击打开按钮 | 进入后续真实工程启动流程入口 | 当前未接线，只停留在按钮层 |
| 扫描本地工程 | 用户点击扫描本地工程 | 进入后续真实工程发现流程入口 | 当前未接线，只停留在按钮层 |
| 执行工程操作 | 用户点击复制路径、迁移、归档 | 进入后续真实工程操作入口 | 当前未接线，只停留在按钮层 |

---

## 2. Happy Path

1. 用户进入 my-projects 页面。
2. 页面展示工程总数、主力引擎版本和工程列表。
3. 页面默认突出一个 selected 工程。
4. 右侧详情区展示该工程的路径、元数据、健康状态和可用操作。
5. 用户可以感知到工程当前可打开，并看到与引擎和 Fab 依赖相关的摘要信息。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| no projects found | 应显示空工程库状态和扫描引导 |
| disabled or legacy project | 应清楚标识为需迁移或不可直接打开 |
| project health degraded later | 应展示稳定健康摘要，而不是散落错误字符串 |
| multiple selection later | 选中工程切换应作为模块内局部状态管理 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| project list visible | future row click | selected project changed | 未来需要真实选中状态 |
| healthy project | future verification mismatch | needs attention | 应来自稳定后端投影 |
| selected project | click open | open requested | 当前只是 UI 意图状态 |
| selected project | click migrate/archive | operation requested | 当前只是 UI 意图状态 |

---

## 5. UI and Side-effect Notes

- 当前模块没有真实副作用，所有操作按钮都只是入口。
- 未来工程健康状态、引擎版本匹配和 Fab 依赖摘要都应来自后端事实投影，而不是前端自己拼装第二套真相。
- 项目列表与详情区是同一模块的两块视图，不应各自维护一套不一致的选中状态。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 工程列表、统计摘要和详情区仍然同时存在
2. 选中工程的视觉突出与详情区内容仍然一致
3. 工程健康状态和操作入口没有退化成含糊文案
4. 新接入的扫描、迁移或归档能力没有把后端流程重新塞回前端