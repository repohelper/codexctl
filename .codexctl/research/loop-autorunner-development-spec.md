# CodexCTL Loop / Autorunner Internal Development Spec

Date: 2026-04-13 (UTC)
Status: Proposed for internal planning only
Commit status: Do not commit yet
Owner: codexctl core

## Why this note exists

We want to evaluate whether `codexctl` should adopt Ralph-style unattended execution and, if so, how to do it without turning the product into a large orchestration framework.

This note separates:
- fact-checked external research
- local codebase constraints
- brainstormed feature scope
- product decisions
- implementation-ready development specs
- pre/post implementation checklists
- validation and fact-check criteria

## Executive summary

Recommendation:
- Build a narrow execution loop layer for `codexctl`, not a general-purpose CAR clone.
- Prioritize deterministic validation and resumable run state before any scheduler, notification fan-out, or multi-agent orchestration.
- Keep the consumer-facing task schema opinionated around Shape Up and DDD instead of supporting a broad spec surface.
- Ship in phases:
  - P0: `validate`, task spec format, `run-loop`, run-state inspection
  - P1: notification hooks, independent review pass, worktree isolation
  - P2: queueing, multi-task scheduling, optional agent dispatch abstractions

Reason:
- Ralph works because it is simple, file-based, and validator-driven.
- CAR adds useful ideas, but most of its value comes from explicit state, tickets, and unattended operation, not from sheer feature count.
- `codexctl` should borrow the narrow, high-signal pieces and avoid building an orchestration platform too early.

## Fact-checked external research

### Verified concepts

#### 1. Ralph is fundamentally an outer retry loop
Verified from Geoffrey Huntley’s write-up and the `vercel-labs/ralph-loop-agent` README:
- Ralph is conceptually “a Bash loop”.
- The agent is re-run with fresh context.
- Completion is decided by an external verifier, not by trusting one model pass.
- Failed verification becomes feedback for the next iteration.

Implication for `codexctl`:
- The core product requirement is not “many agents”; it is “repeat until validated, with fresh context and durable state.”

#### 2. Filesystem state is the durable memory plane
Verified from Ralph write-ups and CAR descriptions:
- Plans, prompts, tickets, logs, and repo files act as persistent memory.
- Long-lived chat context is treated as unreliable or expensive.

Implication for `codexctl`:
- All loop state must live on disk under a deterministic run directory.

#### 3. External validation is the stop condition
Verified from Ralph and Vercel’s `verifyCompletion` model:
- Tests, lint, build, assertions, or independent review gates determine stop/continue.
- “The agent said it is done” is not sufficient.

Implication for `codexctl`:
- `validate` is foundational. It is not optional polish.

#### 4. CAR is a ticket-driven state machine, not just a shell loop
Verified from the current `codex-autorunner` description on PyPI and public summaries:
- CAR is a state machine over incomplete markdown tickets.
- Tickets are the control plane.
- Agents are the execution layer.
- CAR supports unattended operation, project contextspaces, and agent handoffs.

Implication for `codexctl`:
- If we go beyond single-task loop mode, tickets/task files are the correct abstraction.
- We should still avoid prematurely adopting CAR-scale scope.

#### 5. Community feedback converges on one pattern
Cross-source synthesis from GitHub repos and community discussions:
- Loops work best when the problem is already well-scoped.
- Weak prompts/specs cause repeated low-quality iterations.
- Teams/subagents help decomposition and parallelism, but do not replace durable run state.
- The strongest pattern is often “outer loop + validator + optional reviewer,” not “just add more agents.”

Implication for `codexctl`:
- The product should optimize for scoped, validator-backed tasks first.

### Source references used

Primary sources:
- Geoffrey Huntley: https://ghuntley.com/ralph/
- `ghuntley/how-to-ralph-wiggum`: https://github.com/ghuntley/how-to-ralph-wiggum
- `vercel-labs/ralph-loop-agent`: https://github.com/vercel-labs/ralph-loop-agent
- `iannuttall/ralph`: https://github.com/iannuttall/ralph
- `codex-autorunner` public package/project description: https://pypi.org/project/codex-autorunner/
- CAR event summary: https://sf.aitinkerers.org/talks/rsvp_U8eNYBDAmok
- Claude Code hooks docs: https://code.claude.com/docs/en/hooks
- Claude Code hooks guide: https://code.claude.com/docs/en/hooks-guide

