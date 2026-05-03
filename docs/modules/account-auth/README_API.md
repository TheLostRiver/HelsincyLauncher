# Account Auth Module API

> Module: account-auth
> Status: draft

---

## 1. Public Surface

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| AccountLogin | component | components/AccountLogin.tsx | 首页中的账户登录摘要卡片 |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| none | n/a | n/a | 当前组件不接收外部 props |

当前模块仍是静态展示入口。后续若接入真实账户摘要、会话状态或登录动作，应优先显式增加 props、hooks 或模块契约。

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| login entry summary | rendered view | 首页渲染时 | 展示登录按钮与同步价值说明 |
| organization license summary | rendered view | 首页渲染时 | 展示许可证连接状态 |
| login intent | button click target | 用户点击登录按钮时 | 当前只提供 UI 意图入口 |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| login copy | read | local | 账户登录与同步说明文案 |
| license status copy | read | local | 当前“未连接”许可证状态文案 |
| auth icon affordance | read | local | 顶部用户图标只表达账户入口存在 |

---

## 5. Error and Empty States

- 当前没有 loading、empty 或 error 状态；后续接入真实会话源后必须补齐。
- 当前默认假设用户未登录且许可证未连接；未来应能表达登录中、已登录、授权过期和许可证异常等状态。
- 登录按钮当前没有失败反馈路径；接入真实认证后应能表达认证拒绝、网络失败或安全存储错误。
- 许可证状态当前只是静态字符串；未来应提升为稳定状态标签而不是散落文案。

---

## 6. Compatibility Notes

- account-auth 模块应始终对外暴露聚合后的账户和许可证摘要，而不是底层 OAuth 或 token 结构。
- 若未来出现完整账户页，当前首页卡片仍应保持摘要入口角色。
- account-auth 与 projects、fab-inventory、downloads 的关系应通过稳定登录状态和授权摘要表达，而不是直接耦合到它们的内部状态。