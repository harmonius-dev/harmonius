---
blocked_by: []
blocks: []
created: 2026-05-20
domain: ai
effort: L
id: BL-0006
labels: [coverage:design, domain:ai, domain:core-runtime, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Replace Blackboard singleton with ECS component
---

# Replace `Blackboard` singleton with ECS component

## Context

`Blackboard` is currently a behavior-tree-private type with its own scope/value registry.
GOAP uses `WorldState`. Steering uses components directly. The 2026-04-12 design review §3.6
catalogued this as three ways to hold per-agent state — a violation of the ECS-primary
principle in [ADR-0003](../../decisions/adr/ADR-0003-ecs-primary-architecture.md).

`shared-conventions.md` SC-3 already mandates sorted backing stores for blackboards. The
remaining work is to make the blackboard live on the entity that owns it.

## Acceptance criteria

- [ ] `BlackboardComponent` defined in `core-runtime/ecs.md` (or `behavior.md`) with sorted
      `Vec<(BlackboardKey, BlackboardValue)>` backing store per SC-3.
- [ ] `BlackboardKey` is a codegen'd enum, not a string.
- [ ] `behavior.md`, `goap.md`, `utility-ai.md` updated to read/write the component instead
      of a singleton.
- [ ] Steering integration consumes `BlackboardComponent` for hide / interpose / target
      data.
- [ ] Eight integration docs that restated the "use BTreeMap" rule trim the duplicated prose
      and reference SC-3.

## Verification

`grep -r 'Blackboard ::' docs/design/` returns zero singleton usages; only component reads.

## References

- [docs/design/design-review.md §3.6](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/integration/shared-conventions.md SC-3](../../design/integration/shared-conventions.md#sc-3----blackboard-backing-store)
- [ADR-0003 ECS-primary architecture](../../decisions/adr/ADR-0003-ecs-primary-architecture.md)
