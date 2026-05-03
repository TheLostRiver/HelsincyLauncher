# Engines Module Architecture

> Module: engines
> Status: draft
> Owner scope: frontend now, mixed later

---

## 1. Purpose

engines 模块负责呈现 Unreal Engine 安装版本的当前状态、可执行动作以及插件兼容性摘要，并为后续引擎验证与修复流程提供前端入口。

---

## 2. Responsibilities

本模块负责：

- 展示当前引擎版本、体积和可用性摘要
- 暴露启动引擎与安装新版本的用户意图入口
- 呈现插件兼容性摘要
- 为后续验证与修复能力预留稳定入口和文案语义

本模块不负责：

- 直接扫描引擎目录或计算文件校验结果
- 实现下载、修复、安装落盘等后端流程
- 自行决定引擎 manifest、组件语义和 repair plan

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| current engine summary | engines | 当前组件内联的引擎摘要展示数据 |
| plugin compatibility list | engines | 当前组件内联的插件兼容性列表 |
| action intent buttons | engines | 启动与安装版本按钮只表达意图，不承载真实副作用 |

当前模块还没有显式本地交互状态。后续若接入多版本切换、验证任务和修复确认，应由 engines 模块自己拥有对应视图状态。

---

## 4. Boundary Rules

允许依赖：

- components/EngineManagement.tsx 作为当前前端入口
- shell 或首页组合层对该模块的挂载
- docs/TauriEngineVerificationRepairDesign.md 中定义的未来后端边界

禁止依赖：

- 下载模块的内部调度器或下载状态实现
- 安装模块的底层 verifier 或 repair executor 细节
- 未抽象的文件系统扫描逻辑

禁止的跨边界行为：

- 在组件内部直接实现引擎验证或修复逻辑
- 直接读取本地引擎目录来决定 UI 状态
- 让 engines 模块自己承担下载 payload 获取职责

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| summary view | components/EngineManagement.tsx | 当前引擎状态卡片与动作入口 |
| compatibility view | components/EngineManagement.tsx | 插件兼容性摘要列表 |
| future verification entry | components/EngineManagement.tsx | 当前未实现，但应属于同一模块边界 |
| backend alignment | docs/TauriEngineVerificationRepairDesign.md | 约束未来前后端职责划分 |

---

## 6. Coding Constraints

1. engines 只负责引擎语义、展示与用户意图入口，不负责底层文件验证实现。
2. 引擎版本摘要、插件兼容性、验证结果和修复结果应保持为同一模块的统一语义，不要散落到无关页面。
3. 后续接入真实行为时，前端只消费稳定 summary 或 read model，不直接依赖文件级噪声结果。
4. 若引入验证、修复、安装等长任务入口，必须通过明确契约连接到后端 facade，而不是在页面里拼临时流程。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否把引擎语义和下载或安装实现耦合在了一起
2. 是否让前端承担了不该承担的扫描、校验或修复逻辑
3. 是否保持了“引擎是什么”和“如何修复”这两层边界分离
4. 是否需要同步更新 README_API.md 或 README_FLOW.md