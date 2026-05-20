---
blocked_by: []
blocks: []
created: 2026-05-20
domain: animation
effort: S
id: BL-0032
labels: [coverage:design, domain:animation, p2, status:triage, type:tech-debt]
priority: P2
status: triage
title: Canonical AnimEvent location
---

# Canonical `AnimEvent` location

## Context

`AnimEvent` variants are listed per-consumer (audio, VFX, gameplay) instead of canonically
in `animation/skeletal.md`. The 2026-04-12 review §3.8 flagged this.

## Acceptance criteria

- [ ] `animation/skeletal.md` defines the canonical `AnimEvent` enum with all variants.
- [ ] Audio, VFX, and gameplay integration docs reference the canonical enum.
- [ ] `canonical-owners.md` row for `AnimEvent` flips to `Owned`.

## Verification

`grep -rE 'enum AnimEvent' docs/design/` shows exactly one definition.

## References

- [docs/design/design-review.md §3.8](../../design/design-review.md#38-integration-layer)
- [docs/design/animation/skeletal.md](../../design/animation/skeletal.md)
