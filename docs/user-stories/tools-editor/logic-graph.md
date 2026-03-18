# User Stories: Visual Logic Graph

## F-15.8.1 Universal Logic Graph Runtime

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.8.1.1 | designer (P-5)          |          |              |
| US-15.8.1.2 | engine developer (P-26) |          |              |
| US-15.8.1.3 | tech artist (P-13)      |          |              |
| US-15.8.1.4 | engine tester (P-27)    |          |              |

1. **US-15.8.1.1** — a typed, functional graph execution model that serves as the sole authoring
   mechanism for all engine logic
   - **Acceptance:** I can build gameplay, shaders, animation, and audio behavior without writing
     any textual code
2. **US-15.8.1.2** — graphs to compile to optimized bytecode or ahead-of-time native code
   - **Acceptance:** visually authored logic achieves zero overhead versus hand-written code
3. **US-15.8.1.3** — generics, pattern matching, and higher-order functions as first-class node
   types
   - **Acceptance:** I can express complex dataflow compositions without workarounds
4. **US-15.8.1.4** — to verify that AOT native code targets platform-native instruction sets
   (x86-64, ARM64) correctly
   - **Acceptance:** compiled graphs execute identically on all supported platforms

## F-15.8.2 Static Type System

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.8.2.1 | designer (P-5)          |          |              |
| US-15.8.2.2 | tech artist (P-13)      |          |              |
| US-15.8.2.3 | developer (P-15)        |          |              |
| US-15.8.2.4 | engine developer (P-26) |          |              |

1. **US-15.8.2.1** — type errors caught at graph-edit time and displayed inline on the offending pin
   - **Acceptance:** I fix type mismatches immediately rather than discovering them at runtime
2. **US-15.8.2.2** — type inference to propagate bidirectionally through connections so I rarely
   need to annotate types manually
   - **Acceptance:** authoring graphs is fast and natural
3. **US-15.8.2.3** — to register user-defined types through the reflection system with full support
   for primitives, structs, enums, arrays, optionals, and generics with trait bounds
   - **Acceptance:** my custom data structures integrate seamlessly with the graph system
4. **US-15.8.2.4** — no implicit coercion between types, requiring explicit conversion nodes for all
   type transformations
   - **Acceptance:** the type system catches subtle data loss bugs

## F-15.8.3 Strict Validation Before Use

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.8.3.1 | designer (P-5)          |          |              |
| US-15.8.3.2 | developer (P-15)        |          |              |
| US-15.8.3.3 | engine developer (P-26) |          |              |
| US-15.8.3.4 | engine tester (P-27)    |          |              |

1. **US-15.8.3.1** — comprehensive validation that checks type correctness, dangling pins, cycles,
   and missing inputs before a graph can be saved
   - **Acceptance:** I never ship a broken graph to the runtime
2. **US-15.8.3.2** — validation error messages to pinpoint the exact node and pin with suggested
   fixes
   - **Acceptance:** I can resolve issues quickly without hunting through large graphs
3. **US-15.8.3.3** — cycle detection in pure dataflow subgraphs
   - **Acceptance:** the compiler can guarantee termination and prevent infinite loops
4. **US-15.8.3.4** — to verify that invalid graphs are rejected on save, compile, and reference
   - **Acceptance:** no broken graph enters the asset pipeline

## F-15.8.4 Gameplay Logic Graphs

| ID          | Persona          | Features | Requirements |
|-------------|------------------|----------|--------------|
| US-15.8.4.1 | designer (P-5)   |          |              |
| US-15.8.4.2 | designer (P-5)   |          |              |
| US-15.8.4.3 | developer (P-15) |          |              |
| US-15.8.4.4 | modder (P-24)    |          |              |

1. **US-15.8.4.1** — to author ability logic, AI behavior, quest conditions, and dialogue branching
   through visual graphs that access all ECS components and events
   - **Acceptance:** I can build complete gameplay systems without programmer support
2. **US-15.8.4.2** — coroutine-style execution for multi-frame sequences such as phased boss
   encounters and timed quest objectives
   - **Acceptance:** I can author complex temporal gameplay without tracking frame state manually
3. **US-15.8.4.3** — typed graph nodes auto-generated from ECS component, resource, and event type
   registrations
   - **Acceptance:** the graph system stays in sync with the engine's data model automatically
