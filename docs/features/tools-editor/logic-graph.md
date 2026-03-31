# 15.8 — Visual Logic Graph

> **Note:** The universal logic graph system supersedes the visual scripting features in
> game-framework/scripting.md (F-13.4.x). All gameplay logic, shader authoring, animation control,
> audio mixing, and tool automation are authored through this single unified graph system.

## Graph Runtime

| ID | Feature |
| ---------- | ------------------------------- |
| F-15.8.1 | Universal Logic Graph Runtime |
| F-15.8.2 | Static Type System |
| F-15.8.3 | Strict Validation Before Use |

1. **F-15.8.1** — A typed, functional graph execution model that serves as the sole authoring
   mechanism for all engine logic. Nodes are pure functions with statically typed input and output
   pins. Graphs compile to an optimized bytecode representation or ahead-of-time native code,
   achieving zero overhead versus hand-written logic. The runtime supports generics, pattern
   matching, and higher-order functions as first-class node types, enabling expressive dataflow
   composition without any textual code. Bytecode interpreter is platform-agnostic.
   - **Deps:** F-1.3.1 (Type Registry), F-1.5.1 (Typed Event Channels)
   - **Platform:** AOT compilation targets platform-native instruction sets (x86-64, ARM64).
2. **F-15.8.2** — Every pin on every node carries a static type. Type inference propagates
   bidirectionally through connections so users rarely need to annotate types manually. Type errors
   are caught at graph-edit time, not runtime, and displayed inline on the offending pin. Supports
   primitives, structs, enums, arrays, optionals, generics with trait bounds, and user-defined types
   registered through the reflection system. No implicit coercion is permitted; explicit conversion
   nodes are required for all type transformations.
   - **Deps:** F-15.8.1, F-1.3.1 (Type Registry)
3. **F-15.8.3** — Graphs must pass a comprehensive validation pass before they can be saved,
   compiled, or referenced by other assets. Validation checks include type correctness across all
   connections, absence of dangling or disconnected pins, cycle detection in pure dataflow
   subgraphs, verification that all required inputs are connected, and confirmation that all outputs
   are either consumed or explicitly discarded. Error messages pinpoint the exact node and pin, with
   suggested fixes where possible.
   - **Deps:** F-15.8.2

## Gameplay Logic

| ID | Feature |
| ---------- | ----------------------- |
| F-15.8.4 | Gameplay Logic Graphs |

1. **F-15.8.4** — The primary use case for the logic graph system, replacing all gameplay scripting
   with visual graphs. Covers ability logic, AI behavior, quest conditions, dialogue branching, UI
   event handlers, game mode rules, and input processing. Gameplay graphs access all ECS components,
   resources, events, and commands through typed graph nodes auto-generated from the type registry.
   Supports coroutine-style execution for multi-frame sequences such as phased boss encounters and
   timed quest objectives.
   - **Deps:** F-15.8.1, F-15.8.2, F-1.1.5 (Archetype Queries), F-1.5.1 (Typed Event Channels)

## Shader and Material Authoring

| ID | Feature |
| ----------- | ---------------------------------- |
| F-15.8.5a | Shader Graph Core |
| F-15.8.5b | Shader Graph to HLSL Compilation |
| F-15.8.5c | Material Graph Variant |
| F-15.8.6 | Render Graph Configuration |

1. **F-15.8.5a** — Visual authoring of GPU shaders including vertex, fragment, and compute stages
   using the logic graph system. Nodes represent math operations, texture samples, interpolation,
   branching, and buffer access. Nodes are organized into domain categories (math, sampling,
   utility, flow control). Shader graphs validate stage-specific constraints (e.g., vertex outputs
   must include position).
   - **Deps:** F-15.8.1, F-15.8.2, F-2.1.1 (GPU Abstraction)
2. **F-15.8.5b** — The shader graph compiles to HLSL, which DXC compiles to DXIL (for D3D12) and
   SPIR-V (for Vulkan). Metal Shader Converter translates DXIL to MSL (for Metal). DXC and Metal
   Shader Converter are accessed via Rust FFI bindings (DXC via windows-rs on Windows, libloading on
   Linux; Metal Shader Converter via swift-bridge on macOS). HLSL is the sole shader intermediate
   language. Compilation errors map back to the originating graph node. MSL on macOS. All formats
   are produced through DXC and Metal Shader Converter.
   - **Deps:** F-15.8.5a
   - **Platform:** Shader output format is platform-dependent: DXIL on Windows, SPIR-V on Linux,
3. **F-15.8.5c** — Material graphs are a specialized shader graph variant with PBR inputs (base
   color, metallic, roughness, normal, emissive) and live preview in the viewport. Material graphs
   compile through the same HLSL pipeline as shader graphs. Replaces all hand-written shader code
   with visual authoring. Material parameter changes reflect in the viewport in real time.
   - **Deps:** F-15.8.5b, F-15.3.1 (Material Graph)
4. **F-15.8.6** — Configures the rendering pipeline — passes, resources, and dependencies — via a
   visual graph. Nodes represent render passes (geometry, lighting, shadow, post-process); edges
   represent resource dependencies (textures, buffers, attachments). The graph compiler produces the
   frame graph execution plan with automatic barrier insertion and resource aliasing. Artists and
   technical directors adjust the rendering pipeline visually without writing code.
   - **Deps:** F-15.8.1, F-15.8.3, F-2.3.1 (Render Graph)

