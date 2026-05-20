# Harmonius Design Review

Original audit date: 2026-04-12. Status update: 2026-05-20.

Original scope: all design documents under `docs/design/` (~170 files across 17 domains plus 50
pair-wise integration designs at the time of the audit; now 77 subsystem docs + 62 integration
docs).

> **This document is a status snapshot, not a live backlog.** Open items have moved to
> [`docs/backlog/`](../backlog/index.md). The 2026-04-12 audit body below is preserved verbatim for
> historical context. The Status Update section that immediately follows reflects the 2026-05-20
> deslop pass.

## Status Update — 2026-05-20

The deslop pass (`cursor/deslop-docs-corpus-e306`) closed the foundational P0 items, installed the
[canonical-owner registry](canonical-owners.md), wired up
[shared conventions](integration/shared-conventions.md) and
[shared messaging capacities](integration/shared-messaging-capacities.md), and migrated remaining
work to [`docs/backlog/`](../backlog/index.md). Counts below come from the
[2026-05 audit](../coverage/audits/2026-05-audit.md).

### Status legend

| Marker | Meaning                                                                |
|--------|------------------------------------------------------------------------|
| `[x]`  | Done — landed and verified                                             |
| `[~]`  | Partial — substantively done, residual cleanup tracked in backlog      |
| `[ ]`  | Open — tracked in [`docs/backlog/`](../backlog/index.md)               |
| `[r]`  | Reversed — earlier recommendation undone; see linked ADR               |

### P0 — must land before implementation begins

| Item | Status | Notes |
|------|--------|-------|
| 1. `core-runtime/graph-runtime.md` | `[x]` | Lives at [graph-runtime.md](core-runtime/graph-runtime.md) |
| 2. `core-runtime/primitives.md` | `[x]` | Sole owner of primitives; see [canonical-owners.md](canonical-owners.md) |
| 3. `core-runtime/ids.md` | `[x]` | `StableId`, `NetworkEntityId` defined |
| 4. `core-runtime/hot-reload-protocol.md` | `[x]` | Single protocol for all hot-reload consumers |
| 5. Bridge types (`RenderFrame`, `InterpolatedTransform`, `CompileError`, `IoError`, `NetworkEntityId`) | `[~]` | `RenderFrame`, `InterpolatedTransform`, `IoError`, `NetworkEntityId` owned. `CompileError` still defined in 4 docs — [BL-0007](../backlog/issues/BL-0007-compileerror-single-owner.md). |
| 6. Async purge across design tree | `[~]` | Game-runtime async fully removed. Residual prose-only mentions in migration tables — [BL-0040](../backlog/issues/BL-0040-async-cleanup-design.md). |
| 7. Rewrite `network-transport.md` sync | `[x]` | Zero `tokio::` references. |
| 8. `data-systems/composition.md` | `[x]` | Walks through quest/ability/inventory authoring. |
| 9. Game-loop ↔ ECS circular dependency | `[ ]` | [BL-0015](../backlog/issues/BL-0015-game-loop-ecs-circular.md). |
| 10. Editor shadow world | `[x]` | `EditorWorld` + `EventBridge` in [editor-core.md](tools/editor-core.md). |

### P1 — fix before / during the first implementation milestones

| Item | Status | Notes |
|------|--------|-------|
| 11. Unify `ModOp` | `[ ]` | [BL-0001](../backlog/issues/BL-0001-modop-unification.md). |
| 12. Make `NodeStatus` generic | `[ ]` | Tracked alongside [BL-0005](../backlog/issues/BL-0005-conditionexpr-registry-scope.md). |
| 13. Remove `HashMap` from `Archetype` | `[ ]` | [BL-0008](../backlog/issues/BL-0008-hashmap-archetype.md). |
| 14. Declare `Material` type | `[ ]` | [BL-0009](../backlog/issues/BL-0009-material-type-unification.md). |
| 15. Design PSO cache | `[~]` | Doc exists at [pipeline-state-cache.md](rendering/pipeline-state-cache.md); recovery + GC remaining in [BL-0010](../backlog/issues/BL-0010-pso-cache-design.md). |
| 16. Meshlet asset pipeline | `[~]` | Doc exists at [meshlets.md](rendering/meshlets.md); asset import end-to-end in [BL-0011](../backlog/issues/BL-0011-meshlet-asset-pipeline.md). |
| 17. Specify barrier analysis | `[ ]` | Tracked under [BL-0010](../backlog/issues/BL-0010-pso-cache-design.md). |
| 18. Unify 2D / 3D render graph | `[ ]` | [BL-0012](../backlog/issues/BL-0012-2d-3d-render-graph-merge.md). |
| 19. Split camera doc (brain vs render) | `[~]` | [camera-rendering.md](rendering/camera-rendering.md) created; brain split tracked in [BL-0013](../backlog/issues/BL-0013-camera-split.md). |
| 20. Extract coroutines to core-runtime | `[r]` | Reversed by [ADR-0014](../decisions/adr/ADR-0014-no-coroutine-runtime.md). Engine has no coroutine runtime; scripting's `SuspendState` is codegen-internal. |
| 21. Move cloud sync / achievements to platform-services | `[~]` | Save-system reduced; full move tracked in [BL-0024](../backlog/issues/BL-0024-save-schema-versioning.md). |
| 22. Pick pathfinding algorithm | `[ ]` | [BL-0027](../backlog/issues/BL-0027-pathfinding-algorithm-pick.md). |
| 23. QUIC stream multiplexing | `[ ]` | [BL-0022](../backlog/issues/BL-0022-quic-stream-multiplexing.md). |
| 24. Replication delta compression | `[ ]` | [BL-0023](../backlog/issues/BL-0023-replication-delta.md). |
| 25. Save schema versioning | `[ ]` | [BL-0024](../backlog/issues/BL-0024-save-schema-versioning.md). |
| 26. Unify blackboard on ECS components | `[ ]` | [BL-0006](../backlog/issues/BL-0006-blackboard-as-component.md). |
| 27. Voice-chat ownership split | `[ ]` | [BL-0003](../backlog/issues/BL-0003-voicestream-ownership.md). |
| 28. `core-runtime/error.md` | `[x]` | Owns `EngineError`, `ToEngineError`. |
| 29. `core-runtime/io.md` | `[x]` | Owns `IoRequest`/`IoResponse` and main-thread drain. |
| 30. Integration shared conventions | `[~]` | [shared-conventions.md](integration/shared-conventions.md) exists; ~27 pair docs still need to reference SC-N — handled in this deslop pass. |
| 31. Flesh out `input-camera.md` and `input-ui.md` | `[x]` | Both substantive. |
| 32. `audio-physics.md` and `geometry-vfx.md` | `[x]` | Both exist. |
| 33. Composition-density performance profile | `[ ]` | Tracked under [BL-0030](../backlog/issues/BL-0030-data-tables-query-api.md). |
| 34. `GameTime`, pause, determinism for sim primitives | `[ ]` | [BL-0026](../backlog/issues/BL-0026-game-time-pause-determinism.md). |

### P2 — quality and completeness

