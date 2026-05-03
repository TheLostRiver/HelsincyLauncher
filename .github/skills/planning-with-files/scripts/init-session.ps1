# Initialize repo-local planning files for a new session.
# Usage: .\init-session.ps1 [-Template TYPE] [project-name]
# Templates: default, analytics

param(
    [string]$ProjectName = "project",
    [string]$Template = "default"
)

$DATE = Get-Date -Format "yyyy-MM-dd"

# Resolve template directory (skill root is one level up from scripts/)
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$SkillRoot = Split-Path -Parent $ScriptDir
$TemplateDir = Join-Path $SkillRoot "templates"

$ArtifactsDir = Join-Path (Get-Location) ".artifacts/ai"
$TaskPlanPath = Join-Path $ArtifactsDir "task-plan.md"
$FindingsPath = Join-Path $ArtifactsDir "findings.md"
$ProgressPath = Join-Path $ArtifactsDir "progress.md"

Write-Host "Initializing repo-local planning files for: $ProjectName (template: $Template)"

# Validate template
if ($Template -ne "default" -and $Template -ne "analytics") {
    Write-Host "Unknown template: $Template (available: default, analytics). Using default."
    $Template = "default"
}

# Ensure the repo-local workflow directory exists.
if (-not (Test-Path $ArtifactsDir)) {
    New-Item -ItemType Directory -Path $ArtifactsDir -Force | Out-Null
}

# Create .artifacts/ai/task-plan.md if it doesn't exist
if (-not (Test-Path $TaskPlanPath)) {
    $AnalyticsPlan = Join-Path $TemplateDir "analytics_task_plan.md"
    if ($Template -eq "analytics" -and (Test-Path $AnalyticsPlan)) {
        Copy-Item $AnalyticsPlan $TaskPlanPath
    } else {
        @"
# Task Plan: [Brief Description]

## Goal
[One sentence describing the end state]

## Current Phase
Phase 1

## Phases

### Phase 1: Requirements & Discovery
- [ ] Understand user intent
- [ ] Identify constraints
- [ ] Document in .artifacts/ai/findings.md
- **Status:** in_progress

### Phase 2: Planning & Structure
- [ ] Define approach
- [ ] Create project structure
- **Status:** pending

### Phase 3: Implementation
- [ ] Execute the plan
- [ ] Write to files before executing
- **Status:** pending

### Phase 4: Testing & Verification
- [ ] Verify requirements met
- [ ] Document test results in .artifacts/ai/progress.md
- **Status:** pending

### Phase 5: Delivery
- [ ] Review outputs
- [ ] Deliver to user
- **Status:** pending

## Decisions Made
| Decision | Rationale |
|----------|-----------|

## Errors Encountered
| Error | Resolution |
|-------|------------|
"@ | Out-File -FilePath $TaskPlanPath -Encoding UTF8
    }
    Write-Host "Created .artifacts/ai/task-plan.md"
} else {
    Write-Host ".artifacts/ai/task-plan.md already exists, skipping"
}

# Create .artifacts/ai/findings.md if it doesn't exist
if (-not (Test-Path $FindingsPath)) {
    $AnalyticsFindings = Join-Path $TemplateDir "analytics_findings.md"
    if ($Template -eq "analytics" -and (Test-Path $AnalyticsFindings)) {
        Copy-Item $AnalyticsFindings $FindingsPath
    } else {
        @"
# Findings & Decisions

## Requirements
-

## Research Findings
-

## Technical Decisions
| Decision | Rationale |
|----------|-----------|

## Issues Encountered
| Issue | Resolution |
|-------|------------|

## Resources
-
"@ | Out-File -FilePath $FindingsPath -Encoding UTF8
    }
    Write-Host "Created .artifacts/ai/findings.md"
} else {
    Write-Host ".artifacts/ai/findings.md already exists, skipping"
}

# Create .artifacts/ai/progress.md if it doesn't exist
if (-not (Test-Path $ProgressPath)) {
    if ($Template -eq "analytics") {
        @"
# Progress Log

## Session: $DATE

### Current Status
- **Phase:** 1 - Data Discovery
- **Started:** $DATE

### Actions Taken
-

### Query Log
| Query | Result Summary | Interpretation |
|-------|---------------|----------------|

### Errors
| Error | Resolution |
|-------|------------|
"@ | Out-File -FilePath $ProgressPath -Encoding UTF8
    } else {
        @"
# Progress Log

## Session: $DATE

### Current Status
- **Phase:** 1 - Requirements & Discovery
- **Started:** $DATE

### Actions Taken
-

### Test Results
| Test | Expected | Actual | Status |
|------|----------|--------|--------|

### Errors
| Error | Resolution |
|-------|------------|
"@ | Out-File -FilePath $ProgressPath -Encoding UTF8
    }
    Write-Host "Created .artifacts/ai/progress.md"
} else {
    Write-Host ".artifacts/ai/progress.md already exists, skipping"
}

Write-Host ""
Write-Host "Repo-local planning files initialized!"
Write-Host "Files: .artifacts/ai/task-plan.md, .artifacts/ai/findings.md, .artifacts/ai/progress.md"
