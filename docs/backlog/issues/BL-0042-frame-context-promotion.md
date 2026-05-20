---
blocked_by: [BL-0014]
blocks: []
created: 2026-05-20
domain: integration
effort: M
id: BL-0042
labels: [coverage:design, domain:integration, p2, status:triage, type:tech-debt]
priority: P2
status: triage
title: Adopt FrameContext across integration docs
---

# Adopt `FrameContext` across integration docs

## Context

After [BL-0014](BL-0014-frameworld-promote.md) declares the canonical `FrameContext` type,
integration docs must adopt it. Today, several integration docs reference `InterpAlpha`
ad-hoc with slight variations (`profiler-game-loop.md`, `rendering-physics.md`,
`timelines-camera.md`).

## Acceptance criteria

- [ ] Every integration doc that references frame index, interp alpha, game time, or seed
      uses `FrameContext` by name.
- [ ] Ad-hoc copies removed.
- [ ] No new types in integration docs duplicate `FrameContext` fields.

## Verification

`grep -rE 'frame_index.*: u64|interp_alpha.*: f32' docs/design/integration/` shows only
references to `FrameContext`, no redefinitions.

## References

- [docs/design/design-review.md P3 #67](../../design/design-review.md)
- [BL-0014 FrameContext promote](BL-0014-frameworld-promote.md)