Community discussion references:
- Reddit: Ralph loops vs subagents/teams discussion
  - https://www.reddit.com/r/ClaudeCode/comments/1r5ticb/cmv_ralph_loops_are_no_longer_needed_with/
  - https://www.reddit.com/r/ClaudeAI/comments/1qxy1qk/agent_teams_completely_replaces_ralph_loops/
  - https://www.reddit.com/r/ClaudeCode/comments/1q2qvta/share_your_honest_and_thoughtful_review_of_ralph/
  - https://www.reddit.com/r/GithubCopilot/comments/1ro7q1y/ralph_wiggum_hype_deflated/
  - https://www.reddit.com/r/codex/comments/1sbl3n9/improving_ralph_wiggum/

### Fact-check caveats

- GitHub star counts, release counts, and fast-moving community sentiment are time-sensitive and should not be hard-coded into product behavior.
- Community posts are useful for failure modes and operator experience, not for protocol definitions.
- CAR details were validated through public project/package descriptions, not a full source audit of its entire codebase.

## Local codebase constraints

Current local architecture relevant to this feature:
- CLI is a flat command model in [src/main.rs](/home/azuredhruvauser/codexo-public/src/main.rs).
- Persistent app data is managed through `Config` in [src/utils/config.rs](/home/azuredhruvauser/codexo-public/src/utils/config.rs).
- Current state is profile-centric (`profiles_dir`, `backup_dir`, live `codex_dir`).
- Existing user-facing commands already expose structured output in some areas (`status`, `usage`, `verify`, `doctor`).
- The product recently moved to auth-only switching, which means the loop feature must not regress the current separation between auth state and Codex’s own session/history state.

Hard constraints:
- Any unattended mode must be explicit and opt-in.
- Any new loop feature must be resumable from disk.
- Default mode must remain safe and understandable for technical users.
- JSON output must be available for any long-running automation surface.
- No implicit commits, rebases, branch deletes, or destructive git actions by default.

## Product goals

Primary goals:
- Allow a technical user to run a single well-scoped task unattended with external validation.
- Make loop state inspectable and scriptable.
- Keep the system deterministic enough to debug after a failed overnight run.
- Preserve `codexctl` as a developer utility, not a heavy orchestration platform.
- Encourage `codexctl` consumers to express work as shaped bets and bounded-context tasks.
- Optimize for high-agency product builders who want to use AI agents across execution depth, unattended duration, and feature breadth.

Non-goals for the initial feature set:
- Multi-repo orchestration
- Chat/Slack/Telegram control plane on day one
- Autonomous project management / ticket generation in MVP
- Complex agent marketplace abstractions
- Dynamic branch or worktree fan-out in MVP
- Full CAR parity

Strategically important but deferred:
- multi-bet sequencing / queueing
- `bet init`
- `bet lint`
- template scaffolding for common product bet shapes

## Brainstormed scope catalogue

### Scope A: Validation engine
Summary:
- Run deterministic checks as reusable validation steps.

Examples:
- `cargo test --all-features --all-targets`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `npm test`
- file assertions
- grep assertions
- custom shell checks

Value:
- Highest. This is the trust boundary.

Complexity:
- Moderate.

Risks:
- Command execution policy must be explicit.
- Need stable result model and timeouts.

Recommendation:
- P0, must build first.

### Scope B: Task spec format
Summary:
- A file that defines a shaped objective, appetite, bounded contexts, constraints, and validation references.

Value:
- High. Makes loops resumable and reviewable.

Complexity:
- Low to moderate.

Risks:
- Overdesigning the schema or broadening it into a generic workflow language.

Recommendation:
- P0.
- Start with a simple YAML format and keep it explicitly opinionated toward Shape Up + DDD.

### Scope C: Single-task outer loop
Summary:
- Re-run one task until validation passes, a budget is exhausted, or a human stops it.

Value:
- Highest visible product feature.

Complexity:
- Moderate.

Risks:
- Requires robust state persistence, validator wiring, and failure feedback handling.

Recommendation:
- P0 after `validate`.

### Scope D: Run-state inspection
Summary:
- Inspect current and previous run state, validator history, and stop reason.

Value:
- Very high for debugging and operator trust.

Complexity:
- Low.

Risks:
- State schema churn if done too early without versioning.

Recommendation:
- P0.

### Scope E: Notification hooks
Summary:
- Trigger a shell command or webhook on complete, blocked, or failed status.

