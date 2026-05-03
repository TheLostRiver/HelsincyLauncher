# Settings Module API

> Module: settings
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| Settings | component | components/Settings.tsx | 首页中的设置摘要卡片 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前组件不接收外部 props |

当前模块仍是静态展示入口。后续若接入真实设置摘要、路径选择或设置变更，应优先显式增加 props、hooks 或模块契约。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| auto-update summary | rendered view | 首页渲染时 | 展示自动更新引擎设置 |
| verify-after-download summary | rendered view | 首页渲染时 | 展示下载后校验文件设置 |
| library path summary | rendered view | 首页渲染时 | 展示本地库路径 |
| toggle intent | click affordance | 用户点击开关时 | 当前只提供 UI 意图入口 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| auto update label | read | local | 自动更新引擎文案与开关样式 |
| verify label | read | local | 下载后校验文件文案与开关样式 |
| library path label | read | local | D:/EpicLibrary 路径摘要 |
| settings icon affordance | read | local | 顶部设置图标只表达入口存在 |

---

## 5. Error and Empty States

- 当前没有 loading、empty 或 error 状态；后续接入真实配置源后必须补齐。
- 当前默认假设总能读取到设置值和库路径；未来若配置缺失或路径不可用，应有明确降级状态。
- 开关当前没有失败反馈路径；接入真实更新后应能表达持久化失败、权限不足或配置无效。
- 路径摘要当前只展示静态字符串；未来应能表达未设置、不可访问或需要迁移等状态。

---

## 6. Compatibility Notes

- settings 模块应始终对外暴露聚合配置摘要，而不是底层配置文件结构。
- 若未来出现完整设置页面，当前首页卡片仍应保持摘要和快速入口角色。
- settings 与 downloads、engines 的关联应通过稳定配置项语义表达，而不是直接依赖对方内部状态。