#!/bin/bash
# strict-doc-driven-development: Agent stop hook for GitHub Copilot
# Checks the current .artifacts/ai active task state before the agent stops.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
HANDOFF_FILE=".artifacts/ai/handoff.md"
STRICT_SKILL_DIR=".github/skills/strict-doc-driven-development"
LANGUAGE_MODE_FILE=".artifacts/ai/language-mode.txt"
MESSAGE_CATALOG_FILE=""

lookup_message() {
    local key="$1"
    local fallback="$2"
    local line=""

    if [ -n "$MESSAGE_CATALOG_FILE" ] && [ -f "$MESSAGE_CATALOG_FILE" ]; then
        line=$(grep -m1 "^${key}=" "$MESSAGE_CATALOG_FILE" 2>/dev/null || true)
    fi

    if [ -n "$line" ]; then
        printf '%s' "${line#*=}"
    else
        printf '%s' "$fallback"
    fi
}

if [ -n "$MYEPIC_WORKFLOW_LANG" ]; then
    LANGUAGE_MODE="$MYEPIC_WORKFLOW_LANG"
elif [ -f "$LANGUAGE_MODE_FILE" ]; then
    LANGUAGE_MODE=$(head -1 "$LANGUAGE_MODE_FILE" 2>/dev/null | tr -d '\r')
else
    LANGUAGE_MODE="zh-CN"
fi

if [ "$LANGUAGE_MODE" = "en" ]; then
    MESSAGE_CATALOG_FILE="$STRICT_SKILL_DIR/hook-messages.en.txt"
else
    MESSAGE_CATALOG_FILE="$STRICT_SKILL_DIR/hook-messages.zh-CN.txt"
fi

if [ ! -f "$ACTIVE_TASK_FILE" ]; then
    echo '{}'
    exit 0
fi

STATUS=$(sed -n 's/^-[[:space:]]*status:[[:space:]]*//p' "$ACTIVE_TASK_FILE" | head -1 | tr '[:upper:]' '[:lower:]')

case "$STATUS" in
    committed)
        MSG=$(lookup_message "AGENT_STOP_COMMITTED" "[myepiclauncher] Active task is committed. Safe to stop or start the next .artifacts/ai/active-task.md entry.")
        ;;
    blocked)
        if [ -f "$HANDOFF_FILE" ]; then
            MSG=$(lookup_message "AGENT_STOP_BLOCKED_WITH_HANDOFF" "[myepiclauncher] Active task is blocked and .artifacts/ai/handoff.md exists. Safe to stop until the next resume.")
        else
            MSG=$(lookup_message "AGENT_STOP_BLOCKED_NO_HANDOFF" "[myepiclauncher] Active task is blocked. Write .artifacts/ai/handoff.md before stopping so the next session has a safe resume point.")
        fi
        ;;
    in_progress)
        MSG=$(lookup_message "AGENT_STOP_IN_PROGRESS" "[myepiclauncher] Active task is still in progress. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if the task is done.")
        ;;
    validating)
        MSG=$(lookup_message "AGENT_STOP_VALIDATING" "[myepiclauncher] Active task is still validating. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if validation is complete.")
        ;;
    *)
        MSG=$(lookup_message "AGENT_STOP_UNKNOWN" "[myepiclauncher] Active task status is unclear. Review .artifacts/ai/active-task.md before stopping.")
        ;;
esac

echo "{\"hookSpecificOutput\":{\"hookEventName\":\"AgentStop\",\"additionalContext\":\"$MSG\"}}"
exit 0
