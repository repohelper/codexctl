# Cycle 01 Finalized Bets

Date: 2026-04-14 (UTC)
Cycle model: Shape Up 6-week cycle + cooldown
Status: Finalized

## Primary user and job

Primary user:
- an individual builder or high-agency operator using AI agents as a primary execution force

Core job:
- shape product work into bounded bets
- run those bets through AI agents safely
- leave runs unattended when useful
- come back to clear outcomes, failures, and next actions

## Betting principle

We should not queue many implementation streams at once.

Recommended order:
1. Bet 1: Loop Kernel Foundations
2. Bet 2: Safe Unattended Execution
3. Bet 3: Operator Feedback and Review Gates

This aligns with Shape Up because:
- the first bet creates the core domain model and the minimum valuable unattended workflow
- the second bet hardens it for real operator use
- the third bet improves trust and ergonomics without expanding the core domain too early

## Finalized bets

### Bet 1: Loop Kernel Foundations
Appetite:
- 6 weeks
Status:
- Finalized
Build:
- Approved

Outcome:
- `codexctl` can execute one shaped bet/task unattended with deterministic checks, persisted run state, resume support, and inspection.

Includes:
- bet spec v1
- bet authoring scaffolding direction (`bet init`, `bet lint`) defined, even if not shipped in this bet
- `validate`
- `run-loop`
- `runs`
- JSON output and exit-code contract for the new commands

Excludes:
- worktree isolation
- notifications
- agent review phase
- queueing
- multi-agent dispatch

### Bet 2: Safe Unattended Execution
Appetite:
- 2 to 3 weeks
Status:
- Finalized
Build:
- Approved after Bet 1

Outcome:
- unattended runs are safer in real repos and more robust under failure/restart conditions

Includes:
- stop file and cancellation hardening
- profile-aware safety checks
- optional worktree isolation
- better resume and crash recovery
- explicit operator warnings and guardrails
- repo-state policy hardening for dirty repos / non-git repos / detached HEAD

Excludes:
- queueing
- chat integrations
- project management abstractions

### Bet 3: Operator Feedback and Review Gates
Appetite:
- 2 weeks
Status:
- Finalized
Build:
- Approved after Bet 2

Outcome:
- unattended runs become easier to trust and monitor without broadening the domain too much

Includes:
- `review_checks`
- `--notify-cmd`
- better human summaries and final reports
- improved inspection/tail UX

Excludes:
- Telegram/Discord-specific integrations
- agent team orchestration

## Recommended first bet

Start with Bet 1.

Reason:
- It defines the new bounded contexts properly.
- It creates the minimum useful product surface.
- It keeps scope tight enough to follow appetite discipline.
- It keeps the consumer-facing schema opinionated around Shape Up and DDD instead of opening a generic workflow surface.

## Deferred, not rejected

The following are strategically important for this audience but intentionally deferred:
- multi-bet sequencing / queueing
- bet templates and linting as first-class CLI commands
- stronger repo isolation defaults

They are deferred to protect appetite, not because they lack product value.

## Execution rule

Implementation should proceed in bet order.

If a build-phase discovery requires new scope, reopen shaping instead of mutating the finalized bet in place.
