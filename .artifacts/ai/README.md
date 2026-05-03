# AI Workflow Records

This directory is the live workflow surface for this repository.

## Active Source Of Truth

- `active-task.md`: the current atomic slice boundary, scope, validation gate, and stop conditions
- `task-plan.md`: the multi-slice plan, phase status, and AT ledger
- `progress.md`: append-only execution log and recent validation history
- `findings.md`: durable discoveries, decisions, and recurring constraints
- `handoff.md`: suspend/resume checkpoint when work pauses or is blocked

## Archive Boundary

- `legacy-root-planning/` is not active workflow state.
- It only preserves the older root-level `task_plan.md`, `progress.md`, and `findings.md` history that existed before the repo moved to `.artifacts/ai/`.
- Do not resume work from that archive and do not update those files for new tasks.

## Working Rule

- For new work, read and update the active files in this directory.
- If you need old historical context, consult `legacy-root-planning/` as read-only background only.