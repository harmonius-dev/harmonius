---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: S
id: BL-0007
labels: [coverage:design, domain:core-runtime, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Move CompileError to single owner (error.md)
---

# Move `CompileError` to single owner (`error.md`)

## Context

`CompileError` is currently defined in four places:

- `core-runtime/error.md` (canonical owner per `canonical-owners.md`)
- `core-runtime/game-loop.md`
- `game-framework/scripting.md`
- `core-runtime/algorithms.md`

The duplicates have inconsistent variants and missing documentation.

## Acceptance criteria

- [ ] `core-runtime/error.md` is the sole definition of `CompileError`.
- [ ] All other docs reference it via cross-link instead of redefining.
- [ ] Variants cover scripting compile errors, schedule build errors, and graph codegen
      errors.
- [ ] `canonical-owners.md` row for `CompileError` flips from `Pending consolidation` to
      `Owned`.

## Verification

`grep -rE 'enum CompileError' docs/design/` returns exactly one definition in `error.md`.

## References

- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/core-runtime/error.md](../../design/core-runtime/error.md)
