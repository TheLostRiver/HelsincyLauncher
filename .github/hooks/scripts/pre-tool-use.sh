#!/bin/bash
# strict-doc-driven-development: Pre-tool-use hook for GitHub Copilot
# Reads the active atomic task first, then falls back to the .artifacts/ai task plan.
# Always allows tool execution — this hook never blocks tools.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

STRICT_SKILL_DIR=".github/skills/strict-doc-driven-development"
ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
TASK_PLAN_FILE=".artifacts/ai/task-plan.md"
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
    LANGUAGE_MODE="zh-CN"
    MESSAGE_CATALOG_FILE="$STRICT_SKILL_DIR/hook-messages.zh-CN.txt"
fi

ACTIVE_TASK_LABEL=$(lookup_message "ACTIVE_TASK_LABEL" "[myepiclauncher] ACTIVE TASK")
TASK_PLAN_LABEL=$(lookup_message "TASK_PLAN_LABEL" "[myepiclauncher] TASK PLAN")

if [ ! -f "$ACTIVE_TASK_FILE" ] && [ ! -f "$TASK_PLAN_FILE" ]; then
    echo '{}'
    exit 0
fi

CONTEXT=""

if [ -f "$ACTIVE_TASK_FILE" ]; then
    ACTIVE_TASK=$(head -80 "$ACTIVE_TASK_FILE" 2>/dev/null || echo "")
    if [ -n "$ACTIVE_TASK" ]; then
        STATUS=$(printf "%s\n" "$ACTIVE_TASK" | sed -n 's/^-[[:space:]]*status:[[:space:]]*//p' | head -1 | tr '[:upper:]' '[:lower:]' | tr -d '\r')
        case "$STATUS" in
            committed|aborted)
                ;;
            *)
                CONTEXT="$ACTIVE_TASK_LABEL
$ACTIVE_TASK"
                ;;
        esac
    fi
fi

if [ -z "$CONTEXT" ] && [ -f "$TASK_PLAN_FILE" ]; then
    TASK_PLAN=$(head -40 "$TASK_PLAN_FILE" 2>/dev/null || echo "")
    if [ -n "$TASK_PLAN" ]; then
        CONTEXT="$TASK_PLAN_LABEL
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
