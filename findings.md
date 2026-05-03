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
- 当前 settings 模块由 components/Settings.tsx 单独承载，语义集中在两个高频设置开关和一个本地库路径摘要。
- settings 模块最关键的边界是“前端只展示配置摘要与修改意图，真实配置读写和迁移仍应在后端或独立配置层”。
- 当前 settings 文档的最自然 git 拆分是一组 settings 三件套，再单独跟一个 planning 三件套提交。
- 当前 account-auth 模块由 components/AccountLogin.tsx 单独承载，语义集中在登录入口、同步价值说明和组织许可证状态摘要。
- account-auth 模块最关键的边界是“前端只展示认证摘要与登录意图，真实 OAuth、token 生命周期和安全存储仍应在后端或独立认证层”。
- 当前 account-auth 文档的最自然 git 拆分是一组 account-auth 三件套，再单独跟一个 planning 三件套提交。
- 当前仓库仍是纯 Next.js 前端原型：根目录只有 `package.json`、`app/`、`components/` 等前端文件，不存在根 `Cargo.toml`、`src-tauri/` 或 `crates/`。
- 现有 `docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md`、`docs/TauriCompositionRootWiringDesign.md` 和 `docs/TauriFirstCrateApiDrafts.md` 已经定义了后端目标结构，但仍缺一个“基于当前仓库如何真正落骨架”的实现文档。
- 对当前仓库而言，第一版后端骨架最合理的落地路径不是先迁移到 `apps/desktop/`，而是先保留根目录 Next.js 前端，在根目录新增 `Cargo.toml`、`src-tauri/` 和 `crates/`。
- 当后端骨架实现文档进一步用于开工时，还必须继续拆到原子任务级别：每个任务只碰少量文件，并绑定一个最小 `cargo` 验证命令。
- 仅把原子任务拆细还不够；若某个任务第一次引入新的 workspace member，就必须把根 `Cargo.toml` 也列入该任务文件范围。
- E 阶段如果只写成 `cargo check --workspace`，并不能证明宿主层 wiring 真正打通；至少需要一条验证 commands 注册、shared state 注入和 facade 调用路径的 host smoke test。
- 第二轮 review 进一步确认：若宿主没有从第一天抽出 `src-tauri/src/lib.rs` 与 `src-tauri/src/bootstrap.rs` 这类可测试入口，A3 会和 D1 的责任分界打架，E2 的 smoke test 也会被迫复制 main.rs 中的注册逻辑。
- 第二轮 review 也确认：B2 与 D2 这类写成 `cargo test -p ...` 的任务，必须在任务定义里绑定明确的测试文件和命名测试，否则验证强度仍然只是编译检查。
- 当前 docs 里并非完全没有测试内容；测试章节已经散落在 blueprint、kernel-jobs、composition-root、crate drafts 和 backend skeleton 文档里。
- 当前真正缺的是独立的仓库级测试总文档，因此已新增 `docs/TauriTestingStrategyAndQualityGateDesign.md` 来统一测试分层、测试落点和质量门槛。
- 当前 docs 里也并非完全没有日志设计；日志内容已经散落在 blueprint、composition-root、storage/database、downloads、engines、fab-inventory 等文档里。
- 当前真正缺的是独立的仓库级日志总文档，因此已新增 `docs/TauriLoggingAndObservabilityDesign.md` 来统一 correlationId、log sink、observability 边界和 diagnostics 规则。
- 当前 docs 里同样并非完全没有 startup 设计；startup 阶段、blocking 原则和 `StartupPipelineFacade` 的想法已经散落在 blueprint 与 composition-root 文档里。
- 当前真正缺的是独立的仓库级 startup pipeline 总文档，因此已新增 `docs/TauriStartupPipelineDesign.md` 来统一 Stage 0-3、blocking policy、restore policy 和 warmup boundary。
- 当前 docs 里同样并非完全没有错误模型设计；`AppErrorDto`、error envelope、错误 code 前缀和 `retryable` / `severity` 语义已经散落在 blueprint、IPC/schema、downloads、engines 等文档里。
- 当前真正缺的是独立的仓库级错误处理总文档，因此已新增 `docs/TauriErrorHandlingAndProjectionDesign.md` 来统一错误模型、错误投影和 retry ownership。
- 当前 docs 里同样并非完全没有架构原则；前后端边界、contract first、ports/adapters、composition root、job runtime 等原则已经散落在 blueprint 和多份专题文档里。
- 当前真正缺的是独立的仓库级架构原则总文档，因此已新增 `docs/TauriArchitecturePrinciplesDesign.md` 来统一原则层约束和评审 checklist。
- 当前 docs 里同样并非完全没有安全、凭据和权限规则；secure storage、token 生命周期、session restore、host 权限面和日志脱敏要求已经散落在 blueprint、storage、ports、startup、logging 与 account-auth 文档里。
- 当前真正缺的是独立的仓库级安全、凭据与权限总文档，因此已新增 `docs/TauriSecurityCredentialsAndPermissionsDesign.md` 来统一 secret ownership、permission surface 和 redaction 边界。
- 当前 docs 里同样并非完全没有开发环境前提；前端命令存在于 package.json，Tauri/Rust/Windows 前提存在于蓝图、后端骨架文档和官方 prerequisites 页面里。
- 当前真正缺的是独立的仓库级开发环境 bootstrap 总文档，因此已新增 `docs/TauriDevelopmentEnvironmentBootstrapDesign.md` 来统一 Windows 下的 Node、Rust、Build Tools、WebView2 与分阶段验证命令。
- 当前 docs 里同样并非完全没有发布、打包和更新规则；Tauri 宿主打包职责、Updates 模块边界、auto-update UI 和 MSI 前提已经散落在 blueprint、settings 和环境文档里。
- 当前真正缺的是独立的仓库级发布/打包/更新总文档，因此已新增 `docs/TauriReleasePackagingAndUpdateDesign.md` 来统一 app self-update、packaging、channel policy 和 rollback 边界。
- 当前仓库根目录不存在 `README.md`，因此缺少一个解释“现在这个仓库到底是什么状态、能运行什么、应该先读哪些文档”的统一入口。
- 当前真正缺的是根仓库 README，因此已新增 `README.md` 来统一 quick start、当前状态说明和关键文档入口。

## Issues Encountered
| Issue | Resolution |
|-------|------------|

## Resources
- 当前打开文件: docs/TauriAIDevelopmentTransactionProtocolDesign.md
- grep `^> Parent:` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md
- grep `.artifacts/docs/` in docs/*.md after header fix
