---
blocked_by: []
blocks: []
created: 2026-05-20
domain: simulation
effort: M
id: BL-0026
labels: [coverage:design, domain:core-runtime, domain:simulation, p1, status:triage, type:design]
priority: P1
status: triage
title: GameTime, pause, and determinism for simulation primitives
---

## `GameTime`, pause, and determinism for simulation primitives

### Context

The 2026-04-12 review §3.4 / P1 #34 found that the four simulation primitives lack a unified time /
pause / determinism story:

- Time-scale (slow-mo, pause, 0.5×) is not specified for grids or event logs.
- Pause semantics are not unified — grids keep propagating unless callers stop calling.
- Determinism seed plumbing is absent.

### Acceptance criteria

- [ ] `core-runtime/game-loop.md` defines `GameTime { tick: u64, elapsed_secs: f64,
      time_scale: f32, paused: bool, seed: u64 }`.
- [ ] All four simulation primitives consume `GameTime` for their tick advancement.
- [ ] Pause semantics: when `paused`, primitives advance no state (callers respect the
      flag).
- [ ] Determinism seed flows through `GameTime.seed` per SC-8.
- [ ] `canonical-owners.md` row for `GameTime` flips to `Owned`.

### Verification

Pause toggles freeze grid propagation, timeline playback, and event-log threshold evaluation
identically. Replays with the same seed produce identical outputs.

### References

- [docs/design/design-review.md §3.4 / P1 #34](../../design/design-review.md)
- [docs/design/integration/shared-conventions.md SC-8](../../design/integration/shared-conventions.md)
