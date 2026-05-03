# MyEpicLauncher Workspace Instructions

This repository uses strict doc-driven development.
Do not fall back to generic default implementation patterns when the repository docs define stricter rules.

## Baseline Rule

Before making decisions for non-trivial work, identify and read the controlling docs.

Minimum baseline for most implementation tasks:

1. docs/TauriRewriteArchitectureBlueprint.md
2. docs/TauriArchitecturePrinciplesDesign.md
3. docs/TauriAIDevelopmentTransactionProtocolDesign.md
4. docs/TauriTestingStrategyAndQualityGateDesign.md

Add task-specific docs when relevant:

- security and credentials: docs/TauriSecurityCredentialsAndPermissionsDesign.md
- startup and restore: docs/TauriStartupPipelineDesign.md
- error model and projection: docs/TauriErrorHandlingAndProjectionDesign.md
- backend skeleton and transport wiring: docs/TauriBackendSkeletonImplementationDesign.md
- environment bootstrap: docs/TauriDevelopmentEnvironmentBootstrapDesign.md
- release, packaging, updater: docs/TauriReleasePackagingAndUpdateDesign.md
- module work: docs/modules/<module-name>/README_ARCH.md, README_API.md, README_FLOW.md

## Required Behavior

1. Treat repository docs as binding requirements, not optional reference material.
2. If docs are missing, stale, or conflicting, do not improvise architecture. Surface the gap first or update the docs intentionally.
3. When decomposing or delegating subtasks, include exact doc paths, scope limits, allowed files, constraints, and validation gates.
4. Use the narrowest validation required by the controlling docs.
5. Do not move backend-owned truth into the frontend because the current repo still contains a frontend prototype.

## Atomic Task Execution

1. For non-trivial work, read the local task records first: task_plan.md, progress.md, and findings.md.
2. Do not start writing code until you have read the relevant task record plus the controlling architecture or module docs.
3. Work one atomic task at a time. Do not blur multiple unfinished atomic tasks together.
4. After each completed atomic task, run the narrowest compile, test, or validation gate that the docs require for that slice.
5. If that gate passes, update the relevant docs and task records, create a git commit, and attempt to push.
6. If push fails, persist the pending push state and the failure details into the local task records so the commits can be pushed later.
7. If the same blocker cannot be repaired after 5 attempts, persist the bug details and current state, then stop and wait for user direction.
8. When needed, use the workspace skill templates .github/skills/strict-doc-driven-development/pending-push-template.md and .github/skills/strict-doc-driven-development/blocked-bug-template.md to keep those records structured.

## Skill Usage

For non-trivial implementation, planning, or review work, use the workspace skill .github/skills/strict-doc-driven-development.
That skill contains the stricter task protocol and the subtask briefing template.