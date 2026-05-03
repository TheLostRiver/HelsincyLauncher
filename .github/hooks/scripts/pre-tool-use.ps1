# strict-doc-driven-development: Pre-tool-use hook for GitHub Copilot (PowerShell)
# Reads the active atomic task first, then falls back to the .artifacts/ai task plan.
# Always allows tool execution — this hook never blocks tools.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$InputData = [Console]::In.ReadToEnd()

$ActiveTaskFile = ".artifacts/ai/active-task.md"
$TaskPlanFile = ".artifacts/ai/task-plan.md"

if (-not (Test-Path $ActiveTaskFile) -and -not (Test-Path $TaskPlanFile)) {
    Write-Output '{}'
    exit 0
}

$Context = ""

if (Test-Path $ActiveTaskFile) {
    $ActiveTask = (Get-Content $ActiveTaskFile -TotalCount 80 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($ActiveTask) {
        $Context = "[myepiclauncher] ACTIVE TASK`n$ActiveTask"
    }
} elseif (Test-Path $TaskPlanFile) {
    $TaskPlan = (Get-Content $TaskPlanFile -TotalCount 40 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($TaskPlan) {
        $Context = "[myepiclauncher] TASK PLAN`n$TaskPlan"
    }
}

if (-not $Context) {
    Write-Output '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}'
    exit 0
}

$output = @{
    hookSpecificOutput = @{
        hookEventName = "PreToolUse"
        permissionDecision = "allow"
        additionalContext = $Context
    }
}
$output | ConvertTo-Json -Depth 3 -Compress
exit 0
