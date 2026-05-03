#!/bin/sh
# Check if all phases in .artifacts/ai/task-plan.md are complete.
# Always exits 0 and uses stdout for status reporting.

PLAN_FILE="${1:-.artifacts/ai/task-plan.md}"

if [ ! -f "$PLAN_FILE" ]; then
    echo "[planning-with-files] No .artifacts/ai/task-plan.md found -- no active planning session."
    exit 0
fi

TASK_LINES=$(grep -Ei '^[[:space:]]*[0-9]+\.[[:space:]]+.+[[:space:]]-[[:space:]](committed|complete|completed|in_progress|pending|blocked|not_started|not-started)[[:space:]]-[[:space:]]+' "$PLAN_FILE" || true)

if [ -n "$TASK_LINES" ]; then
    TOTAL=$(printf "%s\n" "$TASK_LINES" | sed '/^$/d' | wc -l | tr -d ' ')
    COMPLETE=$(printf "%s\n" "$TASK_LINES" | grep -Eic ' - (committed|complete|completed) - ' || true)
    IN_PROGRESS=$(printf "%s\n" "$TASK_LINES" | grep -Eic ' - in_progress - ' || true)
    PENDING=$((TOTAL - COMPLETE - IN_PROGRESS))
else
    TOTAL=$(grep -c '^### Phase' "$PLAN_FILE" || true)
    COMPLETE=$(grep -cF '**Status:** complete' "$PLAN_FILE" || true)
    IN_PROGRESS=$(grep -cF '**Status:** in_progress' "$PLAN_FILE" || true)
    PENDING=$(grep -cF '**Status:** pending' "$PLAN_FILE" || true)

    if [ "$COMPLETE" -eq 0 ] && [ "$IN_PROGRESS" -eq 0 ] && [ "$PENDING" -eq 0 ]; then
        COMPLETE=$(grep -c '\[complete\]' "$PLAN_FILE" || true)
        IN_PROGRESS=$(grep -c '\[in_progress\]' "$PLAN_FILE" || true)
        PENDING=$(grep -c '\[pending\]' "$PLAN_FILE" || true)
    fi
fi

: "${TOTAL:=0}"
: "${COMPLETE:=0}"
: "${IN_PROGRESS:=0}"
: "${PENDING:=0}"

if [ "$COMPLETE" -eq "$TOTAL" ] && [ "$TOTAL" -gt 0 ]; then
    echo "[planning-with-files] ALL PHASES COMPLETE ($COMPLETE/$TOTAL). If the user has additional work, add new phases to .artifacts/ai/task-plan.md before starting."
else
    echo "[planning-with-files] Task in progress ($COMPLETE/$TOTAL phases complete). Update .artifacts/ai/progress.md before stopping."
    if [ "$IN_PROGRESS" -gt 0 ]; then
        echo "[planning-with-files] $IN_PROGRESS phase(s) still in progress."
    fi
    if [ "$PENDING" -gt 0 ]; then
        echo "[planning-with-files] $PENDING phase(s) pending."
    fi
fi

exit 0
