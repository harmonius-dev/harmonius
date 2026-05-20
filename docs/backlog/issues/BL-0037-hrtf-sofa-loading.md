---
blocked_by: []
blocks: []
created: 2026-05-20
domain: audio
effort: S
id: BL-0037
labels: [coverage:design, domain:audio, p3, status:triage, type:design]
priority: P3
status: triage
title: Specify HRTF SOFA profile loading
---

# Specify HRTF SOFA profile loading

## Context

`audio/audio.md` mentions HRTF spatialization but does not document SOFA (Spatially Oriented
Format for Acoustics) profile loading.

## Acceptance criteria

- [ ] SOFA loader documented (file format parsing, error handling, fallback).
- [ ] Profile-switching behavior at runtime documented.
- [ ] Asset pipeline integration: SOFA files become engine assets via the asset pipeline.

## Verification

Loading a SOFA file from disk produces correct head-related impulse responses; switching
profiles at runtime causes no audio glitches.

## References

- [docs/design/design-review.md §3.6 / P2 #57](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/audio/audio.md](../../design/audio/audio.md)
