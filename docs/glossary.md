# Harmonius Glossary

Engine-specific terminology. Two sections: (1) Harmonius terms that replace third-party engine
names, and (2) Harmonius-coined concepts used across design documents. Use these terms in all
documentation except competitive comparison tables in business docs.

## Replacements for third-party terms

| # | Harmonius Term | Replaces |
|---|----------------|----------|
| 1 | Ability Framework | GAS |
| 2 | Batch Processor | MassEntity |
| 3 | Camera Rig | Cinemachine |
| 4 | Character Creator | MetaHuman |
| 5 | Cinematics Editor | Sequencer |
| 6 | DCC Bridge | Live Link |
| 7 | Dynamic GI | Lumen |
| 8 | Effect Graph | Visual Effect Graph |
| 9 | Entity Template | Prefab |
| 10 | Fracture System | Chaos Destruction |
| 11 | Hierarchical Tags | Gameplay Tags |
| 12 | Logic Graph | Blueprints |
| 13 | Mesh Streaming | Nanite |
| 14 | Sequence Track | Timeline |
| 15 | Shader Graph | Shader Graph |
| 16 | Transform Propagation | TransformSystemGroup |
| 17 | World Grid | World Partition |
| 18 | Zone Streaming | Level Streaming |

1. **Ability Framework** — system for defining, granting, activating, and revoking gameplay
   abilities with cooldowns, costs, and targeting
2. **Batch Processor** — parallel entity processing pipeline that evaluates systems over large
   entity populations in bulk
3. **Camera Rig** — composable camera behavior system with blend stacks, constraints, and procedural
   motion
4. **Character Creator** — pipeline and tools for authoring high-fidelity character models with
   morph targets and modular parts
5. **Cinematics Editor** — track-based sequencing tool for authoring cutscenes, camera moves, and
   scripted events
6. **DCC Bridge** — real-time synchronization protocol between external DCC tools and the engine
   editor
7. **Dynamic GI** — screen-space and probe-based global illumination updated every frame without
   baking
8. **Effect Graph** — node-based visual editor for authoring particle and VFX behaviors
9. **Entity Template** — reusable entity definition with component defaults, nesting, and
   per-instance overrides
10. **Fracture System** — pre-computed and runtime mesh fracturing for destructible objects
11. **Hierarchical Tags** — dot-separated tag hierarchy for categorizing and filtering entities,
    abilities, and events
12. **Logic Graph** — visual node graph for authoring gameplay logic, compiled to native code
13. **Mesh Streaming** — virtualized geometry system that streams meshlet LODs based on screen-space
    error
14. **Sequence Track** — individual channel within the Cinematics Editor controlling one property
    over time
15. **Shader Graph** — visual node graph for authoring material shaders, compiled to HLSL (same
    name, not a third-party reference)
16. **Transform Propagation** — system that propagates local-to-world transforms through the scene
    hierarchy
17. **World Grid** — spatial partitioning of large worlds into streamable grid cells
18. **Zone Streaming** — loading and unloading world grid cells based on player proximity and
    priority

## Harmonius-specific concepts

These terms are coined or redefined by the Harmonius project and appear across design documents.

| #  | Term                       | Category             |
|----|----------------------------|----------------------|
| 1  | AoSoA Tiled Storage        | ECS storage          |
| 2  | Archetype                  | ECS storage          |
| 3  | Attributes and Effects     | Data primitive       |
| 4  | Containers and Slots       | Data primitive       |
| 5  | Data Table                 | Data primitive       |
| 6  | Directed Graph             | Data primitive       |
| 7  | Event Log                  | Simulation primitive |
| 8  | Generic Primitive          | Philosophy           |
| 9  | Grids and Volumes          | Simulation primitive |
| 10 | Hot-Reload Protocol        | Infrastructure       |
| 11 | Middleman Dylib            | Codegen              |
| 12 | Pair-Wise Integration Spec | Design process       |
| 13 | Spatial Awareness          | Simulation primitive |
| 14 | Spatial Index              | Infrastructure       |
| 15 | Static Codegen Pipeline    | Codegen              |
| 16 | Timeline (simulation)      | Simulation primitive |
| 17 | Zero Reflection            | Philosophy           |

1. **AoSoA Tiled Storage** — Array-of-Struct-of-Arrays component layout inside 16 KiB archetype
   chunks, enabling SIMD iteration and cache-friendly query plans
2. **Archetype** — ECS storage bucket for all entities sharing the same component set; transitions
   between archetypes are O(1) via precomputed edges
3. **Attributes and Effects** — data primitive pairing a numeric attribute with a stack of modifiers
   (buffs, debuffs, durations); composes into abilities, stats, and status systems
4. **Containers and Slots** — data primitive: a typed slotted container used for inventory,
   equipment, storage, hotbars, or crafting grids
5. **Data Table** — data primitive: typed row/column grid of static content used for items, loot,
   stats, localization, or configuration
6. **Directed Graph** — data primitive: a DAG of typed nodes and edges used for quests, dialogue,
   progression, or flow control; distinct from the Logic Graph editor, which produces one
7. **Event Log** — simulation primitive: append-only history of entity events used for replay, AI
   memory, audit trails, and analytics
8. **Generic Primitive** — a reusable building block (graphs, tables, attributes, containers, grids,
   timelines, event logs) that composes into any game mechanic; the engine has no "quest" or
   "inventory", only primitives
9. **Grids and Volumes** — simulation primitive: uniform grid or voxel structure used for
   pathfinding, influence maps, fog of war, or volumetric effects
10. **Hot-Reload Protocol** — unified protocol for state migration and reload barriers shared by
    graphs, materials, shaders, assets, data tables, and the middleman dylib
11. **Middleman Dylib** — single shared `.dylib` holding every codegen-generated type and function;
    loaded at editor startup for hot reload, statically linked into ship builds
12. **Pair-Wise Integration Spec** — design document defining the contract between exactly two
    subsystems: channels, capacities, message types, phase ordering, and fallback behavior
13. **Spatial Awareness** — simulation primitive: perception, occlusion, visibility, and
    line-of-sight queries over a shared spatial index, used by AI and gameplay triggers
14. **Spatial Index** — BVH or uniform grid shared across AI, physics broadphase, audio, and
    gameplay queries; multiple specialized indices coexist for different access patterns
15. **Static Codegen Pipeline** — build-time process that compiles visual editor schemas to Rust
    source, then through bundled rustc into the middleman dylib
16. **Timeline** — simulation primitive: time-ordered sequence of values and events over simulation
    time, used for animation curves, state history, or scheduled events; distinct from Sequence
    Track (a Cinematics Editor channel)
17. **Zero Reflection** — no `dyn Reflect`, no `TypeRegistry`, no runtime `TypeId` dispatch; all
    type metadata generated statically by the codegen pipeline
