---
blocked_by: []
blocks: []
created: 2026-05-20
domain: tools
effort: M
id: BL-0025
labels: [coverage:design, domain:rendering, domain:tools, p2, status:triage, type:tech-debt]
priority: P2
status: triage
title: Consolidate shader compilation into one service
---

# Consolidate shader compilation into one service

## Context

`asset-processing.md`, `visual-editors.md`, and `build-deploy.md` each describe shader
compilation independently. The 2026-04-12 review P2 #55 recommended a single
`ShaderCompiler` service in `harmonius_core` with the other docs becoming clients.

## Acceptance criteria

- [ ] New `core-runtime/shader-compiler.md` (or equivalent) defines the canonical service.
- [ ] `asset-processing.md`, `visual-editors.md`, `build-deploy.md` reference the canonical
      service.
- [ ] No glslc subprocess invocation appears outside the canonical service.
- [ ] Hot reload uses the same service.

## Verification

`grep -r 'glslc' docs/design/` shows references only inside the canonical service doc.

## References

- [docs/design/design-review.md P2 #55](../../design/design-review.md)
- [ADR-0007 GLSL via glslc](../../decisions/adr/ADR-0007-glsl-shader-il-via-glslc.md)
