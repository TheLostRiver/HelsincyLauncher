# AI Task Plan

## Goal

Define how professional-grade context management should integrate with the repository's `.artifacts/ai` transaction protocol without reintroducing competing planning records.

## Active Atomic Task

1. AT-2026-05-03-001 - committed - switched hooks, repo instructions, and workflow templates to .artifacts/ai and bootstrapped the new task records.
2. AT-2026-05-03-002 - committed - added zh-CN and en language modes for the repo workflow skill, translated the strict-doc skill assets to Chinese-first wording, and localized the repo-specific hook messages.
3. AT-2026-05-03-003 - committed - defined the mapping that lets planning-with-files act as the context-management layer while `.artifacts/ai` remains the only authoritative task record set.
4. AT-2026-05-03-004 - committed - retargeted the repo-local planning-with-files copy so its hooks, init flow, stop-time checks, and catchup flow operate on `.artifacts/ai` instead of root planning files.

## Follow-up Queue

1. Decide whether to retire or archive the legacy root planning files once they are no longer needed for historical reference.
2. Integrate planning-with-files' 2-action checkpoint cadence more explicitly into repo-level reminders if the adapter slice alone is not enough.

## Legacy Note

1. Root task_plan.md, progress.md, and findings.md are legacy records and are not the source of truth for new tasks.