Value:
- Medium.

Complexity:
- Low.

Risks:
- Easy to let this sprawl into a chat-platform matrix.

Recommendation:
- P1.
- Only generic shell/webhook hook in first pass.

### Scope F: Independent review pass
Summary:
- Run a second review step after implementation and before final success.

Value:
- High.

Complexity:
- Moderate.

Risks:
- Can duplicate validation if not clearly separated.

Recommendation:
- P1.
- Keep deterministic review first; agent review only if clearly bounded.

### Scope G: Worktree isolation
Summary:
- Run loop in a dedicated git worktree to reduce damage to the main checkout.

Value:
- High for unattended use.

Complexity:
- Moderate.

Risks:
- Worktree lifecycle complexity; git edge cases.

Recommendation:
- P1 or P2.
- Valuable, but not required for MVP if loop clearly targets current checkout.

### Scope H: Multi-task queue
Summary:
- Run a sequence of tasks/tickets unattended.

Value:
- Medium to high.

Complexity:
- High.

Risks:
- Scheduler complexity, failure ordering, task dependency modeling.

Recommendation:
- P2.

### Scope I: Multi-agent dispatch
Summary:
- Separate planner, implementer, reviewer, or PM agents.

Value:
- Medium.

Complexity:
- High.

Risks:
- Too much product surface too early.

Recommendation:
- Explicitly out of MVP.

### Scope J: Chat-platform integrations
Summary:
- Telegram/Discord/etc notifications and control.

Value:
- Low for MVP.

Complexity:
- High.

Risks:
- Connector sprawl.

Recommendation:
- Defer.

## Product decision

Chosen direction:
- Build an internal “loop kernel” for `codexctl`.
- Treat validation and persisted run state as the product core.
- Build exactly one unattended execution path first: one task, one repo, one run state, explicit checks.
- Make the user-facing task format a narrow “bet spec” rather than an open-ended automation spec.

Do not build yet:
- Ticket orchestration framework
- Multi-agent scheduler
- CAR-like hub model
- Chat platform control plane

## Proposed command surface

Recommended command surface for MVP:
- `codexctl validate`
- `codexctl run-loop`
- `codexctl runs`

Alternative considered:
- `codexctl loop run`, `codexctl loop status`, `codexctl loop resume`

Decision:
- Prefer top-level commands first because the CLI already uses flat commands and technical users benefit more from immediate discoverability than deep command trees.

### Command: `codexctl validate`

Purpose:
- Run one or more validation checks without looping.

Examples:
```bash
codexctl validate \
  --check "cargo test --all-features --all-targets" \
  --check "cargo clippy --all-targets --all-features -- -D warnings"

codexctl validate --task .codexctl/tasks/fix-auth-mode.yaml
```

Flags:
- `--task <path>`: load checks from shaped bet/task file
- `--check <command>`: repeated shell commands
- `--timeout-seconds <n>`: default per-check timeout
- `--json`: structured output
- `--fail-fast`: stop on first failing check
- `--cwd <path>`: override working directory

Output:
- human summary table by default
- machine-readable JSON with per-check results

### Command: `codexctl run-loop`

Purpose:
- Run an unattended outer loop for one task until validators pass or budgets stop it.

Examples:
```bash
codexctl run-loop \
  --task .codexctl/tasks/fix-auth-mode.yaml \
  --max-iterations 8 \
  --timeout-minutes 90 \
  --json
```

Flags:
- `--task <path>`: required shaped bet/task spec
- `--max-iterations <n>`
- `--timeout-minutes <n>`
- `--max-consecutive-failures <n>`
- `--resume [run-id]`
- `--profile <name>`: optional auth profile for the run
- `--passphrase <value>`: if encrypted profile is used
- `--json`
- `--dry-run`: render state/actions without invoking agent
- `--no-review`: skip optional review phase if configured
- `--notify-cmd <command>`: P1 only

Behavior:
- Create a run directory.
- Snapshot task spec into the run.
- Execute the agent with a generated iteration prompt.
- Run validators.
- If validation passes, mark success.
- If validation fails, write feedback into next iteration context.
- Stop on budgets, repeated failures, explicit stop file, or success.

### Command: `codexctl runs`

Purpose:
- Inspect previous and active runs.

Examples:
```bash
codexctl runs
codexctl runs --latest
codexctl runs --id 20260413T120000Z-abc123 --json
```

