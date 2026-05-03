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

## Skill Usage

For non-trivial implementation, planning, or review work, use the workspace skill .github/skills/strict-doc-driven-development.
That skill contains the stricter task protocol and the subtask briefing template.