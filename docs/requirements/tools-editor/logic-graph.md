# R-15.8 -- Visual Logic Graph Requirements

## Graph Runtime

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-15.8.1 | [F-15.8.1](../../features/tools-editor/logic-graph.md) |
| R-15.8.2 | [F-15.8.2](../../features/tools-editor/logic-graph.md) |
| R-15.8.3 | [F-15.8.3](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.1** — The engine **SHALL** provide a typed, functional graph execution model as the sole
   authoring mechanism for all engine logic, compiling to bytecode or AOT native code with zero
   overhead versus hand-written equivalents (within 5% throughput), supporting generics, pattern
   matching, and higher-order functions as first-class node types.
   - **Rationale:** A no-code engine requires a universal graph runtime that matches native
     performance.
   - **Verification:** Benchmark: compare compiled graph throughput against hand-written equivalents
     and verify within 5%.
2. **R-15.8.2** — Every pin **SHALL** carry a static type with bidirectional inference through
   connections, supporting primitives, structs, enums, arrays, optionals, generics with trait
   bounds, and user-defined types from the reflection system, with no implicit coercion and type
   errors caught at graph-edit time.
   - **Rationale:** Edit-time type safety prevents runtime crashes and provides clear error
     messages.
   - **Verification:** Unit test: connect incompatible pin types and verify rejection with inline
     error. Verify bidirectional inference resolves types correctly.
3. **R-15.8.3** — Graphs **SHALL** pass comprehensive validation (type correctness, no dangling
   pins, cycle detection in pure dataflow, all required inputs connected) before save, compile, or
   reference, with error messages pinpointing the exact node and pin with suggested fixes.
   - **Rationale:** Invalid graphs must never enter the asset pipeline or reach runtime.
   - **Verification:** Unit test: create each violation type and verify the error references the
     correct node and pin.

## Gameplay Logic

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-15.8.4 | [F-15.8.4](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.4** — The engine **SHALL** support gameplay logic graphs for ability logic, quest
   conditions, dialogue branching, game mode rules, and input processing, with coroutine-style
   multi-frame execution and auto-generated nodes for all ECS components and events from the type
   registry.
   - **Rationale:** All gameplay logic must be authorable without code per the no-code constraint.
   - **Verification:** Integration test: execute a multi-frame boss encounter graph across 3 phases
     and verify correct timing and state transitions.

## Shader and Material

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.8.5a | [F-15.8.5a](../../features/tools-editor/logic-graph.md) |
| R-15.8.5b | [F-15.8.5b](../../features/tools-editor/logic-graph.md) |
| R-15.8.5c | [F-15.8.5c](../../features/tools-editor/logic-graph.md) |
| R-15.8.6  | [F-15.8.6](../../features/tools-editor/logic-graph.md)  |

1. **R-15.8.5a** — The engine **SHALL** support visual authoring of vertex, fragment, and compute
   shaders with math, texture, and buffer access nodes, validating stage-specific constraints (e.g.,
   vertex output requires position) at edit time.
   - **Rationale:** No-code shader authoring requires visual graphs with GPU-specific validation.
   - **Verification:** Unit test: create a vertex shader graph missing position output and verify
     edit-time rejection.
2. **R-15.8.5b** — Shader graphs **SHALL** compile to HLSL, with DXC producing DXIL and SPIR-V and
   Metal Shader Converter producing MSL via cxx.rs FFI, with compilation errors mapped to graph
   nodes.
   - **Rationale:** HLSL as sole intermediate language with platform-specific backends ensures
     cross-platform shader support.
   - **Verification:** Integration test: compile a shader graph and verify correct output format per
     platform (DXIL, SPIR-V, MSL).
3. **R-15.8.5c** — Material graphs **SHALL** provide PBR inputs (base color, metallic, roughness,
   normal, emissive), compile through the HLSL pipeline, and update the viewport preview within one
   frame after parameter changes.
   - **Rationale:** Real-time material preview is essential for artist iteration speed.
   - **Verification:** Benchmark: change a material parameter and verify the preview updates within
     one frame.
4. **R-15.8.6** — The engine **SHALL** support visual render pipeline configuration with nodes for
   geometry, lighting, shadow, and post-process passes, with automatic barrier insertion and
   resource aliasing for GPU synchronization.
   - **Rationale:** No-code render pipeline configuration enables technical artists to adjust
     quality/performance tradeoffs visually.
   - **Verification:** Unit test: verify barriers are inserted between dependent render passes.

## Animation and Audio

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-15.8.7 | [F-15.8.7](../../features/tools-editor/logic-graph.md) |
| R-15.8.8 | [F-15.8.8](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.7** — The engine **SHALL** support visual authoring of animation state machines, blend
   trees (linear, additive, masked), and IK solver nodes (two-bone, FABRIK, full-body), with
   animation parameters read from ECS components.
   - **Rationale:** No-code animation logic requires visual state machines driven by gameplay state.
   - **Verification:** Unit test: set an ECS value meeting a transition condition and verify the
     state change triggers.
2. **R-15.8.8** — The engine **SHALL** support visual authoring of adaptive audio behavior with
   music layer mixing driven by game state, RTPC-driven effect nodes, and dialogue sequencing.
   - **Rationale:** No-code audio logic enables reactive soundscapes without programming.
   - **Verification:** Unit test: verify music layer crossfade triggers at the correct ECS threshold
     value.

## Tooling

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.8.9  | [F-15.8.9](../../features/tools-editor/logic-graph.md)  |
| R-15.8.10 | [F-15.8.10](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.9** — The engine **SHALL** support editor tools authored entirely as logic graphs with
   UI panel definitions and asset manipulation, requiring no compiled plugin code.
   - **Rationale:** Project-specific tools must be authorable without compiled code per the no-code
     constraint.
   - **Verification:** Integration test: build and run a custom tool graph without any compiled
     plugin or external toolchain.
2. **R-15.8.10** — The engine **SHALL** provide a standard node library organized by domain (math,
   ECS, physics, audio) with nodes auto-generated from ECS type registrations and support for
   user-defined subgraph nodes saved as reusable assets.
   - **Rationale:** Complete node coverage and auto-generation from the type registry ensure no
     capability gaps.
   - **Verification:** Unit test: register a new ECS component type and verify a corresponding node
     appears automatically.

## Debugging

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.8.11 | [F-15.8.11](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.11** — The engine **SHALL** support breakpoints on graph nodes, step-through execution,
   live pin value inspection, and visual execution flow highlighting during play mode, with an
   integrated per-node performance profiler.
   - **Rationale:** Debugging visual logic requires the same capabilities as code debugging.
   - **Verification:** Unit test: set a breakpoint, trigger it, and verify execution pauses with
     inspectable pin values.

## Compilation

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.8.12 | [F-15.8.12](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.12** — The graph compiler **SHALL** perform dead node elimination, constant folding,
   subgraph inlining, and type specialization (monomorphization), with AOT native code generation
   via LLVM.
   - **Rationale:** Optimization passes are required to achieve zero-overhead versus hand-written
     logic.
   - **Verification:** Benchmark: verify optimized output is measurably faster than unoptimized
     baseline.

## Version Control Integration

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.8.13 | [F-15.8.13](../../features/tools-editor/logic-graph.md) |
| R-15.8.14 | [F-15.8.14](../../features/tools-editor/logic-graph.md) |

1. **R-15.8.13** — The engine **SHALL** provide visual diff showing added, removed, and modified
   nodes, three-way merge for concurrent graph edits, per-node conflict markers with resolve
   options, and a custom Git diff and merge driver.
   - **Rationale:** Graph assets must be diffable and mergeable for collaborative development.
   - **Verification:** Unit test: merge two compatible edits and verify correct result. Merge two
     conflicting edits and verify conflict markers appear.
2. **R-15.8.14** — The engine **SHALL** support finding all uses of a node type, rename refactoring
   propagating through all graphs, structural find-and-replace for node patterns, and one-click
   navigation from search results to nodes.
   - **Rationale:** Large projects with thousands of graphs require project-wide search and
     refactoring.
   - **Verification:** Unit test: rename a node type and verify all referencing graphs are updated.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/logic-graph.md](../../user-stories/tools-editor/logic-graph.md).
Requirements in this document are derived from those user stories.
