# Settings Module Flow

> Module: settings
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 查看设置摘要 | 用户停留在 home 页面 | 看到当前高频设置和本地库路径摘要 | 若未来配置缺失，应显示降级状态 |
| 切换自动更新意图 | 用户点击自动更新引擎开关 | 进入后续真实设置变更入口 | 当前未接线，只停留在 UI 层 |
| 切换下载后校验意图 | 用户点击下载后校验文件开关 | 进入后续真实设置变更入口 | 当前未接线，只停留在 UI 层 |
| 查看本地库路径 | 首页渲染路径摘要 | 用户感知当前库目录 | 若未来路径无效，应显示异常状态 |

---

## 2. Happy Path

1. 首页装配 Settings 组件。
2. 组件展示两个高频开关的当前摘要状态。
3. 组件展示当前本地库路径。
4. 用户能够快速理解几个关键设置的当前配置方向。
5. 后续接入真实设置源后，仍应保持摘要优先、编辑次之的交互密度。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| config not loaded yet | 应显示 loading 或占位态，而不是伪造已开启状态 |
| invalid library path | 应明确显示路径异常或需要重选 |
| settings write failed later | 应给出稳定错误反馈，而不是静默回滚 |
| full settings page added later | 首页卡片继续做摘要入口，完整编辑流转移到独立页面 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| auto-update enabled | future toggle request | pending update | 应由真实设置变更流程驱动 |
| verify disabled | future toggle request | pending update | 当前只是 UI 意图状态 |
| valid path | future path validation failure | invalid path summary | 应来自稳定配置投影 |
| settings summary visible | future open settings action | expanded settings flow | 不应强行塞进首页卡片内部 |

---

## 5. UI and Side-effect Notes

- 当前模块没有真实副作用，开关和路径都只是展示与意图入口。
- 后续真实设置更新必须通过统一配置契约执行，不能让组件自己维护持久化结果。
- 设置摘要属于全局应用配置投影，不应在多个业务模块中各自复制和改写。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 首页仍然能稳定展示设置摘要卡片
2. 两个高频开关和库路径仍然可见
3. 设置摘要没有退化成底层配置细节泄露
4. 新接入的设置变更能力没有把持久化逻辑重新塞回前端