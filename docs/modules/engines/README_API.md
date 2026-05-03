# Engines Module API

> Module: engines
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| EngineManagement | component | components/EngineManagement.tsx | 首页中的引擎状态与动作摘要卡片 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前组件不接收外部 props |

当前模块仍然是静态展示入口。后续若接入多引擎列表、选中状态或验证任务，应优先显式增加 props、hooks 或模块契约，而不是依赖隐式外部状态。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| engine summary card | rendered view | 首页渲染时 | 展示当前引擎版本与容量摘要 |
| launch intent | button click target | 用户点击启动时 | 当前只提供 UI 入口 |
| install-version intent | button click target | 用户点击安装版本时 | 当前只提供 UI 入口 |
| plugin compatibility summary | rendered list | 首页渲染时 | 展示插件 Ready 或 Update 状态 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| current engine labels | read | local | Unreal Engine 5.4.4 与体积文案 |
| plugins | read | local | 插件兼容性 mock 数据 |
| TauriEngineVerificationRepairDesign | semantic alignment | stable | 约束未来验证与修复契约边界 |

---

## 5. Error and Empty States

- 当前没有 loading、empty 或 error 状态；后续接入真实引擎列表后必须补齐。
- 当前默认假设总有一个可展示的当前引擎版本；未来若用户尚未安装引擎，应有明确空态。
- 启动、安装版本、验证与修复入口当前都没有失败反馈路径；接入后应分别建模为可回显的用户操作。
- 插件兼容性当前只有 Ready 和 Update 两类文案；未来应收敛成稳定状态枚举，而不是散落字符串。

---

## 6. Compatibility Notes

- 未来如果扩展为多引擎管理页，应保留当前组件作为摘要视图，而不是直接把复杂列表逻辑塞回首页卡片。
- engines 模块面向前端暴露的结果应保持为引擎级或组件级摘要，不直接泄露文件级差异明细。
- 若接入验证与修复能力，应和 docs/TauriEngineVerificationRepairDesign.md 中的 orchestrator 边界保持一致。