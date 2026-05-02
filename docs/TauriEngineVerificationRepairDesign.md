# Tauri Engine Verification and Repair Design

> Status: local draft v1
> Date: 2026-05-02
> Parent: `docs/TauriRewriteArchitectureBlueprint.md`
> Focus: verify -> diff -> repair plan -> targeted repair for installed Unreal Engine builds

---

## 1. Purpose

本文档把蓝图中的 `12.5 Engine Verification and Repair Model` 展开为专项设计。

目标不是简单复刻“验证按钮”，而是把 Unreal Engine 已安装版本的健康检查和修复建模成一条稳定、低耦合、可恢复的后端流程：

1. `Engines` 决定“应该是什么”
2. `Installations` 决定“现在是什么”以及“怎么验证/修复”
3. `Downloads` 决定“缺什么内容时如何获取”
4. 前端只负责发起、展示差异摘要、确认修复和展示结果

---

## 2. Design Goals

### 2.1 Mandatory Goals

| Goal | Meaning |
|------|---------|
| Explicit ownership | 引擎语义、通用校验、下载获取三类职责完全拆开 |
| Incremental verification | 默认只校验必要范围，不做无意义全量重复扫描 |
| Actionable diff | 验证结果必须产出结构化差异，而不是模糊“有问题” |
| Repair planning | 修复前先生成计划和影响摘要 |
| Resume safety | 长时间验证和修复任务支持暂停/恢复/重启恢复 |
| Stable UI projection | 前端只看稳定 read model，不消费底层文件级噪声 |

### 2.2 Non-goals

| Non-goal | Reason |
|----------|--------|
| 前端自己扫描引擎目录 | 违反前后端职责分离 |
| 发现问题后直接整包重下整个引擎 | 成本过高，且掩盖差异建模不足 |
| `Engines` 模块自己实现文件 hash 和落盘修复 | 会和 Installations 职责重叠 |
| 让 Downloads 直接理解引擎业务组件语义 | 会打穿模块边界 |

---

## 3. Module Ownership

| Concern | Owner |
|---------|-------|
| 引擎版本清单、组件清单、期望 manifest 解析 | `module-engines` |
| 本地安装扫描、文件校验、修复执行 | `module-installations` |
| 缺失 payload 下载、断点续传、限流 | `module-downloads` |
| 平台路径解析、文件系统差异 | platform adapters |
| UI 呈现、用户确认、差异展示 | Frontend |

边界规则：

1. `Engines` 可以定义“某个 UE 版本应该包含哪些组件/文件”，但不直接计算文件 hash。
2. `Installations` 可以执行通用验证和修复，但不能自行臆造某个版本的组件语义。
3. `Downloads` 只处理 repair payload 获取，不直接改安装状态机。
4. 若存在跨域长流程，必须通过显式 `EngineRepairOrchestrator` 编排，不允许模块互相偷调内部仓储。

---

## 4. Core Domain Model

### 4.1 Engine Installation Identity

建议统一标识：

- `engine_installation_id`
- `engine_version_id`
- `install_root`
- `manifest_revision`
- `provider`
- `installed_at`

### 4.2 Expected Manifest

由 `Engines` 负责提供标准化后的期望安装清单：

- 组件列表
- 文件列表
- 文件长度
- hash / checksum
- 可选标记（optional component）
- 平台特定文件规则

### 4.3 Verification Diff

验证结果必须先落成结构化 diff，而不是直接输出文本消息。

建议差异类型：

- `MissingFile`
- `CorruptedFile`
- `UnexpectedFile`
- `MissingComponent`
- `VersionMismatch`
- `PermissionDenied`
- `ManifestUnavailable`

### 4.4 Repair Plan

repair plan 是后续动作的唯一输入，不允许边验证边即兴修。

建议字段：

- `plan_id`
- `engine_installation_id`
- `manifest_revision`
- `actions[]`
- `estimated_download_bytes`
- `estimated_repair_count`
- `requires_user_confirmation`
- `generated_at`

---

## 5. End-to-End Flow

### 5.1 Verify Only

```text
user clicks verify
  -> engines.verify.start(engineInstallationId)
  -> orchestrator loads expected manifest
  -> installations verifier scans local installation
  -> verifier emits structured diff
  -> projector writes verification summary
  -> ui shows Healthy or NeedsRepair
```

### 5.2 Verify and Repair

