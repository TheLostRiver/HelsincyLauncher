# Findings & Decisions

## Requirements
- 修正 docs 目录下文档里的 Parent 所指目录。

## Research Findings
- 当前打开的 docs/TauriAIDevelopmentTransactionProtocolDesign.md 的 Parent 为 `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`。
- 当前文件实际位于 docs 目录，因此 Parent 路径前缀很可能应为 `docs/` 而不是 `.artifacts/docs/`。
- docs 目录下共有 12 个文件包含 Parent 字段，且 12 个 Parent 全部指向 `.artifacts/docs/TauriRewriteArchitectureBlueprint.md`。
- 当前针对 Parent 的最小修正是把该路径统一改成 `docs/TauriRewriteArchitectureBlueprint.md`。
- 继续扫描后，docs 正文里仅剩 3 处 `.artifacts/docs/` 残留，全部位于 docs/TauriAIDevelopmentTransactionProtocolDesign.md。
- 这 3 处分别是：正文说明 1 处、active-task 模板中的 allowed_files 1 处、required_context 1 处。
- 修正完成后，docs/*.md 中已无任何 `.artifacts/docs/` 残留。

## Technical Decisions
| Decision | Rationale |
|----------|-----------|
| 先验证 docs 目录中 Parent 字段的分布再统一修正 | 避免只改当前文件而漏掉同类文档 |
| 只继续修正文档正文中的 `.artifacts/docs/` 路径 | 用户明确要求把正文路径错误也一起改掉 |
| 保留 `.artifacts/ai/` 协议设计 | 这是文档正文里的协议目录约定，不属于错误的 docs 路径引用 |

## Issues Encountered
| Issue | Resolution |
|-------|------------|

## Resources
- 当前打开文件: docs/TauriAIDevelopmentTransactionProtocolDesign.md
- grep `^> Parent:` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md after header fix
