---
blocked_by: []
blocks: []
created: 2026-05-20
domain: data-systems
effort: M
id: BL-0029
labels: [coverage:design, domain:data-systems, p2, status:triage, type:design]
priority: P2
status: triage
title: Conflict-resolution spec for effects and containers
---

## Conflict-resolution spec for effects and containers

### Context

The 2026-04-12 review §3.3 noted no conflict-resolution spec for simultaneous effect applications,
simultaneous threshold crossings, or failed mid-transaction container transfers.

### Acceptance criteria

- [ ] `attributes-effects.md` documents simultaneous-effect-application order and stack
      composition rules.
- [ ] Threshold-crossing tie-breaker rule specified.
- [ ] `containers-slots.md` documents transactional transfer semantics: rollback policy,
      partial-failure handling, atomicity guarantees.
- [ ] Companion test cases cover each conflict scenario.

### Verification

Concurrent effect / container operations produce deterministic outcomes that match the documented
rules.

### References

- [docs/design/design-review.md §3.3](../../design/design-review.md#33-data-systems)
- [docs/design/data-systems/attributes-effects.md](../../design/data-systems/attributes-effects.md)
- [docs/design/data-systems/containers-slots.md](../../design/data-systems/containers-slots.md)
