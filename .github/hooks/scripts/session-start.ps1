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
$StrictReminderFile = "$StrictSkillDir/session-start.txt"
$BootstrapReminderFile = "$StrictSkillDir/session-bootstrap.txt"
$ArtifactsDir = ".artifacts/ai"
$ActiveTaskFile = "$ArtifactsDir/active-task.md"
$TaskPlanFile = "$ArtifactsDir/task-plan.md"
$ProgressFile = "$ArtifactsDir/progress.md"
$HandoffFile = "$ArtifactsDir/handoff.md"
$StrictContext = ""
$BootstrapContext = ""

if (Test-Path $StrictReminderFile) {
    $StrictContext = Get-Content $StrictReminderFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
}

if (Test-Path $BootstrapReminderFile) {
    $BootstrapContext = Get-Content $BootstrapReminderFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
}

$ContextParts = @()

if (Test-Path $ActiveTaskFile) {
    $ContextParts += "[myepiclauncher] ACTIVE TASK`n" + ((Get-Content $ActiveTaskFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue).Trim())
}

if (Test-Path $HandoffFile) {
    $HandoffContext = (Get-Content $HandoffFile -Raw -Encoding UTF8 -ErrorAction SilentlyContinue).Trim()
    if ($HandoffContext) {
        $ContextParts += "[myepiclauncher] HANDOFF`n$HandoffContext"
    }
}

if ((-not $ContextParts) -and (Test-Path $TaskPlanFile)) {
    $TaskPlanContext = (Get-Content $TaskPlanFile -TotalCount 40 -Encoding UTF8 -ErrorAction SilentlyContinue) -join "`n"
    if ($TaskPlanContext) {
        $ContextParts += "[myepiclauncher] TASK PLAN`n$TaskPlanContext"
    }
}

if (Test-Path $ProgressFile) {
    $ProgressTail = Get-Content $ProgressFile -Tail 20 -Encoding UTF8 -ErrorAction SilentlyContinue
    if ($ProgressTail) {
        $ContextParts += "[myepiclauncher] RECENT PROGRESS`n" + (($ProgressTail -join "`n").Trim())
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
