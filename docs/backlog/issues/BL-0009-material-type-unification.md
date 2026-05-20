---
blocked_by: []
blocks: []
created: 2026-05-20
domain: rendering
effort: L
id: BL-0009
labels: [coverage:design, domain:rendering, p1, status:triage, type:design]
priority: P1
status: triage
title: Declare canonical Material type
---

## Declare canonical `Material` type

### Context

The 2026-04-12 design review §3.2 noted that `Material` as a unified type does not exist:

- `MaterialComponent` lives in `rendering-core.md`
- `MaterialGraph` lives in `render-styles.md`
- `CompiledMaterial` is implied in `render-pipeline.md` but never defined

The three are never linked in the design corpus, so callers cannot reason about the material
lifecycle (graph → compiled → component → draw).

### Acceptance criteria

- [ ] `rendering/render-pipeline.md` declares `Material { id, graph, compiled_shaders,
      descriptor_layout }` with clear ownership.
- [ ] `rendering-core.md`, `render-styles.md`, and `pipeline-state-cache.md` reference the
      canonical `Material` type.
- [ ] Material lifecycle (graph → compile → cache → component → draw) is documented in one
      place with a Mermaid sequence diagram.
- [ ] `canonical-owners.md` rows for `Material` and `CompiledMaterial` flip from
      `Pending creation` to `Owned`.

### Verification

A reader can trace a material from author-time graph to GPU draw call by reading one design doc.

### References

- [docs/design/design-review.md §3.2](../../design/design-review.md#32-rendering)
- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/rendering/render-pipeline.md](../../design/rendering/render-pipeline.md)
