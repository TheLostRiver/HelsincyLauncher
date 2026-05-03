# Downloads Module API

> Module: downloads
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| Downloads | component | components/Downloads.tsx | 首页中的下载任务摘要卡片 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前组件不接收外部 props |

当前模块仍是静态展示入口。后续若接入真实下载列表、任务控制或错误反馈，应优先显式增加 props、hooks 或模块契约。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| active download projection | rendered view | 首页渲染时 | 展示任务名称、进度百分比和进度条 |
| queued download projection | rendered view | 首页渲染时 | 展示等待中任务摘要 |
| pause-all intent | button click target | 用户点击暂停全部时 | 当前只提供 UI 入口 |
| open-folder intent | button click target | 用户点击目录按钮时 | 当前只提供 UI 入口 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| active download labels | read | local | City Sample Assets 与 68% 进度文案 |
| queued download labels | read | local | UE 5.5 Preview 与等待中文案 |
| throughput label | read | local | 顶部 42.8 MB/s 摘要 |
| TauriDownloadRuntimeDesign | semantic alignment | stable | 约束未来后端任务与调度边界 |

---

## 5. Error and Empty States

- 当前没有 loading、empty 或 error 状态；后续接入真实任务列表后必须补齐。
- 当前默认假设至少有一个活动任务和一个排队任务；未来若无下载任务，应显示空队列态。
- 暂停全部与打开目录入口当前没有失败反馈路径；接入后应能表达权限、路径或任务状态错误。
- 当前下载百分比和速率都是静态文案；未来应来自稳定任务快照，而不是页面内自行计算。

---

## 6. Compatibility Notes

- downloads 模块面向前端暴露的应始终是聚合任务快照，而不是内部细粒度运行时状态。
- 若后续扩展为独立下载页，首页卡片仍应保持摘要角色，而不是复制完整控制台。
- 与 docs/TauriDownloadRuntimeDesign.md 的一致性要求是：前端只见任务投影，后端拥有 scheduler、fetcher、writer、verifier 和 checkpoint。