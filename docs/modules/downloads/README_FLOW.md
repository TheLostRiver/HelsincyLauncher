# Downloads Module Flow

> Module: downloads
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| 首页查看下载摘要 | 用户停留在 home 页面 | 看到活动任务、排队任务和速率摘要 | 若未来无任务，应显示空队列态 |
| 查看活动任务进度 | 首页渲染活动下载 | 看到聚合百分比和进度条 | 若未来数据异常，应显示可降级状态而不是错误百分比 |
| 发起暂停全部意图 | 用户点击暂停全部 | 进入后续真实任务控制入口 | 当前未接线，只停留在按钮层 |
| 打开下载位置意图 | 用户点击目录按钮 | 进入后续真实目录打开入口 | 当前未接线，只停留在按钮层 |

---

## 2. Happy Path

1. 首页装配 Downloads 组件。
2. 组件展示当前下载速率摘要。
3. 用户看到一个活动下载任务及其聚合进度条。
4. 用户同时看到一个排队任务和基础控制入口。
5. 后续接入真实运行时后，仍应保持“聚合投影先于底层细节”的展示原则。

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| no active downloads | 应显示空任务或仅队列状态，而不是保留失真进度条 |
| all tasks paused | 任务摘要应清楚投影为暂停态 |
| task failed later | 失败任务应显示稳定错误摘要，而不是泄露内部异常噪声 |
| download page added later | 首页卡片继续做摘要，完整控制交给独立下载视图 |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| queued | future scheduler dispatch | downloading | 应来自后端任务投影 |
| downloading | pause all | paused | 前端只发控制意图 |
| downloading | complete | completed | 首页摘要可移出活动卡片 |
| downloading | failure | failed | 应显示聚合错误摘要 |

---

## 5. UI and Side-effect Notes

- 当前模块没有真实副作用，所有按钮都只是操作入口。
- 后续真实下载运行时必须通过统一长任务协议投影到 UI，不能让组件自行推导恢复、并发和校验状态。
- 速率、百分比和任务状态都应视为后端事实投影，而不是前端二次真相。

---

## 6. Regression Checklist

每次修改后至少确认：

1. 首页仍能稳定展示下载摘要
2. 活动任务和排队任务语义仍然清晰分离
3. 聚合进度和速率展示没有退化为底层实现泄露
4. 新接入的任务控制没有把运行时逻辑重新塞回前端