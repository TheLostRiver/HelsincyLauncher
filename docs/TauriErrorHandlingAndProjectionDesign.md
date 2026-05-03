# Tauri Error Handling And Projection Design

> Status: local draft v1
> Date: 2026-05-03
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Depends on: `docs/TauriIPCAndStateContractsDesign.md`, `docs/TauriRustTsSchemaDesign.md`, `docs/TauriLoggingAndObservabilityDesign.md`, `docs/TauriDownloadRuntimeDesign.md`, `docs/TauriEngineVerificationRepairDesign.md`
> Focus: error model, error envelope, severity/retryable semantics, user-facing projection, retry ownership

---

## 1. Purpose

当前仓库不是完全没有错误处理设计。

问题在于：**`AppError` / `AppErrorDto`、错误 code 前缀、`retryable`、`severity`、`correlationId`、用户态错误投影和长任务失败语义已经散落在多份文档里，但还没有一份独立的错误处理总文档。**

这会导致实现阶段出现几个典型退化：

1. 每个模块各自发明错误 code 和 message 风格。
2. 前端开始依赖临时字符串而不是稳定错误契约。
3. `retryable` 被误用成“自动重试策略本身”。
4. 日志、diagnostics 和用户提示混成一层。

---

## 2. Existing State

当前错误相关内容主要散落在：

| Document | What it already defines |
|----------|-------------------------|
| `docs/TauriRewriteArchitectureBlueprint.md` | error envelope、错误 code 前缀、用户态投影、`correlationId` |
| `docs/TauriIPCAndStateContractsDesign.md` | `AppErrorDto` shape、command/query error envelope、长任务失败结果 |
| `docs/TauriRustTsSchemaDesign.md` | Rust/TS 共享 DTO 结构 |
| `docs/TauriDownloadRuntimeDesign.md` | 下载域错误分类和 retryable 语义 |
| `docs/TauriEngineVerificationRepairDesign.md` | 引擎验证域的 severity / retryable 要求 |
| `docs/TauriLoggingAndObservabilityDesign.md` | error log 的 correlation 和 observability 边界 |

结论：

1. 错误模型原则已经存在。
2. 当前缺的是**独立的仓库级错误处理与错误投影总文档**。

---

## 3. Goals

这份文档必须统一回答：

1. 后端内部错误如何收敛为统一 `AppError`。
2. 对前端公开的错误 DTO 必须长什么样。
3. `retryable`、`severity`、`correlationId` 分别表达什么，不表达什么。
4. 用户提示、日志记录和 diagnostics 应如何分层。
5. command、query、job failure 必须如何投影。

这份文档不负责：

1. 取代各模块自己的错误枚举实现。
2. 定义所有模块的完整错误 code 清单。
3. 取代日志 sink、diagnostics 面板或告警策略文档。

---

## 4. Core Principles

1. one public error contract：前端只能看到统一 `AppErrorDto` 形状。
2. stable codes over fragile text：前端决策依赖稳定错误 code，而不是 message 文本。
3. projection over leakage：前端拿到的是投影后的错误，不是原始技术异常。
4. retry hint, not retry engine：`retryable` 只是可重试提示，不是具体重试策略。
5. correlation is mandatory：每个对外失败都必须带 `correlationId`。
6. user message != diagnostics payload：面向用户的信息与面向工程排障的信息必须分层。

---

## 5. Shared Error Contract

对前端公开的错误 DTO 必须保持以下最小稳定 shape：

```ts
type AppErrorDto = {
  code: string;
  message: string;
  retryable: boolean;
  severity: "info" | "warning" | "error" | "fatal";
  correlationId: string;
};
```

字段语义：

| Field | Meaning | Must not mean |
|-------|---------|---------------|
| `code` | 稳定的错误分类键 | 展示给用户的完整文案 |
| `message` | 适合投影到 UI 的简短说明 | 原始 stack trace |
| `retryable` | 当前失败是否存在合理重试空间 | 后端一定会自动重试 |
| `severity` | 失败或异常的重要级别 | 告警策略本身 |
| `correlationId` | 用于串起日志、任务、命令和 diagnostics | 业务实体 ID |

---

## 6. Error Layers

### 6.1 Internal Layer

后端内部可以存在更细粒度的 domain / adapter / infrastructure error。

这些错误允许包含：

1. 第三方库错误
2. 原始 IO 失败
3. 供应商响应细节
4. 技术上下文信息

但这些内部错误不能直接穿透到前端。

### 6.2 Application Layer

在 application / facade / command handler 边界，错误必须被收敛为统一 `AppError`。

`AppError` 至少应能提供：

1. stable code
2. projected message
3. retryable
4. severity
5. correlationId

### 6.3 Transport Layer

跨 IPC 发给前端时，只能传 `AppErrorDto`。

禁止：

1. 直接序列化底层异常类型
2. 直接序列化 Rust backtrace
3. 直接暴露 HTTP body、SQL 语句、绝对文件路径等原始技术细节

