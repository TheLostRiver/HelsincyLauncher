# planning-with-files: display the fixed repo-local planning directory (PowerShell).

param(
    [string]$PlanId = ""
)

$PlanRoot = Join-Path (Get-Location) ".artifacts/ai"

if ($PlanId -eq "") {
    if (Test-Path $PlanRoot) {
        Write-Output "Active repo-local plan: .artifacts/ai"
        Write-Output "Path: $PlanRoot"
    } else {
        Write-Output "Repo-local plan directory has not been initialized yet."
        Write-Output "Run: .\init-session.ps1"
    }
    exit 0
}

Write-Error "Error: this repository keeps one fixed planning directory at .artifacts/ai and does not support switching active plan IDs."
Write-Error "Use .\init-session.ps1 to bootstrap .artifacts/ai if needed."
exit 1
