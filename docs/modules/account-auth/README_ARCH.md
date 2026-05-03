# Account Auth Module Architecture

> Module: account-auth
> Status: draft
> Owner scope: frontend now, mixed later

---

## 1. Purpose

account-auth 模块负责展示账户登录入口、账户同步价值说明以及组织许可证连接状态摘要，并为后续真实认证流程提供稳定前端入口。

---

## 2. Responsibilities

本模块负责：

- 展示 Epic 账户登录入口
- 呈现登录后可同步的能力摘要，例如工程、资产和许可证
- 展示组织许可证连接状态摘要
- 为后续账号会话、授权状态和许可证同步预留稳定边界

本模块不负责：

- 直接执行 OAuth 登录流程
- 自行管理 token 生命周期或安全存储
- 直接读取或写入许可证、账户会话和凭据数据

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| login entry summary | account-auth | 当前登录按钮和说明文案的展示状态 |
| sync capabilities summary | account-auth | 当前“同步工程、资产和许可证”的价值说明 |
| organization license status | account-auth | 当前许可证连接状态摘要 |
| auth action affordance | account-auth | 登录按钮当前只表达用户意图 |

当前模块没有真实本地认证状态。后续若接入登录中、已登录用户摘要和许可证详情，应由 account-auth 模块拥有局部视图状态，而会话真相仍在后端。

---

## 4. Boundary Rules

允许依赖：

- components/AccountLogin.tsx 作为当前前端入口
- shell 或首页组合层对该模块的挂载
- 未来通过稳定 auth session read model 读取账号与许可证摘要

禁止依赖：

- 未抽象的 OAuth SDK 细节
- 安全存储、token 刷新和许可证同步实现细节
- 其他业务模块的内部状态实现

禁止的跨边界行为：

- 在组件内部直接发起完整 OAuth 流程并管理回调状态
- 在前端保存长期凭据或刷新 token
- 让 account-auth 直接操纵工程、下载或库存模块的内部状态

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| auth summary card | components/AccountLogin.tsx | 首页中的登录入口卡片 |
| login entry | components/AccountLogin.tsx | 使用 Epic 账户登录按钮 |
| license status summary | components/AccountLogin.tsx | 组织许可证连接状态展示 |
| future auth bridge | components/AccountLogin.tsx | 后续应通过稳定契约连接真实账户会话源 |

---

## 6. Coding Constraints

1. account-auth 前端只展示认证摘要和登录意图，不持有凭据真相。
2. 登录按钮、账号摘要和许可证状态应来自统一认证投影，不要在多个 UI 组件里重复拼装。
3. 后续接入真实认证后，页面只发出登录、登出或重连意图，token 生命周期和安全存储必须留在后端或独立认证层。
4. 首页 account-auth 卡片应保持轻量入口角色；完整账户管理流若出现，应转移到独立模块视图。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把认证展示和底层 OAuth 实现耦合在了一起
2. 是否让前端承担了凭据、token 或许可证同步逻辑
3. 是否保持了认证摘要与真实会话真相的边界分离
4. 是否需要同步更新 README_API.md 或 README_FLOW.md