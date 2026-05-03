# Active Atomic Task Template

Use this to define the current atomic task before implementation starts.
Suggested file path: .artifacts/ai/active-task.md.

## Identity

- task id:
- title:
- status: ready | in_progress | validating | committed | blocked

## Goal

- exact local outcome:

## Scope

- in scope:
- out of scope:

## Allowed Files

1. path/to/file-or-directory
2. path/to/file-or-directory

## Local Task Records Read

1. .artifacts/ai/active-task.md
2. .artifacts/ai/task-plan.md
3. .artifacts/ai/progress.md
4. .artifacts/ai/findings.md

## Controlling Docs

1. docs/...
2. docs/...

## Hypothesis

- falsifiable local hypothesis:

## Cheap Check

- narrowest check that can disconfirm the hypothesis:

## Validation Gate

1. focused compile, test, or validation command

## Docs And Logs To Update

1. docs to refresh after success
2. task records to update after success

## Post-validation Git Action

1. commit message plan
2. push command plan

## Stop Conditions

1. missing or conflicting docs
2. out-of-scope change request
3. same blocker still failing after 5 repair attempts

## Safe Resume Point

- exact next step if execution is interrupted: