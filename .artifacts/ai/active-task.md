# Active Atomic Task

## Identity

- task id: AT-2026-05-03-024
- title: Bootstrap sqlite adapter crate
- status: committed

## Goal

- exact local outcome: Create the first `launcher-adapter-storage-sqlite` crate shell, wire it into the root workspace, and expose only the minimal SQLite adapter config plus repository constructor shells that the controlling docs require, while keeping schema, migrations, and real SQL execution out of scope.

## Scope

- in scope:
  - update the root `Cargo.toml` members list
  - add `crates/adapter-storage-sqlite/Cargo.toml`
  - add `crates/adapter-storage-sqlite/src/lib.rs`
  - update `.artifacts/ai` records for the C3 slice
- out of scope:
  - adding real SQL schema, migrations, or `rusqlite` connection handling
  - adding composition-root wiring or module-owned port traits
  - touching `adapter-provider-fab` or `src-tauri`

## Allowed Files

1. Cargo.toml
2. crates/adapter-storage-sqlite/Cargo.toml
3. crates/adapter-storage-sqlite/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

## 已读取的本地任务记录

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## 控制性文档

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md
5. docs/TauriBackendSkeletonImplementationDesign.md
6. docs/TauriBackendCrateLayoutAndUseCaseStubDesign.md
7. docs/TauriFirstCrateApiDrafts.md
8. docs/TauriRepositoryPortsAndAdapterDesign.md
9. docs/TauriStorageAndDatabaseDesign.md
10. docs/TauriCompositionRootWiringDesign.md
11. .github/copilot-instructions.md
12. .github/skills/strict-doc-driven-development/SKILL.md
13. .github/skills/planning-with-files/SKILL.md

## Hypothesis

- falsifiable local hypothesis: If `launcher-adapter-storage-sqlite` is introduced as a workspace member that exports only adapter config plus constructor shells for the documented SQLite repositories, then `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` will pass without pulling schema, migration, or runtime SQL execution details into the first adapter stub.

## Cheap Check

- narrowest check that can disconfirm the hypothesis: Run `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`.

## Validation Gate

1. `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml`
2. `git -C q:\DEV\MyEpicLauncher diff --check`

## Validation Result

- `cargo check -p launcher-adapter-storage-sqlite --manifest-path q:\DEV\MyEpicLauncher\Cargo.toml` passed with the minimal SQLite adapter config and repository constructor shell in place.
- `git diff --check` passed; the only adjacent artifact after validation is the expected workspace-member `Cargo.lock` update.

## 需要更新的文档和日志

1. Cargo.toml
2. crates/adapter-storage-sqlite/Cargo.toml
3. crates/adapter-storage-sqlite/src/lib.rs
4. .artifacts/ai/active-task.md
5. .artifacts/ai/task-plan.md
6. .artifacts/ai/progress.md
7. .artifacts/ai/findings.md

## 验证后的 Git 动作

1. commit message plan: Bootstrap sqlite adapter crate
2. push command plan: git push

## 停止条件

1. the slice starts introducing schema, migration, or raw SQL execution behavior into `launcher-adapter-storage-sqlite`
2. `cargo check -p launcher-adapter-storage-sqlite` fails for reasons outside the C3 file set
3. same blocker still failing after 5 repair attempts

## 安全恢复点

- exact next step if execution is interrupted: commit the C3 code slice, then open a tiny lockfile cleanup slice if `Cargo.lock` is still dirty.