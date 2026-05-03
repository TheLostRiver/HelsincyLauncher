# Settings Module Architecture

> Module: settings
> Status: draft
> Owner scope: frontend now, mixed later

---

## 1. Purpose

settings 模块负责展示启动器中与行为偏好和本地库路径相关的高频设置摘要，并为后续真实配置读写提供稳定前端入口。

---

## 2. Responsibilities

本模块负责：

- 展示当前设置项摘要，例如自动更新引擎、下载后校验文件和本地库路径
- 暴露设置开关和路径配置的用户意图入口
- 在首页右侧区域提供轻量设置入口，而不是完整设置工作台
- 为后续配置持久化、平台路径选择和设置校验预留统一边界

本模块不负责：

- 直接把设置写入磁盘或数据库
- 自行决定配置默认值、迁移规则和平台兼容策略
- 实现文件选择器、权限处理和配置恢复逻辑

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| auto update summary | settings | 当前自动更新引擎开关的展示状态 |
| verify after download summary | settings | 当前下载后校验文件开关的展示状态 |
| library path summary | settings | 当前本地库路径的展示文案 |
| settings action affordance | settings | 开关点击和路径入口当前只表达意图 |

当前模块没有真实本地状态管理。后续若接入可编辑设置和脏状态提示，应由 settings 模块拥有局部视图状态，而持久化真相仍应在后端。

---

## 4. Boundary Rules

允许依赖：

- components/Settings.tsx 作为当前前端入口
- shell 或首页组合层对该模块的挂载
- 未来通过稳定 settings read model 读取配置摘要

禁止依赖：

- 未抽象的本地配置文件格式
- 下载、引擎或账户模块的内部状态实现
- 页面内直接调用平台路径选择和持久化逻辑

禁止的跨边界行为：

- 在组件内部直接写配置文件或数据库
- 用 UI 组件自行维护配置默认值和迁移规则
- 把设置模块变成其他业务模块状态的临时入口

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| settings summary card | components/Settings.tsx | 首页中的轻量设置入口 |
| toggle summaries | components/Settings.tsx | 自动更新与下载后校验的展示行 |
| library path summary | components/Settings.tsx | 本地库路径摘要 |
| future config bridge | components/Settings.tsx | 后续应通过稳定契约连接真实设置源 |

---

## 6. Coding Constraints

1. settings 前端只展示配置摘要和用户意图，不拥有持久化真相。
2. 开关状态和路径摘要应来自统一设置投影，不要在多个组件里各自维护一套配置值。
3. 后续接入真实设置时，页面只发出更新意图，配置校验、持久化和迁移必须留在后端或明确的配置层。
4. 首页 settings 卡片应保持轻量摘要角色；完整设置页若出现，应由独立模块视图承接复杂编辑流。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把设置展示和底层配置存储耦合在了一起
2. 是否让前端承担了路径选择、权限或持久化逻辑
3. 是否保持了设置摘要和真实配置真相的边界分离
4. 是否需要同步更新 README_API.md 或 README_FLOW.md