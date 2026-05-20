---
blocked_by: []
blocks: []
created: 2026-05-20
domain: integration
effort: S
id: BL-0033
labels: [coverage:design, domain:integration, p3, status:triage, type:tech-debt]
priority: P3
status: triage
title: Adopt single channel-capacity formula in capacities doc
---

# Adopt single channel-capacity formula in capacities doc

## Context

`shared-messaging-capacities.md` exists but per-pair docs assert capacities (64, 128, 256,
1024, 4096) without referencing the formula `Capacity = producers × burst × safety_margin`
(SC-7). This is an OKR-2 KR-2.4 contributor.

## Acceptance criteria

- [ ] Every pair doc that names a numeric capacity references the SC-7 formula and the
      table entry in `shared-messaging-capacities.md`.
- [ ] Pair docs missing FM-N fallback labels (SC-6) add them.
- [ ] No pair doc redefines a capacity number; all flow from the table.

## Verification

`grep -rE '\| 64 \| ?$|\| 128 \| ?$|\| 256 \| ?$|\| 1024 \| ?$|\| 4096 \|' docs/design/integration/`
shows references only via the shared table.

## References

- [docs/design/integration/shared-conventions.md SC-6 / SC-7](../../design/integration/shared-conventions.md)
- [docs/design/integration/shared-messaging-capacities.md](../../design/integration/shared-messaging-capacities.md)
