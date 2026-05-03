#!/bin/bash
# strict-doc-driven-development: Agent stop hook for GitHub Copilot
# Checks the current .artifacts/ai active task state before the agent stops.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
HANDOFF_FILE=".artifacts/ai/handoff.md"

if [ ! -f "$ACTIVE_TASK_FILE" ]; then
    echo '{}'
    exit 0
fi

STATUS=$(sed -n 's/^-[[:space:]]*status:[[:space:]]*//p' "$ACTIVE_TASK_FILE" | head -1 | tr '[:upper:]' '[:lower:]')

case "$STATUS" in
    committed)
        MSG="[myepiclauncher] Active task is committed. Safe to stop or start the next .artifacts/ai/active-task.md entry."
        ;;
    blocked)
        if [ -f "$HANDOFF_FILE" ]; then
            MSG="[myepiclauncher] Active task is blocked and .artifacts/ai/handoff.md exists. Safe to stop until the next resume."
        else
            MSG="[myepiclauncher] Active task is blocked. Write .artifacts/ai/handoff.md before stopping so the next session has a safe resume point."
        fi
        ;;
    in_progress)
        MSG="[myepiclauncher] Active task is still in progress. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if the task is done."
        ;;
    validating)
        MSG="[myepiclauncher] Active task is still validating. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if validation is complete."
        ;;
    *)
        MSG="[myepiclauncher] Active task status is unclear. Review .artifacts/ai/active-task.md before stopping."
        ;;
esac

echo "{\"hookSpecificOutput\":{\"hookEventName\":\"AgentStop\",\"additionalContext\":\"$MSG\"}}"
exit 0
