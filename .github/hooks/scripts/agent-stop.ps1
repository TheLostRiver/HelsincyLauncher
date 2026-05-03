# strict-doc-driven-development: Agent stop hook for GitHub Copilot (PowerShell)
# Checks the current .artifacts/ai active task state before the agent stops.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$InputData = [Console]::In.ReadToEnd()

$ActiveTaskFile = ".artifacts/ai/active-task.md"
$HandoffFile = ".artifacts/ai/handoff.md"
$StrictSkillDir = ".github/skills/strict-doc-driven-development"
$LanguageModeFile = ".artifacts/ai/language-mode.txt"
$MessageCatalogFile = ""
$Messages = @{}

function Get-LocalizedMessage {
    param(
        [string]$Key,
        [string]$Fallback
    )

    if ($Messages.ContainsKey($Key) -and $Messages[$Key]) {
        return $Messages[$Key]
    }

    return $Fallback
}

$LanguageMode = if ($env:MYEPIC_WORKFLOW_LANG) {
    $env:MYEPIC_WORKFLOW_LANG.Trim()
} elseif (Test-Path $LanguageModeFile) {
    (Get-Content $LanguageModeFile -TotalCount 1 -Encoding UTF8 -ErrorAction SilentlyContinue).Trim()
} else {
    "zh-CN"
}

if ($LanguageMode -eq "en") {
    $MessageCatalogFile = "$StrictSkillDir/hook-messages.en.txt"
} else {
    $LanguageMode = "zh-CN"
    $MessageCatalogFile = "$StrictSkillDir/hook-messages.zh-CN.txt"
}

if (Test-Path $MessageCatalogFile) {
    foreach ($line in Get-Content $MessageCatalogFile -Encoding UTF8 -ErrorAction SilentlyContinue) {
        if ([string]::IsNullOrWhiteSpace($line) -or $line.TrimStart().StartsWith("#")) {
            continue
        }

        $parts = $line -split "=", 2
        if ($parts.Count -eq 2) {
            $Messages[$parts[0].Trim()] = $parts[1]
        }
    }
}

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
        $msg = Get-LocalizedMessage "AGENT_STOP_COMMITTED" "[myepiclauncher] Active task is committed. Safe to stop or start the next .artifacts/ai/active-task.md entry."
    }
    "blocked" {
        if ($hasHandoff) {
            $msg = Get-LocalizedMessage "AGENT_STOP_BLOCKED_WITH_HANDOFF" "[myepiclauncher] Active task is blocked and .artifacts/ai/handoff.md exists. Safe to stop until the next resume."
        } else {
            $msg = Get-LocalizedMessage "AGENT_STOP_BLOCKED_NO_HANDOFF" "[myepiclauncher] Active task is blocked. Write .artifacts/ai/handoff.md before stopping so the next session has a safe resume point."
        }
    }
    "in_progress" {
        $msg = Get-LocalizedMessage "AGENT_STOP_IN_PROGRESS" "[myepiclauncher] Active task is still in progress. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if the task is done."
    }
    "validating" {
        $msg = Get-LocalizedMessage "AGENT_STOP_VALIDATING" "[myepiclauncher] Active task is still validating. Refresh .artifacts/ai/handoff.md before stopping, or mark .artifacts/ai/active-task.md as committed if validation is complete."
    }
    default {
        $msg = Get-LocalizedMessage "AGENT_STOP_UNKNOWN" "[myepiclauncher] Active task status is unclear. Review .artifacts/ai/active-task.md before stopping."
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
