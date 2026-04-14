# Domain Map

Date: 2026-04-14 (UTC)
Status: Working draft

## Ubiquitous language

Core terms:
- `profile`: a saved auth identity snapshot managed by `codexctl`
- `live auth`: the currently active `auth.json` in the Codex home directory
- `task`: a versioned repo-local spec for a shaped unattended objective
- `bet spec`: the opinionated task-spec shape used by `codexctl`, derived from Shape Up and DDD concepts
- `bet template`: a starter bet/task spec that helps users author valid shaped work quickly
- `bet lint`: validation that checks whether a bet spec is structurally and methodologically sound
- `acceptance check`: a deterministic validator that gates completion
- `review check`: a secondary validator phase run after implementation checks
- `run`: one persisted unattended execution instance
- `iteration`: one loop pass within a run
- `run ledger`: the persisted state, events, and artifacts for a run
- `validator`: the subsystem that executes acceptance and review checks
- `bet`: a shaped delivery unit selected for a cycle
- `appetite`: the fixed amount of time we are willing to spend

## Bounded contexts

### 1. Profile Catalog
Purpose:
- save, list, load, delete, diff, and describe saved profiles

Current code:
- `src/commands/save.rs`
- `src/commands/list.rs`
- `src/commands/load.rs`
- `src/commands/delete.rs`
- `src/commands/diff.rs`
- `src/utils/profile.rs`

Domain objects:
- `Profile`
- `ProfileMeta`
- `ProfileName`

### 2. Live Auth Projection
Purpose:
- apply a selected profile's auth material to the live Codex environment without replacing sessions/history/state

Current code:
- `src/commands/load.rs`
- `src/commands/run.rs`
- `src/utils/files.rs`
- `src/utils/crypto.rs`

Domain objects:
- `LiveAuth`
- `AuthSnapshot`
- `AuthMode`

### 3. Usage Intelligence
Purpose:
- interpret auth claims and realtime API quota into human and JSON usage views

Current code:
- `src/commands/usage.rs`
- `src/commands/status.rs`
- `src/commands/verify.rs`
- `src/commands/doctor.rs`
- `src/utils/auth.rs`
- `src/utils/api.rs`

Domain objects:
- `UsageInfo`
- `QuotaInfo`
- `AuthMode`

### 4. Validation
Purpose:
- execute deterministic checks and produce a stable result model

Future code:
- `src/commands/validate.rs`
- `src/utils/validate.rs`

Planned domain objects:
- `ValidationCheck`
- `ValidationResult`
- `ValidationSummary`

### 5. Task Definition
Purpose:
- define versioned bet specs that describe shaped unattended objectives, appetite, constraints, bounded contexts, and checks

Future code:
- `src/utils/task.rs`
- future commands: `bet init`, `bet lint`

Planned domain objects:
- `TaskSpec`
- `TaskBudget`
- `TaskConstraint`
- `BetSpec`

### 6. Run Orchestration
Purpose:
- drive iterations, invoke the agent, interpret validator results, and decide whether to continue or stop

Future code:
- `src/commands/run_loop.rs`
- `src/utils/runs.rs`

Planned domain objects:
- `Run`
- `Iteration`
- `StopReason`
- `RunStatus`

### 7. Run Ledger
Purpose:
- persist run metadata, events, iteration artifacts, and summaries for inspection and resume

Future code:
- `src/commands/runs.rs`
- `src/utils/runs.rs`

Planned domain objects:
- `RunRecord`
- `RunEvent`
- `IterationArtifact`

### 8. Release Engineering
Purpose:
- package, publish, audit, and release the CLI across cargo, npm, Docker, and GitHub releases

Current code and config:
- `.github/workflows/*`
- `Cargo.toml`
- `npm/package.json`
- `npm-platforms/*/package.json`

## Context map

Primary upstream/downstream relationships:
- `Task Definition` -> `Validation`
- `Task Definition` -> `Run Orchestration`
- `Validation` -> `Run Orchestration`
- `Run Orchestration` -> `Run Ledger`
- `Profile Catalog` -> `Live Auth Projection`
- `Live Auth Projection` -> `Run Orchestration` when `--profile` is used

Boundary rules:
- `Validation` must not know about profile persistence details.
- `Run Ledger` stores facts about runs; it does not decide policy.
- `Run Orchestration` decides policy; it should not own shell execution primitives directly if those can live in `Validation` or shared command utilities.
- `Usage Intelligence` stays separate from unattended execution features.
- `Task Definition` is intentionally opinionated; it models shaped bets and scoped implementation tasks, not arbitrary orchestration graphs or generic workflow engines.
- `Task Definition` must help users author good specs quickly; strictness without scaffolding is a product failure for this audience.

## Aggregate guidance

Recommended aggregate roots for new work:
- `TaskSpec` for task definition
- `RunRecord` for persisted run state
- `ValidationResult` for one validator execution set

Do not create one giant automation aggregate.

## Standards from this domain map

- one command should primarily belong to one bounded context
- cross-context flows should happen through explicit types, not loose JSON blobs in process
- CLI help text should use the ubiquitous language defined here
- if a new feature does not fit an existing context, define the boundary before coding
- future bet commands should optimize for fast authoring and strong feedback, not bureaucratic ceremony
