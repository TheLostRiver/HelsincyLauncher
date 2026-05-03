# Engines Module Flow

> Module: engines
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 首页查看当前引擎 | 用户停留在 home 页面 | 看到当前引擎版本、容量和插件兼容性摘要 | 若未来无引擎安装，应显示空态 |
| 发起启动意图 | 用户点击启动按钮 | 进入后续真实启动流程入口 | 当前未接线，只停留在按钮层 |
| 发起安装版本意图 | 用户点击安装版本按钮 | 进入后续版本安装流程入口 | 当前未接线，只停留在按钮层 |
| 查看插件兼容性 | 首页渲染插件列表 | 看到插件 Ready 或 Update 摘要 | 若未来数据缺失，应有未知状态而不是静默消失 |

---

## 2. Happy Path

1. 首页装配 EngineManagement 组件。
2. 组件展示当前引擎版本、容量和源构建可用状态。
3. 用户可以感知到启动与安装版本两个主动作入口。
4. 组件同时展示若干插件的兼容性摘要。
5. 后续接入真实数据时，仍应保持“摘要先于细节”的信息密度。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| no engine installed | 应显示未安装引擎的明确空态和安装引导 |
| plugin needs update | 兼容性区域应清楚标出需要更新的插件 |
| verification available later | 验证与修复应作为受控扩展入口加入，不破坏当前摘要卡片边界 |
| multiple engines later | 摘要卡片应显示当前活动引擎，而复杂列表留给独立模块视图 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| summary visible | click launch | launch requested | 当前只是 UI 意图状态 |
| summary visible | click install version | install requested | 当前只是 UI 意图状态 |
| healthy plugin state | future compatibility refresh | updated compatibility state | 未来应来自稳定 read model |
| installed engine | future verify request | verification in progress | 应由后端长任务驱动而非前端本地实现 |

---

## 5. UI and Side-effect Notes

- 当前模块没有真实副作用，所有动作按钮都只是意图入口。
- 未来引擎验证和修复必须作为后端长任务来投影，不能让组件自己管理文件扫描进度。
- 插件兼容性是引擎语义的从属摘要，不应和下载或设置模块混合建模。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 首页仍然能稳定展示当前引擎摘要
2. 启动与安装版本入口仍然清晰可见
3. 插件兼容性区域仍然存在且有明确状态语义
4. 新接入的验证或修复能力没有把后端流程重新塞回前端