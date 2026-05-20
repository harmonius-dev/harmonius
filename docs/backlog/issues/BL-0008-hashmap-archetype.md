---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: M
id: BL-0008
labels: [coverage:design, domain:core-runtime, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Remove HashMap from ECS Archetype hot path
---

## Remove `HashMap` from ECS `Archetype` hot path

### Context

`core-runtime/ecs.md` Archetype storage uses `HashMap<ComponentId, Column>` on the hot query path.
This violates the project-wide rule "no HashMap on hot paths" (constraints.md
`Performance Patterns`, SC-2). The 2026-04-12 design review §3.1 flagged this.

### Acceptance criteria

- [ ] Archetype column lookup uses sorted `Vec<(ComponentId, Column)>` plus binary search,
      or a dense keyed store, instead of `HashMap`.
- [ ] Benchmark `archetype_column_lookup_p99` is at most equal to the prior HashMap version.
- [ ] `ecs.md` design diagram and pseudocode reflect the new container.
- [ ] `shared-conventions.md` SC-2 row for ECS Archetype examples no longer flags `HashMap`.

### Verification

`grep 'HashMap' docs/design/core-runtime/ecs.md` shows no Archetype hot-path usage; the benchmark
suite reports equal or better p99 lookup.

### References

- [docs/design/design-review.md §3.1](../../design/design-review.md#31-core-runtime)
- [docs/design/integration/shared-conventions.md SC-2](../../design/integration/shared-conventions.md#sc-2----no-hashmap-on-hot-paths)
- [docs/design/core-runtime/ecs.md](../../design/core-runtime/ecs.md)
