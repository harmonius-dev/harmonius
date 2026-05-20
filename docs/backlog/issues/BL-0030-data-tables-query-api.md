---
blocked_by: []
blocks: []
created: 2026-05-20
domain: data-systems
effort: M
id: BL-0030
labels: [coverage:design, domain:data-systems, p2, status:triage, type:design]
priority: P2
status: triage
title: Data tables full query and secondary-index API
---

## Data tables full query and secondary-index API

### Context

`data-systems/data-tables.md` lacks a full query / secondary-index API: multi-column predicates,
filtered scans, joins, and indexed lookups. The 2026-04-12 review §3.3 flagged this as a gap that
prevents authors from building inventory or quest queries efficiently.

### Acceptance criteria

- [ ] Query API documented: predicate combinators, multi-column filters, optional joins.
- [ ] Secondary-index storage (sorted index per indexed column) documented.
- [ ] `DefinitionAsset<T>` shared wrapper for definition-based components introduced.
- [ ] Composition density performance profile (realistic RPG character / inventory scenarios)
      documented.
- [ ] Companion test cases cover query semantics and index maintenance.

### Verification

A 1 000-row crafting recipe table supports sub-millisecond multi-column predicate queries.

### References

- [docs/design/design-review.md §3.3 / P1 #33](../../design/design-review.md#33-data-systems)
- [docs/design/data-systems/data-tables.md](../../design/data-systems/data-tables.md)