| Item | Status | Notes |
|------|--------|-------|
| 35. Move impl pseudocode out of integration docs | `[~]` | Largely done; residual instances tracked under integration cleanup. |
| 36. Skinning weight format | `[ ]` | [BL-0017](../backlog/issues/BL-0017-skinning-weight-format.md). |
| 37. Cloth + fracture formats | `[ ]` | [BL-0018](../backlog/issues/BL-0018-cloth-fracture-formats.md). |
| 38. Fluid solver variant | `[ ]` | [BL-0019](../backlog/issues/BL-0019-fluid-solver-variant.md). |
| 39. ACL animation compression | `[ ]` | [BL-0020](../backlog/issues/BL-0020-animation-compression-acl.md). |
| 40. Determinism contracts | `[ ]` | [BL-0021](../backlog/issues/BL-0021-determinism-contracts.md). |
| 41. GPU timestamp calibration | `[ ]` | Backlog (under [BL-0022](../backlog/issues/BL-0022-quic-stream-multiplexing.md)-adjacent profiler work). |
| 42. Dormancy thresholds (networking-ecs) | `[~]` | Defined in [networking-ecs.md](integration/networking-ecs.md); thresholds open. |
| 43. `NameplateBuffer` capacity | `[~]` | Pulled into [shared-messaging-capacities.md](integration/shared-messaging-capacities.md). |
| 44. Extract `DirtyRegionSet` | `[x]` | Owned in [primitives.md](core-runtime/primitives.md). |
| 45. Extract dispatch / budget primitives | `[x]` | Owned in [primitives.md](core-runtime/primitives.md). |
| 46. `core-runtime/console-variables.md` | `[x]` | Exists. |
| 47. Localization runtime | `[x]` | [game-framework/localization.md](game-framework/localization.md). |
| 48. `platform/crash-reporting.md` | `[x]` | Exists. |
| 49. `platform/telemetry.md` | `[x]` | Exists. |
| 50. `platform/console-integration.md` | `[x]` | Exists. |
| 51. `tools/plugin-marketplace.md` | `[x]` | Exists. |
| 52. `tools/undo-redo.md` | `[x]` | Exists; deepening tracked in [BL-0034](../backlog/issues/BL-0034-undo-redo-deepen.md). |
| 53. `tools/selection-model.md` | `[x]` | Exists. |
| 54. `tools/scene-versioning.md` | `[x]` | Exists. |
| 55. Single shader-compiler service | `[ ]` | [BL-0025](../backlog/issues/BL-0025-shader-compiler-service.md). |
| 56. `codegen-pipeline.md` | `[ ]` | [BL-0016](../backlog/issues/BL-0016-codegen-pipeline-doc.md). |
| 57. IME / ducking / SOFA / WCAG / steering | `[ ]` | [BL-0035](../backlog/issues/BL-0035-input-ime-support.md), [BL-0036](../backlog/issues/BL-0036-audio-ducking.md), [BL-0037](../backlog/issues/BL-0037-hrtf-sofa-loading.md), [BL-0039](../backlog/issues/BL-0039-accessibility-wcag.md), [BL-0041](../backlog/issues/BL-0041-classic-steering-primitives.md). |

### P3 — polish and future-proofing

