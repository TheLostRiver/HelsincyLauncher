# Module Architecture Template

> Module: <module-name>
> Status: draft
> Owner scope: <frontend | backend | mixed>

---

## 1. Purpose

一句话说明这个模块存在的原因，以及它解决的用户问题。

---

## 2. Responsibilities

本模块负责：

- <responsibility-1>
- <responsibility-2>
- <responsibility-3>

本模块不负责：

- <non-responsibility-1>
- <non-responsibility-2>

---

## 3. Owned State

| State | Owner | Notes |
|-------|-------|-------|
| <state-name> | <module / parent / backend> | <why> |

写清哪些状态是权威状态，哪些只是派生视图状态。

---

## 4. Boundary Rules

允许依赖：

- <allowed-dependency>
- <allowed-dependency>

禁止依赖：

- <forbidden-dependency>
- <forbidden-dependency>

禁止的跨边界行为：

- <forbidden-behavior>
- <forbidden-behavior>

---

## 5. Internal Structure

| Area | Current Files | Notes |
|------|---------------|-------|
| view | <files> | <notes> |
| state | <files> | <notes> |
| contracts | <files> | <notes> |
| side effects | <files> | <notes> |

---

## 6. Coding Constraints

1. <constraint>
2. <constraint>
3. <constraint>
4. <constraint>

建议至少覆盖：命名、状态提升、异步边界、副作用位置、共享组件使用限制。

---

## 7. Change Checklist

修改本模块前，至少确认：

1. 是否改变了模块职责边界
2. 是否引入了新的跨模块依赖
3. 是否让展示层承担了不该承担的业务逻辑
4. 是否需要同步更新 README_API.md 或 README_FLOW.md