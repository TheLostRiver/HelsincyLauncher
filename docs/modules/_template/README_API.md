# Module API Template

> Module: <module-name>
> Status: draft

---

## 1. Public Surface

列出允许其他模块或页面直接使用的公开入口。

| Surface | Type | Defined In | Notes |
|---------|------|------------|-------|
| <name> | <component/hook/function/type> | <path> | <purpose> |

---

## 2. Input Contracts

| Input | Shape | Required | Notes |
|-------|-------|----------|-------|
| <prop-or-arg> | <type> | <yes/no> | <notes> |

---

## 3. Output Contracts

| Output | Shape | Trigger | Notes |
|--------|-------|---------|-------|
| <event-or-return> | <type> | <when> | <notes> |

---

## 4. Data Dependencies

| Dependency | Direction | Stability | Notes |
|------------|-----------|-----------|-------|
| <dependency> | <read/write/call> | <stable/local/temporary> | <notes> |

---

## 5. Error and Empty States

写清以下行为：

- 加载失败时如何表现
- 空数据时如何表现
- 非法输入如何处理
- 哪些错误可重试，哪些只可提示

---

## 6. Compatibility Notes

记录任何不能随意改动的对外约束，例如：

- props 名称或结构
- 对外暴露的事件语义
- 页面级组合顺序
- 与后续 IPC / backend contract 的对齐约束