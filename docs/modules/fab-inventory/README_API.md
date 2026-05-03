# Fab Inventory Module API

> Module: fab-inventory
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| FabInventory | component | components/FabInventory.tsx | 首页概览卡片 |
| FabInventoryContent | component | components/FabInventoryContent.tsx | 独立库存页主视图 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前两个组件都不接收外部 props |

当前模块是完全静态的展示 API。后续若引入数据或动作，应优先显式增加 props 或 hooks，而不是读取外部隐式全局状态。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| visual inventory summary | rendered card list | 首页渲染时 | 展示资产数量与若干代表项 |
| visual inventory detail | rendered detail panel | 库存页渲染时 | 展示当前资产的状态、兼容性和依赖 |
| action intent buttons | button click target | 用户点击同步、筛选、导入等按钮时 | 当前仅提供 UI 入口，未绑定真实行为 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| fabItems | read | local | 首页概览数据 |
| assets | read | local | 库存页主列表数据 |
| metaRows | read | local | 详情面板元数据 |
| deps | read | local | 依赖状态展示 |

---

## 5. Error and Empty States

- 当前没有实现真正的 loading、empty 和 error 状态，这也是后续接入真实数据时必须优先补齐的缺口。
- 当前“selected”资产通过 mock 数据静态指定；未来若无选中资产，详情面板应有明确空态而不是假定永远有数据。
- 同步库存、筛选、导入工程、管理缓存等按钮目前没有错误路径；接入后应分别建模为可反馈的操作。

---

## 6. Compatibility Notes

- 首页概览卡片与独立库存页应保持同一命名和资产语义，避免同一库存概念出现两套词汇。
- 后续接入后端时，建议把列表 summary 与详情 read model 分层，不要把重实体直接塞给 UI。
- 若接入搜索、筛选和同步契约，需要保证首页卡片仍然只是摘要视图，而不是第二套完整库存控制台。