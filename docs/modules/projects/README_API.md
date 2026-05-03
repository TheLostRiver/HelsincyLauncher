# Projects Module API

> Module: projects
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| MyProjectsContent | component | components/MyProjectsContent.tsx | 我的工程页面主视图 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前组件不接收外部 props |

当前模块仍是静态展示入口。后续若接入真实工程列表、搜索、筛选和扫描动作，应优先显式增加 props、hooks 或模块契约。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| project list projection | rendered view | 页面渲染时 | 展示工程列表、引擎版本和最近打开摘要 |
| selected project detail | rendered view | 页面渲染时 | 展示选中工程详情、健康状态和操作入口 |
| open project intent | button click target | 用户点击打开时 | 当前只提供 UI 入口 |
| scan projects intent | button click target | 用户点击扫描本地工程时 | 当前只提供 UI 入口 |
| project ops intent | button click target | 用户点击复制路径、迁移、归档时 | 当前只提供 UI 入口 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| projects | read | local | 工程列表 mock 数据 |
| metaRows | read | local | 详情区工程元数据 |
| ops | read | local | 工程操作按钮定义 |
| health summary | read | local | 当前工程状态正常文案 |

---

## 5. Error and Empty States

- 当前没有 loading、empty 或 error 状态；后续接入真实工程扫描后必须补齐。
- 当前默认假设存在一个 selected 工程；未来若工程库为空，应有明确空态与扫描入口。
- 复制路径、迁移、归档、打开目录等操作当前没有失败反馈；接入后应能表达权限、路径和兼容性错误。
- 当前 disabled 项目只以视觉样式区分；未来应提升为稳定状态标签，例如 archived、needs-migration、unavailable。

---

## 6. Compatibility Notes

- projects 模块应保持“只管理 Unreal Engine 工程，不混入商城与资产库存”的语义边界。
- 与 engines、fab-inventory 的关系应通过稳定摘要字段表达，不直接耦合到对方内部组件和状态。
- 若后续出现独立工程详情页，当前页面仍应保留工程库总览和操作分发角色。