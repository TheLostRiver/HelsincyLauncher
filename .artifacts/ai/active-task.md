# Active Atomic Task

## Identity

- task id: AT-2026-05-19-255
- title: Define downloads composition-root segment executor wiring boundary
- status: completed

## Goal

Document README_IMPL 7.42 and root README updates for the next composition-root segment executor wiring slice before any Rust wiring code.

## Scope

- in scope:
  - `README.md`
  - `docs/modules/downloads/README_IMPL.md`
  - `.artifacts/ai/active-task.md`
  - `.artifacts/ai/task-plan.md`
  - `.artifacts/ai/progress.md`
  - `.artifacts/ai/findings.md`
  - `.artifacts/ai/handoff.md`
- out of scope:
  - Rust code changes
  - production execution-port wiring
  - real HTTP range requests
  - provider authentication, CDN policy, or retry/backoff
  - public `DL_NETWORK_*` / `DL_PROVIDER_*` / `DL_WRITE_FAILED` / `DL_VERIFY_FAILED` projection
  - streaming fetch, backpressure, or Tokio worker pools
  - host transport, frontend, or SQLite schema changes
  - unrelated dirty files

## Allowed Files

1. README.md
2. docs/modules/downloads/README_IMPL.md
3. .artifacts/ai/active-task.md
4. .artifacts/ai/task-plan.md
5. .artifacts/ai/progress.md
6. .artifacts/ai/findings.md
7. .artifacts/ai/handoff.md

## Required Context Read

Read before writing:

1. `README.md` current status, module docs, and near-term roadmap sections.
2. `docs/README.md` module implementation-doc routing.
3. `docs/modules/downloads/README_ARCH.md`, `README_API.md`, `README_FLOW.md`, and `README_IMPL.md` 6.1 / 7.41.
4. `docs/TauriCompositionRootWiringDesign.md` composition-root responsibility and one-shot execution helper sections.
5. `docs/TauriDownloadRuntimeDesign.md`, repository ports, error, testing, and comment docs for staging/fetch/write/verify rules.
6. `crates/composition-root/src/bootstrap.rs` current downloads driver wiring.
7. `crates/module-downloads/src/driver.rs` executor and driver constructor surfaces.

## Hypothesis

- falsifiable documentation hypothesis: the next safe composition-root slice should use an explicit private static-source wiring helper for deterministic tests, derive staging from `app_data_dir/.downloads/staging`, and keep default desktop production downloads dispatch deferred until a real provider fetcher or explicit non-empty local-source config exists.

## Cheap Check

1. Update README_IMPL 6.1 and add 7.42 with the composition-root segment executor wiring boundary.
2. Update root README so the top-level project status points to downloads concrete segment execution progress and README_IMPL.
3. Update PWF status, including the corrected AT-254 commit/push state.
4. Run scoped diff/readback checks and commit/push the docs-only boundary.

## Validation Gate

1. README_IMPL records the 7.42 boundary and next Rust slice.
2. Root README reflects current downloads backend progress and links the implementation doc.
3. PWF records AT-254 as pushed and AT-255 as the active boundary task.
4. Scoped docs/PWF diff-check passes before commit/push.

## Completion Evidence

- README_IMPL 6.1 now marks the composition-root executor wiring boundary as defined and names the next code proof.
- README_IMPL 7.42 defines the private explicit static-source wiring boundary, app-data staging root, default deferred production rule, non-goals, and next Rust slice.
- Root README now records downloads concrete segment execution progress and links `docs/modules/downloads/README_IMPL.md`.
- PWF records corrected AT-254 as committed/pushed at `43a44e1`.
- Scoped `git diff --check` passed for the task files with CRLF warnings only.
