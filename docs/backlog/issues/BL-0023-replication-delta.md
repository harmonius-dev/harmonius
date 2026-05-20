---
blocked_by: []
blocks: []
created: 2026-05-20
domain: networking
effort: L
id: BL-0023
labels: [coverage:design, domain:networking, p1, status:triage, type:design]
priority: P1
status: triage
title: Specify replication delta compression
---

## Specify replication delta compression

### Context

`networking/network-transport.md` names delta compression for state replication but does not choose
an algorithm. Bit-packing, RLE, and rkyv-diff are candidates.

### Acceptance criteria

- [ ] Delta algorithm chosen: field-level diffing or rkyv-style archive diff or bit-packing.
- [ ] Baseline strategy documented (full snapshot every N frames, or first-tick baseline).
- [ ] ACK mechanism specified (per-baseline ACKs, missing-ack retransmission).
- [ ] Bandwidth target documented (≤ 100 KB/s per client at 30 Hz, 500-entity scene).

### Verification

A 500-entity 30 Hz benchmark consumes ≤ 100 KB/s per client.

### References

- [docs/design/design-review.md P1 #24](../../design/design-review.md)
- [docs/requirements/networking/state-replication.md](../../requirements/networking/state-replication.md)
- [docs/user-stories/networking/non-functional.md](../../user-stories/networking/non-functional.md)
  US-8.NFR.2
