---
blocked_by: []
blocks: []
created: 2026-05-20
domain: physics
effort: L
id: BL-0018
labels: [coverage:design, domain:physics, p2, status:triage, type:design]
priority: P2
status: triage
title: Specify cloth constraint and fracture pattern formats
---

## Specify cloth constraint and fracture pattern formats

### Context

`physics/advanced.md` references cloth constraint types and fracture patterns by name without
specifying layouts. The 2026-04-12 review P2 #37 flagged this.

### Acceptance criteria

- [ ] Cloth constraint format documented: structural / shear / bending springs, pin
      attachments, wind-coupling fields.
- [ ] Fracture pattern format documented: pre-fractured chunk graph or runtime Voronoi
      parameters.
- [ ] Asset pipeline can import / bake both formats.
- [ ] Companion `advanced-test-cases.md` covers both with concrete examples.

### Verification

A reader of `advanced.md` can construct a cloth or fracture asset without consulting the
implementation.

### References

- [docs/design/design-review.md P2 #37](../../design/design-review.md)
- [docs/design/physics/advanced.md](../../design/physics/advanced.md)
