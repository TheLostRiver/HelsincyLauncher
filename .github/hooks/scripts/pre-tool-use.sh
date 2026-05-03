#!/bin/bash
# strict-doc-driven-development: Pre-tool-use hook for GitHub Copilot
# Reads the active atomic task first, then falls back to the .artifacts/ai task plan.
# Always allows tool execution — this hook never blocks tools.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
TASK_PLAN_FILE=".artifacts/ai/task-plan.md"

if [ ! -f "$ACTIVE_TASK_FILE" ] && [ ! -f "$TASK_PLAN_FILE" ]; then
    echo '{}'
    exit 0
fi

CONTEXT=""

if [ -f "$ACTIVE_TASK_FILE" ]; then
    ACTIVE_TASK=$(head -80 "$ACTIVE_TASK_FILE" 2>/dev/null || echo "")
    if [ -n "$ACTIVE_TASK" ]; then
        CONTEXT="[myepiclauncher] ACTIVE TASK
$ACTIVE_TASK"
    fi
fi

if [ -z "$CONTEXT" ] && [ -f "$TASK_PLAN_FILE" ]; then
    TASK_PLAN=$(head -40 "$TASK_PLAN_FILE" 2>/dev/null || echo "")
    if [ -n "$TASK_PLAN" ]; then
        CONTEXT="[myepiclauncher] TASK PLAN
$TASK_PLAN"
    fi
fi

if [ -z "$CONTEXT" ]; then
    echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}'
    exit 0
fi

# Escape context for JSON
PYTHON=""
for _p in /usr/bin/python3 /usr/local/bin/python3 /opt/homebrew/bin/python3; do
    [ -x "$_p" ] && { PYTHON="$_p"; break; }
done
[ -z "$PYTHON" ] && PYTHON=$(command -v python3 2>/dev/null || command -v python 2>/dev/null)
ESCAPED=$(echo "$CONTEXT" | $PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read(), ensure_ascii=False))" 2>/dev/null || echo "\"\"")

echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"additionalContext\":$ESCAPED}}"
exit 0
