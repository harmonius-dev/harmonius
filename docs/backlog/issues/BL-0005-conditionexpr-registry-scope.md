---
blocked_by: []
blocks: []
created: 2026-05-20
domain: data-systems
effort: M
id: BL-0005
labels: [coverage:design, domain:data-systems, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Decide ConditionExpr registry scope (global vs per-subsystem)
---

# Decide `ConditionExpr` registry scope (global vs per-subsystem)

## Context

`ConditionExpr` and `ConditionRegistry` are referenced by graphs, effects, and containers in
the data-systems layer. The design review §3.3 noted there is no statement about whether
registries are shared across subsystems or owned per subsystem. Either decision is workable;
not deciding produces subtle bugs.

## Acceptance criteria

- [ ] Decision recorded in `data-systems/composition.md`: one global `ConditionRegistry` per
      world, or one registry per subsystem.
- [ ] `data-systems/attributes-effects.md` and `data-systems/directed-graphs.md` reference
      the chosen owner.
- [ ] If global, document how subsystems register their condition fns at startup.
- [ ] If per-subsystem, document the namespacing rules and how cross-subsystem conditions
      reference each other.
- [ ] `canonical-owners.md` row for `ConditionExpr` flips to `Owned`.

## Verification

Reading any of the three subsystem docs makes the registry-ownership story unambiguous; no
backlog comments survive on this question.

## References

- [docs/design/design-review.md §3.3](../../design/design-review.md#33-data-systems)
- [docs/design/data-systems/composition.md](../../design/data-systems/composition.md)
