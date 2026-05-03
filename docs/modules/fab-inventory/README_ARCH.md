# Fab Inventory Module Architecture

> Module: fab-inventory
> Status: draft
> Owner scope: frontend

---

## 1. Purpose

fab-inventory 模块负责展示 Fab 资产库存的概览与详情，表达库存搜索、筛选、同步和导入工程的交互意图。

---

## 2. Responsibilities

本模块负责：

- 展示库存页的列表、分类、详情和缓存管理入口
- 在首页卡片与独立库存页中复用同一业务主题
- 呈现资产状态、兼容性和依赖信息
- 预留同步库存、搜索、筛选、导入等后续真实操作入口

本模块不负责：

- 实际调用远程 Fab API 或本地缓存层
- 维护真实库存同步状态和下载状态机
- 决定工程兼容性的最终业务规则

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| assets mock data | fab-inventory | 当前库存页中的本地展示数据源，占位用 |
| metaRows | fab-inventory | 当前选中资产的展示型元数据 |
| deps | fab-inventory | 当前资产依赖展示列表 |
| fabItems | fab-inventory | 首页卡片中的库存概览数据 |

当前模块还没有交互性本地状态；真实选中项和筛选状态接入后，应仍由 fab-inventory 自己拥有。

---

## 4. Boundary Rules

允许依赖：

- components/FabInventory.tsx 作为首页概览卡片
- components/FabInventoryContent.tsx 作为库存页主体
- shell 模块提供的页面装配入口

禁止依赖：

- 其他业务模块的内部状态
- 下载或工程模块的具体业务实现

禁止的跨边界行为：

- 直接在组件内部接入未抽象的远程 API 细节
- 由首页卡片反向控制库存页内部详情逻辑
- 在 fab-inventory 里实现实际导入、下载或缓存修复流程

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| summary card | components/FabInventory.tsx | 首页中的简化库存概览 |
| main inventory view | components/FabInventoryContent.tsx | 独立库存页列表、工具栏与详情面板 |
| mock inventory model | components/FabInventoryContent.tsx | 暂以内联常量形式存在 |
| preview metadata | components/FabInventoryContent.tsx | 当前选中资产的状态、兼容性和依赖说明 |

---

## 6. Coding Constraints

1. 库存列表、详情和同步操作入口必须保持为同一业务模块，不要散落到无关页面。
2. 在接入真实数据前，展示型 mock 数据可以内联；一旦出现跨文件复用，应提升为模块内独立数据层。
3. 搜索、筛选、同步和导入按钮只能表达用户意图，真实副作用需通过后续契约层下沉。
4. 首页 FabInventory 卡片只能提供概览，不应复制库存页的全部行为和状态机。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把库存展示和其他业务模块耦合在了一起
2. 是否引入了新的共享状态而没有明确 owner
3. 是否让 UI 组件承担了同步、下载或兼容性判断等业务逻辑
4. 是否需要同步更新 README_API.md 或 README_FLOW.md