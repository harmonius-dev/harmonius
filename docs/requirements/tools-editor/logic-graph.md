# R-15.8 — Visual Logic Graph Requirements

## Graph Runtime

### R-15.8.1 Universal Logic Graph Runtime

The engine **SHALL** provide a typed, functional graph execution model as the sole authoring
mechanism for all engine logic, where nodes are pure functions with statically typed pins, graphs
compile to optimized bytecode or AOT native code achieving zero overhead versus hand-written logic,
and the runtime supports generics, pattern matching, and higher-order functions as first-class
node types.

- **Derived from:** [F-15.8.1](../../features/tools-editor/logic-graph.md)
- **Rationale:** A universal logic graph is the foundation of the no-code engine, ensuring all
  logic is authored visually without textual code while maintaining native performance.
- **Verification:** Author a gameplay graph performing 10,000 ECS queries per frame; benchmark
  the compiled graph against an equivalent hand-written implementation and verify throughput is
  within 5%. Confirm generics, pattern matching, and higher-order function nodes execute
  correctly via unit tests.

### R-15.8.2 Static Type System

The engine **SHALL** enforce static types on every node pin with bidirectional type inference
through connections, catching type errors at graph-edit time with inline error display, supporting
primitives, structs, enums, arrays, optionals, generics with trait bounds, and user-defined types,
with no implicit coercion permitted.

- **Derived from:** [F-15.8.2](../../features/tools-editor/logic-graph.md)
- **Rationale:** Static typing at edit time prevents runtime type errors and guides users toward
  correct graph construction without requiring manual type annotations.
- **Verification:** Connect incompatible pin types and verify an inline error appears before
  save. Create a generic node and verify type inference propagates the concrete type through
  downstream connections. Attempt implicit coercion and verify it is rejected.

### R-15.8.3 Strict Validation Before Use

The engine **SHALL** require graphs to pass a comprehensive validation pass (type correctness,
no dangling pins, cycle detection in pure dataflow subgraphs, all required inputs connected, all
outputs consumed or explicitly discarded) before they can be saved, compiled, or referenced, with
error messages pinpointing the exact node and pin with suggested fixes.

- **Derived from:** [F-15.8.3](../../features/tools-editor/logic-graph.md)
- **Rationale:** Strict validation prevents invalid graphs from entering the asset pipeline,
  eliminating an entire class of runtime errors from shipped content.
- **Verification:** Create graphs with each validation violation (type mismatch, dangling pin,
  cycle, missing input, unconsumed output); verify each is rejected with a message identifying
  the exact node and pin. Fix each violation and confirm the graph passes validation.

## Gameplay Logic

### R-15.8.4 Gameplay Logic Graphs

The engine **SHALL** support visual logic graphs for all gameplay logic (ability logic, AI behavior,
quest conditions, dialogue branching, UI event handlers, game mode rules, input processing) with
typed nodes auto-generated from the type registry for ECS component, resource, event, and command
access, and coroutine-style execution for multi-frame sequences.

- **Derived from:** [F-15.8.4](../../features/tools-editor/logic-graph.md)
- **Rationale:** Gameplay logic graphs are the primary interface for designers in a no-code
  engine, replacing all textual scripting with visual authoring.
- **Verification:** Author a multi-frame boss encounter graph using coroutine nodes; verify
  it executes across 3 phases with correct timing. Verify auto-generated nodes for 5 ECS
  component types provide read/write access and query filtering.

## Shader and Material Authoring

### R-15.8.5a Shader Graph Core

The engine **SHALL** support visual authoring of GPU shaders (vertex, fragment, compute stages)
through the logic graph system with nodes for math operations, texture samples, interpolation,
branching, and buffer access, validating stage-specific constraints at edit time.

- **Derived from:** [F-15.8.5a](../../features/tools-editor/logic-graph.md)
- **Rationale:** Visual shader node authoring enables technical artists to create shader logic
  without writing code, consistent with the no-code philosophy.
- **Verification:** Author a vertex and fragment shader graph; verify stage-specific
  constraints (vertex output must include position) are validated at edit time.

### R-15.8.5b Shader Graph to HLSL Compilation

The engine **SHALL** compile shader graphs to HLSL, which DXC compiles to DXIL and SPIR-V and
Metal Shader Converter translates DXIL to MSL per target platform. DXC and Metal Shader
Converter are C++ libraries accessed via cxx.rs FFI bindings. HLSL is the sole shader
intermediate language. Compilation errors **SHALL** map back to the originating graph node.

