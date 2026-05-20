---
blocked_by: []
blocks: []
created: 2026-05-20
domain: rendering
effort: L
id: BL-0012
labels: [coverage:design, domain:rendering, p2, status:triage, type:design]
priority: P2
status: triage
title: Unify 2D and 3D render graph passes
---

## Unify 2D and 3D render graph passes

### Context

The 2026-04-12 review §3.2 noted that the 2D path is completely parallel to 3D. `2d.md` shows an
isolated sprite pipeline with its own lights, camera, physics, and transform. Integration with the
shared render graph is asserted but not shown. `RenderLayerMask` is referenced in 2D and 3D with
inconsistent types.

### Acceptance criteria

- [ ] Decision recorded in `render-pipeline.md`: 2D passes are part of the existing render
      graph, OR a post-3D composite layer with documented hand-off.
- [ ] `RenderLayerMask` unified across 2D and 3D as a single type.
- [ ] `2d.md` and `render-pipeline.md` share a single render-graph diagram showing how 2D
      passes attach.
- [ ] `Camera2D` and `CameraComponent` either share a `CameraBase` trait or are documented
      as deliberately separate with clear interop rules.

### Verification

A reader of `render-pipeline.md` can describe how 2D rendering joins 3D rendering at the swapchain
handoff in one paragraph.

### References

- [docs/design/design-review.md §3.2 / P1 #18](../../design/design-review.md#32-rendering)
- [docs/design/rendering/2d.md](../../design/rendering/2d.md)
- [docs/design/rendering/render-pipeline.md](../../design/rendering/render-pipeline.md)
