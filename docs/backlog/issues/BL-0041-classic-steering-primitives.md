---
blocked_by: []
blocks: []
created: 2026-05-20
domain: ai
effort: S
id: BL-0041
labels: [coverage:design, domain:ai, p3, status:triage, type:design]
priority: P3
status: triage
title: Add Align / Separate / ObstacleAvoid / Hide / Interpose primitives
---

## Add Align / Separate / ObstacleAvoid / Hide / Interpose primitives

### Context

`ai/steering-crowds.md` is missing classic steering primitives (`Align`, `Separate`, `Cohesion`
outside flocking, `ObstacleAvoid`, `Hide`, `Interpose`). The 2026-04-12 review §3.6 / P2 #57 flagged
this.

### Acceptance criteria

- [ ] Each primitive documented with input parameters and output force.
- [ ] Composition rules documented (priority sums, weighted blends).
- [ ] Companion test cases cover each primitive in isolation and in composition.

### Verification

A flock of 100 agents with composed primitives behaves consistently with the documented algorithms.

### References

- [docs/design/design-review.md §3.6 / P2 #57](../../design/design-review.md)
- [docs/design/ai/steering-crowds.md](../../design/ai/steering-crowds.md)
