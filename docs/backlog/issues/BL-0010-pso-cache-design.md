---
blocked_by: []
blocks: []
created: 2026-05-20
domain: rendering
effort: L
id: BL-0010
labels: [coverage:design, domain:rendering, p1, status:triage, type:design]
priority: P1
status: triage
title: Design pipeline-state-object cache (recovery + GC + persistence)
---

# Design pipeline-state-object cache (recovery + GC + persistence)

## Context

`rendering/pipeline-state-cache.md` exists with the basic cache design. The 2026-04-12
review's RF-15 / RF-17 recommended fleshing out:

- Disk persistence layout and versioning
- Recovery from corrupt cache files
- Garbage collection (when to evict entries)
- Barrier analysis algorithm (split barriers, redundancy detection, state-transition
  validation)

## Acceptance criteria

- [ ] `pipeline-state-cache.md` documents the on-disk layout (header, version, entry
      format) and the validation procedure on load.
- [ ] Recovery section explains corrupt-cache handling: drop and rebuild silently with
      log line.
- [ ] GC policy: LRU with high-water / low-water thresholds; document defaults.
- [ ] Barrier analysis algorithm specified pseudocode (resource state transitions, split
      barriers, elision conditions).
- [ ] Companion `pipeline-state-cache-test-cases.md` covers persistence, recovery, eviction,
      and barrier analysis.

## Verification

A second-pass review of `pipeline-state-cache.md` finds zero TODO markers and a complete
test-case companion.

## References

- [docs/design/design-review.md §3.2 / P1 #15 / P1 #17](../../design/design-review.md)
- [docs/design/rendering/pipeline-state-cache.md](../../design/rendering/pipeline-state-cache.md)
