# ECS-primary architecture

## Status

Accepted — 2024-09-18 (backfilled 2026-05-20)

## Context

Game engines historically expose a heterogeneous stack: scene-graph hierarchies (Unity, Unreal),
actor-component objects, and inheritance trees. ECS (Entity Component System) is increasingly the
architecture of choice for performance-critical engines (Bevy, EnTT, Unreal Mass) because it gives
cache-friendly iteration, parallelism, and clean data ownership.

Adopting ECS as a partial pattern (alongside actor trees) leads to the worst-of-both: confused
ownership, double-bookkeeping, and inconsistent threading rules.

## Decision

The engine is ECS-primary. Approximately 90% of simulation data lives as ECS components and ~90% of
gameplay logic as ECS systems. The remaining 10% is explicitly justified:

- **Audio runtime.** Real-time mixer thread with sub-0.5 ms budget. Bridges via SPSC command
  queue.
- **GPU resource management.** Descriptor pools, command allocators, and swap chains live
  inside the rendering backend.
- **Windowing and platform event loops.** OS event loops are not ECS systems; they forward
  events into ECS via channels.
- **Shader compilation.** `glslc` subprocess is a build-time step.
- **Asset processing internals.** Importers manage internal state but expose results as
  components.

There are no separate scene graphs, actor trees, or world singletons.

## Consequences

- All gameplay is built by composing ECS components and systems plus the four data primitives
  ([ADR-0011](ADR-0011-composition-gameplay.md)).
- Saving and replication are uniform: serialize the world, restore the world.
- Threading is uniform: schedule systems against component access sets; the scheduler
  handles parallelism.
- Existing engine patterns (singleton managers, blackboard globals) are explicitly forbidden;
  blackboards become per-entity ECS components.
- Static dispatch is preferred; `dyn` requires explicit justification. See
  [docs/design/constraints.md](../../design/constraints.md).

## Alternatives Considered

- **Hybrid ECS + actor tree** — rejected because the dual ownership model is the primary cost
  driver in legacy engines.
- **Pure ECS** — too narrow; some 10% exceptions (audio RT, GPU resources, OS event loop)
  cannot be ECS without sacrificing performance or platform fidelity.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Architecture"
- [docs/design/core-runtime/ecs.md](../../design/core-runtime/ecs.md)
- [docs/architecture.md](../../architecture.md) "High-Level Architecture"
