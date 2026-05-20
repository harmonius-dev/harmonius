---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: M
id: BL-0014
labels: [coverage:design, domain:core-runtime, p2, status:triage, type:design]
priority: P2
status: triage
title: Promote FrameContext to a first-class type
---

# Promote `FrameContext` to a first-class type

## Context

The 2026-04-12 review P3 #67 recommended promoting `FrameContext` to a first-class type
defined in one place and used everywhere. Today the same fields (`frame_index`,
`interp_alpha`, `game_time`, `frame_budgets`, `seed`) are recreated ad hoc in integration
docs and per-subsystem code.

## Acceptance criteria

- [ ] `core-runtime/game-loop.md` defines `FrameContext { frame_index, interp_alpha,
      game_time, frame_budgets, seed }`.
- [ ] Integration docs that reference these fields cross-link instead of redefining.
- [ ] `canonical-owners.md` row for `FrameContext` flips from `Pending consolidation` to
      `Owned`.
- [ ] `GameTime` is included in this work or tracked as a sibling issue (see BL-0026).

## Verification

`grep -rE 'struct FrameContext|frame_index.*: u64' docs/design/` shows the canonical
definition in exactly one file.

## References

- [docs/design/design-review.md P3 #67](../../design/design-review.md)
- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/core-runtime/game-loop.md](../../design/core-runtime/game-loop.md)
