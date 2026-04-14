# .codexctl

This directory is the project-owned control plane for `codexctl` development.

## Product promise

`codexctl` should help high-agency individuals build products by using AI agents to execute shaped bets with durable state, deterministic validation, and clear stop conditions.

This product is for users who want maximum leverage from AI across:
- depth of execution
- length of unattended operation
- breadth of feature delivery

## Rule

Any repo-local files that define how this project is shaped, planned, validated, or executed should live under `.codexctl/`.

That includes:
- Shape Up betting and shaping docs
- domain maps and ubiquitous language
- internal research notes
- future task specs for unattended execution
- project-local validation presets or run templates

That does not include:
- user runtime data generated outside the repo
- build artifacts
- ad hoc scratch files outside an agreed structure

## Structure

- `.codexctl/shapeup/`: shaping notes, cycles, bets, and delivery specs
- `.codexctl/research/`: source-backed internal research notes
- `.codexctl/tasks/`: future task specs for unattended execution features

## Working standard

We are using:
- Shape Up for shaping, appetite, betting, and scoped delivery
- DDD for bounded contexts, ubiquitous language, aggregates, and model boundaries

Planning rules:
- shape work as bets, not backlog sprawl
- set appetite first, then cut scope to fit
- write explicit no-gos and rabbit holes
- map each bet to bounded contexts before implementation starts
- keep specs implementation-facing, not aspirational
- keep repo-local execution specs aligned with Shape Up and DDD, not a generic automation format
- optimize for high-agency product builders, not generic workflow consumers

Implementation rules:
- keep JSON and exit-code contracts explicit
- treat validators as the trust boundary
- keep project-owned files inside `.codexctl/`
- prefer narrow, composable commands over framework sprawl
- do not broaden task specs into a general workflow DSL
- favor throughput and clarity over process ceremony
