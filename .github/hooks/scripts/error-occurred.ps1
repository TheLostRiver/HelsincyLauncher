# strict-doc-driven-development: Error hook for GitHub Copilot (Windows PowerShell)
# Reminds the agent to record errors in the .artifacts/ai workflow files.

$activeTaskFile = ".artifacts/ai/active-task.md"
$progressFile = ".artifacts/ai/progress.md"

if (-not (Test-Path $activeTaskFile) -and -not (Test-Path $progressFile)) {
    Write-Output '{}'
    exit 0
}

# Read stdin
$input = [Console]::In.ReadToEnd()

try {
    $data = $input | ConvertFrom-Json
    $errorMsg = ""
    if ($data.error -is [PSCustomObject]) {
        $errorMsg = $data.error.message
    } elseif ($data.error) {
        $errorMsg = [string]$data.error
    }

    if ($errorMsg) {
        $truncated = $errorMsg.Substring(0, [Math]::Min(200, $errorMsg.Length))
        $context = "[myepiclauncher] Error detected: $truncated. Log this in .artifacts/ai/progress.md. If this is the 5th failed repair attempt for the same blocker, persist the blocker in .artifacts/ai/handoff.md using blocked-bug-template.md and stop."
        $escaped = $context | ConvertTo-Json
        Write-Output "{`"hookSpecificOutput`":{`"hookEventName`":`"ErrorOccurred`",`"additionalContext`":$escaped}}"
    } else {
        Write-Output '{}'
    }
} catch {
    Write-Output '{}'
}

exit 0
