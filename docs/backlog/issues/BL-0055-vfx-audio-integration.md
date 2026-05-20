---
blocked_by: []
blocks: []
created: 2026-05-20
domain: integration
effort: M
id: BL-0055
labels: [coverage:design, domain:integration, p2, status:triage, type:design]
priority: P2
status: triage
title: Author vfx ↔ audio integration
---

# Author vfx ↔ audio integration

## Context

The 2026-04-12 review §3.8 listed `vfx-audio` as a missing integration pair. VFX events
(explosions, impacts, fire) drive audio playback; audio events (loud sounds) drive VFX
(camera shake, particle bursts). No integration doc specifies the contract.

## Acceptance criteria

- [ ] New `design/integration/vfx-audio.md` defines the contract.
- [ ] Companion `*-test-cases.md`.
- [ ] References `shared-conventions.md` SC-1..SC-14.

## Verification

An effect graph node that emits a sound and a sound event that emits VFX both compile and
run end-to-end via this integration.

## References

- [docs/design/design-review.md §3.8](../../design/design-review.md#38-integration-layer)
- [docs/design/integration/shared-conventions.md](../../design/integration/shared-conventions.md)
