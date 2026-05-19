# R-15.8 -- Logic Graph Requirements

## Requirements

1. **R-15.8.1** — The engine **SHALL** provide a typed, functional graph runtime compiling to native
   code or bytecode with performance matching hand-written logic.
   - **Rationale:** The logic graph is the sole authoring mechanism; it must incur zero overhead.
   - **Verification:** Benchmark a graph against equivalent hand-written Rust and verify less than
     5% overhead.

2. **R-15.8.2** — The engine **SHALL** enforce static types on all pins with bidirectional
   inference, supporting generics, enums, optionals, and user-defined types with no implicit
   coercion.
   - **Rationale:** Type safety catches errors at edit time rather than runtime.
   - **Verification:** Connect mismatched pin types and verify the editor displays an inline error.

3. **R-15.8.3** — The engine **SHALL** validate graphs for type correctness, dangling pins, cycles,
   and unconnected required inputs before save or compile.
   - **Rationale:** Invalid graphs must never reach runtime.
   - **Verification:** Create a graph with a cycle and verify validation rejects it with a
     node-level error message.

4. **R-15.8.4** — The engine **SHALL** support gameplay logic graphs covering abilities, AI, quests,
   dialogue, UI, game modes, and input processing with coroutine-style multi-frame execution.
   - **Rationale:** All gameplay must be authorable visually including multi-frame sequences.
   - **Verification:** Author a two-phase boss encounter graph and verify it spans multiple frames
     correctly.

5. **R-15.8.5** — The engine **SHALL** support visual shader graph authoring compiling through the
   glslc/glslc pipeline to SPIR-V, with a material graph variant
   providing PBR inputs and live preview.
   - **Rationale:** Shader authoring must be visual and produce shaders for all GPU backends.
   - **Verification:** Create a shader graph, compile it, and verify correct rendering on Vulkan,
     Vulkan, and Vulkan.

6. **R-15.8.6** — The engine **SHALL** support visual render graph configuration with automatic
   barrier insertion and resource aliasing.
   - **Rationale:** Visual pipeline configuration enables non-programmers to modify the rendering
     pipeline.
   - **Verification:** Add a post-process pass via the graph editor and verify correct execution
     with barriers.

7. **R-15.8.7** — The engine **SHALL** support animation logic graphs for state machines, blend
   trees, and IK setups.
   - **Rationale:** Animation control uses the same visual system as gameplay logic for consistency.
   - **Verification:** Author a locomotion state machine graph and verify correct blend weights
     during playback.

8. **R-15.8.8** — The engine **SHALL** support audio logic graphs for adaptive music, RTPC effects,
   and spatial audio driven by ECS component data.
   - **Rationale:** Audio designers must author reactive soundscapes without code.
   - **Verification:** Create an audio graph driven by a health component and verify volume changes
     with health value.

9. **R-15.8.9** — The engine **SHALL** support custom tool graphs defining editor panels and
   responding to user input, compiled as editor-only bytecode.
   - **Rationale:** Studio-specific editors should not require native code or external toolchains.
   - **Verification:** Author a tool graph with a custom panel and verify it appears in the editor.

10. **R-15.8.10** — The engine **SHALL** provide a node library auto-generated from type registry
    entries covering math, ECS, physics, audio, rendering, input, and networking.
    - **Rationale:** Auto-generation ensures the library stays in sync with engine types.
    - **Verification:** Register a new ECS component and verify a corresponding node appears in the
      library.

11. **R-15.8.11** — The engine **SHALL** provide graph debugging with breakpoints, step-through,
    live pin inspection, and per-node timing during play mode.
    - **Rationale:** Debugging visual logic requires the same capabilities as traditional code
      debugging.
    - **Verification:** Set a breakpoint, trigger it, and verify pin values are displayed and
      execution pauses.

12. **R-15.8.12** — The engine **SHALL** compile graphs with dead node elimination, constant
    folding, subgraph inlining, and type specialization via LLVM on all platforms.
    - **Rationale:** Compilation optimization ensures visual authoring matches hand-written
      performance.
    - **Verification:** Compile a graph with dead nodes and verify they are absent from the output
      binary.

