#!/bin/bash
# strict-doc-driven-development: Error hook for GitHub Copilot
# Reminds the agent to record errors in the .artifacts/ai workflow files.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

ACTIVE_TASK_FILE=".artifacts/ai/active-task.md"
PROGRESS_FILE=".artifacts/ai/progress.md"

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
    CONTEXT="[myepiclauncher] Error detected: ${ERROR_MSG}. Log this in .artifacts/ai/progress.md. If this is the 5th failed repair attempt for the same blocker, persist the blocker in .artifacts/ai/handoff.md using blocked-bug-template.md and stop."
    ESCAPED=$($PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read(), ensure_ascii=False))" <<< "$CONTEXT" 2>/dev/null || echo "\"\"")
    echo "{\"hookSpecificOutput\":{\"hookEventName\":\"ErrorOccurred\",\"additionalContext\":$ESCAPED}}"
else
    echo '{}'
fi

exit 0
