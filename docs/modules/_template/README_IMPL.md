# Module Implementation Template

> Module: <module-name>
> Status: draft
> Owner scope: <frontend implementation | backend implementation | mixed implementation>

---

## 1. Purpose

说明本文档记录哪些实现落点、切片顺序和验证门槛。

本文档不替代 README_ARCH / README_API / README_FLOW，也不替代专题设计文档。

---

## 2. Required Reading Before Coding

修改本模块代码前，至少阅读：

1. `README.md`
2. `CONTRIBUTING.md`
3. `docs/README.md`
4. `docs/modules/<module-name>/README_ARCH.md`
5. `docs/modules/<module-name>/README_API.md`
6. `docs/modules/<module-name>/README_FLOW.md`
7. `docs/modules/<module-name>/README_IMPL.md`
8. <related architecture/design docs>
9. <related collaboration/testing docs>

如果切片涉及错误、IPC、adapter、composition root 或 storage，在这里追加对应专题文档。

---

## 3. Current Landing Zones

| Area | Current Files | Notes |
|------|---------------|-------|
| public surface | <path> | <notes> |
| module boundary | <path> | <notes> |
| adapter/wiring | <path> | <notes> |
| tests | <path> | <notes> |

---

## 4. Current Implementation State

| Capability | Current State | Validation Shape |
|------------|---------------|------------------|
| <capability> | <implemented/not wired/placeholder> | <test or check> |

---

## 5. Implementation Slice Order

记录当前模块后续实现顺序：

1. <slice-1>
2. <slice-2>
3. <slice-3>

每个切片都应能用一个最便宜的验证动作证明。

---

## 6. Port and Dependency Status

| Dependency or Port | Status | Notes |
|--------------------|--------|-------|
| <name> | <defined/not defined/placeholder/concrete> | <notes> |

---

## 7. Error Semantics

| Code | Trigger | Retryable | Severity |
|------|---------|-----------|----------|
| <CODE> | <trigger> | <true/false> | <info/warning/error/fatal> |

---

## 8. Required Validation

1. <focused test or check>
2. <module test or check>
3. scoped `git diff --check`
4. <wider smoke only if boundary changed>

---

## 9. Non-goals

1. 不记录临时任务日志。
2. 不替代 `.artifacts/ai`。
3. 不重复专题设计文档的大段内容。
