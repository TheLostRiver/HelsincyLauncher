#!/bin/bash
# strict-doc-driven-development: Session start hook for GitHub Copilot
# Always injects the repository strict doc-driven reminder first.
# Then reads the .artifacts/ai/ task records when they exist.
# When no .artifacts/ai/ task record exists yet, injects a workflow bootstrap reminder.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

STRICT_SKILL_DIR=".github/skills/strict-doc-driven-development"
ARTIFACTS_DIR=".artifacts/ai"
ACTIVE_TASK_FILE="$ARTIFACTS_DIR/active-task.md"
TASK_PLAN_FILE="$ARTIFACTS_DIR/task-plan.md"
PROGRESS_FILE="$ARTIFACTS_DIR/progress.md"
HANDOFF_FILE="$ARTIFACTS_DIR/handoff.md"
LANGUAGE_MODE_FILE="$ARTIFACTS_DIR/language-mode.txt"
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
    STRICT_REMINDER_FILE="$STRICT_SKILL_DIR/session-start.en.txt"
    BOOTSTRAP_REMINDER_FILE="$STRICT_SKILL_DIR/session-bootstrap.en.txt"
    MESSAGE_CATALOG_FILE="$STRICT_SKILL_DIR/hook-messages.en.txt"
else
    LANGUAGE_MODE="zh-CN"
    STRICT_REMINDER_FILE="$STRICT_SKILL_DIR/session-start.zh-CN.txt"
    BOOTSTRAP_REMINDER_FILE="$STRICT_SKILL_DIR/session-bootstrap.zh-CN.txt"
    MESSAGE_CATALOG_FILE="$STRICT_SKILL_DIR/hook-messages.zh-CN.txt"
fi

ACTIVE_TASK_LABEL=$(lookup_message "ACTIVE_TASK_LABEL" "[myepiclauncher] ACTIVE TASK")
HANDOFF_LABEL=$(lookup_message "HANDOFF_LABEL" "[myepiclauncher] HANDOFF")
TASK_PLAN_LABEL=$(lookup_message "TASK_PLAN_LABEL" "[myepiclauncher] TASK PLAN")
PROGRESS_LABEL=$(lookup_message "RECENT_PROGRESS_LABEL" "[myepiclauncher] RECENT PROGRESS")

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
    append_context "$ACTIVE_TASK_LABEL" "$(cat "$ACTIVE_TASK_FILE" 2>/dev/null)"
fi

if [ -f "$HANDOFF_FILE" ]; then
    append_context "$HANDOFF_LABEL" "$(cat "$HANDOFF_FILE" 2>/dev/null)"
fi

if [ -z "$CONTEXT_PARTS" ] && [ -f "$TASK_PLAN_FILE" ]; then
    append_context "$TASK_PLAN_LABEL" "$(head -40 "$TASK_PLAN_FILE" 2>/dev/null)"
fi

if [ -f "$PROGRESS_FILE" ]; then
    append_context "$PROGRESS_LABEL" "$(tail -20 "$PROGRESS_FILE" 2>/dev/null)"
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