```text
user confirms repair
  -> engines.repair.plan(engineInstallationId)
  -> orchestrator builds repair plan from diff
  -> downloads fetch required payloads only
  -> installations repair executor applies replacements
  -> lightweight re-verify
  -> projector marks Repaired or Failed
```

### 5.3 Auto-repair Policy

仅允许对低风险动作自动修复，例如：

1. 已知缺失的单个小型 metadata 文件
2. 已有 payload 且无需额外交互的损坏文件替换

默认情况下，存在以下情况必须要求用户确认：

1. 预计下载量较大
2. 需要覆盖用户本地修改文件
3. 需要修复多个核心组件

---

## 6. Verification Modes

### 6.1 Quick Verify

适用场景：页面首次展示健康状态、后台轻量复检。

策略：

1. 先校验 manifest 元数据和关键入口文件
2. 命中异常后再升级为 deeper scan
3. 不立即做全量 hash

### 6.2 Full Verify

适用场景：用户主动点击验证、更新后自检、修复后的复验。

策略：

1. 遍历 manifest 记录的全部关键文件
2. 计算长度、存在性、必要时计算 hash
3. 收敛为结构化 diff

### 6.3 Targeted Re-verify

适用场景：repair 之后。

策略：

1. 只重检 repair plan 触及的组件/文件
2. 若 targeted re-verify 仍失败，再升级为 full verify

---

## 7. Manifest and Component Strategy

### 7.1 Engine Manifest Provider Port

建议由 `Engines` 暴露统一端口：

```text
EngineManifestProviderPort
  -> get_engine_manifest(engineVersionId, platform)
  -> get_component_catalog(engineVersionId)
  -> resolve_repair_payload(diffItems)
```

不同 provider 或不同版本格式的差异只留在 adapter 层。

### 7.2 Component-aware Verification

不要只做平铺文件校验，还要保留组件层摘要：

- Core engine
- Templates
- Debug symbols
- Optional tools
- Platform SDK bindings

这样 UI 和 repair plan 才能展示“哪一类组件损坏”，而不是只吐一堆文件路径。

### 7.3 Platform-specific Rules

平台差异统一收口在 manifest 解释和路径 adapter：

1. Windows/macOS/Linux 路径大小写和分隔符规则不同
2. 某些平台的符号链接、权限位需要特殊处理
3. 核心验证逻辑不直接写平台分支

---

## 8. Orchestrator Design

建议显式引入：

- `EngineVerificationFacade`
- `EngineRepairOrchestrator`

### 8.1 `EngineVerificationFacade`

职责：

1. 接收验证/修复命令
2. 检查当前安装记录和并发冲突
3. 创建 job 并交给 orchestrator

### 8.2 `EngineRepairOrchestrator`

职责：

1. 加载 expected manifest
2. 调用 Installations verifier
3. 生成 diff
4. 生成 repair plan
5. 需要时驱动 Downloads 获取 payload
6. 驱动 Installations repair executor
7. 复验并更新投影

这层只能编排，不得把校验或下载细节内联进去。

---

## 9. Job Runtime and State Projection

### 9.1 Internal States

建议内部状态：

- `Queued`
- `LoadingManifest`
- `ScanningInstallation`
- `Diffing`
- `AwaitingConfirmation`
- `PlanningRepair`
- `DownloadingRepairPayload`
- `ApplyingRepair`
- `ReVerifying`
- `Completed`
- `Failed`
- `Canceled`

### 9.2 UI Projection

对前端只公开收敛状态：

- `Healthy`
- `Verifying`
- `NeedsRepair`
- `Repairing`
- `Repaired`
- `Failed`
- `Canceled`

### 9.3 Snapshot Fields

建议统一 job snapshot 字段：

- `jobId`
- `engineInstallationId`
- `engineVersion`
- `uiState`
- `progress.phase`
- `progress.scannedFiles`
- `progress.totalFiles`
- `diffSummary`
- `estimatedRepairBytes`
- `retryable`
- `error`
- `updatedAt`

---

## 10. Diff and Repair Plan Projection

### 10.1 Verification Summary Read Model

建议单独维护：

- `engine_verification_summary`
- `engine_verification_diff_item`
- `engine_repair_plan_projection`

### 10.2 Summary Fields

`engine_verification_summary` 建议字段：

