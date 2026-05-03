# Downloads Module Architecture

> Module: downloads
> Status: draft
> Owner scope: frontend now, mixed later

---

## 1. Purpose

downloads 模块负责展示下载任务摘要、聚合进度、下载速率和基础控制入口，并为后续统一下载运行时的前端接入提供稳定边界。

---

## 2. Responsibilities

本模块负责：

- 展示当前活动下载与排队下载的摘要信息
- 呈现聚合进度、吞吐和队列状态
- 暴露暂停全部和打开下载位置等控制入口
- 为后续断点续传、多任务并行和安装串接保留前端入口语义

本模块不负责：

- 实现分段规划、调度、落盘、校验和 checkpoint 持久化
- 直接决定安装、修复或引擎语义
- 自行维护底层下载状态机和恢复逻辑

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| active download summary | downloads | 当前活动下载名称、百分比和进度条展示 |
| queued download summary | downloads | 当前排队下载及等待状态展示 |
| aggregate throughput label | downloads | 顶部速率摘要 |
| queue action intents | downloads | 暂停全部和打开目录按钮只表达意图 |

当前模块还没有显式本地状态。后续若接入多任务列表、筛选、恢复与错误反馈，应由 downloads 模块自己拥有相应视图状态。

---

## 4. Boundary Rules

允许依赖：

- components/Downloads.tsx 作为当前前端入口
- shell 或首页组合层对该模块的挂载
- docs/TauriDownloadRuntimeDesign.md 中定义的未来后端运行时边界

禁止依赖：

- 引擎模块或安装模块的内部状态实现
- 未抽象的 HTTP 拉取、文件写入和 checkpoint 细节
- 将下载 worker 细节直接暴露给 UI

禁止的跨边界行为：

- 在组件内部实现断点续传或分段调度逻辑
- 让前端自己拼接 checkpoint 和恢复状态
- 把安装、解压或验证行为直接塞进 downloads 展示组件

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| summary card | components/Downloads.tsx | 首页中的下载摘要卡片 |
| active task projection | components/Downloads.tsx | 当前活动下载的名称、百分比与进度条 |
| queued task projection | components/Downloads.tsx | 排队下载与等待状态摘要 |
| backend alignment | docs/TauriDownloadRuntimeDesign.md | 约束未来下载运行时边界与任务模型 |

---

## 6. Coding Constraints

1. downloads 前端只消费聚合后的任务摘要，不消费底层 segment 或 chunk 噪声。
2. 活动任务、排队任务、吞吐和控制入口必须保持同一模块语义，不要把下载状态拆散到无关卡片。
3. 后续接入真实运行时时，前端只表达 start、pause、resume、cancel 等意图，不承接 checkpoint 逻辑。
4. 安装串接、修复复用和 provider 差异应留在后端 facade 与 adapter 层，不能回流到展示层。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把下载展示和底层运行时实现耦合在了一起
2. 是否让前端承担了恢复、校验或写盘逻辑
3. 是否保持了聚合任务投影这一前端边界
4. 是否需要同步更新 README_API.md 或 README_FLOW.md