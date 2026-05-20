---
blocked_by: []
blocks: []
created: 2026-05-20
domain: rendering
effort: L
id: BL-0011
labels: [coverage:design, domain:content-pipeline, domain:rendering, p1, status:triage, type:design]
priority: P1
status: triage
title: Specify meshlet asset pipeline end-to-end
---

# Specify meshlet asset pipeline end-to-end

## Context

`rendering/meshlets.md` exists. The 2026-04-12 review's P1 #16 noted the asset pipeline path
(import → `MeshletAsset` → GPU upload) was incomplete, and the link to BLAS for ray tracing
was unspecified.

## Acceptance criteria

- [ ] Import path specified: source mesh → meshlet builder → `MeshletAsset` (rkyv archive).
- [ ] `MeshletAsset` schema documented (vertex layout, triangle indices, bounding cones,
      LOD chain).
- [ ] GPU upload path: from rkyv archive to bindless GPU buffer; staging buffer use.
- [ ] BLAS construction: meshlet → BLAS proxy mapping for ray tracing.
- [ ] Companion `meshlets-test-cases.md` covers import, archive validation, GPU upload, and
      BLAS construction.

## Verification

A reader can take a `.glb` mesh asset to a rendered meshlet draw using only `meshlets.md`,
the asset pipeline, and the canonical `Material` type.

## References

- [docs/design/design-review.md P1 #16](../../design/design-review.md)
- [docs/design/rendering/meshlets.md](../../design/rendering/meshlets.md)
- [docs/design/content-pipeline/asset-pipeline.md](../../design/content-pipeline/asset-pipeline.md)
