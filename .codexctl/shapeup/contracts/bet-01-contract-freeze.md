# Bet 01 Contract Freeze

Date: 2026-04-14 (UTC)
Status: Finalized
Applies to: Bet 01 - Loop Kernel Foundations

This document freezes the implementation-facing contracts for Bet 01.

If implementation discovers a need to broaden these contracts, that is a reshaping event, not an implementation detail.

## Scope of this freeze

This freeze covers:
- command names
- command flags and argument rules
- `codexctl-bet/v1` schema fields
- JSON output contracts
- exit-code contracts
- reserved future command names
- minimum repo-state policy for Bet 01

This freeze does not cover:
- worktree mode
- notification hooks
- review-check execution
- queueing / sequencing
- `bet init` / `bet lint` implementation

## Command names

Finalized command names for Bet 01:
- `codexctl validate`
- `codexctl run-loop`
- `codexctl runs`

Reserved future names:
- `codexctl bet init`
- `codexctl bet lint`

These reserved names must not be reused for unrelated commands.

## Command contracts

### 1. `codexctl validate`

Purpose:
- Run deterministic acceptance checks from CLI flags or a bet spec.

Invocation rules:
- Requires at least one of:
  - `--task <path>`
  - one or more `--check <command>`
- If both `--task` and `--check` are provided:
  - CLI `--check` values append to the bet spec `acceptance_checks`
- `review_checks` are ignored by `validate` in Bet 01

Flags:
- `--task <path>`
- `--check <command>` repeated
- `--timeout-seconds <n>`
- `--cwd <path>`
- `--fail-fast`
- `--json`

Human output:
- concise summary
- one line per failing or timed-out check when needed

JSON mode:
- no progress noise
- one final JSON object only

### 2. `codexctl run-loop`

Purpose:
- Execute one shaped bet/task until acceptance checks pass or stop conditions are reached.

Invocation rules:
- Requires exactly one of:
  - `--task <path>`
  - `--resume <run-id>`
- `--resume` may be passed without `--task`; the saved snapshot is source of truth
- If both are supplied, fail with CLI usage error

Flags:
- `--task <path>`
- `--resume <run-id>`
- `--max-iterations <n>`
- `--timeout-minutes <n>`
- `--max-consecutive-failures <n>`
- `--profile <name>`
- `--passphrase <value>`
- `--dry-run`
- `--json`

Behavior rules:
- `--max-*` values override bet spec budgets for that invocation only
- `--profile` reuses current auth-only switching semantics
- `--dry-run` creates no persistent run unless explicitly needed for validation of inputs; Bet 01 default is no persistent run directory in dry-run mode
- no review phase in Bet 01
- no notification hooks in Bet 01

Human output:
- run id
- task name
- iteration count
- current/final status
- stop reason on non-success

JSON mode:
- no progress noise
- one final JSON object only

### 3. `codexctl runs`

Purpose:
- Inspect existing run ledger entries.

Invocation rules:
- list mode is default
- detail mode requires `--id <run-id>` or `--latest`
- `--latest` and `--id` are mutually exclusive

Flags:
- `--latest`
- `--id <run-id>`
- `--json`
- `--tail`

Behavior rules:
- `--tail` is human-output only in Bet 01
- `--tail` and `--json` are incompatible in Bet 01

## `codexctl-bet/v1` schema contract

File path convention:
- `.codexctl/tasks/<name>.yaml`

Required top-level fields:
- `type`
- `name`
- `appetite`
- `objective`
- `bounded_contexts`
- `success_signal`
- `no_gos`
- `acceptance_checks`

Optional top-level fields:
- `context_files`
- `constraints`
- `review_checks`
- `agent`
- `budgets`
- `notes`

### Required field rules

#### `type`
- required string
- exact value: `codexctl-bet/v1`

#### `name`
- required string
- non-empty
- should be filesystem-safe and human-readable

#### `appetite`
- required string
- non-empty
- expected convention in Bet 01: `<number>_<unit>`
- examples:
  - `3_days`
  - `1_week`
  - `2_weeks`
  - `6_weeks`

#### `objective`
- required string
- non-empty
- should describe the shaped outcome, not implementation trivia only

#### `bounded_contexts`
- required array of strings
- at least one entry
- entries should align with the domain map naming

#### `success_signal`
- required string
- non-empty
- should describe what success looks like for the operator/user

#### `no_gos`
- required array of strings
- at least one entry
- each entry should be explicit enough to constrain scope

#### `acceptance_checks`
- required array of shell command strings
- at least one entry

### Optional field rules

#### `context_files`
- array of relative repo paths
- may be empty

#### `constraints`
- array of strings
- may be empty

#### `review_checks`
- array of shell command strings
- accepted in schema but not executed in Bet 01

#### `agent`
- object
- supported fields in Bet 01:
  - `prompt_preamble`: string
  - `command`: array of argv strings

#### `budgets`
- object
- supported fields in Bet 01:
  - `max_iterations`: integer > 0
  - `max_runtime_minutes`: integer > 0
  - `max_consecutive_failures`: integer > 0

#### `notes`
- freeform string
- ignored by execution logic in Bet 01

### Bet 01 schema example

