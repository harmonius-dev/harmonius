---
blocked_by: []
blocks: []
created: 2026-05-20
domain: data-systems
effort: L
id: BL-0028
labels: [coverage:design, domain:data-systems, domain:networking, p2, status:triage, type:design]
priority: P2
status: triage
title: Replication protocol for the four data primitives
---

# Replication protocol for the four data primitives

## Context

The 2026-04-12 review §3.3 noted that no network replication protocol exists for any of the
four data primitives (graphs, tables, attributes, containers). Authoritative state on the
server cannot mirror to clients deterministically.

## Acceptance criteria

- [ ] `data-systems/composition.md` (or per-primitive docs) specifies the replication
      contract for graphs, tables, attributes, containers.
- [ ] Deterministic graph traversal documented for networked clients.
- [ ] Author / authority / proxy roles named per primitive.
- [ ] Companion test cases verify replication parity with the server-authoritative state.

## Verification

A networked test scene with all four primitives exhibits server / client state parity for
60 seconds.

## References

- [docs/design/design-review.md §3.3](../../design/design-review.md#33-data-systems)
- [docs/design/data-systems/composition.md](../../design/data-systems/composition.md)