- **Derived from:** [F-15.8.5b](../../features/tools-editor/logic-graph.md)
- **Rationale:** A single HLSL intermediate language simplifies the compilation pipeline
  while DXC and MSC produce platform-optimized output.
- **Verification:** Compile a shader graph and verify correct output format per platform
  (DXIL on Windows, SPIR-V on Linux, MSL on macOS). Introduce an error and verify the
  diagnostic maps to the originating graph node.

### R-15.8.5c Material Graph Variant

The engine **SHALL** provide a material graph variant with PBR inputs (base color, metallic,
roughness, normal, emissive) that compiles through the same HLSL pipeline and provides live
viewport preview with real-time parameter updates.

- **Derived from:** [F-15.8.5c](../../features/tools-editor/logic-graph.md)
- **Rationale:** Material graphs give artists direct control over PBR material authoring with
  immediate visual feedback, replacing hand-written shader code.
- **Verification:** Author a PBR material graph; verify it renders correctly in the viewport.
  Modify a parameter and confirm live preview updates within one frame.

### R-15.8.6 Render Graph Configuration

The engine **SHALL** support visual configuration of the rendering pipeline (passes, resources,
dependencies) through a graph where nodes represent render passes and edges represent resource
dependencies, with the compiler producing a frame graph execution plan with automatic barrier
insertion and resource aliasing.

- **Derived from:** [F-15.8.6](../../features/tools-editor/logic-graph.md)
- **Rationale:** Visual render graph configuration allows technical directors to adjust the
  rendering pipeline without code, while automatic barrier insertion prevents synchronization
  errors.
- **Verification:** Configure a render graph with geometry, lighting, shadow, and post-process
  passes via the visual editor; verify the compiled execution plan inserts correct barriers
  between dependent passes and aliases non-overlapping resources.

## Animation and Audio

### R-15.8.7 Animation Logic Graphs

The engine **SHALL** support visual authoring of animation state machines, blend trees (linear,
additive, masked), and IK setups (two-bone, FABRIK, full-body) through logic graphs that read
animation parameters from ECS components and drive the animation system, replacing all code-driven
animation configuration.

- **Derived from:** [F-15.8.7](../../features/tools-editor/logic-graph.md)
- **Rationale:** Visual animation graphs give animators direct control over state machines and
  blend logic without requiring programmer support.
- **Verification:** Author an animation state machine with 3 states and conditional transitions;
  verify state changes occur when ECS component values meet transition conditions. Test blend
  tree output and IK solver positioning against expected results.

### R-15.8.8 Audio Logic Graphs

The engine **SHALL** support visual authoring of adaptive audio behavior (music layer mixing, RTPC
effects, dialogue sequencing, spatial audio configuration) through logic graphs that read game
state from ECS components and drive audio parameters (volume, pitch, reverb, music stems).

- **Derived from:** [F-15.8.8](../../features/tools-editor/logic-graph.md)
- **Rationale:** Visual audio graphs enable sound designers to build reactive soundscapes without
  writing code, maintaining the no-code authoring philosophy across all engine domains.
- **Verification:** Author an audio graph that crossfades 2 music layers based on an ECS
  component value; verify the crossfade occurs at the correct threshold. Test RTPC-driven
  reverb parameter changes and confirm audio output matches expected values.

## Custom Tooling

### R-15.8.9 Custom Tool Graphs

The engine **SHALL** support extending the editor with custom tools authored entirely in the logic
graph system, defining UI panels, responding to input events, manipulating assets, and invoking
engine commands, with no plugin code or external toolchain required.

- **Derived from:** [F-15.8.9](../../features/tools-editor/logic-graph.md)
- **Rationale:** Graph-authored custom tools let studios build project-specific editors (quest
  editors, loot table configurators) using the same visual system used for gameplay, eliminating
  the need for compiled plugins.
- **Verification:** Author a custom tool graph that creates a UI panel with a button; verify
  clicking the button invokes an engine command (e.g., spawning an entity). Confirm the tool
  loads without any compiled plugin or external toolchain.

## Node Library

### R-15.8.10 Graph Node Library

The engine **SHALL** provide a standard node library organized by domain (math, string, collection,
ECS, physics, audio, rendering, input, networking, UI) with nodes auto-generated from ECS
component, resource, and event type registrations via the reflection system, and user-created
custom node types composed from existing nodes saved as reusable subgraph assets.

- **Derived from:** [F-15.8.10](../../features/tools-editor/logic-graph.md)
- **Rationale:** A comprehensive node library with auto-generated ECS nodes ensures the graph
  system covers all engine functionality without manual node authoring for each new type.
