# Bet 03: Operator Feedback and Review Gates

Status: Finalized
Appetite: 2 weeks
Depends on: Bet 02
Build approval: Approved after Bet 02 completion review

## Problem

Even a correct loop kernel can feel opaque. Operators need better signals at the moments that matter and a clearer final trust gate.

For the target user, low-attention supervision is part of the product value. They want to execute broadly without watching every step.

## Success signal

A user can understand run outcomes quickly, receive minimal but useful alerts, and distinguish implementation success from final review approval.

## Bounded contexts touched

Primary:
- `Run Ledger`
- `Run Orchestration`
- `Validation`

## Shaped solution

Deliver:
- `review_checks` as a distinct post-implementation phase
- `--notify-cmd` with a small stable payload
- stronger final reports and run summaries
- product-building summaries that explain what changed, why the run stopped, and what remains

## Freeze

This bet is frozen for implementation after Bet 02 completes.

Change control:
- review remains deterministic in this bet
- provider-specific integrations remain out of scope

## No-gos

Do not include:
- Telegram/Discord-specific connectors
- multi-agent review teams
- natural-language-only pass/fail logic without explicit command hooks
- broad notification platform support

## Rabbit holes

- building an alerting platform instead of one generic notification hook
- blending review and acceptance checks into one opaque state
- using the same implementation prompt as the review mechanism

## Delivery slices

### Slice 1: review phase separation
- review checks have their own phase and result model
- final success requires both acceptance and review success when configured

### Slice 2: notify command
- shell command hook for started, blocked, failed, succeeded
- compact JSON payload

### Slice 3: report polish
- final report with task, iterations, failures, and stop reason
- improved `runs` human output
- outcome language tied to the bet success signal where possible

## Pre-build checklist

- define review state transitions before coding
- define hook payload schema before execution wiring
- define hook failure policy before implementation

## Post-build checklist

- review failure is clearly distinct from implementation failure
- hook failure cannot silently mask run success/failure
- human summaries remain concise and technical
- final reports are useful to someone managing many product-building runs with limited attention

## Fact-check / validation checklist

- hook behavior is compatible with unattended shell usage
- review model stays deterministic unless explicitly expanded later
- operator output remains aligned with the ubiquitous language

## Acceptance test matrix

Required automated coverage:
- review phase success path
- review phase failure path
- review skipped when empty
- notify hook payload shape
- notify hook failure handling
- final report generation
- `runs` output reflecting review state distinctly from implementation state
