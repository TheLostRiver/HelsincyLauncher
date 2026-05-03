# Tauri Security, Credentials, and Permissions Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriStorageAndDatabaseDesign.md`, `docs/TauriRepositoryPortsAndAdapterDesign.md`, `docs/TauriStartupPipelineDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`, `docs/TauriIPCAndStateContractsDesign.md`
> Focus: security boundaries, credential ownership, secret storage, permission surface, logging redaction, rollout order

---

## 1. Purpose

当前仓库并不是完全没有安全、凭据和权限相关设计。

问题在于：**secure storage、token 生命周期、auth session 边界、Tauri host 权限面、startup 中的 secure storage 初始化，以及日志脱敏要求已经散落在多份文档里，但还没有一份独立的安全/凭据/权限总文档。**

这会导致后续实现时出现几个典型退化：

1. 前端开始偷拿 OAuth 或 token 语义。
2. SQLite、配置系统和系统安全存储的边界再次混掉。
3. Tauri host 权限声明越配越宽，没有统一的最小权限原则。
4. 日志和 diagnostics 在失败排障时把敏感字段一起泄出去。

---

## 2. Existing State

当前相关内容主要散落在：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | 前端不得管理 OAuth token 生命周期；Tauri 宿主承担权限声明 |
| `docs/TauriStorageAndDatabaseDesign.md` | token / refresh token / sensitive secrets 默认不进 SQLite |
| `docs/TauriRepositoryPortsAndAdapterDesign.md` | `SessionSecretStorePort`、`ProviderCredentialStorePort` 和 secure storage adapter 边界 |
| `docs/TauriStartupPipelineDesign.md` | secure storage init 的阻塞规则和启动失败边界 |
| `docs/TauriLoggingAndObservabilityDesign.md` | secure storage access failure 的观测要求和日志边界 |
| `docs/modules/account-auth/README_ARCH.md` | 前端 account-auth 只表达登录意图，不拥有凭据真相 |

结论：

1. 这些规则并不缺失。
2. 当前缺的是**独立的仓库级安全、凭据与权限总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 谁拥有凭据真相，谁只能消费投影。
2. 哪些敏感数据可以进入 SQLite、配置系统或日志，哪些绝对不行。
3. Tauri host 的权限面应怎样保持最小化。
4. secure storage、auth session、provider credentials 和前端 read model 之间如何分层。
5. 认证失败、权限失败和凭据恢复失败应如何投影，而不是直接泄漏底层细节。

这份文档不负责：

1. 定义具体 OAuth 提供方协议细节。
2. 写出完整的系统级安全加固手册。
3. 取代未来真正的 auth module 设计文档。

---

## 4. Security Principles

1. backend owns credentials：凭据真相只在后端与系统安全存储边界存在。
2. least authority by default：宿主、adapter、module 和前端都只拿自己最小必需权限。
3. secret storage is distinct：secret store 不与 SQLite、JSON 配置和普通缓存混存。
4. projection over leakage：前端只拿 auth session read model，不拿 token 或 provider secret。
5. explicit permission surface：宿主权限必须被显式列出和审查，不能因为省事而全开。
6. observability without secret leakage：日志、错误和 diagnostics 必须可排障，但不能把敏感值直接暴露出去。

---

## 5. Ownership Model

| Concern | Owner | Must Not Own |
|---------|-------|--------------|
| access token / refresh token | secure storage backed backend layer | frontend shell, SQLite, plain JSON config |
| auth session truth | backend auth/session layer | 首页组件、前端临时 store |
| auth summary projection | frontend via typed read model | raw OAuth payload |
| provider credential wiring | composition root + secure-storage adapters | Tauri command handler business logic |
| host permission declaration | `src-tauri` host layer | 任意业务模块自行扩权 |

固定规则：

1. 前端只发出 sign-in、sign-out、reauth 等意图。
2. 后端负责 token 获取、刷新、失效判定和安全存储。
3. `account-auth` 页面或卡片只消费聚合后的会话摘要与许可证摘要。

---

## 6. Credential Classes

### 6.1 Session Secrets

典型内容：

1. access token
2. refresh token
3. provider-issued session secret

默认存放位置：

1. OS secure storage

禁止存放位置：

1. SQLite
2. 明文 JSON 配置
3. 前端 local storage / session storage
4. 前端内存中长期缓存

### 6.2 Provider Credentials

典型内容：

1. provider device-code exchange secret
2. 第三方接入凭据
3. 长期 provider secret

规则：

1. 通过 `ProviderCredentialStorePort` 之类的内部 port 抽象。
2. 不穿透到 public IPC contracts。
3. 不在日志或 diagnostics 中原样输出。

### 6.3 Non-secret Session Metadata

可进入普通持久化或 read model 的通常是：

1. account id
2. display name
3. token expiry summary
4. last sign-in status
5. reauth required summary

这些是投影数据，不等于 secret 本体。

---

## 7. Storage Boundary

### 7.1 What Must Stay out of SQLite

下面内容默认不进 SQLite：

1. access token
2. refresh token
3. provider secret
4. raw OAuth exchange payload

### 7.2 What May Enter SQLite

下面内容可以作为结构化事实或投影进入 SQLite：

