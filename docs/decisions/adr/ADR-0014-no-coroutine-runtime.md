# No coroutine runtime (reverses extraction)

## Status

Accepted — 2026-04-12

## Context

The 2026-04-12 design review §2.9 originally recommended extracting coroutine support to a new
`core-runtime/coroutines.md` so the scripting crate could become a client of a shared
coroutine runtime. The recommendation was based on the existence of `CoroutineState` inside
the scripting design.

On further inspection of the corpus, several integration designs already treated "no coroutine
runtime" as an invariant:

- `integration/ai-scripting.md` — multi-frame AI sequencing flows through behavior trees, not
  coroutines.
- `integration/directed-graphs-scripting.md` — graph traversal across frames flows through
  delayed events.
- `integration/timelines-scripting.md` — multi-frame sequencing flows through timelines.
- `integration/high-level.md` G1 — explicit "no coroutine runtime" invariant.

Treating `CoroutineState` as a shared primitive would conflict with the invariants the rest
of the corpus already encodes.

## Decision

The engine has no coroutine runtime. Cross-subsystem multi-frame sequencing uses one of:

- **Timelines** for cinematics, music, and timed events.
- **Behavior trees** for AI multi-frame sequencing.
- **Delayed events** (fire-and-forget) for ad-hoc scheduling.

The `CoroutineState` type inside the scripting crate is a codegen-internal yield-lowering
state machine, not a shared primitive. It is renamed to `SuspendState` and scoped to the
scripting crate. It is not exported.

This ADR explicitly **reverses** the 2026-04-12 design-review P1 task #20 ("Extract
coroutines to `core-runtime/coroutines.md`").

## Consequences

- No `core-runtime/coroutines.md` is written.
- Scripting's yield lowering remains internal to the scripting crate.
- Multi-frame sequencing across subsystems is documented per consumer:
  - AI: behavior trees
  - Animation: timelines + state machine
  - Cinematics: timelines
  - Quest / dialogue: directed-graph nodes with frame-driven evaluation
- The
  [`design-review.md` Status Update P1 #20](../../design/design-review.md#p1--fix-before--during-the-first-implementation-milestones)
  marks the task `[r] Reversed` and points here.

## Alternatives Considered

- **Extract coroutines to core-runtime** (the original recommendation) — would create a
  shared primitive that the rest of the corpus explicitly avoids.
- **Allow coroutines only in scripting** — accepted as the current state, but explicitly
  *not* exposed as a shared primitive. `SuspendState` stays internal.

## References

- [docs/design/design-review.md](../../design/design-review.md) §2.9 and P1 task #20
- [docs/design/integration/high-level.md](../../design/integration/high-level.md) G1
- [docs/design/game-framework/scripting.md](../../design/game-framework/scripting.md)
