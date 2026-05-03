#!/bin/bash
# strict-doc-driven-development: Error hook for GitHub Copilot
# Reminds the agent to record errors in the .artifacts/ai workflow files.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
PROGRESS_FILE=".artifacts/ai/progress.md"
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

if [ ! -f "$ACTIVE_TASK_FILE" ] && [ ! -f "$PROGRESS_FILE" ]; then
    echo '{}'
    exit 0
fi

# Extract error message from input JSON
PYTHON=""
for _p in /usr/bin/python3 /usr/local/bin/python3 /opt/homebrew/bin/python3; do
    [ -x "$_p" ] && { PYTHON="$_p"; break; }
done
[ -z "$PYTHON" ] && PYTHON=$(command -v python3 2>/dev/null || command -v python 2>/dev/null)
ERROR_MSG=$($PYTHON -c "
import sys, json
try:
    data = json.load(sys.stdin)
    msg = data.get('error', {}).get('message', '') if isinstance(data.get('error'), dict) else str(data.get('error', ''))
    print(msg[:200])
except:
    print('')
" <<< "$INPUT" 2>/dev/null || echo "")

if [ -n "$ERROR_MSG" ]; then
    TEMPLATE=$(lookup_message "ERROR_OCCURRED" "[myepiclauncher] Error detected: {{ERROR}}. Log this in .artifacts/ai/progress.md. If this is the 5th failed repair attempt for the same blocker, persist the blocker in .artifacts/ai/handoff.md using blocked-bug-template.md and stop.")
    CONTEXT=${TEMPLATE//\{\{ERROR\}\}/$ERROR_MSG}
    ESCAPED=$($PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read(), ensure_ascii=False))" <<< "$CONTEXT" 2>/dev/null || echo "\"\"")
    echo "{\"hookSpecificOutput\":{\"hookEventName\":\"ErrorOccurred\",\"additionalContext\":$ESCAPED}}"
else
    echo '{}'
fi

exit 0
