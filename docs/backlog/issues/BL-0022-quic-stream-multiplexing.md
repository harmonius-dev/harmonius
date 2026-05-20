---
blocked_by: []
blocks: []
created: 2026-05-20
domain: networking
effort: M
id: BL-0022
labels: [coverage:design, domain:networking, p1, status:triage, type:design]
priority: P1
status: triage
title: Specify QUIC stream multiplexing policy
---

# Specify QUIC stream multiplexing policy

## Context

Per [ADR-0010](../../decisions/adr/ADR-0010-quic-unified-transport.md), all networking rides
QUIC. The 2026-04-12 review P1 #23 noted the per-stream policy is unspecified: stream count,
fairness, backpressure, voice vs replication multiplexing.

## Acceptance criteria

- [ ] `networking/network-transport.md` specifies stream types: replication (per-channel),
      RPC (per-call), voice (per-session), asset streaming.
- [ ] Flow control policy per stream type: priorities, quota limits, fairness rules.
- [ ] Backpressure handling: how stalls propagate, when to drop.
- [ ] QUIC-over-TCP fallback path for restrictive firewalls.

## Verification

A sustained 500-client session with 30 Hz state replication plus voice plus asset streaming
maintains documented per-stream priorities under network stress.

## References

- [docs/design/design-review.md P1 #23](../../design/design-review.md)
- [docs/design/networking/network-transport.md](../../design/networking/network-transport.md)
- [ADR-0010 QUIC unified transport](../../decisions/adr/ADR-0010-quic-unified-transport.md)
