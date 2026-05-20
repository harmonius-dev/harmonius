---
blocked_by: []
blocks: []
created: 2026-05-20
domain: audio
effort: XL
id: BL-0050
labels: [coverage:design, domain:audio, p2, status:triage, type:design]
priority: P2
status: triage
title: Decompose audio domain into mid-level designs
---

## Decompose audio domain into mid-level designs

### Context

`design/audio/` has only one file (`audio.md`) covering audio runtime, mixing, streaming, spatial
audio, codecs, voice chat, and HRTF. The 2026-05 audit and the
[design coverage roadmap](../../design/coverage-roadmap.md) propose splitting into:

- `audio-runtime.md` — real-time mixer thread, command queue, lifecycle
- `audio-mixing.md` — mixer graph, buses, ducking (BL-0036)
- `audio-streaming.md` — file streaming, decoder, ring buffers
- `audio-spatial.md` — HRTF (BL-0037), positional audio, occlusion
- `audio-codecs.md` — supported codecs, profile loading

### Acceptance criteria

- [ ] Five new design docs replace the monolithic `audio.md`.
- [ ] Each has a companion `*-test-cases.md`.
- [ ] `architecture.md` Audio section updated to list all five.
- [ ] Existing cross-references from integration docs updated.

### Verification

`audio.md` becomes a short overview file pointing at the five sub-docs; each sub-doc is
self-contained and shorter than 1 000 lines.

### References

- [docs/design/coverage-roadmap.md](../../design/coverage-roadmap.md)
- [docs/coverage/audits/2026-05-audit.md](../../coverage/audits/2026-05-audit.md)
