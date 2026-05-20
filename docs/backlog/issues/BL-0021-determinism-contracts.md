---
blocked_by: []
blocks: []
created: 2026-05-20
domain: cross-cutting
effort: M
id: BL-0021
labels: [coverage:design, domain:cross-cutting, p2, status:triage, type:design]
priority: P2
status: triage
title: Determinism contracts for animation, procedural, event-logs
---

## Determinism contracts for animation, procedural, event-logs

### Context

The 2026-04-12 review §3.4 / P2 #40 noted that physics has a determinism contract but animation
state-machine evaluation, procedural animation (IK, foot placement), and event-log threshold
evaluation do not. Each subsystem claims determinism without specifying it.

### Acceptance criteria

- [ ] `animation/state-machine.md` documents determinism: tick order, RNG plumbing
      (`GameTime.seed`), float-precision policy.
- [ ] `animation/procedural.md` documents IK solver determinism (iteration count, tolerance).
- [ ] `simulation/event-logs.md` documents threshold evaluation determinism (tie-breaking,
      tick-stamp order).
- [ ] All three reference SC-8 (`GameTime.seed` plumbing).

### Verification

Replaying a recorded session produces identical animation poses, identical procedural outputs, and
identical event-log threshold firings frame for frame.

### References

- [docs/design/design-review.md §3.4 / P2 #40](../../design/design-review.md)
- [docs/design/integration/shared-conventions.md SC-8](../../design/integration/shared-conventions.md)
