# strict-doc-driven-development: Error hook for GitHub Copilot (Windows PowerShell)
# Reminds the agent to record errors in the .artifacts/ai workflow files.

$activeTaskFile = ".artifacts/ai/active-task.md"
$progressFile = ".artifacts/ai/progress.md"
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

if (-not (Test-Path $activeTaskFile) -and -not (Test-Path $progressFile)) {
    Write-Output '{}'
    exit 0
}

# Read stdin using a non-reserved local variable so PowerShell pipeline state does not interfere.
$rawInput = [Console]::In.ReadToEnd()

try {
    if ([string]::IsNullOrWhiteSpace($rawInput)) {
        Write-Output '{}'
        exit 0
    }

    $data = ConvertFrom-Json -InputObject $rawInput -ErrorAction Stop
    $errorMsg = ""
    if ($data.error -is [PSCustomObject]) {
        $errorMsg = $data.error.message
    } elseif ($data.error) {
        $errorMsg = [string]$data.error
    }

    if ($errorMsg) {
        $truncated = $errorMsg.Substring(0, [Math]::Min(200, $errorMsg.Length))
        $template = Get-LocalizedMessage "ERROR_OCCURRED" "[myepiclauncher] Error detected: {{ERROR}}. Log this in .artifacts/ai/progress.md. If this is the 5th failed repair attempt for the same blocker, persist the blocker in .artifacts/ai/handoff.md using blocked-bug-template.md and stop."
        $context = $template.Replace("{{ERROR}}", $truncated)
        $escaped = $context | ConvertTo-Json
        Write-Output "{`"hookSpecificOutput`":{`"hookEventName`":`"ErrorOccurred`",`"additionalContext`":$escaped}}"
    } else {
        Write-Output '{}'
    }
} catch {
    Write-Output '{}'
}

exit 0