---

## 7. Envelope Rules

### 7.1 Command / Query

统一响应格式：

```ts
type CommandResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: AppErrorDto };
```

```ts
type QueryResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: AppErrorDto };
```

### 7.2 Long Jobs

长任务失败必须通过统一快照或结果对象暴露 `error` 字段，而不是另起一套协议。

统一要求：

1. job 接受失败时，直接返回 `AppErrorDto`
2. job 运行中失败时，在 snapshot/result 中带统一错误 DTO
3. UI 不应因为模块不同而写多套失败解析逻辑

---

## 8. Code Taxonomy

错误 code 必须可稳定匹配，并建议使用模块前缀。

当前已明确存在的前缀包括：

1. `AUTH_*`
2. `LIB_*`
3. `DL_*`
4. `INS_*`
5. `ENG_*`

约束：

1. 前缀表示领域归属，不表示 transport 类型。
2. code 一旦公开给前端，就必须保持向后兼容。
3. 新模块必须新增自己的清晰前缀，不能复用别人的领域 code。

禁止：

1. 用自然语言句子当 code
2. 把临时调试信息拼进 code
3. 让前端靠 `message.includes(...)` 判断错误类型

---

## 9. Severity Semantics

建议统一语义如下：

| Severity | Meaning |
|----------|---------|
| `info` | 非失败性提示或低风险异常状态 |
| `warning` | 功能受限但可继续使用 |
| `error` | 当前操作失败，需要用户关注或重试 |
| `fatal` | 当前启动或核心能力不可继续 |

规则：

1. `severity` 描述影响程度，不描述是否自动重试。
2. `fatal` 只应用于真正阻断核心流程的失败。
3. 大多数 provider/network 短暂失败应落在 `warning` 或 `error`，不是默认 `fatal`。

---

## 10. Retry Semantics

`retryable` 的语义必须固定为：**从产品和系统视角看，这个失败是否存在合理重试空间。**

它不表达：

1. 立即自动重试
2. 重试次数
3. backoff 策略
4. 是否由前端执行重试

责任边界：

1. backend 决定是否自动重试以及如何重试
2. frontend 只根据 `retryable` 决定 UI 是否展示“重试” affordance
3. 一个错误即使 `retryable = true`，也不代表当前瞬间必须马上重试

---

## 11. User-facing Projection

用户态错误投影必须满足：

1. 文案简短
2. 可被本地化
3. 不泄露技术细节
4. 必要时可引导下一步动作

推荐策略：

1. 前端优先基于 `code` 做展示映射
2. 若无专门映射，再退回 `message`
3. `correlationId` 可在高级详情或复制诊断信息时暴露

禁止：

1. 把原始 adapter 异常直接展示给用户
2. 把调试堆栈拼进 toast
3. 让不同模块对同类错误给出完全不同的用户动作语义

---

## 12. Logging And Diagnostics Boundary

错误处理与 logging/diagnostics 的关系如下：

1. `AppErrorDto` 是用户和前端消费的失败投影
2. 结构化日志负责保留工程可排查信息
3. diagnostics 负责汇总可展示的诊断摘要，而不是直接转储原始日志

因此：

1. 用户 message 不能替代日志
2. 日志也不能替代用户 message
3. diagnostics 面板不能要求前端理解底层异常类型

所有对外失败至少应可追踪到：

1. `correlationId`
2. `code`
3. `severity`
4. `retryable`

---

## 13. Module Responsibilities

| Concern | Owner |
|---------|-------|
| low-level exception capture | adapters / infrastructure |
| domain-level classification | domain / use-case layer |
| public `AppError` projection | application / facade layer |
| user-facing rendering | frontend shell / module UI |
| log enrichment and correlation | logging boundary |

禁止：

1. 前端自己猜测底层 HTTP/IO 错误类型
2. adapter 直接产出最终 UI 文案
3. command handler 直接把未分类异常传给 IPC 层

---

## 14. Testing Requirements

错误模型至少要有以下测试：

1. foundation contract tests：验证 `AppError` / `AppErrorDto` shape 和映射稳定
2. module-level tests：验证关键模块错误 code、`retryable`、`severity` 分类是否正确
3. transport tests：验证 command/query/job failure 都走统一 envelope

特别是第一版骨架最少要保证：

1. `AppError` 到 `AppErrorDto` 的映射测试存在
2. `CommandResult` / `QueryResult` 的失败 envelope 测试存在

---

## 15. Exit Criteria

当满足下面几条时，可以认为“错误处理总文档缺口已经补上”：

1. 仓库里已经存在独立的错误处理与错误投影总文档。
2. `AppErrorDto`、code taxonomy、`severity`、`retryable` 和用户态投影边界已被统一定义。
3. logging/diagnostics 与 user-facing error projection 的职责不再混淆。
4. 后续实现可以直接引用这份文档，而不是从 schema、IPC、下载、引擎文档里拼错误规则。