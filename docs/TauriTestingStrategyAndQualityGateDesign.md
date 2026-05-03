# Tauri Testing Strategy and Quality Gate Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriBackendSkeletonImplementationDesign.md`, `docs/TauriCompositionRootWiringDesign.md`, `docs/TauriKernelJobsRuntimeDesign.md`, `docs/TauriFirstCrateApiDrafts.md`
> Focus: repository-level testing map, current-repo test layout, quality gates, first-slice required tests

---

## 1. Purpose

当前仓库并不是完全没有测试相关内容。

问题在于：**测试要求已经散落在多份架构文档里，但还没有一份仓库级、可直接执行的测试总文档。**

这会带来三个直接问题：

1. 开始实现时，开发者很容易只记住 `cargo check` 和 `npm run build`，忘掉真正需要的 smoke test、contract test 和 wiring test。
2. 同一个测试规则会在不同文档里重复出现，但没有统一优先级和落地位置。
3. 当前仓库还是前端原型，后端代码尚未落地，因此如果不把测试路线先钉住，后续最容易被省掉的就是测试层。

这份文档就是为了补这个缺口。

---

## 2. Existing State

当前测试内容主要分散在以下文档中：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | 仓库级 Testing Strategy 原则 |
| `docs/TauriKernelJobsRuntimeDesign.md` | job runtime 的队列、lease、恢复测试 |
| `docs/TauriCompositionRootWiringDesign.md` | composition smoke / builder failure / startup ordering 测试 |
| `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md` | skeleton 阶段的 use case / adapter / composition 测试 |
| `docs/TauriFirstCrateApiDrafts.md` | 第一批 crate 的最小测试矩阵 |
| `docs/TauriBackendSkeletonImplementationDesign.md` | 当前仓库骨架原子任务的质量门槛 |

结论：

1. 测试内容已经存在，但没有独立总览。
2. 当前真正缺的是**统一测试文档入口**，不是零散测试想法。

---

## 3. Goals

这份文档必须解决四件事：

1. 定义当前仓库的测试分层。
2. 指定测试文件应该落在哪里。
3. 给出第一版后端骨架的最小强制质量门槛。
4. 给出前端原型转向真实实现时的测试落地顺序。

这份文档不负责：

1. 为每个模块写出完整测试用例明细。
2. 取代具体模块设计文档里的专项测试章节。
3. 立刻引入所有测试工具链实现。

---

## 4. Testing Principles

1. 测试优先贴着拥有该语义的抽象边界写，不跨层抢职责。
2. 默认先写最窄、最便宜、最能证伪当前假设的测试，再考虑更宽的集成测试。
3. 任何写成质量门槛的 `cargo test` 都必须绑定明确存在的命名测试，而不是包级空跑。
4. 宿主层测试必须验证真实 bootstrap / wiring surface，不能只复制 `main.rs` 启动逻辑。
5. 前端测试验证的是 UI 契约、交互状态和投影绑定，不是伪造后端业务真相。
6. 不把 E2E 当默认主验证手段；E2E 只覆盖关键跨层闭环。

---

## 5. Test Taxonomy

| Layer | Owner | Purpose | Default Tool |
|------|-------|---------|--------------|
| foundation contract tests | kernel-foundation | 验证错误模型、ID、分页等基础契约稳定存在 | `cargo test` |
| job runtime tests | kernel-jobs | 验证 job 状态、lease、恢复和 snapshot 语义 | `cargo test` |
| module facade tests | module-* | 验证 facade 输出、DTO 映射和 use case 边界 | `cargo test` |
| adapter contract tests | adapter-* | 验证 port 实现满足模块边界期望 | `cargo test` |
| composition wiring smoke tests | composition-root | 验证 concrete adapter 装配和 startup 接线 | `cargo test` |
| transport wiring smoke tests | `src-tauri` | 验证 commands 注册、state 注入和 facade 调用路径 | `cargo test` |
| frontend component/hook tests | Next.js frontend | 验证组件投影、交互状态和壳层切换 | Vitest + Testing Library |
| e2e flow tests | cross-layer | 验证关键业务闭环，不覆盖所有细节 | Playwright 或 Tauri driver |

---

## 6. Current-repo Test Layout

当前仓库仍是“根目录 Next.js 前端 + 未来 `src-tauri/` + `crates/`”的渐进式结构，因此测试布局应按当前仓库而不是理想 monorepo 去定义。

建议布局：

```text
MyEpicLauncher/
├─ src-tauri/
│  ├─ src/
│  └─ tests/
│     └─ transport_wiring_smoke.rs
├─ crates/
│  ├─ kernel-foundation/
│  │  └─ tests/
│  ├─ kernel-jobs/
│  │  └─ tests/
│  ├─ composition-root/
│  │  └─ tests/
│  ├─ module-fab/
│  │  └─ tests/
│  └─ module-downloads/
│     └─ tests/
├─ tests/
│  ├─ frontend/
│  └─ e2e/
└─ docs/
```