Flags:
- `--latest`
- `--id <run-id>`
- `--json`
- `--tail`: show latest log/events

Output:
- current status
- iteration count
- validator summary
- stop reason
- task path
- timestamps

## Bet spec format

Recommended path:
- `.codexctl/tasks/<name>.yaml`

Recommended schema v1:
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
  - src/commands/status.rs
  - src/commands/usage.rs
constraints:
  - Do not change profile switching semantics.
  - Preserve JSON output stability.
acceptance_checks:
  - cargo fmt --all -- --check
  - cargo test --all-features --all-targets
  - cargo clippy --all-targets --all-features -- -D warnings
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

Schema design rules:
- Keep it human-editable.
- Prefer explicit shell strings for checks over a custom DSL in v1.
- Include a `type`/version header from day one.
- Keep optional freeform markdown notes out of the core schema.
- Require Shape Up and DDD concepts in the schema from v1.
- Do not support arbitrary step graphs, branching workflows, or provider-specific task semantics.

## Run-state filesystem layout

Recommended path:
- `$CODEXCTL_DIR/runs/<run-id>/`

Recommended contents:
- `task.snapshot.yaml`
- `run.json`
- `events.jsonl`
- `iterations/001.prompt.md`
- `iterations/001.summary.md`
- `iterations/001.validation.json`
- `iterations/002.prompt.md`
- `iterations/002.summary.md`
- `iterations/002.validation.json`
- `final-report.md`
- `logs/agent.stdout.log`
- `logs/agent.stderr.log`

Required run metadata fields in `run.json`:
- `run_id`
- `schema_version`
- `status` (`queued`, `running`, `blocked`, `failed`, `succeeded`, `cancelled`)
- `task_name`
- `task_path`
- `repo_root`
- `started_at`
- `updated_at`
- `finished_at`
- `iteration_count`
- `stop_reason`
- `profile`
- `auth_mode`
- `last_validation_result`

Event model in `events.jsonl`:
- `timestamp`
- `event_type`
- `iteration`
- `message`
- `payload`

## Validator execution model

Validator categories for v1:
- shell command checks
- file existence checks
- grep/assertion checks

Decision:
- Implement shell command checks first.
- Add file/assertion helpers only if they simplify common usage.

Execution semantics:
- Each check has:
  - id
  - command
  - cwd
  - timeout
  - exit code
  - duration_ms
  - stdout_path
  - stderr_path
  - passed
- Default behavior is run all checks and aggregate failures.
- `--fail-fast` is opt-in.

Validation feedback shaping:
- Feed only high-signal failure summary to the next loop iteration.
- Do not dump full logs back into the prompt.
- Cap feedback size.

## Loop execution model

Per iteration:
1. Load run state.
2. Generate iteration prompt from:
   - task objective
   - constraints
   - previous iteration summary
   - validator failures
3. Invoke agent command.
4. Capture stdout/stderr and timestamps.
5. Write iteration summary.
6. Run validators.
7. Decide success, continue, block, or fail.
8. Persist event records.

Stop conditions:
- all required acceptance checks pass
- max iterations reached
- max runtime exceeded
- max consecutive failures reached
- `.stop` file present in run directory
- manual cancellation command added later

Blocked conditions:
- missing required file/context
- invalid task schema
- validator command not executable
- repeated infrastructure errors

## Review phase design

MVP decision:
- Support `review_checks` in the task schema, but treat them as optional and deterministic.
- Do not add agent-review in v1.

P1 extension:
- Optional independent review command or review prompt step after acceptance checks pass.
- Review must be separate from implementation prompt and state.

## Notification design

MVP decision:
- Do not implement platform-specific notifications.

P1 design:
- `--notify-cmd <shell command>` on `run-loop`
- Trigger on:
  - `started`
  - `blocked`
  - `failed`
  - `succeeded`
- Pass a small JSON payload by stdin or temp file path.

## Worktree isolation design

Recommendation:
- Do not couple worktrees to MVP.
- Design for future compatibility.

P1/P2 extension:
- `--worktree` flag that creates/uses a dedicated worktree for the run.
- Store the worktree path in `run.json`.
- Clean-up should always be explicit, never implicit.

## Security and safety requirements

Required:
- No destructive git commands by default.
- No implicit push or commit.
- Explicit command logging for every validator and every agent invocation.
- Timeout enforcement for validator commands.
- Stable state writes using atomic file replacement where relevant.
- No silent overwrite of existing run directories.
- Respect current auth-only switching model when `--profile` is used.