1. session existence summary
2. last successful auth timestamp
3. reauth required flag
4. provider account linkage summary

判断标准：

1. 若泄漏后会直接扩大认证攻击面，它就不应进入 SQLite。
2. 若它只是用户态或系统态摘要，并且对恢复/展示有价值，则可以作为投影进入 SQLite。

### 7.3 What May Enter Config

配置系统只应保存非敏感策略，例如：

1. 是否自动尝试恢复登录
2. provider 功能开关
3. 权限相关的非敏感用户偏好

禁止：

1. 把 token 冒充成“配置项”保存

---

## 8. Permission Surface

### 8.1 Host Permissions

Tauri host 的权限面应只覆盖真实需要的能力，例如：

1. secure storage access
2. app data directory access
3. logs directory access
4. network egress to provider endpoints
5. user-chosen filesystem locations when required

### 8.2 Permission Rules

1. 默认 deny，按功能显式开启。
2. 权限属于 host / adapter 边界，不属于业务模块自行声明。
3. 若某能力可以延后到用户触发时申请，就不要在启动时预先扩权。
4. 文件系统访问应尽量收敛到用户明确选择的目录，而不是给全盘模糊权限。

### 8.3 Cross-layer Constraint

禁止：

1. 前端以“只是 UI 调用”为理由绕过 host 权限设计。
2. module facade 自己偷偷依赖更宽的平台权限。
3. 为了调试临时打开宽权限后忘记收回。

---

## 9. Startup and Restore Rules

安全与启动的关系必须固定：

1. secure storage init 只在它是最小可运行前提时允许阻塞 startup。
2. auth session restore 只恢复最小必要的登录态摘要，不在 startup 中直接做大规模 provider 同步。
3. 若 secure storage 当前不可用，应返回结构化失败，并允许系统在可能情况下以受限模式降级。

启动阶段禁止：

1. 因为 session restore 失败就强行打断所有非认证相关壳层能力
2. 在 startup 阶段输出原始 secret 内容做排障

---

## 10. IPC and Read-model Rules

前端永远不应直接看到：

1. access token
2. refresh token
3. provider secret
4. 原始授权交换 payload

前端可以看到的通常是：

1. `signedOut`
2. `signingIn`
3. `authenticatedSummary`
4. `reauthRequired`
5. `licenseStatusSummary`

规则：

1. IPC contract 公开的是 auth session projection，不是 credential payload。
2. account-auth 模块只消费稳定摘要模型。
3. 若前端想根据会话状态做 UX 决策，应依赖稳定枚举或 summary 字段，而不是底层 token 细节。

---

## 11. Logging, Error, and Diagnostics Boundary

### 11.1 Logging

日志应记录：

1. secure storage init success / failure
2. session restore success / failure summary
3. permission denied / permission missing classification
4. reauth required transitions

日志不应记录：

1. token 值
2. refresh secret
3. 原始 provider secret
4. 完整授权响应体

### 11.2 Errors

对外错误投影应表达：

1. `AUTH_*` 或其他稳定 code
2. `retryable`
3. `severity`
4. `correlationId`

但不应表达：

1. 原始安全存储实现细节
2. 具体 secret 值
3. 原始系统 API 敏感输出

### 11.3 Diagnostics

diagnostics 只暴露受控摘要，例如：

1. secure storage available / unavailable
2. session restore last result
3. permission health summary

不能把 diagnostics 做成“原始 secret 浏览器”或“原始 host capability dump”。

---

## 12. Module and Adapter Responsibilities

| Layer | Responsibility |
|-------|----------------|
| frontend account-auth | 展示登录意图和摘要，不持有 secret |
| `src-tauri` host | 声明最小权限面、挂接 transport、承接平台入口 |
| composition root | 组装 secure-storage adapter 与 auth/use-case 依赖 |
| auth/session application layer | 管理 token 生命周期、刷新、失效和投影 |
| secure-storage adapters | 提供系统级保护语义，不写业务决策 |

禁止：

1. 在组件层实现 token refresh
2. 在 command handler 中手写 secret 持久化策略
3. 在 secure-storage adapter 中决定产品层 reauth UX 语义

---

## 13. First-slice Rollout Order

对当前仓库而言，这份文档的落地顺序应是：

1. 先明确安全、凭据和权限总规则。
2. 当 `src-tauri` 首次落地时，先把 host permission surface 和 secure storage init 边界建清楚。
3. 当 auth/session 真实进入后端代码时，再引入 `SessionSecretStorePort` 和相应 adapter。
4. 当 account-auth 不再只是静态卡片时，再接稳定 auth session projection，而不是把 OAuth 细节穿给前端。

---

## 14. Exit Criteria

当满足下面几条时，可以认为“安全、凭据与权限文档缺口已经补上”：

1. 仓库里已经存在独立的安全、凭据与权限总文档。
2. secure storage、SQLite、config、frontend projection 和 host permission surface 的边界已被统一定义。
3. 前端不再有理由碰 token 真相，后端也不再有理由把 secret 混进日志和普通持久化。
4. 后续实现可以直接引用这份文档，而不是从存储、ports、startup、logging 和 account-auth 文档里拼规则。