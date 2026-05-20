---
blocked_by: []
blocks: []
created: 2026-05-20
domain: integration
effort: M
id: BL-0054
labels: [coverage:design, domain:integration, p2, status:triage, type:design]
priority: P2
status: triage
title: Author event-logs ↔ animation integration
---

## Author event-logs ↔ animation integration

### Context

The 2026-04-12 review §3.8 listed `event-logs-animation` as a missing integration pair. Event logs
feed animation triggers (replay, AI memory animation, action history); no integration doc specifies
the contract.

### Acceptance criteria

- [ ] New `design/integration/event-logs-animation.md` defines the contract.
- [ ] Companion `*-test-cases.md`.
- [ ] References `shared-conventions.md` SC-1..SC-14.
- [ ] `AnimEvent` references the canonical owner per BL-0032.

### Verification

A replay viewer can drive animation playback by reading event logs through this integration.

### References

- [docs/design/design-review.md §3.8](../../design/design-review.md#38-integration-layer)
- [BL-0032 AnimEvent canonical](BL-0032-anim-event-canonical.md)
