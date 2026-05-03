# strict-doc-driven-development: Agent stop hook for GitHub Copilot (PowerShell)
# Checks the current .artifacts/ai active task state before the agent stops.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$InputData = [Console]::In.ReadToEnd()

$ActiveTaskFile = ".artifacts/ai/active-task.md"
$HandoffFile = ".artifacts/ai/handoff.md"

if (-not (Test-Path $ActiveTaskFile)) {
    Write-Output '{}'
    exit 0
}

$content = Get-Content $ActiveTaskFile -Raw -Encoding UTF8
$statusMatch = [regex]::Match($content, "(?m)^\s*-\s*status:\s*([A-Za-z_]+)")
$status = if ($statusMatch.Success) { $statusMatch.Groups[1].Value.ToLowerInvariant() } else { "unknown" }
$hasHandoff = Test-Path $HandoffFile

switch ($status) {
    "committed" {
        $msg = "[myepiclauncher] Active task is committed. Safe to stop or start the next .artifacts/ai/active-task.md entry."
    }
    "blocked" {
        if ($hasHandoff) {
            $msg = "[myepiclauncher] Active task is blocked and .artifacts/ai/handoff.md exists. Safe to stop until the next resume."
        } else {
            $msg = "[myepiclauncher] Active task is blocked. Write .artifacts/ai/handoff.md before stopping so the next session has a safe resume point."
        }
    }
    "in_progress" {
        $msg = "[myepiclauncher] Active task is still in progress. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if the task is done."
    }
    "validating" {
        $msg = "[myepiclauncher] Active task is still validating. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if validation is complete."
    }
    default {
        $msg = "[myepiclauncher] Active task status is unclear. Review .artifacts/ai/active-task.md before stopping."
    }
}

$output = @{
    hookSpecificOutput = @{
        hookEventName = "AgentStop"
        additionalContext = $msg
    }
}
$output | ConvertTo-Json -Depth 3 -Compress
exit 0
