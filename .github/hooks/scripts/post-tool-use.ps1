# strict-doc-driven-development: Post-tool-use hook for GitHub Copilot (PowerShell)
# Reminds the agent to update the .artifacts/ai workflow records after tool use.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$InputData = [Console]::In.ReadToEnd()

$output = @{
    hookSpecificOutput = @{
        hookEventName = "PostToolUse"
        additionalContext = "[myepiclauncher] Update .artifacts/ai/progress.md with what you just did. If the active task changed state, update .artifacts/ai/active-task.md. If the task is pausing, refresh .artifacts/ai/handoff.md."
    }
}
$output | ConvertTo-Json -Depth 3 -Compress
exit 0
