---
blocked_by: []
blocks: []
created: 2026-05-20
domain: ui
effort: XL
id: BL-0052
labels: [coverage:design, domain:ui, p2, status:triage, type:design]
priority: P2
status: triage
title: Decompose UI domain (layout, rendering, accessibility)
---

## Decompose UI domain (layout, rendering, accessibility)

### Context

`design/ui/` has only `ui-framework.md`. The 2026-05 audit and the
[design coverage roadmap](../../design/coverage-roadmap.md) propose three new docs:

- `ui-framework.md` (kept; trimmed) — widget framework, lifecycle, event routing
- `ui-layout.md` — Flexbox-style or constraint-based layout engine
- `ui-rendering.md` — text shaping, glyph atlas, batching, scissor, masking
- `ui-accessibility.md` — WCAG plan (BL-0039), keyboard nav, screen reader

### Acceptance criteria

- [ ] Three new design docs alongside the trimmed `ui-framework.md`.
- [ ] Each has a companion `*-test-cases.md`.
- [ ] `architecture.md` UI section updated.

### Verification

A reader can compose a custom widget without consulting the framework code; each sub-doc is
self-contained.

### References

- [docs/design/coverage-roadmap.md](../../design/coverage-roadmap.md)
- [BL-0039 WCAG compliance](BL-0039-accessibility-wcag.md)
