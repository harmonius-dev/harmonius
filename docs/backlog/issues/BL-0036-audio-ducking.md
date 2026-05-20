---
blocked_by: []
blocks: []
created: 2026-05-20
domain: audio
effort: S
id: BL-0036
labels: [coverage:design, domain:audio, p3, status:triage, type:design]
priority: P3
status: triage
title: Specify audio ducking (voice > SFX > music)
---

## Specify audio ducking (voice > SFX > music)

### Context

`audio/audio.md` has no ducking spec. Industry standard is voice > SFX > music ducking with
configurable thresholds.

### Acceptance criteria

- [ ] Ducking algorithm documented: priority levels, attack / release times, threshold
      configuration.
- [ ] Per-source priority enum exposed.
- [ ] Cross-references to mixer graph in `audio.md` and audio integration docs.

### Verification

Concurrent voice + SFX + music produces audible ducking matching the documented thresholds.

### References

- [docs/design/design-review.md §3.6 / P2 #57](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/audio/audio.md](../../design/audio/audio.md)
