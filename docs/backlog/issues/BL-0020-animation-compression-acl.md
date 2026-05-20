---
blocked_by: []
blocks: []
created: 2026-05-20
domain: animation
effort: M
id: BL-0020
labels: [coverage:design, domain:animation, p2, status:triage, type:design]
priority: P2
status: triage
title: Specify animation compression (ACL or equivalent)
---

# Specify animation compression (ACL or equivalent)

## Context

`animation/skeletal.md` claims 10:1+ compression ratios but does not pick a format.
[ACL](https://github.com/nfrechette/acl) is the de-facto open-source choice; alternatives
include custom quantization or Bitsquid-style.

## Acceptance criteria

- [ ] `skeletal.md` picks a compression format and cites the reference.
- [ ] Target ratio (≥ 10:1) and quality bounds (per-bone error tolerance) specified.
- [ ] Asset pipeline integration documented (offline compression at bake time).
- [ ] Companion test cases include round-trip quality measurements.

## Verification

A clip baked through the pipeline meets the documented compression and quality targets.

## References

- [docs/design/design-review.md P2 #39](../../design/design-review.md)
- [docs/design/animation/skeletal.md](../../design/animation/skeletal.md)
