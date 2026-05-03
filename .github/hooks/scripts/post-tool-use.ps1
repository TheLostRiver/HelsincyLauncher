# strict-doc-driven-development: Post-tool-use hook for GitHub Copilot (PowerShell)
# Reminds the agent to update the .artifacts/ai workflow records after tool use.
# Always exits 0 — outputs JSON to stdout.

# Read stdin (required — Copilot pipes JSON to stdin)
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$InputData = [Console]::In.ReadToEnd()

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

$PostToolUseMessage = Get-LocalizedMessage "POST_TOOL_USE" "[myepiclauncher] Update .artifacts/ai/progress.md with what you just did. If the active task changed state, update .artifacts/ai/active-task.md. If the task is pausing, refresh .artifacts/ai/handoff.md."

$output = @{
    hookSpecificOutput = @{
        hookEventName = "PostToolUse"
        additionalContext = $PostToolUseMessage
    }
}
$output | ConvertTo-Json -Depth 3 -Compress
exit 0
