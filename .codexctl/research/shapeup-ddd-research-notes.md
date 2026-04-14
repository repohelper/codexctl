# Shape Up + DDD Research Notes

Date: 2026-04-14 (UTC)
Status: Internal reference

## Shape Up points used

From Basecamp's Shape Up book and materials:
- appetite should be fixed and scope should be cut to fit
- shaping should define rough solution, boundaries, and rabbit holes before building starts
- betting chooses shaped work rather than feeding a large backlog directly into implementation
- teams should get uninterrupted time on selected bets

## DDD points used

From Eric Evans' DDD reference:
- focus on the core domain
- speak a ubiquitous language inside an explicitly bounded context
- define bounded contexts clearly so models do not leak into each other
- keep code and model aligned
- use continuous integration to keep a bounded context coherent

## Research references

- Shape Up book: https://basecamp.com/shapeup/shape-up.pdf
- Shape Up overview: https://basecamp.com/shapeup
- DDD Reference: https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf
- Ralph overview: https://ghuntley.com/ralph/
- Ralph repo: https://github.com/ghuntley/how-to-ralph-wiggum
- Vercel ralph loop agent: https://github.com/vercel-labs/ralph-loop-agent
- CAR event page: https://sf.aitinkerers.org/talks/rsvp_U8eNYBDAmok
- Claude Code hooks docs: https://code.claude.com/docs/en/hooks

## Product interpretation for codexctl

Shape Up tells us how to shape the work.
DDD tells us how to keep the model boundaries coherent.

For this CLI, that means:
- define one clear unattended-execution bet first
- keep the domain split between task definition, validation, orchestration, and run ledger
- avoid building a broad orchestration framework before the core model is proven
