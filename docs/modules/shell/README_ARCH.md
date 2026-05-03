# Shell Module Architecture

> Module: shell
> Status: draft
> Owner scope: frontend

---

## 1. Purpose

shell 模块负责承载启动器当前的顶层工作区框架：导航、页面切换、标题区以及内容区装配。

---

## 2. Responsibilities

本模块负责：

- 定义当前可导航页面集合与页面元数据
- 持有当前活动页面与切换中的视图状态
- 组合 Sidebar、TopBar 与页面内容区域
- 提供一致的页面切换过渡体验

本模块不负责：

- 具体业务页面内部的数据建模与展示细节
- 下载、库存、引擎、账号等业务逻辑
- 持久化当前页面状态或读取外部数据

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| activePage | shell | 当前唯一的页面真相源，驱动导航激活态与内容装配 |
| transitioning | shell | 仅用于页面切换时的临时视图过渡状态 |
| pageMeta | shell | 页面标题与 kicker 映射，属于壳层展示配置 |

当前 shell 不拥有业务数据状态，只拥有页面级视图状态。

---

## 4. Boundary Rules

允许依赖：

- app/page.tsx 作为壳层入口
- components/Sidebar.tsx
- components/TopBar.tsx
- 页面级内容组件，如 HomeContent、MyProjectsContent、FabInventoryContent

禁止依赖：

- 任意页面内部的局部实现细节
- 后续真实后端接入时的底层 IPC 调用细节

禁止的跨边界行为：

- shell 直接操纵某个业务模块的内部状态
- Sidebar 或 TopBar 自行决定页面内容装配逻辑
- 在 shell 中堆积具体业务规则或远程数据请求

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| entry | app/page.tsx | 组合壳层并持有页面切换状态 |
| navigation | components/Sidebar.tsx | 定义当前导航项及点击行为 |
| header | components/TopBar.tsx | 展示页面标题、搜索占位与账户入口 |
| page composition | app/page.tsx | 根据 activePage 决定挂载哪个页面内容组件 |

---

## 6. Coding Constraints

1. shell 只维护页面级 UI 状态，不承接业务模块真实状态。
2. 新增页面时，优先扩展 PageId、pageMeta 和内容装配映射，不要在多个位置散落条件分支。
3. Sidebar 只能表达导航意图，不能直接导入和操作页面内容组件的内部状态。
4. TopBar 保持为展示型壳层组件；真实搜索、主题和账户逻辑接入后，应通过明确契约注入。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否新增或变更了页面级导航边界
2. 是否把业务逻辑错误地提升到了 shell
3. 是否保持了 activePage 作为唯一页面切换真相源
4. 是否需要同步更新 README_API.md 或 README_FLOW.md