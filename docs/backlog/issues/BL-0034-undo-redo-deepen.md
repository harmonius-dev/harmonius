---
blocked_by: []
blocks: []
created: 2026-05-20
domain: tools
effort: M
id: BL-0034
labels: [coverage:design, domain:tools, p3, status:triage, type:design]
priority: P3
status: triage
title: Deepen tools/undo-redo.md
---

## Deepen `tools/undo-redo.md`

### Context

`tools/undo-redo.md` exists but covers only the basic stack. The 2026-04-12 review §3.7 P2 #52
recommended adding memory budget, persistent history, network sync for collaborative undo, and gizmo
selection coupling.

### Acceptance criteria

- [ ] Memory budget per session documented (cap, eviction policy).
- [ ] Persistent history (across editor restarts) specified.
- [ ] Network sync for collaborative undo specified (CRDT semantics).
- [ ] Gizmo selection coupling documented (how transformations enter the undo stack).

### Verification

A 1-hour multi-user editing session retains undo history, respects the documented memory budget, and
reconciles concurrent edits.

### References

- [docs/design/design-review.md §3.7 / P2 #52](../../design/design-review.md)
- [docs/design/tools/undo-redo.md](../../design/tools/undo-redo.md)
