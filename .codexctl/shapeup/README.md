# Shape Up Operating Model

This directory contains the shaping and betting artifacts for `codexctl`.

## Primary user

The primary user is:
- an individual builder or other high-agency operator
- using AI agents as a core execution force
- trying to build products faster by increasing execution depth, run duration, and feature throughput

This is not a generic enterprise workflow buyer persona.

## Shape Up rules we are adopting

From Basecamp's Shape Up method:
- use fixed appetite instead of open-ended estimates
- shape before betting
- define rough solution, boundaries, rabbit holes, and no-gos before building
- choose a small number of bets for uninterrupted delivery
- avoid speculative backlog churn and over-detailed task management before shaping is done

## DDD rules we are adopting

From DDD reference material:
- define bounded contexts explicitly
- keep a shared ubiquitous language in docs and code
- keep model boundaries visible in modules and types
- avoid mixing unrelated models in one command or utility surface
- treat the code as an expression of the model, not just implementation detail

## Internal file conventions

- `domain-map.md`: bounded contexts, context map, and ubiquitous language
- `cycle-XX.md`: candidate bets for a cycle
- `bets/*.md`: one shaped bet per file

## Delivery convention

Each bet should include:
- problem
- appetite
- success signal
- bounded contexts touched
- solution outline
- no-gos
- rabbit holes
- delivery slices
- pre-build checklist
- post-build checklist
- fact-check / validation checklist

## Freeze rule

Once a bet is marked `Finalized`, implementation should treat the bet as frozen scope.

Allowed during implementation:
- clarifications that do not expand scope
- cuts made to stay within appetite
- technical substitutions that preserve the same outcome

Not allowed during implementation without reshaping:
- new user-facing scope
- new bounded contexts
- broadening the bet spec format
- adding convenience features that were explicitly excluded

## Product stance for future loop features

The unattended execution features in `codexctl` are intended to reinforce this operating model for consumers of the CLI.

That means:
- task specs should represent shaped work, not arbitrary workflow graphs
- the schema should encode Shape Up concepts such as appetite, bounded scope, no-gos, and success criteria
- the schema should encode DDD concepts such as bounded contexts and ubiquitous language
- we should avoid turning task specs into a broad automation or ticketing language
- we should make strictness usable through templates, linting, and clear errors
