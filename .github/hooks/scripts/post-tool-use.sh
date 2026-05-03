#!/bin/bash
# strict-doc-driven-development: Post-tool-use hook for GitHub Copilot
# Reminds the agent to update the .artifacts/ai workflow records after tool use.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
INPUT=$(cat)

echo '{"hookSpecificOutput":{"hookEventName":"PostToolUse","additionalContext":"[myepiclauncher] Update .artifacts/ai/progress.md with what you just did. If the active task changed state, update .artifacts/ai/active-task.md. If the task is pausing, refresh .artifacts/ai/handoff.md."}}'
exit 0
