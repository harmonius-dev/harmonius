---
blocked_by: []
blocks: []
created: 2026-05-20
domain: physics
effort: M
id: BL-0019
labels: [coverage:design, domain:physics, p2, status:triage, type:design]
priority: P2
status: triage
title: Pick fluid solver variant (SPH / FLIP / Eulerian)
---

## Pick fluid solver variant (SPH / FLIP / Eulerian)

### Context

`physics/advanced.md` mentions fluid simulation but does not pick a solver. SPH (smoothed particle
hydrodynamics), FLIP / PIC, and Eulerian grid solvers each have different trade-offs.

### Acceptance criteria

- [ ] `advanced.md` picks one solver variant and documents the trade-offs.
- [ ] Solver parameters specified (particle count budgets, grid resolution, viscosity model).
- [ ] Source citation included per `constraints.md` Documentation Standards.
- [ ] Determinism contract specified (see BL-0021).

### Verification

`advanced.md` reads as a complete, citable design for fluid simulation.

### References

- [docs/design/design-review.md P2 #38](../../design/design-review.md)
- [docs/design/physics/advanced.md](../../design/physics/advanced.md)
