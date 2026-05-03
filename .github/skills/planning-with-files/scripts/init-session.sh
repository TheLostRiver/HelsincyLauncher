#!/bin/sh
# Initialize repo-local planning files for a new session.

set -e

TEMPLATE="default"
PROJECT_NAME="project"

while [ "$#" -gt 0 ]; do
    case "$1" in
        --template|-t)
            TEMPLATE="${2:-default}"
            shift 2
            ;;
        *)
            PROJECT_NAME="$1"
            shift
            ;;
    esac
done

DATE=$(date +%Y-%m-%d)
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
SKILL_ROOT=$(dirname -- "$SCRIPT_DIR")
TEMPLATE_DIR="$SKILL_ROOT/templates"
ARTIFACTS_DIR="${PWD}/.artifacts/ai"
TASK_PLAN_PATH="${ARTIFACTS_DIR}/task-plan.md"
FINDINGS_PATH="${ARTIFACTS_DIR}/findings.md"
PROGRESS_PATH="${ARTIFACTS_DIR}/progress.md"

if [ "$TEMPLATE" != "default" ] && [ "$TEMPLATE" != "analytics" ]; then
    echo "Unknown template: $TEMPLATE (available: default, analytics). Using default."
    TEMPLATE="default"
fi

write_default_task_plan() {
    cat > "$1" <<'EOF'
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
EOF
}

write_default_findings() {
    cat > "$1" <<'EOF'
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
EOF
}

write_default_progress() {
    date_value="$1"
    cat > "$2" <<EOF
# Progress Log

## Session: ${date_value}

### Current Status
- **Phase:** 1 - Requirements & Discovery
- **Started:** ${date_value}

### Actions Taken
-

### Test Results
| Test | Expected | Actual | Status |
|------|----------|--------|--------|

### Errors
| Error | Resolution |
|-------|------------|
EOF
}

write_analytics_progress() {
    date_value="$1"
    cat > "$2" <<EOF
# Progress Log

## Session: ${date_value}

### Completed
-

### In Progress
-

### Metrics
| Metric | Value |
|--------|-------|

### Next Steps
-

### Errors
| Error | Resolution |
|-------|------------|
EOF
}

mkdir -p "$ARTIFACTS_DIR"

echo "Initializing repo-local planning files for: $PROJECT_NAME (template: $TEMPLATE)"

if [ ! -f "$TASK_PLAN_PATH" ]; then
    if [ "$TEMPLATE" = "analytics" ] && [ -f "$TEMPLATE_DIR/analytics_task_plan.md" ]; then
        cp "$TEMPLATE_DIR/analytics_task_plan.md" "$TASK_PLAN_PATH"
    else
        write_default_task_plan "$TASK_PLAN_PATH"
    fi
    echo "Created .artifacts/ai/task-plan.md"
else
    echo ".artifacts/ai/task-plan.md already exists, skipping"
fi

if [ ! -f "$FINDINGS_PATH" ]; then
    if [ "$TEMPLATE" = "analytics" ] && [ -f "$TEMPLATE_DIR/analytics_findings.md" ]; then
        cp "$TEMPLATE_DIR/analytics_findings.md" "$FINDINGS_PATH"
    else
        write_default_findings "$FINDINGS_PATH"
    fi
    echo "Created .artifacts/ai/findings.md"
else
    echo ".artifacts/ai/findings.md already exists, skipping"
fi

if [ ! -f "$PROGRESS_PATH" ]; then
    if [ "$TEMPLATE" = "analytics" ]; then
        write_analytics_progress "$DATE" "$PROGRESS_PATH"
    else
        write_default_progress "$DATE" "$PROGRESS_PATH"
    fi
    echo "Created .artifacts/ai/progress.md"
else
    echo ".artifacts/ai/progress.md already exists, skipping"
fi

echo ""
echo "Repo-local planning files initialized!"
echo "Files: .artifacts/ai/task-plan.md, .artifacts/ai/findings.md, .artifacts/ai/progress.md"
