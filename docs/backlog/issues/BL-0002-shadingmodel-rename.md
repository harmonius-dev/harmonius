---
blocked_by: []
blocks: []
created: 2026-05-20
domain: rendering
effort: S
id: BL-0002
labels: [coverage:design, domain:rendering, p1, status:triage, type:tech-debt]
priority: P1
status: triage
title: Resolve ShadingModel vs ShadingModelId duplication
---

## Resolve `ShadingModel` vs `ShadingModelId` duplication

### Context

The 2026-04-12 design review §2.2 noted that `ShadingModel` and `ShadingModelId` are the same
concept under two names:

- `rendering/rendering-core.md` defines `ShadingModel` as the runtime enum.
- `rendering/render-styles.md` defines `ShadingModelId` for serialization and material graph
  export.

Two names for the same concept causes drift in error messages, editor enums, and shader codegen.

### Acceptance criteria

- [ ] Pick one name (`ShadingModel`) and remove the other.
- [ ] Update `rendering-core.md` and `render-styles.md` consistently.
- [ ] Update material graph codegen output to emit the canonical name.
- [ ] `canonical-owners.md` row for `ShadingModel` flips to `Owned`.

### Verification

`grep -r 'ShadingModelId' docs/` returns zero matches.

### References

- [docs/design/design-review.md §2.2](../../design/design-review.md#22-foundational-type-duplication)
- [docs/design/canonical-owners.md](../../design/canonical-owners.md)
- [docs/design/rendering/rendering-core.md](../../design/rendering/rendering-core.md)
- [docs/design/rendering/render-styles.md](../../design/rendering/render-styles.md)
