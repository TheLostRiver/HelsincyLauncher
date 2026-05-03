# planning-with-files: resolve the repo-local planning directory (PowerShell mirror).
#
# This repository keeps one authoritative planning directory at .artifacts/ai.

param(
    [string]$PlanRoot = (Join-Path (Get-Location) ".artifacts/ai")
)

if (Test-Path $PlanRoot -PathType Container) {
    Write-Output (Resolve-Path $PlanRoot).Path
} else {
    Write-Output $PlanRoot
}

exit 0
