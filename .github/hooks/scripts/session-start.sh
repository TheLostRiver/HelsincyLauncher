#!/bin/bash
# strict-doc-driven-development: Session start hook for GitHub Copilot
# Always injects the repository strict doc-driven reminder first.
# Then reads the .artifacts/ai/ task records when they exist.
# When no .artifacts/ai/ task record exists yet, injects a workflow bootstrap reminder.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

STRICT_SKILL_DIR=".github/skills/strict-doc-driven-development"
STRICT_REMINDER_FILE="$STRICT_SKILL_DIR/session-start.txt"
BOOTSTRAP_REMINDER_FILE="$STRICT_SKILL_DIR/session-bootstrap.txt"
ARTIFACTS_DIR=".artifacts/ai"
ACTIVE_TASK_FILE="$ARTIFACTS_DIR/active-task.md"
TASK_PLAN_FILE="$ARTIFACTS_DIR/task-plan.md"
PROGRESS_FILE="$ARTIFACTS_DIR/progress.md"
HANDOFF_FILE="$ARTIFACTS_DIR/handoff.md"
STRICT_CONTEXT=$(cat "$STRICT_REMINDER_FILE" 2>/dev/null || echo "")
BOOTSTRAP_CONTEXT=$(cat "$BOOTSTRAP_REMINDER_FILE" 2>/dev/null || echo "")

CONTEXT_PARTS=""

append_context() {
    if [ -z "$2" ]; then
        return
    fi

    if [ -n "$CONTEXT_PARTS" ]; then
        CONTEXT_PARTS="$CONTEXT_PARTS

$1
$2"
    else
        CONTEXT_PARTS="$1
$2"
    fi
}

if [ -f "$ACTIVE_TASK_FILE" ]; then
    append_context "[myepiclauncher] ACTIVE TASK" "$(cat "$ACTIVE_TASK_FILE" 2>/dev/null)"
fi

if [ -f "$HANDOFF_FILE" ]; then
    append_context "[myepiclauncher] HANDOFF" "$(cat "$HANDOFF_FILE" 2>/dev/null)"
fi

if [ -z "$CONTEXT_PARTS" ] && [ -f "$TASK_PLAN_FILE" ]; then
    append_context "[myepiclauncher] TASK PLAN" "$(head -40 "$TASK_PLAN_FILE" 2>/dev/null)"
fi

if [ -f "$PROGRESS_FILE" ]; then
    append_context "[myepiclauncher] RECENT PROGRESS" "$(tail -20 "$PROGRESS_FILE" 2>/dev/null)"
fi

if [ -n "$CONTEXT_PARTS" ]; then
    CONTEXT="$CONTEXT_PARTS"
elif [ -n "$BOOTSTRAP_CONTEXT" ]; then
    CONTEXT="$BOOTSTRAP_CONTEXT"
fi

if [ -n "$STRICT_CONTEXT" ] && [ -n "$CONTEXT" ]; then
    CONTEXT="$STRICT_CONTEXT

$CONTEXT"
elif [ -n "$STRICT_CONTEXT" ]; then
    CONTEXT="$STRICT_CONTEXT"
fi

if [ -z "$CONTEXT" ]; then
    echo '{}'
    exit 0
fi

# Escape context for JSON
ESCAPED=$(echo "$CONTEXT" | $PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read(), ensure_ascii=False))" 2>/dev/null || echo "\"\"")

echo "{\"hookSpecificOutput\":{\"hookEventName\":\"SessionStart\",\"additionalContext\":$ESCAPED}}"
exit 0