Required operator clarity:
- Human output must state:
  - active task
  - run id
  - current iteration
  - validation status
  - stop reason
- JSON output must be stable enough for CI tooling.

## Detailed development specs

### Spec 1: `validate`

Goal:
- Provide a standalone validation engine that later becomes the trust boundary for unattended loops.

Implementation scope:
- new command module: `src/commands/validate.rs`
- validator utilities module: `src/utils/validate.rs`
- top-level CLI wiring in `src/main.rs`
- optional shared JSON types in `src/utils` if reused by `run-loop`

Behavior requirements:
- Accept repeated `--check` shell commands.
- Accept `--task` task file input.
- Return non-zero exit code if any required check fails.
- Emit deterministic JSON with per-check status.
- Preserve human-readable output for local use.

Acceptance criteria:
- deterministic handling of pass/fail/timeout
- stable JSON schema
- useful summaries without dumping excessive logs
- unit tests for timeout, failure, multiple checks, and task-file loading
- integration tests for JSON output and exit codes

Pre-implementation checklist:
- Define JSON schema first.
- Define timeout semantics first.
- Decide whether shell runs through `bash -lc` or direct argv parsing.
- Decide whether task-file checks override or merge with CLI-provided checks.

Post-implementation checklist:
- Verify exit codes are documented and tested.
- Verify no hanging subprocesses on timeout.
- Verify stdout/stderr truncation policy if any.
- Verify JSON stays clean under `--quiet`.

### Spec 2: bet/task file parser

Goal:
- Add a small, versioned bet schema that defines shaped loop inputs and checks.

Implementation scope:
- `src/utils/task.rs`
- schema structs with serde
- file loading and validation helpers
- tests for malformed/missing fields

Behavior requirements:
- versioned `type` field
- required objective and acceptance checks
- required appetite, bounded contexts, success signal, and no-gos
- optional context files, constraints, budgets, review checks, agent command
- strict parse errors with actionable messages

Acceptance criteria:
- invalid schema fails fast with useful errors
- minimal valid task file is easy to author
- task schema can be snapshotted into run state without lossy transformations
- schema cannot silently degrade into a generic workflow model

Pre-implementation checklist:
- Freeze v1 field names before coding.
- Keep schema intentionally smaller than CAR tickets and explicitly narrower than a general workflow DSL.

Post-implementation checklist:
- Test forward-compatibility posture via version field.
- Test path handling for context files.
- Test missing-check behavior.

### Spec 3: `run-loop`

Goal:
- Execute one resumable outer loop with validator-driven stop conditions.

Implementation scope:
- new command module: `src/commands/run_loop.rs`
- run-state module: `src/utils/runs.rs`
- reuse validator and task modules
- optional reuse of current profile switching helpers from `run.rs`

Behavior requirements:
- create and persist run directory
- capture iteration artifacts and validator results
- support `--resume`
- support optional `--profile`
- stop on acceptance or budget exhaustion
- never claim success if validation fails

Acceptance criteria:
- interrupted runs can be resumed safely
- state files are not corrupted on crash/restart
- loop output explains why it continued or stopped
- integration tests cover success, repeated validator failure, timeout, resume, and profile use

Pre-implementation checklist:
- Decide exact agent invocation contract.
- Decide whether loop writes a generated prompt file before each iteration.
- Decide if agent stdout is summarized by the loop or stored raw only.
- Decide whether summaries are generated by the agent or by the loop process.

Post-implementation checklist:
- Verify all stop reasons are explicit.
- Verify resume behavior after partial iteration write.
- Verify `--profile` restores auth correctly even on failure.
- Verify budgets are enforced even when validators hang or the agent exits oddly.

### Spec 4: `runs`

Goal:
- Make unattended execution debuggable and inspectable.

Implementation scope:
- new command module: `src/commands/runs.rs`
- shared run-state reader utilities

Behavior requirements:
- list runs with brief status table
- inspect one run in detail
- emit JSON
- optionally tail recent events/logs

Acceptance criteria:
- one command answers “what happened last night?”
- no need to inspect internal files manually for common failures

Pre-implementation checklist:
- Define summary fields and sorting order.
- Decide retention policy later; do not auto-delete in v1.

