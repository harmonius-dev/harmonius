# integration_ai_physics

Reference implementation and CI-runnable tests for the AI ↔ physics integration design
([`ai-physics.md`](../../docs/design/integration/ai-physics.md)).

This crate provides pure, deterministic helpers plus a small in-memory scene used by unit tests. It
is not wired into a runtime loop yet; it encodes the data contracts and fallback counters (FM-*)
called out in the design.

## Workspace merge note

Plan branches often use a **virtual workspace** with only this crate. When merging into the full
Harmonius repo, **append** `crates/integration_ai_physics` to the existing `[workspace].members`
list instead of replacing the root workspace definition.

## Scope notes

- **3D harness** — public types use `glam::Vec3`; there is no separate 2D API yet.
- **Criterion** — benches pull Criterion (and its Rayon graph) as **dev-dependencies** only; the
  library crate stays free of Rayon in non-bench builds.
