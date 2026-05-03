# AI Task Plan

## Goal

Define how professional-grade context management should integrate with the repository's `.artifacts/ai` transaction protocol without reintroducing competing planning records.

## Active Atomic Task

1. AT-2026-05-03-001 - committed - switched hooks, repo instructions, and workflow templates to .artifacts/ai and bootstrapped the new task records.
2. AT-2026-05-03-002 - committed - added zh-CN and en language modes for the repo workflow skill, translated the strict-doc skill assets to Chinese-first wording, and localized the repo-specific hook messages.
3. AT-2026-05-03-003 - committed - defined the mapping that lets planning-with-files act as the context-management layer while `.artifacts/ai` remains the only authoritative task record set.

## Follow-up Queue

1. Implement a repo-local adapter surface so planning-with-files writes planning, progress, findings, and catchup state into `.artifacts/ai`.
2. Decide whether to retire or archive the legacy root planning files once they are no longer needed for historical reference.

## Legacy Note

1. Root task_plan.md, progress.md, and findings.md are legacy records and are not the source of truth for new tasks.