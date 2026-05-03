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

## Atomic Task Loop

1. Read the local task records first: task_plan.md, progress.md, and findings.md.
2. Do not begin coding until you have read both the task record and the relevant architecture or module docs.
3. Keep exactly one active atomic task in progress until it reaches validation, commit, or a blocked state.
4. After each successful atomic task, run the narrowest compile, test, or validation gate required by the docs.
5. If that gate passes, update the relevant docs and task records, create a git commit, and attempt to push immediately.
6. If push fails, do not panic and do not discard the commit. Persist the pending push details in the task records so the outstanding commits can be pushed later.
7. If a blocker still fails after 5 repair attempts, persist the bug details, what was tried, and the current state, then stop and wait for user direction.

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

## Subtask Briefing Contract

When you split work into subtasks, do not hand off a vague goal.
Every subtask must carry repository-specific constraints.

Required subtask fields:

1. goal
2. scope and non-goals
3. allowed files
4. controlling docs
5. task-specific constraints
6. validation gates
7. done-when criteria

Use the bundled template in subtask-template.md.
If a subtask cannot name controlling docs and a cheap validation gate, it is not ready to execute.

## Delegation Checklist

Before delegating, verify all of the following:

1. The subtask names the exact docs that govern the change.
2. The subtask limits file scope so the executor does not roam.
3. The subtask names the narrowest validation to run immediately after editing.
4. The subtask states which architectural moves are forbidden.
5. The subtask does not assume frontend-only shortcuts when the docs assign ownership to backend, host, or composition root.
6. The subtask names which task records and docs must be updated before commit.
7. The subtask states whether a successful validation must be followed by git commit and push.

## Workflow Templates

Use the bundled templates to keep the workflow structured:

1. active-atomic-task-template.md: define the current atomic task before coding, including scope, controlling docs, cheap check, and stop conditions.
2. docs-update-log-template.md: record which docs were updated, why they changed, and which validation or code slice they belong to.
3. pending-push-template.md: record commits that are ready but not yet pushed, including the reason push failed and the exact follow-up command.
4. blocked-bug-template.md: record a blocker that still fails after 5 repair attempts, including attempted fixes, latest error, affected files, and the exact point where work stopped.

Do not leave active-task state, docs updates, push failures, or 5-attempt blockers only in transient chat context.
Persist them in the task records using these templates.

## Prohibited Behavior

- Do not move business truth into the frontend because the prototype happens to be UI-first today.
- Do not invent cross-module coupling that bypasses facade, port, contract, or projection boundaries.
- Do not widen permissions, secret storage, or startup blocking rules without an explicit doc basis.
- Do not skip required validation just because a broader build appears green.
- Do not treat repository docs as optional suggestions.