| Item | Status | Notes |
|------|--------|-------|
| 58. `core-runtime/change-detection.md` | `[x]` | Exists. |
| 59. Integration-boundaries section in timelines | `[ ]` | Backlog. |
| 60. Standardize `FM-N` fallback labels | `[~]` | Adopted in [shared-conventions.md SC-6](integration/shared-conventions.md#sc-6----fallback-mode-naming-fm-n). |
| 61. Algorithm citations universally applied | `[ ]` | Tracked alongside backlog audits. |
| 62. Rename `PhaseNode` / collapse with `Phase` | `[ ]` | Backlog. |
| 63. `SpatialUpdateSystem` phase ordering | `[ ]` | Backlog. |
| 64. Event-channel contention guarantees | `[ ]` | Backlog. |
| 65. Composition tests cross-subsystem | `[ ]` | Backlog. |
| 66. Single canonical `VoiceStream` / `AnimEvent` / `AudioCommand` | `[~]` | Tracked under [BL-0003](../backlog/issues/BL-0003-voicestream-ownership.md) and [BL-0032](../backlog/issues/BL-0032-anim-event-canonical.md). |
| 67. Promote `FrameContext` first-class | `[ ]` | [BL-0014](../backlog/issues/BL-0014-frameworld-promote.md). |
| 68. Factor LOD-tier tables out of integration docs | `[~]` | Animation tier consolidated; remaining backlog. |

### Roll-up

| Phase | Done | Partial | Open | Reversed | Total |
|-------|-----:|--------:|-----:|---------:|------:|
| P0    | 7    | 2       | 1    | 0        | 10    |
| P1    | 6    | 5       | 12   | 1        | 24    |
| P2    | 9    | 4       | 10   | 0        | 23    |
| P3    | 1    | 4       | 6    | 0        | 11    |
| **Total** | **23** | **15** | **29** | **1**  | **68**|

29 fully-open items have moved to [`docs/backlog/`](../backlog/index.md) under `BL-0001` …
`BL-0042`. The 15 partial items have either an issue tracking the residual work or are sufficiently
complete that follow-up is opportunistic.

### Forward pointer

New design-review-style audits land as dated audits under
[`docs/coverage/audits/`](../coverage/audits/) rather than amending this file. This keeps the
historical 2026-04-12 audit pristine and lets coverage gates evolve.

---

## Executive Summary

Harmonius has a **strong architectural core** — ECS-primary, codegen-driven, static dispatch,
deterministic fixed-timestep, platform-native I/O, clear thread model — and the overall
decomposition (Foundation → Domain → Simulation → Data Systems → Application) is sound and layered
cleanly. Integration docs are unusually rigorous: 27 of 29 A–L pairs and 19 of 21 M–Z pairs meet the
template, with concrete types, channel capacities, and fallbacks.

The design is also **simpler than it looks** once the composition model clicks: four data primitives
(graphs, tables, attributes, containers) plus four simulation primitives (grids, awareness,
timelines, event logs) genuinely cover the feature matrix.

However, the corpus suffers from five **systemic** flaws that cut across every subsystem and will
cause pain during implementation if not addressed first:

1. **Graph-system proliferation.** At least six independent graph runtimes (`DirectedGraph`,
   logic/scripting, material, behavior tree, VFX effect, animation state machine, procedural
   generation, timeline tracks) each reinvent validation, topological sort, hot-reload, and codegen.
   There is no `GraphRuntime<N, E>` trait in core-runtime.
2. **Foundational type duplication.** `Handle<T>`, `UniformGrid<T>`,
   `ConditionExpr`/`ConditionRegistry`, `ModOp`/`ModifierOp`, `Blackboard`/`BTreeMap` constraint,
   `ShadingModel`/`ShadingModelId`, `VoiceStream`, `InterpAlpha`, `DirtyRegionSet`, `RenderFrame`,
   and `InterpolatedTransform` are defined — or redefined, or left undefined — in several files at
   once.
3. **Undefined bridge types between layers.** `RenderFrame`, `CompileError`, `IoError`,
   `InterpolatedTransform`, `NetworkEntityId`, the PSO cache, meshlet asset format, descriptor
   layout inference, and the hot-reload wire protocol are all *referenced* but never *defined*.
   These are the seams between subsystems — exactly the types that most need to be concrete.
4. **Constraint violations in networking, tools, and a few subsystems.** 30 design files contain
   `async fn`, `.await`, `tokio::`, or `Future<>` despite the explicit "no async/await in engine"
   constraint (`constraints.md:96`). Some are correctly scoped to backend services;
   `networking/network-transport.md:1238–1240` is a game-runtime violation that must be rewritten
   sync.
5. **Cross-document prose duplication.** Core constraints ("`Arc` only for immutable", "no `HashMap`
   on hot paths", "`BTreeMap` for blackboard", "MPSC preferred over SPSC", "rkyv only for
   persistence") are repeated verbatim in 15+ integration docs instead of being centralized in
   `constraints.md`. Same for channel-capacity tables, LOD tier tables, and fallback conventions.

This review organizes findings in two parts: **Section 2** catalogs the cross-cutting issues;
**Section 3** gives per-subsystem findings; **Section 4** provides a prioritized task list with
file-level actions.

## 1. What the design gets right

- **Layered architecture is clean.** Foundation → Mid-Level → Domain → Data Systems → Simulation →
  Application has almost no circular dependencies, and the `architecture.md` module map matches the
  directory layout.
- **ECS-primary decision is consistently applied.** Exceptions (audio RT, GPU management, windowing,
  shader tools, asset internals) are explicit and justified.
- **Thread model is unambiguous.** Main / Workers / Render (pinned) / Audio RT with MPSC and
  triple-buffer handoffs; `high-level.md` and `performance-budget.md` give concrete capacities,
  fallbacks, and per-phase budgets scaled across 30/60/120 fps tiers.
- **Composition model works.** The feature matrix in `architecture.md` maps every gameplay feature
  (quests, abilities, inventory, crafting, NPC memory, etc.) to a combination of the four data
  primitives and four simulation primitives. Nothing is locked to a dedicated subsystem.
- **Integration docs are substantive.** Unusually for pair-wise integration designs, most include
  concrete channel names, capacities, fallbacks, and classDiagrams rather than prose.
  `networking-ecs`, `networking-physics`, `physics-spatial-index`, `save-system-serialization`,
  `scripting-ecs`, and `rendering-vfx` are exemplary.
- **Performance budget is complete.** Every phase, subsystem, thread, and platform tier has numbers
  that add up, with headroom called out explicitly.
- **Codegen-first discipline.** Zero runtime reflection; every user-authored thing becomes real Rust
  or GLSL in the middleman `.dylib`. This removes a large class of dispatch, versioning, and
  hot-reload bugs that plague engines with interpreters or reflection-based serialization.

## 2. Cross-cutting design flaws

### 2.1 Graph-system proliferation

At least **eight independent graph runtimes** exist in the corpus, each with its own validation,
topological sort, codegen, hot-reload, and editor integration:

| Graph                 | Owner doc                                   | Output      |
|-----------------------|---------------------------------------------|-------------|
| Logic / scripting     | `game-framework/scripting.md`               | Rust source |
| Directed graph (data) | `data-systems/directed-graphs.md`           | Rust source |
| Behavior tree         | `ai/behavior.md`                            | Rust source |
| Material graph        | `rendering/render-styles.md`                | GLSL        |
| VFX effect graph      | `vfx/effects.md`                            | GLSL        |
| Animation state mach. | `animation/state-machine.md`                | Rust source |
| Procedural generation | `geometry/procedural-generation.md`         | Rust + GLSL |
| Timeline track        | `simulation/timelines.md`                   | Rust source |

Each reinvents: cycle detection, topological sort, DAG validation, pin-type inference, constant
folding, dead-code elimination, hot-reload triggering, editor UI, error reporting, and schema
versioning. The review agents flagged this from three different angles (rendering, physics/vfx/anim,
data systems, platform/tools).

**Root cause:** No `harmonius_core::graph` module defining a `GraphRuntime<N, E, Out>` trait that
the eight systems parameterize. The shared ground — "validate a DAG, codegen some target, hot-reload
the output, surface errors back into the editor" — is written eight times.

**Fix:** Extract a `core-runtime/graph-runtime.md` design doc with:

- `trait GraphNode { type Output; }`, `trait GraphEdge`, `trait GraphValidator`
- `DagValidator`, `TopologicalSorter`, `CycleDetector` as concrete shared types
- `trait CodegenBackend { fn emit(&self, ir: &GraphIr) -> Result<String, CodegenError>; }` with
  concrete `RustBackend`, `GlslBackend`, `TypeDescriptorBackend` implementations
- A shared `HotReloadBarrier` type in `core-runtime/hot-reload-protocol.md`
- A shared `GraphEditor<N, E>` widget in `tools/visual-editors.md`

Every existing graph-owning design then shrinks to "node palette + evaluation semantics". Current
surface area should drop by **at least 40%**.

### 2.2 Foundational type duplication

The following types are defined in multiple places, often with subtly different APIs:

| Type                     | Defined in                                                                                                    | Problem                                          |
|--------------------------|---------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| `Handle<T>`, `HandleMap` | `core-runtime/memory-async-io.md`, `core-runtime/algorithms.md`                                               | Two independent definitions, claimed as "Tier 1" in both |
| `UniformGrid<T>`         | `core-runtime/spatial-index.md`, `core-runtime/algorithms.md`, `simulation/grids-volumes.md`                   | Three with different APIs; network AOI vs gameplay propagation vs core primitive |
| `ConditionExpr` + `ConditionRegistry` | `data-systems/attributes-effects.md`, `data-systems/directed-graphs.md` | Unclear if registries are shared or per-subsystem |
| `ModOp` / `ModifierOp`   | `data-systems/attributes-effects.md`, `data-systems/containers-slots.md`                                      | Two enums with overlapping but incompatible semantics |
| Blackboard backing store | Repeated in 7+ integration docs                                                                               | Same "use `BTreeMap` not `HashMap`" constraint repeated |
| `ShadingModel` / `ShadingModelId` | `rendering/rendering-core.md`, `rendering/render-styles.md`                                          | Same concept under two names                     |
| `VoiceStream`            | `audio/audio.md`, `networking/network-services.md`                                                            | Two definitions of the "same" voice type; ownership unclear |
| `DirtyRegionSet`         | `integration/rendering-grids-volumes.md`, `integration/save-system-serialization.md`, `integration/rendering-geometry.md` | Three ad-hoc dirty-region implementations |
| `PreviousGlobalTransform2D` | (undefined)                                                                                                | 3D has `Previous…`, 2D does not, breaking interpolation for 2D |

**Root cause:** No canonical home for shared primitives. `core-runtime/algorithms.md` *tries* to be
that home but only partially supersedes definitions elsewhere.

**Fix:**

1. Declare `core-runtime/algorithms.md` (or a new `core-runtime/primitives.md`) as the **sole**
   owner of `Handle<T>`, `HandleMap<T>`, `SlotMap<T>`, `UniformGrid<T>`, `SortedVecMap<K, V>`,
   `RingBuffer<T, N>`, `DirtyRegionSet`, `DispatchTable<F>`, `BudgetAllocator`, `DeterministicRng`.
2. Delete duplicate definitions from every subsystem doc and replace with one-line references.
3. Rename the collision: if two docs really do need different "uniform grids", rename one to
   `AoiGrid` (networking) and keep `UniformGrid<T>` for gameplay.
4. Unify `ModOp`: one enum in `core-runtime/primitives.md`, used by both attributes and containers.
5. Unify `ShadingModel`: pick one name, delete the other.
6. Unify `VoiceStream`: transport type in `networking/`, codec/spatialization in `audio/`. Add a
   short note in both with a cross-reference.

### 2.3 Undefined bridge types

The types that carry data across subsystem boundaries are **exactly the ones not defined** — the
opposite of what the design process should deliver first:

| Type                      | Referenced in                             | Defined in              |
|---------------------------|-------------------------------------------|-------------------------|
| `RenderFrame` struct      | `game-loop.md`, `rendering-core.md`, `high-level.md` | Partial, inconsistent |
| `InterpolatedTransform`   | `game-loop.md`                            | Nowhere                 |
| `CompileError`            | `game-loop.md` (`Schedule::build`)        | Nowhere                 |
| `IoError`                 | `memory-async-io.md`                      | Nowhere                 |
| `NetworkEntityId`         | replication assumed                       | Nowhere                 |
| PSO cache                 | `rendering/render-pipeline.md` RF-9       | TODO                    |
| Meshlet asset format      | `rendering/rendering-core.md`, geometry   | TODO                    |
| Descriptor layout inference | `render-pipeline.md`                    | TODO                    |
| Hot-reload wire protocol  | 6+ subsystems                             | Nowhere                 |
| Skinning weight format    | `animation/skeletal.md`                   | TODO                    |
| Cloth constraint types    | `physics/advanced.md`                     | TODO                    |
| Fracture pattern format   | `physics/advanced.md`                     | TODO                    |
| `ConVar` / console vars   | Debug visualization needs it              | Nowhere                 |
| Localization table loader | Runtime string lookup                     | Nowhere                 |

**Fix:** Every one of these should be a short (1–3 page) design doc or a clearly-owned section in an
existing doc, landing **before** any implementation work begins. Without them, implementation teams
will invent their own incompatible versions.

### 2.4 ID / handle fragmentation

The engine currently defines (or implies) `Entity`, `EntityId`, `ComponentId`, `ComponentTypeId`,
`SystemId`, `GraphInstanceId`, `GraphId`, `AssetId`, `AssetHandle`, `ContentHash`, `NodeId`,
`BoneIndex`, `Handle<T>`, `VoiceId`, `NetworkEntityId`, `SaveSlotId`, `PanelId`, `RowId`, `TableId`,
`ColumnId`, `SocketDefinitionId`, `ContainerDefinitionId`, `MeterDefinitionId`, `AttributeDefId`,
`EventLogId`, `EntryId`, `BlackboardKey`, and more — each defined independently in its owning doc.
Many are the same underlying generational index; a few have stability guarantees across save/load;
none share a common trait.

**Fix:** `core-runtime/ids.md` — declare `trait StableId`, a `GenerationalIndex` newtype template,
and a single convention for which IDs persist across saves, hot-reloads, and network boundaries.
Migrate existing IDs to the new scheme by reference (no content rewrite needed).

### 2.5 Error hierarchy

Every subsystem declares its own error enum (`SaveError`, `LoadError`, `GraphError`, `CycleError`,
`TransitionError`, `ValidationError`, `HotReloadError`, `ImportError`, `RebindResult`, …) with no
shared supertype. `CompileError`, `IoError`, and `MigrationError` are referenced but undefined.

**Fix:** `core-runtime/error.md` — define `enum EngineError` as a sum type over the major categories
(`Io`, `Serialization`, `Validation`, `Codegen`, `Platform`, `Asset`), plus a `trait ToEngineError`
so subsystems can keep their own enums but converge at the boundary. Does not require rewriting
subsystem errors.

### 2.6 Hot-reload protocol is everywhere and nowhere

Seven subsystems claim hot-reload support: logic graphs (`scripting.md`), material graphs
(`render-styles.md`), shaders (`render-pipeline.md` RF-9), assets (`asset-pipeline.md` F-12.4),
logic-graph state preservation (`scripting.md` F-13.4.3), table schemas (`data-tables.md`), and the
middleman `.dylib` itself (`constraints.md`). Each describes the feature in prose with slightly
different semantics (atomic swap, drain-then-swap, pointer update, recompile, reload).

No doc defines:

- The wire format for reload requests (editor → runtime).
- State-migration callback signature (old layout → new layout).
- The drain-then-swap synchronization barrier (which frame the swap happens on).
- Rollback semantics on compile error.
- Interaction with in-flight graph instances, PSO caches, and asset handles.

**Fix:** `core-runtime/hot-reload-protocol.md` as the single source of truth, referenced from all
seven consumers. Define `HotReloadManager`, `ReloadRequest`, `ReloadBarrier`, `StateMigrationFn`,
`ReloadResult` in one place.

### 2.7 No-async constraint is violated in game-runtime code

`constraints.md:96` is unambiguous: "No async/await in engine, editor, or headless game server."
However, 30 design files contain `async fn`, `.await`, `Future<>`, or `tokio::`:

| Doc                                   | Status                                    |
|---------------------------------------|-------------------------------------------|
| `networking/network-transport.md:1238–1240` | **Violation** — claims `tokio::net` for the game transport |
| `networking/network-infrastructure.md:277, 287, 1250, 2014–2026` | Backend services (matchmaker, TiKV) — **allowed** per constraint, but the doc itself says "Replace all `Future` return types"; it's self-contradictory |
| `audio/audio.md`                      | Streaming playback uses Tokio — **violation**, should use platform-native I/O |
| `tools/level-world.md`                | Self-flagged "replace 10+ `async fn` with sync" |
| `tools/profiler.md`                   | Self-flagged "remove async/await and Tokio" |
| `tools/build-deploy.md`, `content-services.md`, `team-tools.md` | Mixed; partly backend (OK), partly editor (not OK) |
| `game-framework/scripting.md`, `save-system.md` | Should be sync |
| Platform / core-runtime / content-pipeline | Must be sync |

The reviewer reports treated this as a single critical violation. The reality is worse: this is a
**systemic hygiene failure** in the design docs. Many of them know they need to be sync (they
contain TODO notes to "replace async") but haven't been updated.

**Fix:** Mechanical sweep — grep for `async fn`, `.await`, `Future<`, `tokio::` across
`docs/design/` and replace with the request/handle pattern from constraints.md § 4 ("User-Facing API
Principle"). Permitted exceptions (backend services on Kubernetes) must be explicitly scoped inside
each affected doc.

### 2.8 Integration-doc redundancy

The 50 pair-wise integration docs are high quality, but the template compliance has a cost: the same
constraints and channel specs are repeated over and over.

| Repeated prose                                                    | Times |
|-------------------------------------------------------------------|-------|
| "Use `BTreeMap` or sorted `Vec`, not `HashMap`" (blackboard)      | 8+    |
| "`Arc` only for immutable shared data"                            | 7+    |
| "rkyv for persistence, no serde"                                  | 5+    |
| "MPSC preferred over SPSC"                                        | 6+    |
| `AudioCommand` channel capacity (4096)                            | 4+    |
| AnimationLodTier table                                            | 2+    |
| Fallback policy list                                              | every doc |

**Fix:** A short `integration/shared-conventions.md` (or expand `constraints.md`) with every
cross-cutting rule stated once. Integration docs reference it and stop restating. Estimated saving:
~400 lines of repeated prose, plus the review benefit that any future constraint change updates one
place.

### 2.9 Game-framework scope creep

`game-framework/` is supposed to hold only **camera, save system, scripting** — three primitives on
top of the data systems / simulation composition model. In practice:

- `camera.md` contains spring-arm collision retraction, cine camera, picture-in-picture, and camera
  sequencer — the last three are rendering or sequencing concerns.
- `save-system.md` pulls cloud sync, achievement queuing, and platform SDK hooks — these belong in
  `platform-services.md`.
- `scripting.md` defines coroutines — ~~a general-purpose core-runtime primitive~~.
  **Reversed 2026-04-12** — the corpus already decided the engine has no coroutine runtime (see the
  "no coroutines" invariant repeated across `integration/ai-scripting.md`,
  `integration/directed-graphs-scripting.md`, `integration/timelines-scripting.md`, and G1 in
  `integration/high-level.md`). The scripting crate's yield lowering is a codegen-internal state
  machine (`SuspendState`), not a shared primitive. Cross-subsystem multi-frame sequencing goes
  through timelines (cinematics / music / timers), behavior trees (AI), or delayed events
  (fire-and-forget). No `core-runtime/coroutines.md` exists.

**Fix:** Split camera doc into `game-framework/camera.md` (brain, priority, blend) and
`rendering/camera-rendering.md` (spring arm, lens, viewport composition). Move cloud/SDK hooks from
save-system to platform-services. ~~Extract coroutine support to `core-runtime/coroutines.md`.~~
Rename `CoroutineState` to `SuspendState` in `scripting.md` and keep it scoped to the scripting
crate; do not export as a shared primitive.

## 3. Per-subsystem findings

### 3.1 Core Runtime

**Strengths:** ECS, game loop, scheduler, events, and memory designs are internally consistent.
Scene transforms and spatial index are cleanly separated.

**Gaps:**

- `RenderFrame`, `InterpolatedTransform`, `CompileError`, `IoError` are referenced but not defined
  (see §2.3).
- `Archetype` uses `HashMap<ComponentId, Column>` on the hot query path, violating the "no HashMap
  on hot paths" rule (`ecs.md` line ~348).
- `ObserverRegistry` is defined twice under the same name in `ecs.md` and `events-plugins.md` for
  different concepts. Rename one.
- `PhaseNode` enum in `game-loop.md` is too broad; most phases need only `Systems`, and the enum
  creates a naming collision with the `Phase` label.
- Change-detection tick lifecycle (when does the tick increment?) is unspecified.
- `SpatialUpdateSystem` ordering relative to game-loop phases is not stated.
- Persistent-stream vs. double-buffered events (`EventChannel<T>` vs `PersistentStream<T>`) have no
  clear selection guidance.
- 2D `PreviousGlobalTransform2D` is missing, so 2D rendering cannot interpolate like 3D.
- `CommandBuffer` accretes domain-specific extensions across ecs / events-plugins /
  scene-transforms.
- `Observer` vs event-observer naming collision needs disambiguation.

### 3.2 Rendering

**Strengths:** Render graph, GPU abstraction, and GLSL-first shader pipeline are well thought out.
Material layer stack, bindless model, and GPU-driven culling are architecturally right.

**Gaps (mostly missing designs, not wrong designs):**

- `Material` as a type does not exist — `MaterialComponent` (rendering-core), `MaterialGraph`
  (render-styles), and `CompiledMaterial` (implied in render-pipeline) are never linked.
- PSO cache is called for but not designed.
- Meshlet asset pipeline (import → `MeshletAsset` → GPU buffers) is unspecified.
- Descriptor layout inference from compiled shader bytecode is TODO.
- `ShadingModel` / `ShadingModelId` duplication.
- `RenderFrame` triple-buffer lifecycle, acquire/swap/present fences — undesigned.
- Barrier optimizer is a type name without an algorithm.
- Ray-tracing acceleration structures are orphaned: BLAS/TLAS declared, but no link to meshlet
  proxies.
- Render layer mask (`RenderLayerMask`) is referenced in 2D and 3D with inconsistent types.
- 2D path is completely parallel to 3D. `2d.md` shows an isolated sprite pipeline with its own
  lights, camera, physics, and transform. Integration with the shared render graph is asserted but
  not shown.
- Transform interpolation (`alpha = (elapsed - frame_time) / frame_time`) is not threaded through
  the extract pipeline.
- Camera split: `CameraComponent` (3D), `Camera2D` (2D) — no `CameraBase`.
- Debug draw API is a feature ticket without a design.
- `Swapchain` trait is declared with no methods.

### 3.3 Data Systems

**Strengths:** The four-primitive model (graphs / tables / attributes / containers) is the right
abstraction. The per-primitive designs are individually reasonable.

**Gaps:**

- **Composition is undesigned.** There is no document explaining how a user authors a quest,
  ability, or inventory system by wiring these four primitives. The feature matrix in
  `architecture.md` asserts feasibility but no integration guide proves it.
- `ModOp` / `ModifierOp` duplication (§2.2).
- `NodeStatus` on `DirectedGraph` is quest-specific (Locked, Available, Completed, Failed, Skipped)
  instead of generic, so the "generic graph primitive" is not actually generic.
- `ConditionExpr` and `ConditionRegistry` are referenced by graphs, effects, and containers with no
  statement about whether they share one global registry or each subsystem has its own.
- Attributes and containers are tightly coupled to data tables (`RowRef` fields are mandatory in
  definitions). Effects cannot be authored inline.
- Containers' socket system directly propagates stat modifiers via the effects system, making
  containers unusable without attributes/effects.
- No conflict-resolution spec for simultaneous effect applications, simultaneous threshold
  crossings, or failed mid-transaction container transfers.
- No save/load migration story for renamed or removed definitions.
- No network replication protocol for any of the four primitives.
- No deterministic graph-traversal guarantee for networked clients.
- Composition performance profile is missing — per-primitive benchmarks exist, but a "realistic RPG
  character density" scenario does not.
- No shared `DefinitionAsset<T>` wrapper; each primitive invents its own definition → component
  binding.
- Data-tables lacks a full query / secondary-index API (multi-column, filtered, join).

### 3.4 Simulation

**Strengths:** The four-primitive split (grids, awareness, timelines, event logs) is coherent.
Fixed-timestep semantics are consistent with physics and the game loop.

**Gaps:**

- `UniformGrid<T>` (grids) vs `UniformGrid` (core-runtime) — two types with the same name, different
  APIs. Rename or merge (§2.2).
- Time-scale (slow-mo, pause, 0.5×) is not specified for any primitive. Timelines have
  `PlaybackState::speed`; grids and event-logs have no equivalent.
- Pause semantics are not unified. On global pause, grids keep propagating unless the caller stops
  calling `propagate()`.
- Determinism seed handling is absent. Each primitive claims determinism but none specifies seed
  plumbing, floating-point guarantees, or hash stability.
- Time representation: timelines use `f64` seconds, event logs use `u64` ticks, grids and awareness
  use ticks. No `GameTime` with both views.
- Event-log positions are `Option<Vec3>` with "2D games set z=0" — no typed 2D position.
- Spatial-awareness selection queries do not document 2D variants explicitly.
- No rollback or replay protocol even though primitives are called "deterministic".
- Network replication for event-logs and grids is unspecified.
- No master phase-ordering diagram showing all four primitives in one timeline.
- Grids-vs-BVH decision matrix is missing. "When to use which" is never stated.
- Timelines/animation/scripting boundaries are asserted but not documented.

### 3.5 Physics / Geometry / VFX / Animation

**Strengths:** Physics foundation is numerically solid (islands, sleeping, CCD, warm start).
Skeletal animation, state machines, and GPU skinning are architected clearly. Meshlet-first geometry
and GPU-driven culling are the right call.

**Gaps:**

- Collision shapes are defined in `physics/foundation.md`, but meshlet-based meshes from geometry
  are not referenced. If geometry LOD changes, the collision shape does not update. `Collider`
  should reference `Handle<MeshAsset>` instead of embedding vertices.
- Graph infrastructure is duplicated in procedural generation, VFX effect graphs, animation state
  machine, and (implicitly) physics constraint graphs (§2.1).
- Debris + destruction + particle + cloth each have their own GPU simulation infrastructure, budget
  manager, and LOD pipeline. No `DestructionBudgetManager` caps them in aggregate.
- Ragdoll bridge between skeletal animation and physics joints is manual. `BoneIndex` must be mapped
  by hand to physics `Entity`, then back.
- Wind field is shared across cloth / hair / debris but is not updated by physics wind sources in
  the same frame.
- Skinning weight format is not defined. `BonePalette` is `Handle<GpuBuffer>` with no layout
  contract.
- Root motion API is vague — `RootMotion --> TR` in a diagram, no struct.
- Cloth constraint types, fracture patterns, and fluid-solver variants are named but not specified.
- Particle GPU sort/cull kernel interfaces are undefined.
- Procedural mesh output type (`ProceduralMeshOutput`) does not exist; the procedural graph produces
  "meshes" with no component to hold them.
- Animation state machine does not state a determinism contract; physics does.
- `Compression` for animation clips is claimed at 10:1+ but no format is selected (ACL? custom?).
- Animation procedural animation (IK, foot placement) does not use CCD; physics does. Swept queries
  are not shared.

### 3.6 AI / Audio / Input / UI / Networking

**Strengths:** AI (BT + GOAP + utility) covers the three major paradigms without over-specifying a
single approach. UI-as-ECS is a bold and defensible choice. Networking replication, lag
compensation, and prediction are rigorously specified.

**Gaps and issues:**

- Blackboard is a behavior-tree-private type with its own scope/value registry; GOAP uses
  `WorldState`; steering uses components. Three ways to hold per-agent state.
- Audio mixer graph, VFX effect graph, and material graph are three independent graph systems
  (§2.1).
- Audio thread is the single biggest ECS exception. The SPSC command bridge is OK, but audio could
  still be ECS-primary with a high-priority audio thread reading triple- buffered state (consistent
  with render thread model).
- Networking transport doc violates no-async (§2.7). Game transport must be sync on top of
  `quinn-proto` (sans-io) + platform-native I/O.
- Voice chat is duplicated: `VoiceStream` is defined in both `audio/audio.md` and
  `networking/network-services.md`. Pick an ownership split (transport vs codec).
- UI depends on input `ActionState` directly; should be abstract `PointerEvent` / `KeyboardEvent`
  consumed via ECS events so input backends can swap.
- `input/input.md` has no IME story (CJK text input).
- Audio has no ducking spec (voice > SFX > music).
- HRTF SOFA profile loading is named but not designed.
- A* pathfinding algorithm variant, heuristic, and tie-breaker are unspecified. The Recast-style nav
  mesh generation has no citation or parameters.
- Some classic steering primitives (`Align`, `Separate`, `Cohesion` outside of flocking;
  `ObstacleAvoid`, `Hide`, `Interpose`) are missing.
- QUIC stream multiplexing policy (how many streams, fairness, backpressure) is unspecified.
- Replication delta compression algorithm is named but not chosen (bit-packing? RLE? rkyv-diff?).
- Entity-interest AOI thresholds, update rates, and hysteresis are not stated.
- Anti-cheat surface is partial: movement/damage validators exist, but input validation, teleport
  detection thresholds, and reconciliation bounds are missing.
- Accessibility is duplicating core UI event routing via `AccessibilityTree` rather than hooking
  into it.
- WCAG compliance is aspirational — no automated checks defined.

### 3.7 Platform / Tools / Game Framework / Content Pipeline

**Strengths:** Custom windowing, platform I/O mapping, the middleman .dylib philosophy, and the
editor-as-plugin-host architecture are all sound choices.

**Gaps:**

- Hot-reload protocol (§2.6) missing.
- Undo/redo is a stub: `EditorCommand`, `UndoStack`, `Transaction` are declared without memory
  budget, persistent history, network sync for collaborative undo, or latency targets.
- Editor selection model (multi-viewport, sub-object picking, gizmo coupling) is not fully
  specified.
- PSO cache persistence (disk layout, versioning, GC) is missing.
- Localization runtime loading (where does `LocalizedStringId` resolution happen, how do tables
  hot-reload?) is missing — despite being called out as a core-runtime service in constraints.md.
- Profiler marker API for user code is missing; the profiler design describes what the profiler
  does, not how code declares scopes.
- Console variable system (`ConVar`) does not exist.
- Crash reporting is a feature list in platform-services.md without an architecture.
- Console SDK integration (PS5, Xbox GDK, Nintendo) is sparse.
- Telemetry (opt-in framework, GDPR) is missing.
- Plugin marketplace distribution (storage, dependency resolution, update, signing) is not designed.
- Asset dependency graph invalidation algorithm is prose, not pseudocode.
- Scene diffing / three-way merge algorithm is not written down.
- Save versioning: `MigrationRegistry` exists, migration failure rollback, autosave retention, and
  procedural-asset save handling are not.
- Editor writes to the live ECS world instead of operating on a shadow copy and syncing via
  `EventBridge`. Concurrent play-in-editor will race.
- Save system is codegen-coupled: if codegen changes a component layout, saves break, and the
  migration is *also* codegen, producing a chicken/egg problem.
- Shader compilation (naga) is described independently in
  `asset-processing.md`, `visual-editors.md`, and `build-deploy.md`. Three call sites, one canonical
  service needed.
- Codegen ownership: `scripting.md` and `visual-editors.md` both claim parts of the codegen
  pipeline. Who owns IR, optimizer, and backends is unclear.

### 3.8 Integration layer

**Strengths:** Most docs are concrete, typed, budgeted, and include fallbacks. Template compliance
is ~90%. The high-level doc is an actually-useful glue layer.

**Issues:**

- `input-camera.md` and `input-ui.md` are stubs (~30 lines each). They need full class diagrams,
  sequence diagrams, MPSC specs, and fallbacks like the other pairs.
- Several integration docs overspecify implementation (e.g., `ai-grids-volumes.md` contains
  `enqueue_influence_write()` pseudocode that belongs in grids-volumes itself;
  `asset-pipeline-rendering.md` contains naga in-process compilation launch code that belongs in
  tools). Move implementation into subsystem designs; integration docs should carry the
  **contract**, not the implementation.
- Missing pairs (present in high-level but not fully documented): `audio-physics`, `ai-physics`,
  `ui-physics`, `save-system-profiler`, `vfx-audio`, `geometry-vfx`, `attributes-rendering`,
  `event-logs-animation`, `editor-core-runtime`, `editor-asset-pipeline`, `localization-*` (per
  §3.7).
- `InterpAlpha` frame-global ownership is unclear — documented in `rendering-physics.md` and
  `timelines-camera.md` with slight variance. Should live in `profiler-game-loop.md`'s
  `FrameContext` definition, or in `game-loop.md` directly.
- `AnimEvent` variants are listed per-consumer instead of canonically in `animation/skeletal.md`.
- Dirty-region tracking appears in three integration docs with three implementations (§2.2).
- Channel-capacity rationale is absent: capacities are asserted (64, 128, 256, 1024, 4096) with no
  formula. An `integration/shared-messaging-capacities.md` with `Capacity = producers × burst × 1.5`
  would help future authors.

## 4. Prioritized task list

The items below are ordered so that higher-priority items unblock later ones. Each item names the
file(s) to touch and a one-sentence "done when" criterion.

### P0 — Must land before implementation begins

1. **Create `core-runtime/graph-runtime.md`** defining the shared graph trait, validator,
   topological sorter, and codegen backend traits. Done when the six graph systems can each be
   described as "`GraphRuntime<N, E>` + node palette + target backend".
2. **Create `core-runtime/primitives.md`** (or promote `algorithms.md`) as the canonical home for
   `Handle<T>`, `HandleMap`, `SlotMap`, `UniformGrid<T>`, `SortedVecMap<K,V>`, `RingBuffer<T, N>`,
   `DirtyRegionSet`, `DispatchTable<F>`, `BudgetAllocator`, `DeterministicRng`. Done when all
   duplicate definitions elsewhere are deleted and replaced with references.
3. **Create `core-runtime/ids.md`** with `StableId`, `GenerationalIndex`, and the convention for IDs
   that persist across save/load, hot-reload, and network. Done when `Entity`, `AssetId`,
   `ComponentId`, `NetworkEntityId`, and subsystem IDs all link to this doc.
4. **Create `core-runtime/hot-reload-protocol.md`** with `HotReloadManager`, `ReloadRequest`,
   `ReloadBarrier`, `StateMigrationFn`, `ReloadResult`. Done when scripting, shaders, material
   graphs, tables, and assets all reference one protocol.
5. **Define the missing bridge types** (short sections in existing docs): `RenderFrame`
   (rendering-core.md), `InterpolatedTransform` (scene-transforms.md), `PreviousGlobalTransform2D`
   (scene-transforms.md), `CompileError` (game-loop.md), `IoError` (memory-async-io.md),
   `NetworkEntityId` (networking/network-transport.md).
6. **Purge async from game-runtime designs.** Grep `docs/design/` for `async fn`, `.await`,
   `Future<`, `tokio::`. Convert all non-backend-service uses to the request/handle pattern.
   Explicitly scope remaining Tokio uses to Kubernetes backend services. Done when only
   `networking/network-infrastructure.md` (GameDb, matchmaker) contains async, and it says so.
7. **Rewrite `networking/network-transport.md`** to use platform-native I/O (quinn-proto sans-io on
   top of IOCP / io_uring / Networking.framework) and drop `tokio::net`. Done when the doc has zero
   `tokio::` references and the main thread drains QUIC events via the standard request/handle
   pattern.
8. **Create `data-systems/composition.md`** walking through quest, ability, and inventory authoring
   using all four data primitives and the simulation primitives. Done when a reader can see how to
   build an ability system without any dedicated "ability" subsystem.
9. **Resolve the game-loop ↔ ECS circular dependency.** Move `GameLoopGraph` and `CompiledFrame`
   into `game-loop.md` only; have `ecs.md::Schedule` produce a `SystemGraph` that game-loop
   compiles. Done when each doc only defines its own types.
10. **Decouple editor from live ECS world.** Add an `EditorWorld` shadow copy and an `EventBridge`
    contract (editor → game mutations only) in `tools/editor-core.md`. Done when edit-mode mutations
    cannot race the game loop.

### P1 — Fix before / during the first implementation milestones

11. **Unify `ModOp`** — single enum in `core-runtime/primitives.md`; update attributes-effects and
    containers-slots. Decouple socket stat propagation to emit events instead of directly applying
    effects.
12. **Make `NodeStatus` generic** in `directed-graphs.md` — remove quest-specific vocabulary; quest
    system provides its own `QuestNodeStatus`.
13. **Remove `HashMap` from `Archetype`** (`ecs.md`); use sorted `Vec<(ComponentId, Column)>` or a
    dense keyed store.
14. **Declare the `Material` type** spanning `rendering-core.md`, `render-styles.md`, and
    `render-pipeline.md`. One `Material { id, graph, compiled_shaders, descriptor_layout }` struct
    with clear ownership. Resolve `ShadingModel` / `ShadingModelId` duplication.
15. **Design the PSO cache** in `render-pipeline.md` (key format, invalidation, disk layout,
    recovery).
16. **Design the meshlet asset pipeline** in `rendering-core.md` or a new `rendering/meshlets.md`
    (import → `MeshletAsset` → GPU upload; shape of `MeshletBuilder`; link to BLAS for ray tracing).
17. **Specify barrier analysis** in `render-pipeline.md` — algorithm, split barriers, redundancy
    detection, state-transition validation.
18. **Unify 2D / 3D render graph.** Either add 2D passes to the existing render graph, or define a
    post-3D composite layer. Pick one and write it up in `2d.md` and `render-pipeline.md`.
19. **Split `game-framework/camera.md`** into `game-framework/camera.md` (brain, priority, blend)
    and `rendering/camera-rendering.md` (spring arm, cine, PiP).
20. ~~**Extract coroutines** to `core-runtime/coroutines.md`; `scripting.md` becomes a client.~~
    **Reversed.** The engine has no coroutine runtime — the corpus (ai-scripting,
    directed-graphs-scripting, timelines-scripting, high-level G1) already treats "no coroutines" as
    an invariant. Instead: rename scripting's internal `CoroutineState` to `SuspendState` and keep
    it scoped to the scripting crate as a codegen-internal yield-lowering state machine. For
    cross-subsystem multi-frame sequencing, use timelines, behavior trees, or delayed events.
21. **Move cloud sync and achievement queuing** from `save-system.md` to `platform-services.md`.
22. **Pick the pathfinding algorithm** in `ai/navigation.md` (A* variant, heuristic, tie-breaker,
    heap type, citation). Specify nav-mesh-generation parameters (cell size, partition algorithm,
    filter thresholds) with Recast citation.
23. **Specify QUIC stream multiplexing** in `networking/network-transport.md` (stream count, types,
    flow control, backpressure).
24. **Specify replication delta compression** (field-level diffing, baseline strategy, ACK
    mechanism) in `networking/network-transport.md`.
25. **Specify save schema versioning & migration** (stable GUID IDs, migration chain ordering,
    rollback on failure) in `game-framework/save-system.md`.
26. **Unify blackboard on ECS components** — replace `Blackboard`/`BlackboardScope` with a single
    ECS component backed by sorted `Vec`. Update behavior.md and delete the redundant "use BTreeMap"
    prose from 8+ integration docs.
27. **Pick voice-chat ownership:** transport in `networking/network-services.md`, codec and
    spatialization in `audio/audio.md`. Delete one `VoiceStream` definition.
28. **Create `core-runtime/error.md`** with `EngineError` and `ToEngineError` trait.
29. **Create `core-runtime/io.md`** with `IoRequest`/`IoResponse` and the canonical main-thread
    drain protocol (referenced from platform, content-pipeline, save).
30. **Write `integration/shared-conventions.md`** (or expand `constraints.md`) to absorb the
    repeated prose (Arc, HashMap, BTreeMap, MPSC, rkyv). Integration docs reference it and stop
    restating.
31. **Flesh out `input-camera.md` and `input-ui.md`** stub integration docs to full template
    compliance.
32. **Add `audio-physics.md` and `geometry-vfx.md`** integration docs.
33. **Specify the composition-density performance profile** in `data-systems/attributes-effects.md`
    and `containers-slots.md` (realistic RPG character, inventory, combat-encounter scenarios) so
    subsystem benchmarks compose.
34. **Specify time-scale, pause, determinism-seed semantics** for all four simulation primitives.
    Add a shared `GameTime` (`tick: u64`, `elapsed_secs: f64`) in core.

### P2 — Quality and completeness

35. **Move implementation pseudocode out of integration docs** into subsystem designs:
    `ai-grids-volumes` (drain loop → grids-volumes), `animation-rendering` (GpuSkinner dispatch →
    rendering), `attributes-effects-*` (sync loops → attributes/effects), `asset-pipeline-rendering`
    (naga in-process compilation → tools/build-deploy).
36. **Specify skinning weight format** (`[u8;4]` bones + quantized weights, palette layout) in
    `animation/skeletal.md`.
37. **Specify cloth constraint types and fracture pattern format** in `physics/advanced.md`.
38. **Specify fluid solver variant** (SPH vs FLIP/PIC vs Eulerian) and parameters in
    `physics/advanced.md`.
39. **Specify ACL (or equivalent) animation compression** with target ratios and quality bounds.
40. **Declare a determinism contract** for animation state machine eval, procedural animation, and
    event-log threshold evaluation (not just physics).
41. **Add GPU timestamp calibration parameters** (interval, drift threshold, fallback) in
    `integration/profiler-rendering.md`.
42. **Specify dormancy thresholds** (interest count, wake-up condition, stale eviction) in
    `integration/networking-ecs.md`.
43. **Specify `NameplateBuffer` arena capacity and overflow** in `integration/rendering-ui.md`.
44. **Extract `DirtyRegionSet` to core primitives** (§2.2) and have three integration docs reference
    it instead of reimplementing.
45. **Extract `ParallelCommandWriter`, `DispatchTable<F>`, `BudgetAllocator`, and `SortedVecMap<K,
    V>`** into core-runtime primitives; reference from the integration docs that currently reinvent
    them.
46. **Write `core-runtime/console-variables.md`** (`ConVar` registry, scope, type safety). Profiler
    and debug-visualization features depend on it.
47. **Write `game-framework/localization.md`** or `core-runtime/localization.md` covering
    `LocalizedStringId` runtime resolution, table loading, hot-reload, and fallback.
48. **Write `platform/crash-reporting.md`** (out-of-process handler, signal registration, minidump
    format, symbolication).
49. **Write `platform/telemetry.md`** (opt-in UI, data scope, upload, GDPR).
50. **Write `platform/console-integration.md`** isolating PS5/Xbox/Nintendo SDK usage.
51. **Write `tools/plugin-marketplace.md`** (storage, signing, dependency resolution, update path).
52. **Write `tools/undo-redo.md`** expanding beyond the current stub (memory budget, persistent
    history, network sync, gizmo selection coupling).
53. **Write `tools/selection-model.md`** (multi-viewport, sub-object picking).
54. **Write `tools/scene-versioning.md`** (structural diff algorithm, three-way merge, Git merge
    driver).
55. **Consolidate shader compilation** into a single `ShaderCompiler` service in `harmonius_core`
    (or similar); both `visual-editors.md` and `asset-processing.md` become clients.
56. **Define `codegen-pipeline.md`** establishing `harmonius_codegen` as the single owner of IR,
    optimizer, and backends; `scripting.md` and `visual-editors.md` become clients.
57. **Add IME support, audio ducking, HRTF SOFA profile loading, WCAG compliance test plan, and
    classic steering primitives** to their owning docs.

### P3 — Polish and future-proofing

58. **Write `core-runtime/change-detection.md`** specifying tick lifecycle and the `Changed<T>`
    query contract. Update `ecs.md` and `game-loop.md` to reference it.
59. **Add "Integration Boundaries" sections** to `simulation/timelines.md` clarifying what timelines
    can and cannot do (no scripting at keyframes; events only).
60. **Standardize fallback naming (`FM-N` labels)** across all integration docs.
61. **Add algorithm citations** to every non-trivial algorithm (already a documented constraint; not
    uniformly applied).
62. **Rename `PhaseNode` or collapse `Phase` + `PhaseNode`** in `game-loop.md` to reduce naming
    collision.
63. **Document `SpatialUpdateSystem` phase ordering** in `game-loop.md` and `spatial-index.md`.
64. **Document event-channel contention guarantees** (one writer per type per frame, scheduler
    enforces) in `events-plugins.md`.
65. **Add composition tests** cross-subsystem: save → load → hot-reload-shader → verify-state;
    rollback → replay; etc.
66. **Unify `VoiceStream`, `AnimEvent`, and `AudioCommand`** each to a single canonical definition
    in their owning subsystem and delete redefinitions in integration docs.
67. **Promote `FrameContext` to a first-class type** (defined once, used everywhere) with
    `frame_index`, `interp_alpha`, `game_time`, `frame_budgets`, and seed.
68. **Factor `AnimationLodTier` and similar LOD-tier tables** out of integration docs into their
    owning subsystem.

## 5. Closing assessment

On the **simple / cohesive / modular / consistent** dimensions:

- **Simple:** The overall picture is remarkably simple for an engine targeting this much surface
  area (all genres, VR, consoles, 2D/3D/2.5D, no-code, hot reload). The composition-over-subsystems
  decision keeps the gameplay layer simple. Within core, however, pockets of over-engineering exist
  (PhaseNode, two event systems, three ways to hold per-agent AI state).
- **Cohesive:** Individual subsystems are cohesive. The boundaries between subsystems are less
  cohesive — too many bridge types are left undefined (§2.3).
- **Modular:** Layering is good, but
  **the modules leak into each other through shared types that are defined on both sides**
  (UniformGrid, Handle, ConditionExpr, VoiceStream …). Modularity requires these types to have a
  single owner.
- **Consistent:** Within any one doc, usually yes. Across docs, inconsistent naming (Phase /
  PhaseNode, ShadingModel / ShadingModelId, ModOp / ModifierOp, tick vs seconds, position types) and
  inconsistent template fidelity (integration stubs vs. exemplary docs).

On the **coupling** question: the intentional coupling is good (render thread reads an immutable
RenderFrame snapshot, audio reads an SPSC command stream, etc.). The **unintentional** coupling is
the problem — editor writes to the live ECS world, save serialization is chicken-and-egg with
codegen, attributes leak into containers via effects, sockets require the effects system, graph
infrastructure is reinvented per subsystem, and integration docs restate core constraints.

On **missing primitives**: yes. `GraphRuntime`, `Material`, `PsoCache`, `MeshletAsset`,
`HotReloadManager`, `DefinitionAsset<T>`, `RenderFrame`, `ConVar`, `EngineError`, `IoRequest`,
`LocalizedStringLoader`, and several smaller ones (DirtyRegionSet, DispatchTable, SortedVecMap,
BudgetAllocator, GenericRingBuffer) would each remove hundreds of lines of subsystem-specific
reinvention.

On **"is it designed well?"**: at the layer of vision and decomposition, **yes** — unusually so. At
the layer of foundational types and cross-module contracts, **not yet** — the design is ~80%
complete and the missing 20% is concentrated in exactly the places that most need to be concrete
before code is written. The task list above is the work needed to close that gap.

Estimated effort to complete the P0 tasks: 2–3 weeks of focused design work, most of it extraction
and consolidation rather than new invention. Estimated reduction in subsystem-doc surface area after
P0+P1: 25–35%.
