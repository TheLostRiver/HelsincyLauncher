#!/bin/bash
# strict-doc-driven-development: Post-tool-use hook for GitHub Copilot
# Reminds the agent to update the .artifacts/ai workflow records after tool use.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

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

POST_TOOL_USE_MESSAGE=$(lookup_message "POST_TOOL_USE" "[myepiclauncher] Update .artifacts/ai/progress.md with what you just did. If the active task changed state, update .artifacts/ai/active-task.md. If the task is pausing, refresh .artifacts/ai/handoff.md.")

PYTHON=""
for _p in /usr/bin/python3 /usr/local/bin/python3 /opt/homebrew/bin/python3; do
	[ -x "$_p" ] && { PYTHON="$_p"; break; }
done
[ -z "$PYTHON" ] && PYTHON=$(command -v python3 2>/dev/null || command -v python 2>/dev/null)
ESCAPED=$(echo "$POST_TOOL_USE_MESSAGE" | $PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read(), ensure_ascii=False))" 2>/dev/null || echo "\"\"")

echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PostToolUse\",\"additionalContext\":$ESCAPED}}"
exit 0
