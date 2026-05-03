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

### Phase 10: Settings Rollout
- [x] Instantiate settings module docs
- [x] Commit settings module guides separately
- [x] Keep task_plan.md, findings.md, and progress.md together for the follow-up planning commit
- **Status:** complete

### Phase 11: Account Auth Rollout
- [x] Instantiate account-auth module docs
- [x] Commit account-auth module guides separately
- [x] Keep task_plan.md, findings.md, and progress.md together for the follow-up planning commit
- **Status:** complete

### Phase 12: Backend Skeleton Documentation
- [x] Confirm whether backend skeleton code already exists in the repo
- [x] Confirm whether a current-repo backend skeleton implementation doc already exists
- [x] Add a backend skeleton implementation design for the current repo layout
- **Status:** complete

### Phase 13: Backend Skeleton Atomic Breakdown
- [x] Decompose the backend skeleton implementation doc into atomic tasks
- [x] Bind each atomic task to a minimal validation step
- **Status:** complete

### Phase 14: Backend Skeleton Review Fixes
- [x] Tighten the workspace-member onboarding rule inside the atomic task plan
- [x] Add a host-level smoke-test gate for E-phase transport wiring proof
- **Status:** complete

### Phase 15: Backend Skeleton Review Round Two
- [x] Add a testable host bootstrap surface to the target layout and early host phase
- [x] Make A3 self-consistent before composition-root wiring exists
- [x] Require explicit named smoke tests for B2 and D2
- **Status:** complete

### Phase 16: Testing Documentation Baseline
- [x] Verify whether standalone testing documentation already exists
- [x] Add a repository-level testing strategy and quality-gate document
- [x] Wire the new testing document into the architecture blueprint companion list
- **Status:** complete

### Phase 17: Logging Documentation Baseline
- [x] Verify whether standalone logging / observability documentation already exists
- [x] Add a repository-level logging and observability document
- [x] Wire the new logging document into the architecture blueprint companion list
- **Status:** complete

### Phase 18: Startup Documentation Baseline
- [x] Verify whether standalone startup pipeline documentation already exists
- [x] Add a repository-level startup pipeline document
- [x] Wire the new startup document into the architecture blueprint companion list
- **Status:** complete

### Phase 19: Error Documentation Baseline
- [x] Verify whether standalone error handling documentation already exists
- [x] Add a repository-level error handling and projection document
- [x] Wire the new error document into the architecture blueprint companion list
- **Status:** complete

### Phase 20: Architecture Principles Documentation Baseline
- [x] Verify whether standalone architecture-principles documentation already exists
- [x] Add a repository-level architecture principles document
- [x] Wire the new architecture-principles document into the architecture blueprint companion list
- **Status:** complete

### Phase 21: Security Documentation Baseline
- [x] Verify whether standalone security, credentials, and permissions documentation already exists
- [x] Add a repository-level security, credentials, and permissions document
- [x] Wire the new security document into the architecture blueprint companion list
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
| 选择继续时补 settings | 用户在上一步明确选择了继续补 settings 模块文档 |
| 选择继续时补 account-auth | 用户随后明确选择继续补 account-auth 模块文档 |
| 当前仓库的第一版后端骨架先采用 root frontend + src-tauri + crates 方案 | 现有工作区仍是根目录 Next.js 原型，先落可编译骨架比先做目录大迁移更稳 |
| 后端骨架文档需要继续拆到原子任务级别 | 真正开始 scaffolding 前，每一步都应限定文件范围和最小验证，避免大块不可回退施工 |
| 后端骨架原子任务中，首次引入某个包路径的任务必须同步更新根 Cargo.toml | 否则后续包级 cargo check 无法成立，workspace 规则与任务表会互相矛盾 |
| E 阶段的链路打通必须由宿主级 smoke test 证明 | 单纯 cargo check 只能证明可编译，不能证明 commands 注册和 shared state 注入真的生效 |
| 宿主 crate 从第一天就应抽出可测试的 bootstrap / app-builder surface | 否则 A3 和 E2 都会被迫把逻辑塞进 main.rs，测试只能复制启动路径而不是验证 wiring |
| 写成 cargo test 门槛的原子任务必须显式绑定命名测试文件 | 否则 B2、D2 这类任务会退化成“零测试也能通过”的伪验证 |
| 当前仓库需要一份独立的仓库级测试总文档 | 测试内容虽然散落在多份 Tauri 设计文档里，但缺少统一入口会让后续实现时丢失测试边界和质量门槛 |
| 当前仓库也需要一份独立的仓库级日志总文档 | 日志、correlationId、diagnostics 和 log sink 规则虽然散落存在，但缺少统一入口会让实现阶段再次各写各的 |
| 当前仓库还需要一份独立的 startup pipeline 总文档 | 启动阶段、阻塞规则、restore 和 warmup 边界虽然散落存在，但缺少统一入口会让实现阶段重新把启动职责打散 |
| 当前仓库还需要一份独立的错误处理总文档 | `AppError`、error envelope、severity、retryable 和用户态错误投影虽然散落存在，但缺少统一入口会让实现阶段重新各自发明错误语义 |
| 当前仓库还需要一份独立的架构原则总文档 | 前后端边界、contract first、composition root、job runtime 等原则如果只留在大蓝图里，后续实现和文档扩展都会缺少统一引用锚点 |
| 当前仓库还需要一份独立的安全、凭据与权限总文档 | secure storage、token 生命周期、host permission surface 和日志脱敏规则虽然散落存在，但缺少统一入口会让实现阶段再次把 secret 边界打穿 |

## Errors Encountered
| Error | Resolution |
|-------|------------|
