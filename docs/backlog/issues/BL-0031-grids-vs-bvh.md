---
blocked_by: []
blocks: []
created: 2026-05-20
domain: simulation
effort: S
id: BL-0031
labels: [coverage:design, domain:simulation, p2, status:triage, type:design]
priority: P2
status: triage
title: Grids vs BVH decision matrix
---

# Grids vs BVH decision matrix

## Context

The 2026-04-12 review §3.4 noted that the choice between `CellGrid` (uniform grid) and
`BvhTree` for spatial queries is unstated. Subsystems pick whichever feels right.

## Acceptance criteria

- [ ] Decision matrix in `simulation/grids-volumes.md` or `core-runtime/spatial-index.md`:
      when to use grid vs BVH (entity count, density, query type).
- [ ] Cross-references from physics, AI, and audio integration docs that pick spatial
      structures.

## Verification

A new subsystem author can pick the right spatial structure without consulting the team.

## References

- [docs/design/design-review.md §3.4](../../design/design-review.md#34-simulation)
- [docs/design/core-runtime/spatial-index.md](../../design/core-runtime/spatial-index.md)
