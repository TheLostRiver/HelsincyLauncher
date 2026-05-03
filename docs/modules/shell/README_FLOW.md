# Shell Module Flow

> Module: shell
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 打开启动器首页 | 应用首次渲染 | shell 以 home 作为默认页面完成装配 | 若页面元数据缺失，标题区无法正确渲染 |
| 切换到其他页面 | 点击 Sidebar 中有效导航项 | activePage 更新，内容区切换并完成过渡动画 | 若切换被阻止，则保持当前页面不变 |
| 查看页面标题 | activePage 变更后 | TopBar 展示新的 kicker 和 title | 若映射缺失，标题区语义失真 |

---

## 2. Happy Path

1. shell 以 home 初始化 activePage。
2. Home 根据 activePage 读取 pageMeta 并渲染 TopBar。
3. Sidebar 渲染导航项并标记当前激活页面。
4. 用户点击其他有效导航项后，shell 先进入 transitioning。
5. 延时后更新 activePage，再退出 transitioning，完成内容区过渡。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| repeated click | 点击当前页面时不应重复触发切换 |
| transition lock | transitioning 为 true 时忽略新的切页请求 |
| null nav item | activeOn 为 null 的导航项只展示，不执行切页 |
| future route growth | 新页面接入时继续走统一 pageMeta + activePage 装配流程 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| home | click my-projects | transitioning=true | 先进入过渡锁定期 |
| transitioning=true | timeout 200ms | activePage=my-projects | 内容区切换发生 |
| activePage changed | timeout 50ms | transitioning=false | 解除视觉过渡状态 |
| any page | click same page | same page | 不产生状态变化 |

---

## 5. UI and Side-effect Notes

- 目前唯一副作用是 setTimeout 驱动的视觉过渡。
- Sidebar 按钮的点击只会上抛 page id，不直接产生内容副作用。
- TopBar 的搜索框、主题切换、账户入口目前都是视觉占位；接入真实行为时要保持壳层边界稳定。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 默认仍然进入 home 页面
2. Sidebar 激活态与内容区页面一致
3. 切换过渡不会因重复点击而抖动
4. TopBar 标题随 activePage 正确变化