- `engine_installation_id`
- `last_verified_at`
- `verification_mode`
- `result_state`
- `missing_file_count`
- `corrupted_file_count`
- `missing_component_count`
- `estimated_repair_bytes`
- `last_error_code`

### 10.3 Diff Item Fields

- `diff_item_id`
- `diff_type`
- `component_id`
- `relative_path`
- `expected_hash`
- `actual_hash`
- `severity`
- `repair_strategy`

repair strategy 只能是显式枚举，例如：

- `ReDownloadFile`
- `RestoreComponent`
- `IgnoreOptional`
- `ManualActionRequired`

---

## 11. Interaction with Downloads and Installations

### 11.1 Downloads Contract

引擎修复不应调用“普通下载整个引擎”的粗粒度接口，而应走 payload 级 contract，例如：

```text
downloads.fetch_repair_payload({ provider, payloadRefs[], priority, reason })
```

Downloads 返回：

- payload artifact refs
- job status
- retry / failure classification

### 11.2 Installations Contract

Installations 需要提供通用能力：

```text
installations.verify_installation({ installRoot, manifestRef, mode })
installations.apply_repair({ installRoot, repairActions[], payloadRefs[] })
installations.reverify_targets({ installRoot, targets[] })
```

### 11.3 Boundary Guardrails

1. `Engines` 不直接写安装目录。
2. `Installations` 不自己解析引擎版本 catalog 业务语义。
3. `Downloads` 不自己决定哪些 diff item 应该被修复。

---

## 12. Resume and Recovery

### 12.1 Checkpoint Model

长任务需要至少记录：

- 当前 phase
- 已扫描文件游标或分区进度
- diff 摘要中间结果
- repair plan ref
- repair payload download job refs

### 12.2 Restart Recovery

应用重启后：

1. 恢复未终态验证/修复 job
2. 若停在 `AwaitingConfirmation`，恢复到可继续确认状态
3. 若停在 `DownloadingRepairPayload`，复用 Downloads 的 checkpoint
4. 若停在 `ApplyingRepair`，先做 staging / temp state 检查，再决定继续或回滚

### 12.3 Safe Rollback

对会覆盖原文件的 repair action，建议至少支持：

1. 写入前备份或 staging 替换
2. 原子 rename 或等价安全切换
3. repair 失败时保留最小恢复线索

---

## 13. Frontend Integration Rules

前端只负责：

1. 展示引擎健康状态标签
2. 发起 `verify` 命令
3. 展示 diff summary 和 repair 影响摘要
4. 在需要时让用户确认 repair
5. 订阅统一 job snapshot

前端不负责：

1. 遍历磁盘
2. 计算 hash
3. 推断 repair plan
4. 驱动细粒度文件替换

建议 UI read model：

- `Healthy`
- `Needs Repair`
- `Repairing`
- `Last Verified At`
- `Missing Files`
- `Corrupted Files`
- `Estimated Download`

---

## 14. Observability

建议至少采集：

- `engine_verify_duration_ms`
- `engine_verify_scanned_files`
- `engine_verify_diff_items`
- `engine_repair_estimated_bytes`
- `engine_repair_download_duration_ms`
- `engine_repair_apply_duration_ms`
- `engine_repair_success_rate`

建议日志字段：

- `engineInstallationId`
- `engineVersionId`
- `manifestRevision`
- `verificationMode`
- `phase`
- `diffType`
- `componentId`
- `repairStrategy`
- `durationMs`

---

## 15. Acceptance Criteria

满足以下条件，才算这条专项设计达标：

1. 引擎验证已经被明确定义为 `verify -> diff -> repair plan -> targeted repair` 流程。
2. `Engines`、`Installations`、`Downloads` 的职责边界清晰，没有互相偷写内部状态。
3. 验证结果能产出结构化 diff 和 repair plan，而不是只有文本提示。
4. 修复默认优先做定向修复，而不是直接整包重下整个引擎。
5. 验证和修复任务都能纳入统一长任务 runtime，并支持恢复。

---

## 16. Anti-patterns

以下做法视为违规：

1. 前端直接做目录扫描和 hash 计算。
2. `Engines` 自己实现文件校验和写盘修复。
3. 发现少量损坏后直接重下整个引擎，不生成 diff 和 repair plan。
4. `Downloads` 直接决定引擎业务组件怎么修。
5. 把验证和修复做成一堆不可恢复的同步阻塞操作。
