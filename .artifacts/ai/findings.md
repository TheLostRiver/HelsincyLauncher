# AI Findings

- The stale startup context was caused by .github/hooks/scripts/session-start.ps1 and session-start.sh reading root task_plan.md.
- The stricter repo instructions and workflow templates still referenced root task_plan.md, progress.md, and findings.md, which conflicted with docs/TauriAIDevelopmentTransactionProtocolDesign.md.
- The protocol document defines .artifacts/ai/active-task.md, task-plan.md, progress.md, findings.md, and handoff.md as the intended local workflow records.
- New sessions should bootstrap .artifacts/ai records instead of falling back to the legacy root planning workflow.
- The repo-specific workflow is now aligned to .artifacts/ai across session-start, pre-tool-use, post-tool-use, error handling, agent-stop, instructions, and strict-doc templates.
- .sh hooks need LF preservation at the git layer; without .gitattributes, future git operations can reintroduce CRLF and break bash parsing.