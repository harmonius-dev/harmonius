# Canonical Owners — Shared-Type Registry

The single index of who owns which shared type. Per
[design-review.md §2.2](design-review.md#22-foundational-type-duplication), foundational type
duplication is the second-largest systemic flaw in the corpus. This file is the authoritative
registry. If you need a shared type and it is not here, add the row before introducing the type
elsewhere.

> **Normative.** Every design and integration document MUST reference, not redefine, the types
> listed below. Adding a duplicate definition is a folder-rule violation. Discovering a duplicate
> creates a backlog issue under [BL-0001..BL-0014]
> (../backlog/index.md#canonical-owner-consolidations-bl-0001--bl-0014).

## Status legend

| Status                  | Meaning                                                             |
|-------------------------|---------------------------------------------------------------------|
| Owned                   | Single owner; no duplicates remain                                  |
| Pending consolidation   | Sole owner declared but at least one stale duplicate still exists   |
| Pending split-rename    | Two distinct concepts share a name; rename pending                  |
| Pending creation        | Sole owner not yet declared; type referenced but undefined          |
| Reversed                | Earlier consolidation plan reversed; documented in linked ADR/note  |

## Container and utility primitives

Owner: [`core-runtime/primitives.md`](core-runtime/primitives.md).

| Type                          | Owner doc                              | Status                | Backlog                                                       |
|-------------------------------|----------------------------------------|-----------------------|---------------------------------------------------------------|
| `Handle<T>`                   | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `HandleMap<T>`                | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `SlotMap<T>`                  | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `GenerationalIndex`           | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `SortedVecMap<K, V>`          | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `RingBuffer<T, N>`            | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `DirtyRegionSet`              | `core-runtime/primitives.md`           | Pending consolidation | [BL-0044](https://github.com/cjhowe-us/harmonius/issues?q=BL-0044) |
| `DispatchTable<F>`            | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `BudgetAllocator`             | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `DeterministicRng`            | `core-runtime/primitives.md`           | Owned                 | —                                                             |
| `SmallVec` / `FixedBitSet`    | `core-runtime/primitives.md` (aliases) | Owned                 | —                                                             |

## Spatial structures

| Type                          | Owner doc                                            | Status                  | Backlog                                                                  |
|-------------------------------|------------------------------------------------------|-------------------------|--------------------------------------------------------------------------|
| `UniformGrid<T>`              | `core-runtime/spatial-index.md`                      | Pending split-rename    | [BL-0004](https://github.com/cjhowe-us/harmonius/issues?q=BL-0004)               |
| `CellGrid` (gameplay)         | `simulation/grids-volumes.md`                        | Pending split-rename    | [BL-0004](https://github.com/cjhowe-us/harmonius/issues?q=BL-0004)               |
| `AoiGrid` (networking)        | `networking/network-transport.md`                    | Pending split-rename    | [BL-0004](https://github.com/cjhowe-us/harmonius/issues?q=BL-0004)               |
| `BvhTree`                     | `core-runtime/spatial-index.md`                      | Owned                   | —                                                                        |
| `PhysicsBvh`                  | `physics/foundation.md`                              | Owned                   | —                                                                        |
| `SpatialIndex` trait          | `core-runtime/spatial-index.md`                      | Owned                   | —                                                                        |

`UniformGrid<T>` is the core algorithmic primitive. The two specializations differ in purpose
(gameplay propagation vs networking AOI). The pending rename keeps the generic in core, renames the
gameplay specialization `CellGrid`, and renames the networking specialization `AoiGrid` to remove
the namespace clash.

## ID and handle taxonomy

Owner: [`core-runtime/ids.md`](core-runtime/ids.md).

| Type                  | Owner doc                          | Status            |
|-----------------------|------------------------------------|-------------------|
| `StableId` (trait)    | `core-runtime/ids.md`              | Owned             |
| `GenerationalIndex`   | `core-runtime/primitives.md`       | Owned             |
| `Entity` / `EntityId` | `core-runtime/ecs.md`              | Owned             |
| `ComponentId`         | `core-runtime/ecs.md`              | Owned             |
| `AssetId`             | `content-pipeline/asset-pipeline.md` | Owned           |
| `NetworkEntityId`     | `core-runtime/ids.md`              | Owned             |
| `BlackboardKey`       | `core-runtime/ids.md`              | Owned             |
| `LocalizedStringId`   | `game-framework/localization.md`   | Owned             |

## Errors and result types

Owner: [`core-runtime/error.md`](core-runtime/error.md).

| Type            | Owner doc                          | Status                |
|-----------------|------------------------------------|-----------------------|
| `EngineError`   | `core-runtime/error.md`            | Owned                 |
| `ToEngineError` | `core-runtime/error.md`            | Owned                 |
| `CompileError`  | `core-runtime/error.md`            | Pending consolidation |
| `IoError`       | `core-runtime/error.md`            | Owned                 |
| `MigrationError`| `core-runtime/error.md`            | Owned                 |

`CompileError` is currently defined in `error.md`, `game-loop.md`, `scripting.md`, and
`algorithms.md`. The owner is `error.md`; remove the others. See
[BL-0007](https://github.com/cjhowe-us/harmonius/issues?q=BL-0007).

## Frame and time

| Type                       | Owner doc                              | Status                 |
|----------------------------|----------------------------------------|------------------------|
| `RenderFrame`              | `core-runtime/game-loop.md`            | Owned                  |
| `FrameContext`             | `core-runtime/game-loop.md`            | Pending consolidation  |
| `InterpAlpha`              | `core-runtime/game-loop.md`            | Owned                  |
| `GameTime` (`tick`+`secs`) | `core-runtime/game-loop.md`            | Pending creation       |
| `InterpolatedTransform`    | `core-runtime/scene-transforms.md`     | Owned                  |
| `PreviousGlobalTransform2D`| `core-runtime/scene-transforms.md`     | Owned                  |

`FrameContext` and `GameTime` carry frame index, interpolation alpha, deterministic seed, pause
flag, and budgets. Promoting these to canonical types removes ad-hoc copies in integration docs. See
[BL-0014](https://github.com/cjhowe-us/harmonius/issues?q=BL-0014) and
[BL-0026](https://github.com/cjhowe-us/harmonius/issues?q=BL-0026).

## Hot reload protocol

Owner: [`core-runtime/hot-reload-protocol.md`](core-runtime/hot-reload-protocol.md).

| Type                | Owner doc                              | Status |
|---------------------|----------------------------------------|--------|
| `HotReloadManager`  | `core-runtime/hot-reload-protocol.md`  | Owned  |
| `ReloadRequest`     | `core-runtime/hot-reload-protocol.md`  | Owned  |
| `ReloadBarrier`     | `core-runtime/hot-reload-protocol.md`  | Owned  |
| `StateMigrationFn`  | `core-runtime/hot-reload-protocol.md`  | Owned  |
| `ReloadResult`      | `core-runtime/hot-reload-protocol.md`  | Owned  |

## Graph runtime

Owner: [`core-runtime/graph-runtime.md`](core-runtime/graph-runtime.md).

| Type                          | Owner doc                              | Status |
|-------------------------------|----------------------------------------|--------|
| `GraphNode` (trait)           | `core-runtime/graph-runtime.md`        | Owned  |
| `GraphEdge` (trait)           | `core-runtime/graph-runtime.md`        | Owned  |
| `GraphValidator` (trait)      | `core-runtime/graph-runtime.md`        | Owned  |
| `DagValidator`                | `core-runtime/graph-runtime.md`        | Owned  |
| `TopologicalSorter`           | `core-runtime/graph-runtime.md`        | Owned  |
| `CycleDetector`               | `core-runtime/graph-runtime.md`        | Owned  |
| `CodegenBackend` (trait)      | `core-runtime/graph-runtime.md`        | Owned  |
| `RustBackend`                 | `core-runtime/graph-runtime.md`        | Owned  |
| `GlslBackend`                 | `core-runtime/graph-runtime.md`        | Owned  |
| `TypeDescriptorBackend`       | `core-runtime/graph-runtime.md`        | Owned  |

The eight visual graph systems (logic / scripting, directed-graph data, behavior tree, material
graph, VFX effect graph, animation state machine, procedural generation, timeline track) all
parameterize this owner. They never reinvent topological sort, cycle detection, validation, or
hot-reload triggering.

## Data systems shared types

| Type                       | Owner doc                                       | Status                 | Backlog                                                                  |
|----------------------------|-------------------------------------------------|------------------------|--------------------------------------------------------------------------|
| `ConditionExpr`            | `data-systems/composition.md`                   | Pending consolidation  | [BL-0005](https://github.com/cjhowe-us/harmonius/issues?q=BL-0005)     |
| `ConditionRegistry`        | `data-systems/composition.md`                   | Pending consolidation  | [BL-0005](https://github.com/cjhowe-us/harmonius/issues?q=BL-0005)     |
| `ModOp` / `ModifierOp`     | `core-runtime/primitives.md`                    | Pending consolidation  | [BL-0001](https://github.com/cjhowe-us/harmonius/issues?q=BL-0001)                |
| `NodeStatus` (generic)     | `data-systems/directed-graphs.md`               | Pending split-rename   | quest-specific `QuestNodeStatus` to live in game-specific code           |
| `DefinitionAsset<T>`       | `data-systems/composition.md`                   | Pending creation       | [BL-0030](https://github.com/cjhowe-us/harmonius/issues?q=BL-0030)            |
| `BlackboardComponent`      | `core-runtime/ecs.md`                           | Pending consolidation  | [BL-0006](https://github.com/cjhowe-us/harmonius/issues?q=BL-0006)          |

## Rendering shared types

| Type                       | Owner doc                                       | Status                 | Backlog                                                                  |
|----------------------------|-------------------------------------------------|------------------------|--------------------------------------------------------------------------|
| `Material`                 | `rendering/render-pipeline.md`                  | Pending creation       | [BL-0009](https://github.com/cjhowe-us/harmonius/issues?q=BL-0009)        |
| `ShadingModel`             | `rendering/rendering-core.md`                   | Pending consolidation  | [BL-0002](https://github.com/cjhowe-us/harmonius/issues?q=BL-0002)              |
| `MaterialGraph`            | `rendering/render-styles.md`                    | Owned                  | —                                                                        |
| `CompiledMaterial`         | `rendering/render-pipeline.md`                  | Pending creation       | [BL-0009](https://github.com/cjhowe-us/harmonius/issues?q=BL-0009)        |
| `MeshletAsset`             | `rendering/meshlets.md`                         | Pending creation       | [BL-0011](https://github.com/cjhowe-us/harmonius/issues?q=BL-0011)           |
| `PsoCache`                 | `rendering/pipeline-state-cache.md`             | Pending creation       | [BL-0010](https://github.com/cjhowe-us/harmonius/issues?q=BL-0010)                 |
| `RenderLayerMask`          | `rendering/rendering-core.md`                   | Pending consolidation  | [BL-0012](https://github.com/cjhowe-us/harmonius/issues?q=BL-0012)         |
| `Camera2D`                 | `rendering/camera-rendering.md`                 | Owned                  | —                                                                        |
| `CameraComponent`          | `game-framework/camera.md`                      | Owned                  | —                                                                        |
| `Swapchain` (trait)        | `rendering/render-pipeline.md`                  | Pending creation       | [BL-0010](https://github.com/cjhowe-us/harmonius/issues?q=BL-0010) (related)       |

## Audio and networking shared types

| Type                       | Owner doc                                       | Status                 | Backlog                                                                  |
|----------------------------|-------------------------------------------------|------------------------|--------------------------------------------------------------------------|
| `VoiceStream` (transport)  | `networking/network-services.md`                | Pending consolidation  | [BL-0003](https://github.com/cjhowe-us/harmonius/issues?q=BL-0003)            |
| `VoiceStream` (codec)      | `audio/audio.md`                                | Pending consolidation  | [BL-0003](https://github.com/cjhowe-us/harmonius/issues?q=BL-0003)            |
| `AudioCommand`             | `audio/audio.md`                                | Owned                  | —                                                                        |
| `AnimEvent`                | `animation/skeletal.md`                         | Pending consolidation  | [BL-0032](https://github.com/cjhowe-us/harmonius/issues?q=BL-0032)             |
| `AnimationLodTier`         | `animation/skeletal.md`                         | Owned                  | —                                                                        |

## Tooling and platform shared types

| Type                       | Owner doc                                       | Status                 |
|----------------------------|-------------------------------------------------|------------------------|
| `ConVar`                   | `core-runtime/console-variables.md`             | Owned                  |
| `EditorWorld`              | `tools/editor-core.md`                          | Owned                  |
| `EventBridge`              | `tools/editor-core.md`                          | Owned                  |
| `EditorCommand`            | `tools/undo-redo.md`                            | Owned                  |
| `Transaction`              | `tools/undo-redo.md`                            | Owned                  |
| `MigrationRegistry`        | `game-framework/save-system.md`                 | Owned                  |
| `LocalizedStringLoader`    | `game-framework/localization.md`                | Owned                  |
| `ReportSink`               | `platform/crash-reporting.md`                   | Owned                  |

## Conventions

1. The Owner column points to the file whose `## API Design` section defines the type.
   All other docs reference this file or use the type without redefining it.
2. Aliases (e.g. type aliases over external crates) are still listed for visibility.
3. `Pending consolidation` rows have a backlog issue. Closing the issue flips the row
   to `Owned`.
4. Adding a new entry requires either an existing Owned/Pending row to update, or a
   new ADR if the introduction is consequential.

## Cross-document compliance check

Use this section as a checklist when reviewing a design or integration doc:

- [ ] Does the doc define a type listed in this file? If yes, replace with a reference.
- [ ] Does the doc introduce a new shared type? If yes, add a row here first.
- [ ] Does the doc rename a type? If yes, supersede the row and link the rename.
