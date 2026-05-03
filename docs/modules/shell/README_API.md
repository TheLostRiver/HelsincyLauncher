# Shell Module API

> Module: shell
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| Home | page component | app/page.tsx | 当前启动器壳层入口 |
| Sidebar | component | components/Sidebar.tsx | 页面导航入口 |
| TopBar | component | components/TopBar.tsx | 顶部标题与操作区 |
| PageId | type | components/Sidebar.tsx | 当前页面标识联合类型 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| activePage | PageId | yes | Sidebar 用于决定激活态 |
| onNavigate | (page: PageId) => void | yes | Sidebar 通过它把导航意图上抛给 shell |
| kicker | string | yes | TopBar 上方小标题 |
| title | string | yes | TopBar 主标题 |

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| onNavigate(page) | callback invocation | 用户点击有效导航项时 | 只表达导航意图，不保证一定切页 |
| active page render | conditional component mount | activePage 变化后 | shell 根据页面 id 切换内容区 |
| transition class change | visual state change | transitioning 切换时 | 控制透明度与位移过渡 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| pageMeta | read | local | 为 PageId 提供固定标题元数据 |
| HomeContent | call/render | stable | 首页内容组合入口 |
| MyProjectsContent | call/render | stable | 我的工程页内容入口 |
| FabInventoryContent | call/render | stable | Fab 库存页内容入口 |

---

## 5. Error and Empty States

- 当前 shell 没有显式错误态；未来如果内容区接入真实数据，错误态应由各业务模块自己拥有。
- 当前 shell 也没有“空壳”状态；至少应保证存在一个默认 activePage。
- 非法导航 id 不应从 Sidebar 外部构造；PageId 联合类型负责收敛此类错误。
- 切换中的短暂过渡失败不应演变为业务错误，只能退化为无动画切页。

---

## 6. Compatibility Notes

- PageId 是当前导航契约核心，新增页面时必须同步扩展 Sidebar 与 pageMeta。
- TopBar 当前是静态展示 API，未来接入真实搜索或账户逻辑时，应避免破坏现有 props 语义。
- 如果 shell 后续迁移到真正路由系统，需保留“壳层只负责装配，不负责业务真相”的边界。