---
blocked_by: []
blocks: []
created: 2026-05-20
domain: integration
effort: M
id: BL-0053
labels: [coverage:design, domain:integration, p2, status:triage, type:design]
priority: P2
status: triage
title: Author attributes ↔ rendering integration
---

# Author attributes ↔ rendering integration

## Context

The 2026-04-12 review §3.8 listed `attributes-rendering` as a missing integration pair.
Attributes (health, status effects, buffs) drive rendering features (damage flash, status
icons, debuff overlays). No integration doc currently specifies the contract.

## Acceptance criteria

- [ ] New `design/integration/attributes-effects-rendering.md` defines the contract.
- [ ] Companion `*-test-cases.md`.
- [ ] References `shared-conventions.md` SC-1..SC-14 per the integration template.

## Verification

A new attribute-driven shader effect (e.g., poison overlay) can be implemented by reading
this single integration doc.

## References

- [docs/design/design-review.md §3.8](../../design/design-review.md#38-integration-layer)
- [docs/design/integration/shared-conventions.md](../../design/integration/shared-conventions.md)
