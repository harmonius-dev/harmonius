---
blocked_by: []
blocks: []
created: 2026-05-20
domain: networking
effort: M
id: BL-0038
labels: [coverage:design, domain:networking, p2, status:triage, type:design]
priority: P2
status: triage
title: Complete anti-cheat surface (input, teleport, reconciliation)
---

## Complete anti-cheat surface (input, teleport, reconciliation)

### Context

The 2026-04-12 review §3.6 found anti-cheat partial: movement and damage validators exist, but input
validation, teleport detection thresholds, and reconciliation bounds are missing.

### Acceptance criteria

- [ ] Input validation: per-action rate limits, per-frame action caps.
- [ ] Teleport detection: position-delta thresholds, server reconcile distance bounds.
- [ ] Reconciliation bounds: maximum client / server divergence before snap.
- [ ] Companion test cases include adversarial scenarios.

### Verification

Adversarial test scenarios (high-rate input, teleport, drift) trigger documented mitigations without
false positives in normal play.

### References

- [docs/design/design-review.md §3.6](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/networking/network-services.md](../../design/networking/network-services.md)
- [docs/user-stories/networking/non-functional.md](../../user-stories/networking/non-functional.md)
  US-8.NFR.7
