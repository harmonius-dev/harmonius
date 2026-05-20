---
blocked_by: []
blocks: []
created: 2026-05-20
domain: networking
effort: M
id: BL-0003
labels: [coverage:design, domain:audio, domain:networking, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Split VoiceStream ownership (transport vs codec)
---

## Split `VoiceStream` ownership (transport vs codec)

### Context

The 2026-04-12 design review §2.2 found two `VoiceStream` definitions:

- `audio/audio.md` defines codec and HRTF spatialization.
- `networking/network-services.md` defines transport over QUIC.

Both refer to the "same" voice stream but cover different concerns.

### Acceptance criteria

- [ ] `networking/network-services.md` owns the transport-side `VoiceStream` (QUIC stream
      framing, jitter buffer, NAT considerations).
- [ ] `audio/audio.md` owns the audio-side `VoiceStream` (Opus codec, HRTF, mixer routing).
- [ ] The two layers exchange opaque PCM-frame messages over a documented adapter interface.
- [ ] `canonical-owners.md` rows for `VoiceStream (transport)` and `VoiceStream (codec)`
      flip to `Owned`.

### Verification

Each definition exists in exactly one file; the other file references the canonical definition by
name.

### References

- [docs/design/design-review.md §2.2](../../design/design-review.md#22-foundational-type-duplication)
- [docs/design/audio/audio.md](../../design/audio/audio.md)
- [docs/design/networking/network-services.md](../../design/networking/network-services.md)
- [ADR-0010 QUIC unified transport](../../decisions/adr/ADR-0010-quic-unified-transport.md)