4. **US-15.8.4.4** — to use gameplay logic graphs in the mod SDK to create custom abilities and
   quests
   - **Acceptance:** I can add gameplay content using the same tools as the development team

## F-15.8.5a Shader Graph Core

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.8.5a.1 | tech artist (P-13)      |          |              |
| US-15.8.5a.2 | artist (P-8)            |          |              |
| US-15.8.5a.3 | engine developer (P-26) |          |              |
| US-15.8.5a.4 | engine tester (P-27)    |          |              |

1. **US-15.8.5a.1** — to author vertex, fragment, and compute shaders using visual graph nodes for
   math operations, texture samples, and buffer access
   - **Acceptance:** I can create GPU programs without writing HLSL code
2. **US-15.8.5a.2** — shader nodes organized into domain categories (math, sampling, utility, flow
   control)
   - **Acceptance:** I can find the right node quickly in a searchable palette
3. **US-15.8.5a.3** — shader graphs to validate stage-specific constraints (e.g., vertex outputs
   must include position)
   - **Acceptance:** invalid shader configurations are caught at authoring time
4. **US-15.8.5a.4** — to verify that shader graphs spanning multiple stages (vertex to fragment)
   pass data correctly across stage boundaries
   - **Acceptance:** inter-stage communication works reliably

## F-15.8.5b Shader Graph to HLSL Compilation

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.8.5b.1 | tech artist (P-13)      |          |              |
| US-15.8.5b.2 | engine developer (P-26) |          |              |
| US-15.8.5b.3 | DevOps engineer (P-16)  |          |              |
| US-15.8.5b.4 | engine tester (P-27)    |          |              |

1. **US-15.8.5b.1** — my shader graph to compile to HLSL, then to DXIL, SPIR-V, or MSL per target
   platform with errors mapped back to the originating node
   - **Acceptance:** I can target all platforms from a single visual graph
2. **US-15.8.5b.2** — DXC and Metal Shader Converter accessed via cxx.rs FFI bindings
   - **Acceptance:** the shader compilation pipeline integrates safely with the Rust codebase
3. **US-15.8.5b.3** — compiled shader variants cached by platform in the shared build cache
   - **Acceptance:** developers download pre-compiled shaders instead of recompiling locally
4. **US-15.8.5b.4** — to verify that the same shader graph produces functionally identical output on
   D3D12 (DXIL), Vulkan (SPIR-V), and Metal (MSL)
   - **Acceptance:** cross-platform rendering is consistent

## F-15.8.5c Material Graph Variant

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.8.5c.1 | artist (P-8)            |          |              |
| US-15.8.5c.2 | creative director (P-2) |          |              |
| US-15.8.5c.3 | tech artist (P-13)      |          |              |
| US-15.8.5c.4 | engine tester (P-27)    |          |              |

1. **US-15.8.5c.1** — a specialized material graph variant with PBR inputs (base color, metallic,
   roughness, normal, emissive) and live viewport preview
   - **Acceptance:** I can author materials visually and see real-time results
2. **US-15.8.5c.2** — material parameter changes to reflect in the viewport in real time
   - **Acceptance:** I can direct material look during reviews without waiting for recompilation
3. **US-15.8.5c.3** — material graphs to compile through the same HLSL pipeline as shader graphs
   - **Acceptance:** all visual shader authoring shares a single compilation path
4. **US-15.8.5c.4** — to verify that material graph PBR outputs match the expected physically based
   rendering model
   - **Acceptance:** materials look correct under all lighting conditions

## F-15.8.6 Render Graph Configuration

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.8.6.1 | tech artist (P-13)      |          |              |
| US-15.8.6.2 | creative director (P-2) |          |              |
| US-15.8.6.3 | engine developer (P-26) |          |              |
| US-15.8.6.4 | engine tester (P-27)    |          |              |

1. **US-15.8.6.1** — to configure the rendering pipeline (passes, resources, dependencies) via a
   visual graph
   - **Acceptance:** I can adjust the rendering pipeline without writing code
2. **US-15.8.6.2** — to visually reorder or disable render passes to achieve a specific visual style
   - **Acceptance:** I can experiment with rendering approaches during pre-production
3. **US-15.8.6.3** — the graph compiler to produce the frame graph execution plan with automatic
   barrier insertion and resource aliasing
   - **Acceptance:** the render pipeline is always correctly synchronized
