# Module Flow Template

> Module: <module-name>
> Status: draft

---

## 1. Main User Flows

| Flow | Trigger | Success Outcome | Failure Outcome |
|------|---------|-----------------|-----------------|
| <flow-name> | <trigger> | <result> | <result> |

---

## 2. Happy Path

1. <step>
2. <step>
3. <step>

---

## 3. Alternate Paths

| Case | Expected Behavior |
|------|-------------------|
| loading | <behavior> |
| empty | <behavior> |
| partial data | <behavior> |
| failure | <behavior> |

---

## 4. State Transitions

| From | Event | To | Notes |
|------|-------|----|-------|
| <state> | <event> | <state> | <notes> |

---

## 5. UI and Side-effect Notes

记录：

- 哪些交互只改本地视图状态
- 哪些交互会触发远端数据或后端动作
- 哪些动画或过渡是功能性的，不能随意删掉

---

## 6. Regression Checklist

每次修改后至少确认：

1. 主流程仍然可走通
2. loading / empty / error 三类状态仍然存在且语义一致
3. 没有新增跨模块耦合
4. 文档中声明的状态迁移仍然成立