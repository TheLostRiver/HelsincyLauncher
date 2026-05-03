# Tauri Release, Packaging, and Update Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriDevelopmentEnvironmentBootstrapDesign.md`, `docs/TauriSecurityCredentialsAndPermissionsDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`, `docs/TauriStartupPipelineDesign.md`
> Focus: desktop release artifacts, packaging boundaries, update lifecycle, channel policy, rollback rules, UI/backend ownership

---

## 1. Purpose

当前仓库并不是完全没有发布、打包和更新相关设计。

问题在于：**Tauri host 的打包职责、Updates 模块边界、auto-update UI、Windows MSI 前提、更新检查/下载/切换策略等内容已经散落在蓝图、settings 模块和环境文档里，但还没有一份独立的发布/打包/更新总文档。**

这会直接导致几个问题：

1. 内容下载和应用自更新容易被混成一套流程。
2. 前端的 auto-update 开关容易越权成真正的更新执行器。
3. 渠道、签名、回滚和失败恢复没有统一边界时，后续实现会各写各的。

---

## 2. Existing State

当前相关内容主要散落在：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | Tauri 宿主承担打包、更新、权限声明；Updates 模块前后端职责 |
| `docs/modules/settings/README_ARCH.md` | 当前前端已有 auto-update summary 的轻量展示入口 |
| `docs/TauriDevelopmentEnvironmentBootstrapDesign.md` | Windows MSI 前提与 VBSCRIPT 只在打包阶段相关 |
| `docs/TauriSecurityCredentialsAndPermissionsDesign.md` | 权限面和签名/信任边界不应与普通业务混层 |
| `docs/TauriLoggingAndObservabilityDesign.md` | 更新失败和宿主事件需要可观测 |

结论：

1. 相关规则并不缺失。
2. 当前缺的是**独立的仓库级发布、打包与更新总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 什么叫“应用自更新”，什么叫“内容下载/引擎下载”，两者边界如何划分。
2. 发布产物、安装包、更新包和更新清单由谁产生、谁消费。
3. 前端 Updates UI、settings summary、Tauri host 和 backend 策略层分别负责什么。
4. 渠道、签名、回滚和失败恢复应遵守什么规则。

这份文档不负责：

1. 绑定某个具体 CI 平台配置。
2. 现在就决定最终发布基础设施供应商。
3. 取代未来真正的 module-updates 细化设计。

---

## 4. Core Principles

1. app update != content download：应用自更新和引擎/资产下载不是同一类任务。
2. host owns package switching：桌面应用版本切换属于宿主层责任，不属于普通业务模块。
3. UI expresses intent, backend/host enforces policy：前端只负责展示可更新状态和用户意图，真正执行策略在后端/宿主。
4. channels are explicit：stable、beta 等渠道不能靠隐式字符串和 if/else 散落实现。
5. signed artifacts only：任何可安装或可切换的应用包都必须经过明确的签名/完整性校验边界。
6. rollback beats broken install：更新失败时优先保证应用可恢复，而不是把用户留在半更新状态。

---

## 5. Boundary Model

| Concern | Owner | Must Not Own |
|---------|-------|--------------|
| update prompt / UI summary | frontend updates/settings surface | 真实包切换和安装逻辑 |
| update policy evaluation | backend/host policy layer | 页面局部状态 |
| package download and switch | Tauri host / updater boundary | 普通 downloads module facade |
| content downloads | downloads module | app binary replacement |
| release artifact creation | release/packaging pipeline | frontend runtime |

固定规则：

1. `Downloads` 模块负责内容、payload、repair artifact 等业务下载。
2. `Updates` 模块或宿主更新边界负责应用自身版本检查、下载与切换。
3. 这两条链不共用同一套产品语义，最多共享底层少量网络/日志设施。

---

## 6. Artifact Model

发布体系里至少要区分四类东西：

1. app release artifact：可分发的桌面应用产物
2. installer package：面向用户安装/升级的安装包
3. update payload：供应用内更新消费的版本切换产物
4. update manifest / metadata：描述版本、渠道、签名和兼容信息的清单

规则：

1. 不能把更新清单简化成“只有一个下载 URL”的弱协议。
2. 不能把引擎内容下载 manifest 和应用更新 manifest 混成一套结构。
3. 不能让前端自己解析和信任发布产物元数据。

---

## 7. Update Lifecycle