说明：

1. Rust crate 的窄测试优先放在各自 crate 的 `tests/` 下。
2. `src-tauri/tests/` 只放宿主 transport / wiring smoke tests。
3. 前端测试先统一落在根 `tests/frontend/`，等后续前端目录真正迁移时再调整。
4. E2E 单独放在 `tests/e2e/`，避免和单层测试混在一起。

---

## 7. Mandatory Quality Gates

### 7.1 Docs-only Changes

纯文档改动不强制跑代码测试，但至少要满足：

1. 文档间路径正确。
2. 文档间规则不互相冲突。
3. 若文档新增了“强制测试门槛”，必须明确到测试名或测试文件级别。

### 7.2 Backend Skeleton Gates

第一版后端骨架至少必须过以下门槛：

1. `cargo check --workspace`
2. `cargo test -p launcher-kernel-foundation foundation_contract_smoke`
3. `cargo test -p launcher-composition-root bootstrap_wiring_smoke`
4. `cargo test -p my-epic-launcher-desktop transport_wiring_smoke`
5. `npm run build`

这五项是**硬门槛**，不是建议项。

### 7.3 Frontend-only Gates

当前前端仍以原型为主。进入真实实现后，前端改动默认至少要过：

1. 受影响组件或 hook 的窄测试
2. `npm run build`

如果改动影响 shell 导航、页面切换或跨模块装配，再补极少量壳层集成测试。

### 7.4 E2E Gates

只有涉及跨层关键闭环时才要求 E2E，例如：

1. 登录流
2. 下载暂停 / 恢复
3. 安装 / 修复
4. 引擎验证

E2E 不是默认门槛。

---

## 8. First-slice Required Tests

第一批后端骨架落地时，最少需要这三条明确存在的命名测试：

| Test | Location | Purpose |
|------|----------|---------|
| `foundation_contract_smoke` | `crates/kernel-foundation/tests/foundation_contract_smoke.rs` | 验证基础错误、ID、分页等最小公开契约存在 |
| `bootstrap_wiring_smoke` | `crates/composition-root/tests/bootstrap_wiring_smoke.rs` | 验证 composition root 能装配 `DesktopAppServices` |
| `transport_wiring_smoke` | `src-tauri/tests/transport_wiring_smoke.rs` | 验证宿主 commands 注册、state 注入和 bootstrap 调用路径 |

它们分别对应三个最容易被偷懒跳过的切面：

1. 基础契约
2. 组合接线
3. 宿主 transport

---

## 9. Frontend Test Rollout

当前前端组件大多仍是静态摘要或 UI 原型，因此前端测试的第一阶段不应伪造复杂业务。

第一阶段更适合先覆盖：

1. shell 页面切换和导航状态
2. settings 的配置摘要渲染
3. account-auth 的登录入口与许可证状态摘要
4. downloads / engines / projects / fab-inventory 的只读投影渲染

第一阶段不应做：

1. 在前端测试里模拟完整下载状态机
2. 在前端测试里伪造 OAuth token 生命周期
3. 用组件测试去验证后端 job runtime 语义

---

## 10. Module-level Test Docs

当前模块文档标准仍然是：

1. `README_ARCH.md`
2. `README_API.md`
3. `README_FLOW.md`

在当前仓库还是原型阶段时，不强制所有模块立刻新增 `README_TEST.md`。

但以下模块一旦开始拥有真实后端语义，就应该补独立测试文档：

1. downloads
2. installations
3. engines
4. auth
5. fab-inventory

原因是这些模块都会拥有：

1. 状态机
2. 长任务
3. adapter 边界
4. 较高回归风险

---

## 11. Rollout Order

建议测试文档和测试实现按下面顺序推进：

1. 先有这份仓库级测试总文档。
2. 再开始创建后端骨架时，同时落 `foundation_contract_smoke`、`bootstrap_wiring_smoke`、`transport_wiring_smoke`。
3. 再为前端壳层和关键静态模块补 Vitest 基础测试。
4. 等真实 backend module 落地后，再为高风险模块补 `README_TEST.md` 与更细的模块测试矩阵。
5. 最后才是关键跨层 E2E 流程。

---

## 12. Exit Criteria

当满足下面四条时，可以认为“测试文档缺口已经补上”：

1. 仓库里已经存在独立的测试总文档。
2. 后端骨架文档中的质量门槛已经能映射到明确测试名。
3. 当前仓库测试文件布局已经被明确定义。
4. 后续实现工作不再需要把测试规则分散重复写在每一份文档里。