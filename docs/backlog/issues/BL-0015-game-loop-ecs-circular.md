---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: M
id: BL-0015
labels: [coverage:design, domain:core-runtime, p1, status:triage, type:design]
priority: P1
status: triage
title: Resolve game-loop ↔ ECS circular dependency
---

## Resolve game-loop ↔ ECS circular dependency

### Context

`core-runtime/game-loop.md` defines `GameLoopGraph` and `CompiledFrame`, but those types also appear
in `ecs.md`. The 2026-04-12 review P0 #9 recommended moving them into `game-loop.md` only and having
`ecs.md::Schedule` produce a `SystemGraph` that the game loop compiles.

### Acceptance criteria

- [ ] `GameLoopGraph` and `CompiledFrame` are defined exactly once, in `game-loop.md`.
- [ ] `Schedule` in `ecs.md` produces a `SystemGraph` (or equivalent intermediate) with no
      knowledge of frame phases.
- [ ] `game-loop.md::GameLoopGraph::compile(systems: SystemGraph)` is the single integration
      point.
- [ ] No type is defined in both `ecs.md` and `game-loop.md`.

### Verification

A reader can describe the dependency direction (game-loop depends on ecs, not vice versa) from the
docs alone.

### References

- [docs/design/design-review.md P0 #9](../../design/design-review.md#p0--must-land-before-implementation-begins)
- [docs/design/core-runtime/ecs.md](../../design/core-runtime/ecs.md)
- [docs/design/core-runtime/game-loop.md](../../design/core-runtime/game-loop.md)
