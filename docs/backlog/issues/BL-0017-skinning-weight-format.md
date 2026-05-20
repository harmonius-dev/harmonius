---
blocked_by: []
blocks: []
created: 2026-05-20
domain: animation
effort: M
id: BL-0017
labels: [coverage:design, domain:animation, p2, status:triage, type:design]
priority: P2
status: triage
title: Specify skinning weight format
---

# Specify skinning weight format

## Context

`animation/skeletal.md` references `BonePalette` as `Handle<GpuBuffer>` with no layout
contract. The 2026-04-12 review P2 #36 noted this leaves consumers to invent the layout.

## Acceptance criteria

- [ ] Skinning weight format documented in `skeletal.md`: `[u8; 4]` bone indices plus
      quantized weights, palette layout, alignment.
- [ ] Bone palette layout (matrix array, dual quaternion array) specified.
- [ ] Companion `skeletal-test-cases.md` covers the weight format with concrete byte layouts.

## Verification

GPU skinning shaders (`skeletal-test-cases.md` benchmarks) read the documented byte layout
without ambiguity.

## References

- [docs/design/design-review.md P2 #36](../../design/design-review.md)
- [docs/design/animation/skeletal.md](../../design/animation/skeletal.md)
