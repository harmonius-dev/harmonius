---
blocked_by: []
blocks: []
created: 2026-05-20
domain: ai
effort: M
id: BL-0027
labels: [coverage:design, domain:ai, p2, status:triage, type:design]
priority: P2
status: triage
title: Pick A* variant and nav-mesh parameters
---

## Pick A* variant and nav-mesh parameters

### Context

`ai/navigation.md` mentions A* without specifying variant, heuristic, tie-breaker, or heap type.
Recast-style nav-mesh generation has no citation or parameters.

### Acceptance criteria

- [ ] A* variant chosen (vanilla, Theta*, JPS) with heuristic and tie-breaker documented.
- [ ] Heap type chosen (binary heap, Fibonacci, pairing) with rationale.
- [ ] Nav-mesh generation: cell size, partition algorithm, filter thresholds, agent radius
      handling, with Recast citation.
- [ ] Companion test cases benchmark per-query latency against R-7.NFR.2 targets.

### Verification

500 A* queries per tick on a 50-tile NavMesh with p95 < 0.1 ms (R-7.NFR.2).

### References

- [docs/design/design-review.md P1 #22](../../design/design-review.md)
- [docs/requirements/ai/navigation.md](../../requirements/ai/navigation.md)
- [docs/requirements/ai/non-functional.md](../../requirements/ai/non-functional.md) R-7.NFR.2
