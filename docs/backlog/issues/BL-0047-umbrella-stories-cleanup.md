---
blocked_by: []
blocks: []
created: 2026-05-20
domain: meta
effort: M
id: BL-0047
labels: [coverage:audit, domain:meta, p3, status:triage, type:user-story]
priority: P3
status: triage
title: Remove umbrella / parent compound stories
---

# Remove umbrella / parent compound stories

## Context

Per `user-stories/AGENTS.md` rule 3, each story is granular: 1 persona + 1 action + 1 use
case + 1 feature. The 2026-05 audit found ~100 files with umbrella / parent rollup stories
that combine multiple actions or use cases.

## Acceptance criteria

- [ ] Each compound story split into granular stories or removed.
- [ ] Verify no story uses words like "various" or "multiple" to combine actions.
- [ ] `user-stories/README.md` counts updated.

## Verification

A grep for compound-story patterns (`,`-laden actions, "and" connectors) shows zero hits in
post-cleanup story bodies.

## References

- [docs/user-stories/AGENTS.md](../../user-stories/AGENTS.md) Rule 3
- [docs/coverage/audits/2026-05-audit.md](../../coverage/audits/2026-05-audit.md)