```yaml
type: codexctl-bet/v1
name: fix-auth-mode-ux
appetite: 6_weeks
objective: >
  Make auth-mode messaging explicit across status, usage, doctor, and setup.
bounded_contexts:
  - Validation
  - Run Orchestration
success_signal: >
  A technical user can run this shaped task unattended and trust the validator results.
no_gos:
  - Do not add queueing.
  - Do not add multi-agent scheduling.
context_files:
  - README.md
  - src/main.rs
constraints:
  - Preserve JSON output stability.
acceptance_checks:
  - cargo fmt --all -- --check
  - cargo test --all-features --all-targets
review_checks: []
agent:
  prompt_preamble: |
    Work only on the stated objective. Keep changes minimal and production-safe.
  command:
    - codex
    - exec
budgets:
  max_iterations: 8
  max_runtime_minutes: 90
  max_consecutive_failures: 3
```

## JSON contract freeze

### Shared rules
- JSON mode emits one final JSON document only
- no ANSI color
- no progress bars
- paths are absolute where produced by the runtime, relative where copied from bet specs
- enum-like fields use snake_case strings
- each JSON root includes `schema_version`

### 1. `validate --json`

Root object:
```json
{
  "schema_version": "validate/v1",
  "command": "validate",
  "status": "passed|failed|timed_out|error",
  "task_path": "string|null",
  "summary": {
    "total_checks": 0,
    "passed": 0,
    "failed": 0,
    "timed_out": 0,
    "errors": 0,
    "duration_ms": 0
  },
  "checks": []
}
```

Check object shape:
```json
{
  "id": "string",
  "kind": "shell",
  "command": "string",
  "cwd": "string",
  "timeout_seconds": 0,
  "status": "passed|failed|timed_out|error",
  "exit_code": 0,
  "duration_ms": 0,
  "stdout_path": "string|null",
  "stderr_path": "string|null"
}
```

### 2. `run-loop --json`

Root object:
```json
{
  "schema_version": "run_loop/v1",
  "command": "run-loop",
  "run_id": "string",
  "status": "succeeded|failed|blocked|cancelled|budget_exhausted|dry_run",
  "stop_reason": "string",
  "task_name": "string",
  "task_path": "string",
  "repo_root": "string",
  "profile": "string|null",
  "auth_mode": "string|null",
  "iteration_count": 0,
  "started_at": "string",
  "updated_at": "string",
  "finished_at": "string|null",
  "latest_validation": {
    "status": "passed|failed|timed_out|error|null",
    "passed": 0,
    "failed": 0,
    "timed_out": 0,
    "errors": 0
  }
}
```

### 3. `runs --json`

List mode root object:
```json
{
  "schema_version": "runs/v1",
  "command": "runs",
  "items": []
}
```

Detail mode root object:
```json
{
  "schema_version": "runs/v1",
  "command": "runs",
  "run": {}
}
```

Run item/detail shared minimum fields:
```json
{
  "run_id": "string",
  "status": "queued|running|succeeded|failed|blocked|cancelled|budget_exhausted",
  "stop_reason": "string|null",
  "task_name": "string",
  "task_path": "string",
  "repo_root": "string",
  "profile": "string|null",
  "auth_mode": "string|null",
  "iteration_count": 0,
  "started_at": "string",
  "updated_at": "string",
  "finished_at": "string|null"
}
```

## Exit-code contract freeze

General rule:
- `0` means the command achieved its intended successful outcome
- `2` remains reserved for CLI usage/argument parsing errors from Clap

Reserved Bet 01 exit codes:
- `10`: validation checks failed
- `11`: validation timed out
- `12`: bet spec invalid
- `13`: bet spec missing required Shape Up / DDD fields
- `20`: run blocked by infrastructure or execution precondition
- `21`: run budget exhausted
- `22`: run cancelled by operator stop signal
- `23`: run state corrupt or resume impossible
- `24`: profile activation or decryption failed
- `25`: agent invocation failed before usable iteration result

Command mapping:
- `validate`
  - success: `0`
  - failing checks: `10`
  - timeout: `11`
  - invalid bet spec: `12` or `13`
- `run-loop`
  - success: `0`
  - blocked: `20`
  - budget exhausted: `21`
  - cancelled: `22`
  - corrupt resume state: `23`
  - profile/decrypt failure: `24`
  - agent invocation failure: `25`
- `runs`
  - success: `0`
  - unknown run id or unreadable run state: `23`

## Minimum repo-state policy for Bet 01

Bet 01 policy is intentionally minimal and conservative.

- non-git repo: allowed
- dirty git repo: allowed
- detached HEAD: allowed
- worktree isolation: not supported

Required behavior in all three allowed cases:
- emit a warning in human mode when repository state reduces safety or traceability
- include `repo_root` in run state
- do not mutate git state automatically to compensate

Stronger policy enforcement belongs to Bet 02.

## Implementation notes frozen for Bet 01

- shell checks run through `bash -lc` on Unix-like systems for Bet 01
- `review_checks` are parsed but not executed in Bet 01
- `validate --json`, `run-loop --json`, and `runs --json` are required from first implementation slice
- `runs --tail` stays human-only in Bet 01
- future bet-authoring commands must reuse `codexctl-bet/v1` rather than define a second competing spec

## Approval rule

Bet 01 implementation may proceed only if code changes remain inside these frozen contracts.
