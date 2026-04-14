# Codex Usage Analytics Parity Notes (ccusage -> codexctl)

Date: 2026-04-11 (UTC)
Status: Deferred for later implementation
Scope: Capture what `@ccusage/codex` does for usage analytics and what `codexctl` should adopt.

## Why this note exists

`codexctl` currently focuses on:
- profile/auth metadata parsing from `auth.json` JWT claims
- optional quota calls via legacy dashboard billing endpoints
- plan/subscription display and profile switching heuristics

This does not provide true token-level session analytics, model-level cost tracking, or daily/monthly/session rollups.

## Verified `ccusage` behavior (source-inspected)

### 1) Session log ingestion model
- Reads session JSONL files from `CODEX_HOME` (default `~/.codex/sessions`).
- Loads all matching session files and iterates line-by-line.
- Parses `turn_context` and `event_msg` entries, specifically `token_count` payloads.

### 2) Token normalization and delta logic
- Handles both `last_token_usage` and `total_token_usage`.
- If only cumulative totals are present, computes per-event delta by subtracting previous totals.
- Normalizes fields:
  - `input_tokens`
  - `cached_input_tokens` (and legacy alias)
  - `output_tokens`
  - `reasoning_output_tokens`
  - `total_tokens` (fallback synthesized when missing)

### 3) Billing semantics
- Does **not** double-charge reasoning tokens:
  - `reasoning_output_tokens` treated as informational only.
  - billable output uses `output_tokens`.
- Separates cached input from non-cached input.
- Applies pricing for:
  - non-cached input
  - cached input
  - output

### 4) Model detection and fallback
- Extracts model from context/payload metadata.
- Tracks current model across entries.
- Applies legacy fallback model when metadata is missing.
- Marks fallback usage in event/report outputs (`isFallback`) so approximation is explicit.

### 5) Reporting and UX
- Produces grouped reports:
  - daily
  - monthly
  - session
- Supports filters:
  - `--since`
  - `--until`
- Supports `--json`, timezone, locale, and compact table mode.
- Includes per-model aggregation and estimated cost totals.

### 6) Pricing source
- Uses LiteLLM-based model pricing source.
- Includes alias mapping for model names.
- Supports offline pricing mode/cache and graceful fallback when pricing unavailable.

## Current `codexctl` gap summary

Current implementation (high-level):
- `src/commands/usage.rs`: display usage from auth claims + optional realtime API fetch.
- `src/utils/auth.rs`: parse JWT custom auth claims for plan/subscription/account/org data.
- `src/utils/api.rs`: call `/v1/dashboard/billing/subscription` and `/v1/dashboard/billing/usage`.

Gaps vs product intent ("full end-to-end profile management + usage visibility"):
- No session JSONL parser for actual Codex token usage.
- No daily/monthly/session token rollups.
- No per-model usage/cost breakdown.
- No explicit fallback annotations for inferred model pricing.
- Reliance on dashboard billing endpoints may be brittle over time.

## Proposed implementation plan for codexctl (deferred)

### Phase 1: Core usage data model
1. Add Rust types for token usage event + deltas:
   - `input_tokens`
   - `cached_input_tokens`
   - `output_tokens`
   - `reasoning_output_tokens`
   - `total_tokens`
   - `model`
   - `is_fallback_model`
   - `session_id`
   - `timestamp`
2. Add parser for session JSONL files:
   - default path from `CODEX_HOME` or profile session dirs
   - robust handling of malformed lines
   - cumulative->delta conversion

### Phase 2: Aggregation/reporting
1. Add `usage daily`, `usage monthly`, `usage sessions` subcommands.
2. Add filters:
   - `--since YYYY-MM-DD`
   - `--until YYYY-MM-DD`
3. Add output modes:
   - table (human)
   - `--json` (automation/integration)

### Phase 3: Pricing and cost estimation
1. Add pricing module (model lookup + alias mapping).
2. Add cached/offline pricing source.
3. Implement cost formula:
   - `(non_cached_input * input_price)`
   - `(cached_input * cached_input_price)`
   - `(output * output_price)`
4. Keep fallback pricing warnings explicit in report output.

### Phase 4: Multi-profile integration
1. Allow analytics per selected profile and all profiles.
2. Add profile-aware score that can use real usage metrics (not only plan/date heuristics).
3. Keep existing JWT subscription view as a separate mode:
   - `usage plan` (or similar)
   - analytics under `usage report|daily|monthly|sessions`

## Acceptance criteria for future implementation

- Correct delta computation from cumulative logs across mixed event shapes.
- No double counting of reasoning tokens.
- Correct cached-input treatment in totals and cost.
- Deterministic date grouping under timezone option.
- JSON output stable enough for scripting.
- Clear fallback markers when model/pricing is inferred.
- Unit tests covering:
  - legacy and modern token payload formats
  - model extraction fallback
  - aggregation correctness
  - pricing calculation correctness

## Source references used

- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/data-loader.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/token-utils.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/pricing.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/daily-report.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/monthly-report.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/src/session-report.ts
- https://github.com/ryoppippi/ccusage/blob/main/apps/codex/README.md

