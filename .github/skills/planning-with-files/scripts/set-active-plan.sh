#!/bin/sh
# planning-with-files: display the fixed repo-local planning directory.

set -e

PLAN_ROOT="${PWD}/.artifacts/ai"

if [ "${1:-}" = "" ]; then
    if [ -d "$PLAN_ROOT" ]; then
        echo "Active repo-local plan: .artifacts/ai"
        echo "Path: $PLAN_ROOT"
    else
        echo "Repo-local plan directory has not been initialized yet."
        echo "Run: ./init-session.sh"
    fi
    exit 0
fi

echo "Error: this repository keeps one fixed planning directory at .artifacts/ai and does not support switching active plan IDs." >&2
echo "Use ./init-session.sh to bootstrap .artifacts/ai if needed." >&2
exit 1
