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
# AI Task Plan

## Goal
[One sentence describing the end state]

## Current Phase
Phase 1 - Requirements & Discovery

## Current Focus
- [What this phase is doing right now]

## Phases

### Phase 1: Requirements & Discovery
- Outcome: [What this phase should produce]
- Atomic tasks: [AT-...]
- **Status:** in_progress

### Phase 2: Planning & Structure
- Outcome: [What this phase should produce]
- Atomic tasks: [AT-...]
- **Status:** pending

### Phase 3: Implementation
- Outcome: [What this phase should produce]
- Atomic tasks: [AT-...]
- **Status:** pending

### Phase 4: Testing & Verification
- Outcome: [What this phase should produce]
- Atomic tasks: [AT-...]
- **Status:** pending

### Phase 5: Delivery
- Outcome: [What this phase should produce]
- Atomic tasks: [AT-...]
- **Status:** pending

## Atomic Task Ledger
1. AT-000 - in_progress - [Describe the current atomic task]

## Key Questions
1. [Question to answer]
2. [Question to answer]

## Decisions Made
| Decision | Rationale |
|----------|-----------|
|          |           |

## Follow-up Queue
1. [Next phase or follow-up task]

## Errors Encountered
| Error | Resolution |
|-------|------------|
|       |            |

## Legacy Note
1. Keep `.artifacts/ai` as the only authoritative task record set.
2. Archive obsolete planning surfaces instead of reviving them.
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
# AI Findings & Decisions

## Requirements
- [User requirement]

## Research Findings
- [Key discovery]

## Technical Decisions
| Decision | Rationale |
|----------|-----------|
|          |           |

## Issues Encountered
| Issue | Resolution |
|-------|------------|
|       |            |

## Resources
- [Doc, file, or URL]

## Visual/Browser Findings
- [Only when applicable]
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
# AI Progress Log

## Current Status
- Active atomic task: [AT-...]
- Current phase: Phase 1 - Requirements & Discovery
- Started: $DATE
- Next validation gate: [command or check]

## Session Timeline

### Session: $DATE
- [Chronological actions]

## Validation Snapshot
- Latest completed validation: [summary]
- Pending validation: [summary]

## Files Created/Modified
- [path]

## Error Log
| Timestamp | Error | Attempt | Resolution |
|-----------|-------|---------|------------|
|           |       | 1       |            |

## 5-Question Reboot Check
| Question | Answer |
|----------|--------|
| Where am I? | |
| Where am I going? | |
| What's the goal? | |
| What have I learned? | See `.artifacts/ai/findings.md` |
| What have I done? | See the session timeline above |
"@ | Out-File -FilePath $ProgressPath -Encoding UTF8
    }
    Write-Host "Created .artifacts/ai/progress.md"
} else {
    Write-Host ".artifacts/ai/progress.md already exists, skipping"
}

Write-Host ""
Write-Host "Repo-local planning files initialized!"
Write-Host "Files: .artifacts/ai/task-plan.md, .artifacts/ai/findings.md, .artifacts/ai/progress.md"
