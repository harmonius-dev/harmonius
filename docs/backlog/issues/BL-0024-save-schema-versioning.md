---
blocked_by: []
blocks: []
created: 2026-05-20
domain: game-framework
effort: L
id: BL-0024
labels: [coverage:design, domain:game-framework, p1, status:triage, type:design]
priority: P1
status: triage
title: Specify save schema versioning and migration
---

## Specify save schema versioning and migration

### Context

`game-framework/save-system.md` has `MigrationRegistry` declared but not specified. The 2026-04-12
review §3.7 / P1 #25 noted: stable GUID IDs, migration chain ordering, rollback on failure, autosave
retention, and procedural-asset save handling are missing.

### Acceptance criteria

- [ ] Stable GUID IDs documented (per-component, per-asset).
- [ ] Migration chain ordering: linear vs DAG, conflict policy.
- [ ] Rollback-on-failure: save the original, write the new, swap atomically.
- [ ] Autosave retention policy (keep N most recent, oldest evicted).
- [ ] Procedural-asset save handling: serialize generation seeds, not generated artifacts.

### Verification

A save file written under v1 schema migrates cleanly to v2 with rollback test cases for
mid-migration failure.

### References

- [docs/design/design-review.md §3.7 / P1 #25](../../design/design-review.md)
- [docs/design/game-framework/save-system.md](../../design/game-framework/save-system.md)
