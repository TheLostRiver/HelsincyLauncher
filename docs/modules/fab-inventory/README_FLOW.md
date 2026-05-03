# Fab Inventory Module Flow

> Module: fab-inventory
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 首页浏览库存概览 | 用户停留在 home 页面 | 看到库存总量和代表性资产条目 | 若无数据，将来应显示空概览态 |
| 打开库存页 | 用户切换到 fab 页面 | 看到完整库存列表、工具栏和详情面板 | 若加载失败，将来应显示页级错误态 |
| 发起库存同步意图 | 用户点击“同步库存” | 进入后续真实同步流程入口 | 当前未接线，只能停留为按钮占位 |
| 发起资产导入意图 | 用户点击“导入工程” | 进入后续导入/兼容性校验流程入口 | 当前未接线，只能停留为按钮占位 |

---

## 2. Happy Path

1. 用户在首页看到 FabInventory 概览卡片。
2. 切换到 fab 页面后，FabInventoryContent 渲染工具栏、分类、资产列表和详情面板。
3. 页面默认展示一个被标记为 selected 的资产及其详情。
4. 用户可感知到库存规模、资产状态、兼容引擎和依赖信息。
5. 页面提供同步、筛选、导入和缓存管理等后续操作入口。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| no inventory data | 未来应显示空列表和空详情态，而不是渲染假数据 |
| sync in progress | 未来应在按钮或页头明确反馈同步状态 |
| incompatible asset | 当前以文案展示兼容性；未来应投影为稳定状态标签 |
| missing dependency | 详情面板应保留依赖缺失可见性，不能静默吞掉 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| home summary | navigate to fab page | inventory detail page visible | 由 shell 切页驱动 |
| selected asset A | future asset click | selected asset B | 未来需要显式本地选中状态 |
| idle | future sync request | syncing | 未来需要真正的同步状态机 |
| compatible | dependency mismatch | warning state | 未来应从文案过渡到结构化状态 |

---

## 5. UI and Side-effect Notes

- 当前模块全部行为都是静态渲染，没有真实副作用。
- 同步库存、筛选、导入工程、管理缓存按钮都应被视为占位入口，不应让调用方误判为已具备业务能力。
- 库存页已经隐含列表区与详情区双栏结构，未来接入真实状态时要避免让两侧各自维护一套不一致的选中状态。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 首页概览卡片仍然只展示摘要而不是完整库存页逻辑
2. 库存页仍包含列表、详情和操作入口三部分
3. 资产状态、兼容性和依赖信息仍然可见
4. 新接入的真实行为没有把同步或导入逻辑直接塞回展示层