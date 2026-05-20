---
blocked_by: []
blocks: []
created: 2026-05-20
domain: core-runtime
effort: L
id: BL-0016
labels: [coverage:design, domain:core-runtime, domain:tools, p2, status:triage, type:design]
priority: P2
status: triage
title: Author docs/design/core-runtime/codegen-pipeline.md
---

## Author `core-runtime/codegen-pipeline.md`

### Context

`scripting.md` and `visual-editors.md` both claim parts of the codegen pipeline. The 2026-04-12
review §3.7 / P2 #56 recommended a single owner for IR, optimizer, and backends.

### Acceptance criteria

- [ ] New `core-runtime/codegen-pipeline.md` documents the IR, optimizer passes, and the
      `RustBackend`, `GlslBackend`, `TypeDescriptorBackend` from `graph-runtime.md`.
- [ ] `scripting.md` and `visual-editors.md` reference this doc instead of redefining
      pieces of the pipeline.
- [ ] Companion `codegen-pipeline-test-cases.md` covers IR construction, optimization, and
      each backend's output.
- [ ] Hot reload integration is explicit (codegen output → middleman .dylib → reload).

### Verification

A reader of `codegen-pipeline.md` understands the flow from visual graph to compiled Rust / GLSL
without reading scripting or visual-editors.

### References

- [docs/design/design-review.md §3.7 / P2 #56](../../design/design-review.md#37-platform--tools--game-framework--content-pipeline)
- [docs/design/core-runtime/graph-runtime.md](../../design/core-runtime/graph-runtime.md)
- [ADR-0005 codegen middleman .dylib](../../decisions/adr/ADR-0005-codegen-middleman-dylib.md)
