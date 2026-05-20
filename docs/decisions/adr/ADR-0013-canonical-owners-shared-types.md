# Canonical owners for shared types

## Status

Accepted — 2026-05-20

## Context

The [2026-04-12 design review](../../design/design-review.md) §2.2 catalogued nine shared types that
were defined or redefined in multiple files with subtly different APIs:

- `Handle<T>`, `HandleMap`, `SlotMap`
- `UniformGrid<T>` (three independent definitions)
- `ConditionExpr`, `ConditionRegistry`
- `ModOp` / `ModifierOp` (incompatible enums)
- Blackboard backing store (8+ restatements)
- `ShadingModel` / `ShadingModelId`
- `VoiceStream`
- `DirtyRegionSet`
- `RenderFrame`, `InterpolatedTransform`, `PreviousGlobalTransform2D`

Foundational primitives existed (e.g., `core-runtime/primitives.md` after the P0 work) but no single
index recorded ownership. New design docs continued to introduce duplicates because no authoritative
list existed to consult.

## Decision

A single normative registry — [`docs/design/canonical-owners.md`](../../design/canonical-owners.md)
— names the sole owner for every shared type. Each row carries a status:

| Status                  | Meaning                                                            |
|-------------------------|--------------------------------------------------------------------|
| Owned                   | Single owner; no duplicates remain                                 |
| Pending consolidation   | Sole owner declared; duplicate definitions still exist             |
| Pending split-rename    | Two distinct concepts share a name; rename pending                 |
| Pending creation        | Sole owner not yet declared; type referenced but undefined         |
| Reversed                | Earlier consolidation plan reversed; documented in linked ADR/note |

The registry covers container primitives, IDs, errors, frame and time types, hot-reload protocol,
graph runtime, data-systems shared types, rendering shared types, audio and networking shared types,
and tooling/platform shared types.

Every design and integration document MUST reference, not redefine, types listed in the registry.
Adding a duplicate is a folder-rule violation. Discovering a duplicate creates a backlog issue under
[BL-0001..BL-0014](../../backlog/index.md).

## Consequences

- Reviewers have a single page to consult before approving any new type.
- The `Pending consolidation` rows are tracked as backlog issues, each with a clear
  acceptance criterion (delete N duplicate definitions, update X cross-references).
- The glossary's "Harmonius-coined cross-cutting types" section lists the most-referenced
  types by canonical owner.
- Adding a new shared type requires a registry update first; there is no path to add a
  duplicate accidentally.

## Alternatives Considered

- **No central registry** — relies on review discipline; the design review found that
  discipline-only enforcement led to the §2.2 problem.
- **Type registry inside `constraints.md`** — `constraints.md` is constraint-focused;
  ownership is a design-doc concern that deserves its own page.

## References

- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/design-review.md](../../design/design-review.md) §2.2
- [docs/design/core-runtime/primitives.md](../../design/core-runtime/primitives.md)
- [docs/glossary.md](../../glossary.md)
