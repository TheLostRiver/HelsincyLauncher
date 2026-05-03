---
name: strict-doc-driven-development
description: "Use when implementing, planning, reviewing, or decomposing MyEpicLauncher tasks that must strictly follow repository development specification docs. Trigger phrases: 严格按照开发规范文档, 开发规范, 架构文档, 设计文档, blueprint, 模块文档, 后端骨架."
user-invocable: true
allowed-tools: "Read Write Edit Bash Glob Grep"
---

# Strict Doc-Driven Development

This repository does not allow default generic implementation behavior.
Treat the repository docs as binding engineering requirements.

## Core Rules

1. Before editing, identify the controlling docs for the requested change.
2. If the task touches a business module, read the module docs before changing code.
3. If the docs are missing, stale, or conflicting, do not improvise architecture. Surface the gap first or update the docs intentionally.
4. When splitting or delegating subtasks, include exact doc paths, scope, allowed files, and validation gates.
5. Use the narrowest validation that the controlling docs require. Do not replace it with a looser check.

## Required Baseline Docs

Read these first for most implementation tasks:

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md

## Task-specific Doc Map

Add the relevant docs based on the task shape:

- Security, credentials, permissions: docs/TauriSecurityCredentialsAndPermissionsDesign.md
- Startup, restore, warmup: docs/TauriStartupPipelineDesign.md
- Error model and user-facing projection: docs/TauriErrorHandlingAndProjectionDesign.md
- Release, packaging, updater: docs/TauriReleasePackagingAndUpdateDesign.md
- Backend skeleton, crate layout, transport wiring: docs/TauriBackendSkeletonImplementationDesign.md
- Environment bootstrap and prerequisites: docs/TauriDevelopmentEnvironmentBootstrapDesign.md
- Module work: docs/modules/<module-name>/README_ARCH.md, README_API.md, README_FLOW.md

## Implementation Protocol

1. Name the docs that control the change before making decisions.
2. Reduce the task to one falsifiable local hypothesis and one cheap check.
3. Make the smallest edit that satisfies the documented boundary.
4. Validate with the doc-defined gate for the touched slice.
5. If code and docs disagree, do not silently pick one. Call out the conflict and resolve it explicitly.

## Prohibited Behavior

- Do not move business truth into the frontend because the prototype happens to be UI-first today.
- Do not invent cross-module coupling that bypasses facade, port, contract, or projection boundaries.
- Do not widen permissions, secret storage, or startup blocking rules without an explicit doc basis.
- Do not skip required validation just because a broader build appears green.
- Do not treat repository docs as optional suggestions.