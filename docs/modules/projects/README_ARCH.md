# Projects Module Architecture

> Module: projects
> Status: draft
> Owner scope: frontend now, mixed later

---

## 1. Purpose

projects 模块负责展示 Unreal Engine 工程库、当前选中工程详情以及与工程相关的高频操作入口。

---

## 2. Responsibilities

本模块负责：

- 展示本地工程列表、引擎版本、最近打开时间和基础统计摘要
- 呈现当前选中工程的详情、健康状态和常见操作入口
- 暴露搜索、筛选、扫描、新建工程、打开工程、打开目录等用户意图入口
- 为后续工程扫描、迁移、归档和依赖分析保留稳定前端边界

本模块不负责：

- 直接扫描磁盘并发现工程
- 自行决定引擎迁移、归档或健康校验的底层业务规则
- 直接操作 Fab 库存、下载任务或引擎安装内部状态

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| projects list summary | projects | 当前列表中工程名称、路径、引擎版本和最近打开信息 |
| selected project summary | projects | 当前被标记为 selected 的工程详情 |
| project metrics | projects | 本地工程数量与主力引擎版本摘要 |
| project health message | projects | 当前工程状态正常的展示文案 |
| project action intents | projects | 打开、扫描、复制路径、迁移、归档等按钮只表达意图 |

当前模块尚未实现真实交互状态。后续若接入搜索词、筛选器、选中项切换和扫描结果，应由 projects 模块自己拥有这些视图状态。

---

## 4. Boundary Rules

允许依赖：

- components/MyProjectsContent.tsx 作为当前前端入口
- shell 模块对该页面的挂载和切换
- 未来通过稳定 read model 读取引擎版本和 Fab 依赖摘要

禁止依赖：

- 引擎模块、下载模块、Fab 模块的内部实现细节
- 未抽象的文件系统扫描逻辑
- 直接操纵工程迁移、归档或校验的后端流程实现

禁止的跨边界行为：

- 在页面组件里直接遍历磁盘路径寻找工程
- 用 projects 模块直接读写其他模块内部状态
- 在 UI 中临时拼接“迁移到新版引擎”的业务流程

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| project list view | components/MyProjectsContent.tsx | 左侧工程列表、工具栏与统计摘要 |
| selected project detail | components/MyProjectsContent.tsx | 右侧详情、健康状态与操作入口 |
| project mock dataset | components/MyProjectsContent.tsx | 目前以内联常量保存工程摘要 |
| project actions | components/MyProjectsContent.tsx | 打开、扫描、复制路径、迁移、归档等入口 |

---

## 6. Coding Constraints

1. projects 模块只消费工程级摘要和健康状态，不承接底层扫描、迁移和校验实现。
2. 工程列表、选中详情和工程操作必须保持同一模块语义，不要拆散到无关页面组件里。
3. 未来接入真实数据时，工程依赖、引擎版本和健康结果应来自稳定 read model，而不是页面内重新推导。
4. 若项目页后续接入真正的搜索和筛选状态，应优先留在 projects 模块内部，不要让 shell 代管这些局部状态。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把工程库展示和底层扫描逻辑耦合在了一起
2. 是否让前端承担了不该承担的迁移、归档或健康校验逻辑
3. 是否保持了工程列表摘要与选中详情的统一语义
4. 是否需要同步更新 README_API.md 或 README_FLOW.md