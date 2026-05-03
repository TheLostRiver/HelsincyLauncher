# Account Auth Module Flow

> Module: account-auth
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 查看账户摘要 | 用户停留在 home 页面 | 看到登录入口、同步价值和许可证状态摘要 | 若未来会话缺失或异常，应显示明确降级状态 |
| 发起登录意图 | 用户点击登录按钮 | 进入后续真实认证流程入口 | 当前未接线，只停留在 UI 层 |
| 查看许可证状态 | 首页渲染许可证行 | 用户感知当前许可证未连接 | 若未来状态异常，应显示稳定错误摘要 |

---

## 2. Happy Path

1. 首页装配 AccountLogin 组件。
2. 组件展示账户登录标题和同步价值说明。
3. 用户看到使用 Epic 账户登录按钮。
4. 组件同时展示组织许可证当前状态。
5. 后续接入真实会话源后，仍应保持“摘要先于复杂认证流”的交互密度。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| session loading | 应显示 loading 或占位态，而不是伪造未登录状态 |
| authenticated user | 应显示账户摘要和已连接状态，而不是继续只显示登录按钮 |
| license disconnected | 应明确展示未连接或需重连状态 |
| auth expired later | 应显示会话失效或需重新登录状态，而不是静默失败 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| logged out summary | future login request | authenticating | 应由真实认证流程驱动 |
| authenticating | future auth success | authenticated summary | 应来自稳定会话投影 |
| authenticated summary | future token expiry | reauth required | 不应让前端自行维护 token 状态 |
| license disconnected | future license sync success | license connected | 应来自稳定许可证投影 |

---

## 5. UI and Side-effect Notes

- 当前模块没有真实副作用，登录按钮只是入口。
- 后续真实认证更新必须通过统一认证契约执行，不能让组件自己保存和刷新凭据。
- 账户摘要与许可证状态属于同一认证投影，不应在多个模块中各自复制和改写。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 首页仍然能稳定展示账户登录卡片
2. 登录入口、同步说明和许可证状态仍然可见
3. 认证摘要没有退化成底层 OAuth 细节泄露
4. 新接入的认证能力没有把会话和凭据逻辑重新塞回前端