Post-implementation checklist:
- Verify large logs do not break terminal UX.
- Verify `--json` returns stable field names.

### Spec 5: notifications (P1)

Goal:
- Notify operators only when attention is needed.

Implementation scope:
- optional hook execution in `run-loop`
- no provider-specific integrations in first pass

Acceptance criteria:
- one shell command path works reliably
- payload includes run id, status, task name, iteration count, and stop reason

Pre-implementation checklist:
- define trust model for hook commands
- define retry/no-retry policy

Post-implementation checklist:
- verify hook failures do not falsely fail successful runs unless explicitly configured

### Spec 6: review checks (P1)

Goal:
- Add independent review after implementation and before final success.

Implementation scope:
- extend task schema
- optional secondary validator phase

Acceptance criteria:
- review failures are distinguishable from acceptance-check failures
- run state clearly shows which phase failed

Pre-implementation checklist:
- keep deterministic review in first pass
- do not collapse review and acceptance into one opaque phase

Post-implementation checklist:
- verify review phase can be skipped when empty

## State schema and JSON output requirements

Required principle:
- JSON output is part of the contract.

Required outputs:
- `validate --json`
- `run-loop --json`
- `runs --json`

Rules:
- no progress noise in JSON mode
- stable field names
- explicit enum-like status values
- include schema version where appropriate

## Testing strategy

Unit tests:
- task schema parsing
- validator timeout and pass/fail handling
- run-state read/write
- stop condition evaluation
- feedback shaping logic

Integration tests:
- `validate --json`
- `validate` exit code semantics
- `run-loop` success path
- `run-loop` repeated failure path
- `run-loop --resume`
- `run-loop --profile` restoration behavior
- `runs --json`

Failure-injection tests:
- malformed task file
- missing context file
- validator timeout
- crashed agent command
- partially written state file recovery

Manual tests before release:
- run in a clean repo
- run in a dirty repo
- run with encrypted profile
- run with API-key-only profile
- run with ChatGPT-only profile
- resume after manual process kill

## Development order and commit slicing

Recommended commit order:
1. task schema + parser + tests
2. validator engine + `validate` command + tests
3. run-state persistence primitives + tests
4. `run-loop` basic success/failure path + tests
5. `runs` inspection command + tests
6. polish: JSON schema, docs, help text, examples
7. optional P1 notification hook

Reason:
- This keeps the trust boundary (`validate`) ahead of autonomy (`run-loop`).

## Pre-development checklist

Before implementation starts:
- Confirm command naming (`validate`, `run-loop`, `runs`).
- Confirm task schema format (`yaml`, versioned header).
- Confirm agent invocation contract for Codex CLI.
- Confirm whether run-state belongs under `$CODEXCTL_DIR/runs`.
- Confirm exit code policy for validation and loop commands.
- Confirm whether worktree support is deferred.
- Confirm whether review phase is P1 and not MVP.
- Confirm docs are internal-only until feature direction is locked.

## Post-development checklist

Before calling the feature production-ready:
- All new commands have `--json` support.
- Exit codes are documented and tested.
- Resume behavior is crash-safe.
- Validator timeouts are enforced.
- Agent failure is distinguishable from validator failure.
- Run-state files are human-inspectable.
- Help text explains unattended risk and stop conditions.
- Integration tests cover the main success/failure/recovery paths.
- No command regresses current auth/profile switching behavior.
- README/public docs are updated only after internal review approves scope.

## Fact-check checklist for future spec review

Before implementation review is approved, re-verify:
- Ralph/Huntley guidance still emphasizes simple outer loops and validator pressure.
- Current CAR public positioning still treats tickets as the control plane.
- Claude Code hooks behavior and caveats have not changed materially.
- No major official Codex CLI surface has appeared that already solves this in-product.
- Community failure patterns still support prioritizing validators and persisted run state over multi-agent complexity.
- The proposed consumer-facing schema still reinforces Shape Up and DDD rather than broadening into generic automation.

## Recommendation for the next planning round

Use this note as the source document for implementation planning, but require one more internal review specifically on:
- command naming
- task schema ergonomics
- agent invocation contract
- exit codes and JSON shape
- whether worktree isolation should move from P1 to P0.5

## Final product judgment

If we build this feature, the right product identity is:
- `codexctl` as a reliable local control plane for Codex unattended execution
- not `codexctl` as a large autonomous project-management framework

That distinction should remain explicit in every implementation decision.