## Animation and Audio

| ID | Feature |
| ---------- | ------------------------ |
| F-15.8.7 | Animation Logic Graphs |
| F-15.8.8 | Audio Logic Graphs |

1. **F-15.8.7** — Visual authoring of animation state machines, blend trees, and inverse kinematics
   setups. Nodes represent animation states, transitions with condition predicates, blend operations
   (linear, additive, masked), and IK solvers (two-bone, FABRIK, full-body). Graphs read animation
   parameters from ECS components and drive the animation system. Replaces all code-driven animation
   configuration with visual authoring.
   - **Deps:** F-15.8.1, F-15.8.2, F-9.1.1 (Skeletal Animation), F-9.4.1 (State Machine)
2. **F-15.8.8** — Visual authoring of adaptive audio behavior including music layer mixing,
   real-time parameter control (RTPC) driven effects, dialogue sequencing, and spatial audio
   configuration. Nodes read game state from ECS components and drive audio parameter changes such
   as volume, pitch, reverb wetness, and music stem activation. Enables sound designers to build
   complex reactive soundscapes without writing code.
   - **Deps:** F-15.8.1, F-15.8.2, F-5.1.1 (Audio Engine), F-5.4.1 (Music System)

## Custom Tooling

| ID | Feature |
| ---------- | -------------------- |
| F-15.8.9 | Custom Tool Graphs |

1. **F-15.8.9** — Extends the editor with custom tools authored entirely in the logic graph. Tool
   graphs define UI panels, respond to user input events, manipulate assets, and invoke engine
   commands. No plugin code or external toolchain is required. Enables studios to build
   project-specific editors (quest editors, loot table configurators, dialogue tree tools) using the
   same visual graph system used for gameplay logic. bytecode produced by tool graphs runs only in
   the editor.
   - **Deps:** F-15.8.1, F-15.8.4, F-15.1.8 (Editor Extensions), F-15.1.1 (Panel Layout)
   - **Platform:** Desktop only. Not available on mobile or console runtime. The compiled graph

## Node Library

| ID | Feature |
| ----------- | -------------------- |
| F-15.8.10 | Graph Node Library |

1. **F-15.8.10** — A comprehensive standard library of nodes organized by domain: math, string,
   collection operations, ECS access, physics queries, audio control, rendering parameters, input
   state, networking RPCs, and UI manipulation. Nodes are auto-generated from ECS component,
   resource, and event type registrations via the reflection system. Users create custom node types
   by composing existing nodes into reusable subgraphs saved as assets.
   - **Deps:** F-15.8.1, F-1.3.1 (Type Registry), F-1.3.3 (Property System)

## Debugging and Profiling

| ID | Feature |
| ----------- | ----------------- |
| F-15.8.11 | Graph Debugging |

1. **F-15.8.11** — Provides breakpoints on nodes, step-through execution, and live value inspection
   on pins during play mode. A visual highlight traces the execution flow through the graph in real
   time, showing which nodes fire and in what order. An integrated performance profiler displays
   per-node timing and invocation counts, enabling designers to identify bottleneck nodes without
   leaving the graph editor. server-side graph inspection.
   - **Deps:** F-15.8.1, F-15.5.1 (Profiling Tools)
   - **Platform:** Remote debugging connects to running game instances over the network for

## Compilation and Optimization

| ID | Feature |
| ----------- | ------------------------------------ |
| F-15.8.12 | Graph Compilation and Optimization |

1. **F-15.8.12** — Graphs compile to optimized bytecode or AOT native code through a multi-pass
   compiler. Dead node elimination removes unreachable subgraphs, constant folding evaluates static
   expressions at compile time, subgraph inlining eliminates call overhead for small reusable
   functions, and type specialization monomorphizes generic nodes. The resulting code achieves
   performance comparable to hand-written logic. Compilation errors include node-level diagnostics
   with source location in the graph.
   - **Deps:** F-15.8.1, F-15.8.2, F-15.8.3
   - **Platform:** AOT native code generation uses LLVM on all platforms.

## Version Control Integration

| ID | Feature |
| ----------- | ------------------------- |
| F-15.8.13 | Graph Diffing and Merge |

1. **F-15.8.13** — A visual diff tool showing added, removed, and modified nodes between two graph
   versions with color-coded overlays. Supports three-way merge for collaborative workflows where
   multiple team members edit the same graph. Conflict markers appear on individual nodes with
   accept-mine, accept-theirs, and manual-resolve options. Integrates with Git through a custom diff
   and merge driver registered in repository configuration. requires a custom merge driver installed
   via project setup.
   - **Deps:** F-15.8.1, F-15.1.3 (Undo/Redo)
   - **Platform:** Desktop only. Not available on mobile or console runtime. Git integration

## Search and Refactoring

| ID | Feature |
| ----------- | ------------------------------ |
| F-15.8.14 | Graph Search and Refactoring |

1. **F-15.8.14** — Find all uses of a node type, variable, subgraph reference, or type across the
   entire project. Rename refactoring propagates through all references in all graphs that use the
   renamed element. Find-and-replace supports structural node patterns, enabling bulk updates such
   as replacing deprecated node types with their successors. Search results link directly to the
   node location in each graph for one-click navigation.
   - **Deps:** F-15.8.1, F-15.8.10, F-12.5.1 (Asset Database)
   - **Platform:** Desktop only. Not available on mobile or console runtime.
