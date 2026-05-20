---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: M
id: BL-0004
labels: [coverage:design, domain:core-runtime, domain:networking, domain:simulation, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Rename gameplay grid CellGrid and AOI grid AoiGrid
---

# Rename gameplay grid `CellGrid` and AOI grid `AoiGrid`

## Context

`UniformGrid<T>` is currently defined in three places with subtly different APIs:

- `core-runtime/spatial-index.md` (canonical generic primitive)
- `core-runtime/algorithms.md` (alternate definition)
- `simulation/grids-volumes.md` (gameplay propagation specialization)
- `networking/network-transport.md` (AOI / interest-management specialization)

The three users have legitimately different access patterns. The fix is to keep the generic
in core, rename the gameplay specialization `CellGrid`, and rename the networking
specialization `AoiGrid`.

## Acceptance criteria

- [ ] `core-runtime/spatial-index.md` owns the `UniformGrid<T>` generic primitive.
- [ ] `simulation/grids-volumes.md` defines `CellGrid` as the gameplay specialization.
- [ ] `networking/network-transport.md` defines `AoiGrid` as the AOI specialization.
- [ ] `core-runtime/algorithms.md` references the spatial-index definition rather than
      redefining.
- [ ] All `canonical-owners.md` rows for grid types flip to `Owned`.

## Verification

`grep -rE 'struct UniformGrid|struct CellGrid|struct AoiGrid' docs/design/` shows exactly
one definition per name.

## References

- [docs/design/design-review.md §2.2](../../design/design-review.md#22-foundational-type-duplication)
- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
