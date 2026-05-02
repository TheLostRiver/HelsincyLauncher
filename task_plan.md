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

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| 先只修正 Parent 字段 | 用户明确指出的是 Parent 所指目录有误，先做最小修正 |
| 将 Parent 统一改为 docs/TauriRewriteArchitectureBlueprint.md | docs 目录下 12 个 Parent 全部错误地指向了 .artifacts/docs 前缀 |
| 正文只改实际错误的 .artifacts/docs/ 引用 | 避免误改协议文档里仍然有意保留的 .artifacts/ai/ 目录设计 |

## Errors Encountered
| Error | Resolution |
|-------|------------|