4. **US-15.8.6.4** — to verify that render graph resource dependencies prevent read-after-write
   hazards
   - **Acceptance:** the visual pipeline configuration produces correct frame output

## F-15.8.7 Animation Logic Graphs

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.8.7.1 | artist (P-8)         |          |              |
| US-15.8.7.2 | designer (P-5)       |          |              |
| US-15.8.7.3 | tech artist (P-13)   |          |              |
| US-15.8.7.4 | engine tester (P-27) |          |              |

1. **US-15.8.7.1** — to visually author animation state machines, blend trees, and IK setups with
   nodes for states, transitions, blend operations, and IK solvers
   - **Acceptance:** I can define complex animation behavior without code
2. **US-15.8.7.2** — transition condition predicates that read animation parameters from ECS
   components
   - **Acceptance:** animation state changes respond to gameplay events automatically
3. **US-15.8.7.3** — IK solver nodes (two-bone, FABRIK, full-body) in the animation graph
   - **Acceptance:** I can set up procedural animation adjustments visually
4. **US-15.8.7.4** — to verify that animation logic graphs produce identical output to equivalent
   code-driven animation
   - **Acceptance:** visual authoring does not introduce animation quality regressions

## F-15.8.8 Audio Logic Graphs

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.8.8.1 | designer (P-5)          |          |              |
| US-15.8.8.2 | artist (P-8)            |          |              |
| US-15.8.8.3 | creative director (P-2) |          |              |
| US-15.8.8.4 | engine tester (P-27)    |          |              |

1. **US-15.8.8.1** — to visually author adaptive audio behavior including music layer mixing and
   parameter-driven effects
   - **Acceptance:** the game's soundscape reacts to gameplay state without code
2. **US-15.8.8.2** — audio graph nodes that configure spatial audio, reverb wetness, and volume
   based on ECS component data
   - **Acceptance:** sound responds to the game environment dynamically
3. **US-15.8.8.3** — music stem activation nodes in the audio graph
   - **Acceptance:** I can design adaptive music transitions that match the emotional arc of
     gameplay
4. **US-15.8.8.4** — to verify that audio graph parameters stay within valid ranges (volume 0-1,
   pitch bounds)
   - **Acceptance:** extreme values do not cause audio distortion or crashes

## F-15.8.9 Custom Tool Graphs

| ID          | Persona            | Features | Requirements |
|-------------|--------------------|----------|--------------|
| US-15.8.9.1 | tech artist (P-13) |          |              |
| US-15.8.9.2 | designer (P-5)     |          |              |
| US-15.8.9.3 | developer (P-15)   |          |              |
| US-15.8.9.4 | modder (P-24)      |          |              |

1. **US-15.8.9.1** — to author custom editor tools entirely in the logic graph (defining UI panels,
   handling input, manipulating assets)
   - **Acceptance:** I can build studio-specific tools without writing plugin code
2. **US-15.8.9.2** — to build a project-specific quest editor using tool graphs
   - **Acceptance:** my team has tailored editing tools for our game's unique content types
3. **US-15.8.9.3** — tool graphs to invoke engine commands and manipulate assets programmatically
   - **Acceptance:** custom tools have full access to editor functionality
4. **US-15.8.9.4** — to create custom tools using the logic graph in the mod SDK
   - **Acceptance:** I can build specialized editing workflows for my mod content

## F-15.8.10 Graph Node Library

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.8.10.1 | designer (P-5)          |          |              |
| US-15.8.10.2 | engine developer (P-26) |          |              |
| US-15.8.10.3 | tech artist (P-13)      |          |              |
| US-15.8.10.4 | modder (P-24)           |          |              |

1. **US-15.8.10.1** — a comprehensive standard library of nodes organized by domain (math, ECS,
   physics, audio, rendering, input, networking, UI)
   - **Acceptance:** I can find the right node for any task quickly
2. **US-15.8.10.2** — nodes auto-generated from ECS component, resource, and event type
   registrations via the reflection system
   - **Acceptance:** the node library stays in sync with the engine's type registry automatically
3. **US-15.8.10.3** — to create custom node types by composing existing nodes into reusable
   subgraphs saved as assets
   - **Acceptance:** I can extend the node library with project-specific operations
