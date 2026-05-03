# strict-doc-driven-development: Pre-tool-use hook for GitHub Copilot (PowerShell)
# Reads the active atomic task first, then falls back to the .artifacts/ai task plan.
# Always allows tool execution — this hook never blocks tools.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$InputData = [Console]::In.ReadToEnd()

$StrictSkillDir = ".github/skills/strict-doc-driven-development"
$ActiveTaskFile = ".artifacts/ai/active-task.md"
$TaskPlanFile = ".artifacts/ai/task-plan.md"
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

$ActiveTaskLabel = Get-LocalizedMessage "ACTIVE_TASK_LABEL" "[myepiclauncher] ACTIVE TASK"
$TaskPlanLabel = Get-LocalizedMessage "TASK_PLAN_LABEL" "[myepiclauncher] TASK PLAN"

if (-not (Test-Path $ActiveTaskFile) -and -not (Test-Path $TaskPlanFile)) {
    Write-Output '{}'
    exit 0
}

$Context = ""

if (Test-Path $ActiveTaskFile) {
    $ActiveTask = (Get-Content $ActiveTaskFile -TotalCount 80 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($ActiveTask) {
        $Context = "$ActiveTaskLabel`n$ActiveTask"
    }
} elseif (Test-Path $TaskPlanFile) {
    $TaskPlan = (Get-Content $TaskPlanFile -TotalCount 40 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($TaskPlan) {
        $Context = "$TaskPlanLabel`n$TaskPlan"
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
