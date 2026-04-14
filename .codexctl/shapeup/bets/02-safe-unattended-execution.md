# Bet 02: Safe Unattended Execution

Status: Finalized
Appetite: 2 to 3 weeks
Depends on: Bet 01
Build approval: Approved after Bet 01 completion review

## Problem

A loop kernel without stronger operational guardrails is usable in controlled conditions but risky in real repositories.

We need to harden unattended execution so technical users trust it in non-trivial repos.

For the target user, this is the difference between an interesting demo and something they will actually leave running while building product features.

## Success signal

A user can run unattended work with clearer safety boundaries, stronger crash recovery, and cleaner isolation from their primary checkout.

## Bounded contexts touched

Primary:
- `Run Orchestration`
- `Run Ledger`

Secondary:
- `Live Auth Projection`
- optional git/worktree utility surface

## Shaped solution

Deliver:
- explicit stop-file and cancellation handling
- better crash-recovery metadata
- stronger profile-run safety checks
- optional worktree isolation if it fits appetite
- explicit repo-state policy for:
  - dirty working tree
  - non-git repo
  - detached HEAD
  - long-running validators

## Freeze

This bet is frozen for implementation after Bet 01 completes.

Change control:
- worktree support is the first thing to cut if appetite is threatened
- no new execution surfaces should be added during implementation

## No-gos

Do not include:
- queueing
- notifications beyond generic hook plumbing if already present
- project manager agent ideas
- planner/reviewer multi-agent flows
- silent mutation of git state to “make things work”

## Rabbit holes

- turning worktree support into a full branch-management system
- auto-cleaning worktrees in surprising ways
- adding policy that belongs in the task spec instead of run state

## Delivery slices

### Slice 1: stop/cancel hardening
- `.stop` handling
- explicit stop reasons
- predictable cancellation state transitions

### Slice 2: crash recovery
- recovery of partially written iteration artifacts
- run-state integrity checks
- clearer blocked vs failed classification

### Slice 3: profile safety
- clearer warnings for `--profile`
- encrypted profile handling within unattended flow
- restore guarantees under abnormal exits

### Slice 4: optional worktree mode
- only if the first three slices fit appetite cleanly
- opt-in only
- explicit lifecycle and no implicit deletion

### Slice 5: repo-state policy
- define and enforce behavior for dirty repo, non-git repo, detached HEAD, and slow validation cases

## Pre-build checklist

- confirm whether worktree isolation is in or out before coding starts
- define cancellation state transitions before implementation
- define run-state integrity checks before recovery logic is written
- freeze repo-state policy before implementation

## Post-build checklist

- interrupted and cancelled runs are distinguishable
- profile restore guarantees are tested under failure paths
- worktree mode, if shipped, cannot silently destroy user state
- operator output makes the safety model explicit
- repo-state handling is explicit, tested, and documented in CLI help

## Fact-check / validation checklist

- worktree design matches git behavior across supported platforms
- no added behavior violates current non-destructive git policy
- stop and recovery semantics remain scriptable and JSON-stable
- repo-state policy remains aligned with the needs of high-agency product builders rather than generic CI assumptions

## Acceptance test matrix

Required automated coverage:
- stop-file detection
- cancelled vs failed state distinction
- profile restore under abnormal exit
- dirty repo behavior
- non-git repo behavior
- detached HEAD behavior
- slow validator timeout behavior
- worktree mode behavior, if shipped
