---
blocked_by: []
blocks: []
created: 2026-05-20
domain: meta
effort: S
id: BL-0048
labels: [coverage:audit, domain:meta, p2, status:triage, type:doc]
priority: P2
status: triage
title: Sync requirements / user-stories / features README counts
---

## Sync `requirements` / `user-stories` / `features` README counts

### Context

Each of `requirements/README.md`, `user-stories/README.md`, `features/README.md` carries a Summary
table with files-per-domain and total counts. The 2026-05 audit found these tables drift from the
filesystem as files are added or removed.

The deslop pass updated some counts in passing (Core Runtime 9 → 11 files; AI 8 → 9; networking
story totals adjusted; rendering requirements +36 from gpu-runtime). A clean re-tally is still
needed.

### Acceptance criteria

- [ ] Programmatic recount of each domain's file count and SHALL / story / feature count.
- [ ] All three README Summary tables match the filesystem to the unit.
- [ ] Counts in `architecture.md` Design Summary verified.
- [ ] Audit recorded in `coverage/audits/`.

### Verification

A diff script comparing README counts to filesystem counts shows zero discrepancies.

### References

- [docs/requirements/README.md](../../requirements/README.md)
- [docs/user-stories/README.md](../../user-stories/README.md)
- [docs/features/README.md](../../features/README.md)
- [docs/architecture.md](../../architecture.md) Design Summary
