# Harmonius Glossary

Engine-specific terminology. Use these terms instead of third-party engine names in all
documentation except competitive comparison tables in business docs.

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
