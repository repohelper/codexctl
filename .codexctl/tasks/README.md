# Tasks

Future unattended execution bet/task specs for `codexctl` should live here.

Target user:
- high-agency individuals using AI agents to build products quickly and repeatedly

Planned convention:
- one task file per shaped objective
- YAML format
- versioned schema header
- references to acceptance and review checks
- explicit Shape Up and DDD fields, not a generic workflow format

Future planned enablement:
- `codexctl bet init`
- `codexctl bet lint`
- starter templates for common product-building bet shapes

Required stance:
- these files are not a generic automation DSL
- they should describe shaped bets or scoped implementation tasks
- they should include enough structure to enforce appetite, boundaries, and validation
- they should be quick to author for high-agency users who care about shipping, not process ceremony

This directory is intentionally present now so the project structure matches the planned `Task Definition` bounded context.
