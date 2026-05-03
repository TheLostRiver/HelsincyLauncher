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
| 模块文档优先面向业务模块和壳层边界 | 当前组件树里真正有独立职责和演进风险的是 shell、fab-inventory 这类切片 |

## Additional Findings
- 当前 shell 模块的核心边界由 app/page.tsx、components/Sidebar.tsx、components/TopBar.tsx 组成。
- shell 当前只拥有页面级视图状态：activePage、transitioning、pageMeta。
- fab-inventory 当前由两个前端入口组成：首页概览卡片 components/FabInventory.tsx 和独立库存页 components/FabInventoryContent.tsx。
- fab-inventory 目前仍是静态展示模块，搜索、筛选、同步、导入工程都还是 UI 意图入口，尚未接入真实副作用。
- 对当前仓库而言，给每个小组件单独写文档的收益低于围绕业务切片写 README_ARCH.md、README_API.md、README_FLOW.md。
- 当前工作区中新生成的模块文档最自然的提交拆分是：规范与模板一组、实例化模块文档一组、planning 三件套一组。
- 当前 engines 模块在前端只有 components/EngineManagement.tsx 这一个入口，但其未来职责边界已经被 docs/TauriEngineVerificationRepairDesign.md 明确约束。
- 当前 downloads 模块在前端只有 components/Downloads.tsx 这一个入口，但其未来边界已被 docs/TauriDownloadRuntimeDesign.md 中的 facade、scheduler、checkpoint 模型约束。
- engines 与 downloads 都仍然是静态摘要组件，因此文档应强调“前端只表达意图，后端拥有真实长任务与状态机”。
- 当前这两组模块文档最自然的 git 拆分就是一组 engines、一组 downloads，分别独立提交。
- 当前 projects 模块由 components/MyProjectsContent.tsx 单独承载，已经包含工程列表、选中详情、健康状态和操作入口四类语义。
- projects 模块与 engines、fab-inventory 的关系更适合通过稳定摘要字段表达，而不是直接依赖其他模块内部状态。
- 当前用户只选择继续该方向，因此最自然的下一步是按既定优先级先补 projects，而不是跳到 settings。

## Issues Encountered
| Issue | Resolution |
|-------|------------|

## Resources
- 当前打开文件: docs/TauriAIDevelopmentTransactionProtocolDesign.md
- grep `^> Parent:` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md after header fix
