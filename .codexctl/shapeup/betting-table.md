# Betting Table

Date: 2026-04-14 (UTC)
Status: Finalized

This is the implementation order and freeze reference for the unattended-execution workstream.

## Product user

This workstream targets:
- individuals and other high-agency operators
- using AI agents to build products with high leverage
- seeking maximum value from AI across execution depth, unattended duration, and feature breadth

## Portfolio

| Bet | Name | Appetite | Depends On | Build Approval | Outcome |
|---|---|---:|---|---|---|
| 01 | Loop Kernel Foundations | 6 weeks | None | Approved | First production-ready vertical slice for bet spec, validation, run-loop, and run ledger |
| 02 | Safe Unattended Execution | 2-3 weeks | 01 | Approved after Bet 01 review | Hardens stop/recovery/profile safety and optionally adds worktree mode if appetite allows |
| 03 | Operator Feedback and Review Gates | 2 weeks | 02 | Approved after Bet 02 review | Adds distinct review checks, generic notifications, and stronger operator-facing summaries |

## Contract references

- Bet 01 contract freeze: `.codexctl/shapeup/contracts/bet-01-contract-freeze.md`

## Deferred capabilities

Important but deferred:
- multi-bet sequencing / queueing
- `bet init`
- `bet lint`
- opinionated bet templates

These are not rejected. They are future leverage multipliers for the target user.

## Frozen sequence

Implementation order:
1. Bet 01
2. Bet 02
3. Bet 03

No parallel bet implementation unless we explicitly reopen shaping.

## Global exclusions

The following are excluded from this finalized bet set:
- multi-task queueing
- multi-agent orchestration
- provider-specific notification integrations
- generic workflow DSL support
- autonomous commit/push/branch cleanup behavior
- project-manager-agent features

## Gate between bets

Before starting the next bet:
- previous bet acceptance criteria must be met
- previous bet post-build checklist must be complete
- previous bet fact-check checklist must still hold
- any discoveries requiring scope expansion must go through reshaping

## Interpretation rule

This is intentionally front-loaded and plan-driven, but not blind waterfall.

Meaning:
- we freeze bets before implementation
- we implement against frozen scope
- if reality invalidates the shape, we stop and reshape instead of silently drifting
