# Integration shared conventions SC-1..SC-14

## Status

Accepted — 2026-04-12

## Context

The 60+ pair-wise integration design documents under
[docs/design/integration/](../../design/integration/) repeatedly restated the same cross-cutting
rules: `Arc` only for immutable shared data, no `HashMap` on hot paths, sorted backing stores for
blackboards, MPSC default channel shape, rkyv for persistence, fallback mode names, channel capacity
formula, and several others. The [2026-04-12 design review](../../design/design-review.md) §2.8
quantified this: same prose appearing across 5–8 integration docs with slight wording variations.
Any change required editing every copy.

The audit also noted that the cross-cutting rules belong in one normative file; per-pair integration
docs should carry the *contract* between the two subsystems, not restate the rules.

## Decision

[`docs/design/integration/shared-conventions.md`](../../design/integration/shared-conventions.md) is
the single normative source for cross-cutting integration rules. Fourteen rules (`SC-1` … `SC-14`)
cover:

| ID    | Rule                                                                      |
|-------|---------------------------------------------------------------------------|
| SC-1  | `Arc` only for immutable shared data                                      |
| SC-2  | No `HashMap` on hot paths                                                 |
| SC-3  | Blackboard backing store is sorted `Vec` or `BTreeMap`                    |
| SC-4  | MPSC is the default channel shape; SPSC only when justified               |
| SC-5  | rkyv only for persistence; no serde                                       |
| SC-6  | Fallback modes named `FM-N` in each integration doc                        |
| SC-7  | Channel capacity = `producers × burst × safety_margin`                    |
| SC-8  | Determinism seed is plumbed as `GameTime.seed`                            |
| SC-9  | Errors propagate as `EngineError`                                         |
| SC-10 | Debug visualization is runtime-toggleable, not compile-gated              |
| SC-11 | Enums are fully enumerated (no `…`) in class diagrams                     |
| SC-12 | Persistent types require rkyv derives                                     |
| SC-13 | `DashMap` replaces `HashMap` when concurrency is required                 |
| SC-14 | Integration docs carry contracts, not implementations                     |

All integration design docs MUST link `shared-conventions.md` and MUST NOT restate the rule text.
Deviations require a "Deviation" subsection citing the specific rule and rationale.

## Consequences

- Each integration doc grew shorter by removing the restated rule prose; the
  [Phase 3 deslop pass](../../coverage/audits/2026-05-audit.md) added compliance lines to
  the 24 pair docs that previously did not link the SC rules.
- Future SC-rule changes propagate from a single edit to every consumer.
- The `SC-N` shorthand becomes pervasive in the integration corpus and is documented in the
  [glossary](../../glossary.md).
- Channel capacity numerics live in
  [`shared-messaging-capacities.md`](../../design/integration/shared-messaging-capacities.md)
  per SC-7; per-pair docs reference the table entry by name.

## Alternatives Considered

- **Repeat rules per doc** — operationally accepted previously; rejected because every
  rule change required N edits.
- **Embed rules in `constraints.md`** — `constraints.md` is the project-wide constraint set.
  Cross-cutting *integration* rules deserve their own normative file because they govern
  document compliance, not engine constraints.

## References

- [docs/design/integration/shared-conventions.md](../../design/integration/shared-conventions.md)
- [docs/design/integration/shared-messaging-capacities.md](../../design/integration/shared-messaging-capacities.md)
- [docs/design/design-review.md](../../design/design-review.md) §2.8