建议的自更新主流程：

```text
check for update
  -> fetch update manifest
  -> verify channel / compatibility / signature metadata
  -> project update summary to UI
  -> user accept or policy auto-accept
  -> download update payload
  -> verify payload integrity
  -> stage switch
  -> restart / relaunch into new version
  -> mark old version replaceable or rollback candidate
```

关键约束：

1. update check 可以是后台行为，但版本切换必须是显式可控动作。
2. 下载完成不等于已成功切换。
3. 切换失败必须留下明确恢复路径和日志。

---

## 8. Channel Policy

建议至少显式支持渠道概念，例如：

1. stable
2. beta
3. internal/dev

规则：

1. 渠道应体现在 update manifest 与本地策略配置中。
2. 渠道切换属于受控设置，不应隐藏在临时 debug 开关里。
3. 不同渠道的更新提示、自动下载和自动切换策略可以不同，但必须是显式配置。

---

## 9. Settings and UI Rules

当前 settings 模块里的 auto-update summary 只应表达：

1. 用户偏好摘要
2. 更新策略入口
3. 可更新状态的轻量提示

它不应负责：

1. 自己下载 app update payload
2. 自己切换安装版本
3. 直接决定渠道兼容性或签名校验结果

前端 Updates UI 可以负责：

1. 显示当前版本与目标版本
2. 显示更新说明摘要
3. 触发“稍后再说”“立即更新”“查看详情”之类的用户意图

---

## 10. Packaging Rules

### 10.1 Desktop Packaging

桌面打包属于宿主和发布流水线职责。

规则：

1. 打包配置不应散落在普通业务模块。
2. installer target 的选择应是显式发布决策，不应由前端或普通模块推断。
3. 若 Windows 上启用 MSI 目标，相关系统前提属于环境 bootstrap / release pipeline，不属于业务代码。

### 10.2 Signing and Trust

发布产物至少要满足：

1. 有明确来源
2. 有完整性验证边界
3. 更新时有可信元数据校验

禁止：

1. 下载后直接切换而不做任何可信校验
2. 让前端自己决定“这个包看起来像是真的”

---

## 11. Rollback and Failure Handling

更新失败最少要区分：

1. check failure
2. download failure
3. verification failure
4. switch / relaunch failure

建议规则：

1. check failure 不影响当前版本继续运行。
2. download failure 不破坏当前已安装版本。
3. verification failure 必须拒绝切换到新版本。
4. switch 失败时应优先恢复当前稳定版本或保留明确 rollback 线索。

---

## 12. Logging and Diagnostics

更新链路至少应可观测以下事件：

1. update check started / completed / failed
2. channel decision made
3. payload download started / completed / failed
4. package verification passed / failed
5. switch scheduled / applied / rolled back

日志和 diagnostics 应包含：

1. `correlationId`
2. target version
3. current version
4. channel
5. failure classification

但不应包含：

1. 敏感签名材料
2. 私有发布凭据
3. 不受控的原始系统安装器输出转储

---

## 13. Testing Expectations

这条链路至少要有以下测试层：

1. policy tests：验证渠道、兼容性和自动更新策略判断
2. updater integration smoke tests：验证宿主层更新链路的最小接线
3. failure-path tests：验证 verification failure 与 rollback path
4. UI projection tests：验证 settings / updates 只消费摘要，不越权执行更新逻辑

当前阶段不要求立即实现完整 updater 测试工具链，但文档必须先把边界说清。

---

## 14. Current-repo Rollout Order

对当前仓库而言，建议顺序如下：

1. 先补这份发布/打包/更新总文档。
2. 后续若真的引入 `module-updates` 或等价边界，再补它的 contracts 和 facade 设计。
3. 当 `src-tauri` 真正落地后，再把 host updater glue 接到宿主层。
4. 当 CI / release pipeline 开始真实存在时，再补专门的 CI / release automation 文档。

---

## 15. Exit Criteria

当满足下面几条时，可以认为“发布、打包与更新文档缺口已经补上”：

1. 仓库里已经存在独立的发布、打包与更新总文档。
2. app update、content download、settings summary 和 host updater 的边界已被统一定义。
3. 渠道、签名、回滚和失败处理不再只是散落提示，而是统一规则。
4. 后续实现可以直接引用这份文档，而不是从蓝图、settings 和环境文档里拼更新语义。