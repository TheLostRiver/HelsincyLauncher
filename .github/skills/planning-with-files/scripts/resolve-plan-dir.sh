#!/bin/sh
# planning-with-files: resolve the repo-local planning directory.
#
# This repository keeps one authoritative planning directory at .artifacts/ai.

set -u

PLAN_ROOT="${1:-${PWD}/.artifacts/ai}"

printf "%s\n" "${PLAN_ROOT}"
exit 0
