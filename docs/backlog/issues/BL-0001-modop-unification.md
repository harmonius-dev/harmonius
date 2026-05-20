---
blocked_by: []
blocks: []
created: 2026-05-20
domain: data-systems
effort: M
id: BL-0001
labels: [coverage:design, domain:data-systems, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Unify ModOp across attributes-effects and containers-slots
---

## Unify `ModOp` across attributes-effects and containers-slots

### Context

The 2026-04-12 design review §2.2 catalogued `ModOp` / `ModifierOp` as defined in two places with
overlapping but incompatible semantics:

- `data-systems/attributes-effects.md` defines a `ModOp` enum with arithmetic + curve
  operators.
- `data-systems/containers-slots.md` defines a `ModifierOp` enum with stat-propagation
  operators.

Containers' socket system propagates stat modifiers through the effects system, which means the two
enums are read by the same code paths. Defining them twice creates subtle merge bugs when a new
operator lands.

The [canonical-owners.md](../../design/canonical-owners.md) registry lists `ModOp` as
`Pending consolidation` with `core-runtime/primitives.md` as the intended owner.

### Acceptance criteria

- [ ] Single `ModOp` enum lives in `core-runtime/primitives.md` covering both attributes
      and containers semantics.
- [ ] `data-systems/attributes-effects.md` references the canonical enum and deletes the
      duplicate definition.
- [ ] `data-systems/containers-slots.md` references the canonical enum and deletes the
      duplicate definition.
- [ ] Sockets emit modifier events rather than directly applying effects (decouples
      containers from effects).
- [ ] `canonical-owners.md` row for `ModOp` flips from `Pending consolidation` to `Owned`.

### Verification

`grep -rE 'enum ModOp|enum ModifierOp' docs/design/` yields exactly one definition in
`core-runtime/primitives.md`.

### References

- [docs/design/design-review.md §2.2](../../design/design-review.md#22-foundational-type-duplication)
- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/data-systems/attributes-effects.md](../../design/data-systems/attributes-effects.md)
- [docs/design/data-systems/containers-slots.md](../../design/data-systems/containers-slots.md)
- [docs/design/core-runtime/primitives.md](../../design/core-runtime/primitives.md)
