# strict-doc-driven-development: Session start hook for GitHub Copilot (PowerShell)
# Always injects the repository strict doc-driven reminder first.
# Then reads the .artifacts/ai/ task records when they exist.
# When no .artifacts/ai/ task record exists yet, injects a workflow bootstrap reminder.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$InputData = [Console]::In.ReadToEnd()

$StrictSkillDir = ".github/skills/strict-doc-driven-development"
$ArtifactsDir = ".artifacts/ai"
$MessageCatalogDir = "$StrictSkillDir"
$ActiveTaskFile = "$ArtifactsDir/active-task.md"
$TaskPlanFile = "$ArtifactsDir/task-plan.md"
$ProgressFile = "$ArtifactsDir/progress.md"
$HandoffFile = "$ArtifactsDir/handoff.md"
$LanguageModeFile = "$ArtifactsDir/language-mode.txt"
$MessageCatalogFile = ""
$StrictContext = ""
$BootstrapContext = ""
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

if ($LanguageMode -ne "en") {
    $LanguageMode = "zh-CN"
}

$MessageCatalogFile = if ($LanguageMode -eq "en") { "$MessageCatalogDir/hook-messages.en.txt" } else { "$MessageCatalogDir/hook-messages.zh-CN.txt" }

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

$StrictReminderFile = if ($LanguageMode -eq "en") { "$StrictSkillDir/session-start.en.txt" } else { "$StrictSkillDir/session-start.zh-CN.txt" }
$BootstrapReminderFile = if ($LanguageMode -eq "en") { "$StrictSkillDir/session-bootstrap.en.txt" } else { "$StrictSkillDir/session-bootstrap.zh-CN.txt" }
$ActiveTaskLabel = Get-LocalizedMessage "ACTIVE_TASK_LABEL" "[myepiclauncher] ACTIVE TASK"
$HandoffLabel = Get-LocalizedMessage "HANDOFF_LABEL" "[myepiclauncher] HANDOFF"
$TaskPlanLabel = Get-LocalizedMessage "TASK_PLAN_LABEL" "[myepiclauncher] TASK PLAN"
$ProgressLabel = Get-LocalizedMessage "RECENT_PROGRESS_LABEL" "[myepiclauncher] RECENT PROGRESS"

if (Test-Path $StrictReminderFile) {
    $StrictContext = Get-Content $StrictReminderFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
}

if (Test-Path $BootstrapReminderFile) {
    $BootstrapContext = Get-Content $BootstrapReminderFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
}

$ContextParts = @()

if (Test-Path $ActiveTaskFile) {
    $ContextParts += "$ActiveTaskLabel`n" + ((Get-Content $ActiveTaskFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue).Trim())
}

if (Test-Path $HandoffFile) {
    $HandoffContext = (Get-Content $HandoffFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue).Trim()
    if ($HandoffContext) {
        $ContextParts += "$HandoffLabel`n$HandoffContext"
    }
}

if ((-not $ContextParts) -and (Test-Path $TaskPlanFile)) {
    $TaskPlanContext = (Get-Content $TaskPlanFile -TotalCount 40 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($TaskPlanContext) {
        $ContextParts += "$TaskPlanLabel`n$TaskPlanContext"
    }
}

if (Test-Path $ProgressFile) {
    $ProgressTail = Get-Content $ProgressFile -Tail 20 -Encoding UTF8 -ErrorAction SilentlyContinue
    if ($ProgressTail) {
        $ContextParts += "$ProgressLabel`n" + (($ProgressTail -join "`n").Trim())
    }
}

if ($ContextParts.Count -gt 0) {
    $Context = ($ContextParts -join "`n`n").Trim()
} elseif ($BootstrapContext) {
    $Context = $BootstrapContext.Trim()
}

if ($StrictContext -and $Context) {
    $Context = (($StrictContext.Trim()), ($Context.Trim()) -join "`n`n")
} elseif ($StrictContext) {
    $Context = $StrictContext.Trim()
} elseif ($Context) {
    $Context = $Context.Trim()
}

if (-not $Context) {
    Write-Output '{}'
    exit 0
}

$output = @{
    hookSpecificOutput = @{
        hookEventName = "SessionStart"
        additionalContext = $Context
    }
}
$output | ConvertTo-Json -Depth 3 -Compress
exit 0
