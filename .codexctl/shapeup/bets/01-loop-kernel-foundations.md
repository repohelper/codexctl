# Bet 01: Loop Kernel Foundations

Status: Finalized
Appetite: 6 weeks
Cycle fit: Recommended first bet
Depends on: None
Build approval: Approved

## Problem

`codexctl` can switch profiles and inspect auth/usage state, but it cannot yet operate as a reliable local control plane for unattended execution.

Today we are missing:
- a versioned bet-spec format
- a reusable deterministic validation engine
- persisted run state for resumed unattended work
- an inspectable run ledger
- a narrow outer loop that repeats until external checks pass

For the target user, this means the CLI still cannot reliably convert a shaped product bet into unattended execution leverage.

## Appetite

We are willing to spend up to 6 weeks on a first complete and production-ready vertical slice.

If scope threatens appetite, cut features before extending time.

## Freeze

This bet is frozen for implementation.

Change control:
- cuts are allowed
- clarifications are allowed
- scope additions require reshaping and explicit approval

## Success signal

A technical user can:
1. write a shaped bet/task spec under `.codexctl/tasks/`
2. run `codexctl run-loop --task ...`
3. inspect progress with `codexctl runs`
4. trust that completion only happens after deterministic checks pass
5. resume a crashed/interrupted run without state corruption

Secondary success signal:
- the same user can understand the spec format quickly enough that future `bet init` and `bet lint` can scaffold it without redefining the model

## Bounded contexts touched

Primary:
- `Task Definition`
- `Validation`
- `Run Orchestration`
- `Run Ledger`

Secondary:
- `Live Auth Projection` only when `--profile` is used

## Shaped solution

Deliver four command/module slices in one bet:
1. bet spec schema v1 and parser
2. `validate` command and validator engine
3. `run-loop` command and run-state persistence
4. `runs` command for inspection

Internal implementation posture:
- treat this as one finalized bet, but execute it in two internal build phases
  - Phase A: bet spec + validate
  - Phase B: run ledger + run-loop + runs

This preserves the finalized product bet while reducing delivery risk.

State lives in:
- `.codexctl/tasks/` for repo-owned task inputs
- `$CODEXCTL_DIR/runs/` for persisted run ledger data

Schema stance:
- the consumer-facing spec is intentionally narrow
- it should model a shaped bet or scoped implementation task
- it should not model arbitrary workflow graphs, generic tickets, or open-ended automation pipelines
- it should be strict enough to improve outcomes and simple enough that a high-agency user can author one quickly

## No-gos

Do not include:
- worktree support
- multi-task queueing
- multi-agent orchestration
- platform-specific notifications
- agent-driven review phase
- autonomous commits/pushes/branch cleanup
- a general workflow DSL or plugin-defined task schema
- first-class queueing
- first-class `bet init` and `bet lint` commands in this bet

## Rabbit holes

Expected rabbit holes to actively avoid:
- designing a ticketing framework instead of a bet/task spec
- inventing a validator DSL instead of using shell checks in v1
- overengineering prompt templating
- making `runs` a dashboard instead of an inspection command
- mixing usage/auth reporting concerns into unattended execution
- broadening the schema to support every possible consumer workflow
- adding authoring scaffolds before the core model and validator contract are stable

## Domain design choices

Aggregate roots:
- `TaskSpec`
- `RunRecord`
- `ValidationResult`

Application services:
- `ValidateCommand`
- `RunLoopCommand`
- `RunsCommand`

Domain services:
- `TaskLoader`
- `ValidatorExecutor`
- `RunStateStore`
- `StopPolicy`

Value objects:
- `RunId`
- `IterationNumber`
- `ValidationCheckId`
- `TimeoutPolicy`
- `StopReason`

## Delivery slices

### Slice 1: Task spec v1
Deliver:
- YAML bet/task schema
- parser and validation
- task snapshot capability

Exit condition:
- a minimal valid shaped bet/task file can be parsed and round-tripped into a run snapshot

### Slice 2: Validation engine
Deliver:
- shell-command checks
- timeout handling
- aggregate pass/fail result
- `validate --json`

Exit condition:
- validators have stable result objects and tested exit codes

### Slice 3: Run ledger primitives
Deliver:
- run directory creation
- `run.json`
- `events.jsonl`
- iteration artifact layout
- atomic writes where needed

Exit condition:
- a synthetic run can be written, read, and resumed safely in tests

### Slice 4: Minimal `run-loop`
Deliver:
- one task execution loop
- agent invocation
- validator feedback summary
- stop conditions and budgets
- `--resume`

Exit condition:
- loop succeeds only when acceptance checks pass

### Slice 5: `runs` inspection
Deliver:
- list latest runs
- inspect one run
- JSON output

Exit condition:
- an operator can answer "what happened?" without opening raw files manually

## Pre-build checklist

Before starting implementation:
- freeze command names: `validate`, `run-loop`, `runs`
- freeze bet spec v1 field names
- freeze JSON status enums for validator and run state
- decide shell execution contract for checks
- decide agent invocation contract for loop iterations
- confirm `--profile` behavior reuses the current auth-only switching semantics
- confirm the schema stays Shape Up + DDD opinionated and does not become generic
- define the future reserved command names `bet init` and `bet lint` so the model can grow without renaming later

## Post-build checklist

Before calling this bet done:
- all new commands support `--json`
- exit codes are explicit and integration-tested
- resume works after an interrupted process
- validator timeout does not leave orphaned processes
- run-state files are human-readable and machine-readable
- `run-loop` never reports success if checks fail
- help text explains unattended stop conditions
- current profile switching behavior is not regressed
- the resulting model clearly supports future bet authoring scaffolds without changing the core schema

## Fact-check / validation checklist

Must validate before approving the bet:
- Shape Up appetite is still treated as fixed and scope is still the variable
- official Ralph references still support validator-led outer loops
- DDD boundaries still make sense against the actual module layout as it evolves
- no official Codex feature has emerged that makes `validate` or `run-loop` redundant
- the schema still reflects shaped work, bounded contexts, and explicit no-gos rather than broad workflow modeling

## Acceptance test matrix

Required automated coverage:
- malformed task file
- task file missing checks
- task file missing appetite / bounded contexts / no-gos
- single passing validator
- failing validator
- validator timeout
- `run-loop` success path
- `run-loop` repeated failure path
- `run-loop --resume`
- `run-loop --profile` restore behavior
- `runs --json`
