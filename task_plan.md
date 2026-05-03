# Task Plan: Fix Docs Parent Paths

## Goal
修正 docs 目录下文档元数据中的 Parent 路径，使其指向实际的 docs 目录位置。

## Current Phase
Phase 5

## Phases

### Phase 1: Requirements & Discovery
- [x] Understand user intent
- [x] Identify all docs files with incorrect Parent paths
- [x] Document in findings.md
- **Status:** complete

### Phase 2: Planning & Structure
- [x] Confirm the correct target prefix for Parent paths
- [x] Define the minimal edit set
- **Status:** complete

### Phase 3: Implementation
- [x] Update remaining body references from .artifacts/docs/ to docs/
- **Status:** complete

### Phase 4: Testing & Verification
- [x] Verify no .artifacts/docs/ references remain under docs
- **Status:** complete

### Phase 5: Delivery
- [x] Summarize corrected files and remaining scope
- **Status:** complete

### Phase 6: Module Documentation Rollout
- [x] Define a reusable module documentation standard and templates
- [x] Instantiate shell module docs
- [x] Instantiate fab-inventory module docs
- **Status:** complete

### Phase 7: Commit Grouping
- [x] Commit module documentation standard and templates separately
- [x] Commit shell and fab-inventory module guides separately
- [x] Keep task_plan.md, findings.md, and progress.md together for the final planning commit
- **Status:** complete

### Phase 8: Engines And Downloads Rollout
- [x] Instantiate engines module docs
- [x] Commit engines module guides separately
- [x] Instantiate downloads module docs
- [x] Commit downloads module guides separately
- **Status:** complete

### Phase 9: Projects Rollout
- [x] Instantiate projects module docs
- [x] Commit projects module guides separately
- [x] Keep task_plan.md, findings.md, and progress.md together for the follow-up planning commit
- **Status:** complete

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| 先只修正 Parent 字段 | 用户明确指出的是 Parent 所指目录有误，先做最小修正 |
| 将 Parent 统一改为 docs/TauriRewriteArchitectureBlueprint.md | docs 目录下 12 个 Parent 全部错误地指向了 .artifacts/docs 前缀 |
| 正文只改实际错误的 .artifacts/docs/ 引用 | 避免误改协议文档里仍然有意保留的 .artifacts/ai/ 目录设计 |
| 模块文档按业务切片建档，不按每个小组件建档 | 当前仓库更适合围绕 shell、fab-inventory 等边界模块建立三件套文档 |
| 新增模块文档按三组提交拆分 | 先提交规范与模板，再提交实例模块文档，最后单独提交 planning 三件套，边界更清晰 |
| 继续补 engines 后再补 downloads | 用户明确要求优先顺序，且两者都应各自独立提交 |
| 在未指定 settings 前先补 projects | 现有模块优先顺序里 projects 在 settings 之前，且用户只选择继续该方向 |

## Errors Encountered
| Error | Resolution |
|-------|------------|