13. **R-15.8.13** — The engine **SHALL** provide visual graph diffing with color-coded overlays and
    three-way merge with per-node conflict resolution, integrated with Git.
    - **Rationale:** Collaborative graph editing requires structural merge rather than text merge.
    - **Verification:** Create conflicting graph edits on two branches, merge, and verify per-node
      conflict markers appear.

14. **R-15.8.14** — The engine **SHALL** provide project-wide graph search for node types,
    variables, and references with rename refactoring that propagates to all referencing graphs.
    - **Rationale:** Refactoring across many graphs must be safe and automated.
    - **Verification:** Rename a custom node type and verify all 10 referencing graphs update.

15. **R-15.8.15** — The engine **SHALL** provide a keyboard-first graph editing workflow with hotkey
    search palette, arrow-key node navigation, auto-connect on placement, and sequential node lists
    with implicit execution flow.
    - **Rationale:** Keyboard-first interaction is faster than mouse wiring for experienced users.
    - **Verification:** Press Tab, search for a node, place it, and verify it auto-connects to the
      previously selected output pin.

16. **R-15.8.16** — The engine **SHALL** provide visual macro group containers that draw colored
    boundary boxes around sets of nodes, supporting expand/collapse, named groups, and promotion to
    reusable subgraph assets.
    - **Rationale:** Visual grouping organizes complex graphs without changing execution semantics.
    - **Verification:** Select 5 nodes, create a macro group, collapse it, expand it, and verify all
      connections remain intact.

17. **R-15.8.17** — The engine **SHALL** provide a shared graph editor framework (pannable/ zoomable
    canvas, typed pin connections, node widgets, minimap, edge interaction) reused by all
    domain-specific graph editors.
    - **Rationale:** A shared framework ensures consistent UX across all 12+ graph editor types and
      reduces maintenance.
    - **Verification:** Open the logic graph, material graph, and animation graph editors and verify
      identical pan, zoom, and connection behavior.

18. **R-15.8.18** — The engine **SHALL** incrementally recompile only the affected subgraph on each
    graph edit, with full recompilation only when structural changes affect the entry point.
    - **Rationale:** Incremental compilation keeps edit-to-test latency low for large graphs.
    - **Verification:** Edit a leaf node in a 200-node graph and verify recompilation completes in
      under 100 ms.

19. **R-15.8.19** — The engine **SHALL** provide inline suggested fixes for graph validation errors,
    including auto-inserting cast nodes and connecting default values.
    - **Rationale:** Suggested fixes accelerate error resolution for designers unfamiliar with type
      system details.
    - **Verification:** Connect mismatched pin types and verify the editor offers an auto-insert
      cast node fix that resolves the error.

20. **R-15.8.20** — The engine **SHALL** provide 2D-specific nodes in all visual editors (2D
    animation timeline, 2D material nodes, 2D physics/spatial query nodes, 2D blend spaces) that
    appear contextually when the graph targets a 2D scene.
    - **Rationale:** 2D games require specialized nodes that differ from their 3D counterparts.
    - **Verification:** Create a graph targeting a 2D scene and verify 2D-specific nodes appear in
      the palette while 3D-only nodes are hidden.

21. **R-15.8.21** — The engine **SHALL** render a minimap overview of the graph canvas with
    click-to-navigate and viewport highlight.
    - **Rationale:** Large graphs require spatial overview for navigation without excessive
      scrolling.
    - **Verification:** Open a 100-node graph, click a region on the minimap, and verify the canvas
      scrolls to that location.

22. **R-15.8.22** — The engine **SHALL** support extracting a selected set of nodes into a reusable
    subgraph asset, replacing the selection with a single subgraph call node with auto-created
    input/output pins.
    - **Rationale:** Extract-to-subgraph is the primary refactoring tool for graph complexity
      management.
    - **Verification:** Select 10 nodes with external connections, extract to subgraph, and verify
      the call node has correct pins and execution matches.

23. **R-15.8.23** — The engine **SHALL** provide a reusable table editor widget with typed columns,
    inline editing, sort, filter, and formula support for game data tables.
    - **Rationale:** Many game systems (loot, abilities, equipment, economy) share tabular data
      editing needs.
    - **Verification:** Create a data table with 5 typed columns, sort by a numeric column, filter
      by a string column, and verify formula cells compute correctly.