- **Verification:** Register a new ECS component type; verify a corresponding node appears in
  the library automatically. Compose 3 existing nodes into a subgraph, save it, and verify it
  appears as a reusable node in the library. Confirm nodes exist for each listed domain.

## Debugging and Profiling

### R-15.8.11 Graph Debugging

The engine **SHALL** provide breakpoints on nodes, step-through execution, live pin value
inspection during play mode, visual execution flow highlighting showing which nodes fire and in
what order, and an integrated per-node performance profiler displaying timing and invocation
counts.

- **Derived from:** [F-15.8.11](../../features/tools-editor/logic-graph.md)
- **Rationale:** Graph debugging tools are essential in a no-code engine where all logic is
  visual, giving designers the same diagnostic power as traditional code debuggers.
- **Verification:** Set a breakpoint on a node; verify execution pauses at that node during play
  mode and pin values are inspectable. Step through 3 nodes and verify execution advances
  correctly. Confirm the profiler reports per-node timing for at least 100 invocations.

## Compilation and Optimization

### R-15.8.12 Graph Compilation and Optimization

The engine **SHALL** compile graphs to optimized bytecode or AOT native code through a multi-pass
compiler performing dead node elimination, constant folding, subgraph inlining, and type
specialization (monomorphization of generics), achieving performance comparable to hand-written
logic, with node-level compilation error diagnostics.

- **Derived from:** [F-15.8.12](../../features/tools-editor/logic-graph.md)
- **Rationale:** Compiler optimizations ensure visual logic graphs achieve native performance,
  removing any incentive to bypass the no-code system for performance reasons.
- **Verification:** Compile a graph with dead nodes, constant expressions, a small subgraph,
  and generic nodes; verify the output eliminates dead nodes, folds constants, inlines the
  subgraph, and monomorphizes generics. Benchmark the optimized output against an unoptimized
  baseline and verify measurable improvement.

## Version Control Integration

### R-15.8.13 Graph Diffing and Merge

The engine **SHALL** provide a visual diff tool showing added, removed, and modified nodes between
two graph versions with color-coded overlays, three-way merge with per-node conflict markers
(accept-mine, accept-theirs, manual-resolve), and Git integration via a custom diff and merge
driver.

- **Derived from:** [F-15.8.13](../../features/tools-editor/logic-graph.md)
- **Rationale:** Visual diff and merge enable collaborative graph editing in teams using version
  control, preventing conflicts from blocking the no-code workflow.
- **Verification:** Modify the same graph on two branches; verify the visual diff highlights
  added, removed, and modified nodes. Perform a three-way merge with a conflict on one node;
  verify conflict markers appear and accept-mine/accept-theirs resolve correctly.

## Non-Functional Requirements

### R-15.8.NF1 Graph Editor Responsiveness

The graph editor **SHALL** maintain interactive frame rates (above 30 FPS) while displaying graphs
containing up to 10,000 nodes with 50,000 connections. Node creation, deletion, and connection
operations **SHALL** complete within one frame (under 33ms). Type inference propagation after a
connection change **SHALL** complete within 100ms for graphs up to 10,000 nodes. Validation
**SHALL** complete within 500ms for graphs up to 10,000 nodes.

- **Derived from:** F-15.8.1 through F-15.8.14 (all logic graph features).
- **Rationale:** Large gameplay and shader graphs in production projects can reach thousands of
  nodes. The editor must remain responsive at this scale to support iterative authoring.
- **Verification:** Load a synthetic graph with 10,000 nodes and 50,000 connections. Measure
  render frame rate and assert it stays above 30 FPS during pan and zoom. Create a node and assert
  the operation completes within 33ms. Change a connection and assert type inference completes
  within 100ms. Trigger full validation and assert completion within 500ms.

## Search and Refactoring

### R-15.8.14 Graph Search and Refactoring

The engine **SHALL** support finding all uses of a node type, variable, subgraph reference, or type
across the entire project, rename refactoring that propagates through all referencing graphs,
structural find-and-replace for bulk node pattern updates, and one-click navigation from search
results to node locations.

- **Derived from:** [F-15.8.14](../../features/tools-editor/logic-graph.md)
- **Rationale:** Project-wide search and refactoring are critical for maintaining large no-code
  projects where hundreds of graphs may reference shared node types and subgraphs.
- **Verification:** Create a custom node type used in 5 graphs; rename it and verify all 5
  graphs update to the new name. Use structural find-and-replace to swap a deprecated node
  type in 3 graphs; verify all instances are replaced. Click a search result and verify
  navigation opens the correct graph at the correct node.