4. **US-15.8.10.4** — the mod SDK to expose a curated subset of the node library appropriate for
   modding
   - **Acceptance:** I can build mod logic safely within the defined mod constraints

## F-15.8.11 Graph Debugging

| ID           | Persona             | Features | Requirements |
|--------------|---------------------|----------|--------------|
| US-15.8.11.1 | designer (P-5)      |          |              |
| US-15.8.11.2 | tech artist (P-13)  |          |              |
| US-15.8.11.3 | developer (P-15)    |          |              |
| US-15.8.11.4 | server admin (P-22) |          |              |

1. **US-15.8.11.1** — to set breakpoints on nodes, step through execution, and inspect live values
   on pins during play mode
   - **Acceptance:** I can debug gameplay logic without leaving the graph editor
2. **US-15.8.11.2** — a visual highlight that traces execution flow through the graph in real time
   - **Acceptance:** I can see which nodes fire and in what order
3. **US-15.8.11.3** — an integrated performance profiler showing per-node timing and invocation
   counts
   - **Acceptance:** I can identify bottleneck nodes without switching to a separate profiling tool
4. **US-15.8.11.4** — remote debugging that connects to running game instances over the network
   - **Acceptance:** I can inspect server-side graph execution without stopping the server

## F-15.8.12 Graph Compilation and Optimization

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.8.12.1 | engine developer (P-26) |          |              |
| US-15.8.12.2 | DevOps engineer (P-16)  |          |              |
| US-15.8.12.3 | developer (P-15)        |          |              |
| US-15.8.12.4 | engine tester (P-27)    |          |              |

1. **US-15.8.12.1** — the multi-pass compiler to perform dead node elimination, constant folding,
   subgraph inlining, and type specialization
   - **Acceptance:** compiled graphs achieve performance comparable to hand-written logic
2. **US-15.8.12.2** — compiled graph bytecode and AOT native code cached in the shared build cache
   - **Acceptance:** unchanged graphs are not recompiled on every build
3. **US-15.8.12.3** — compilation errors to include node-level diagnostics with source location in
   the graph
   - **Acceptance:** I can navigate directly to the problematic node
4. **US-15.8.12.4** — to verify that optimized graph output is semantically identical to unoptimized
   output
   - **Acceptance:** compiler optimizations never change runtime behavior

## F-15.8.13 Graph Diffing and Merge

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.8.13.1 | developer (P-15)       |          |              |
| US-15.8.13.2 | designer (P-5)         |          |              |
| US-15.8.13.3 | DevOps engineer (P-16) |          |              |
| US-15.8.13.4 | engine tester (P-27)   |          |              |

1. **US-15.8.13.1** — a visual diff tool showing added, removed, and modified nodes between two
   graph versions with color-coded overlays
   - **Acceptance:** I can review graph changes during code review
2. **US-15.8.13.2** — three-way merge with conflict markers on individual nodes and
   accept-mine/accept-theirs/manual-resolve options
   - **Acceptance:** I can merge concurrent graph edits without losing work
3. **US-15.8.13.3** — a custom Git diff and merge driver registered automatically in repository
   configuration
   - **Acceptance:** graph merges work through standard Git workflows
4. **US-15.8.13.4** — to verify that three-way graph merges produce correct, valid graphs when
   changes do not conflict
   - **Acceptance:** collaborative workflows proceed without manual intervention in non-conflicting
     cases

## F-15.8.14 Graph Search and Refactoring

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.8.14.1 | designer (P-5)       |          |              |
| US-15.8.14.2 | developer (P-15)     |          |              |
| US-15.8.14.3 | tech artist (P-13)   |          |              |
| US-15.8.14.4 | engine tester (P-27) |          |              |

1. **US-15.8.14.1** — to find all uses of a node type, variable, or subgraph reference across the
   entire project
   - **Acceptance:** I can understand the impact of changes before making them
2. **US-15.8.14.2** — rename refactoring that propagates through all references in all graphs
   - **Acceptance:** I can rename types and variables safely without manually updating each graph
3. **US-15.8.14.3** — structural find-and-replace that supports node pattern matching for bulk
   updates (e.g., replacing deprecated node types with successors)
   - **Acceptance:** I can migrate large projects to updated APIs efficiently
4. **US-15.8.14.4** — to verify that rename and replace operations leave all affected graphs in a
   valid, compilable state
   - **Acceptance:** refactoring never introduces broken references
