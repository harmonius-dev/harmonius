# User Stories — 12.2 Asset Processing

## US-12.2.1 Automatically Optimize Assets for Every Target Platform

**As a** DevOps engineer, **I want** automatic texture compression, LOD generation, meshlet
building, vertex optimization, lightmap UVs, audio encoding, and shader compilation, **so that**
assets are optimally processed for each platform without manual per-asset configuration.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| BC7/ASTC/ETC2 compression per platform | F-12.2.1 | R-12.2.1 |
| Automatic LOD chains via edge-collapse simplification | F-12.2.2 | R-12.2.2 |
| Meshlet partitioning with AABB and normal cone bounds | F-12.2.3 | R-12.2.3 |
| Vertex cache and overdraw optimization per meshlet | F-12.2.4 | R-12.2.4 |
| Non-overlapping lightmap UV atlas generation | F-12.2.5 | R-12.2.5 |
| Opus, ADPCM, and PCM audio encoding by preset | F-12.2.6 | R-12.2.6 |
| DAG dependency tracking with circular reference detection | F-12.2.8 | R-12.2.8 |

## US-12.2.2 Generate Human-Readable HLSL from Shader Graphs

**As a** technical artist, **I want** the shader graph compiler to produce clean, readable
HLSL source files with comments tracing each section to its graph node, **so that** I can
debug shaders in Visual Studio with full HLSL Tools support (IntelliSense, error squiggles,
go-to-definition) and understand what the graph compiled to.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Generated .hlsl files are valid standalone HLSL | F-12.2.7 | R-12.2.7 |
| No template markers or non-standard syntax in output | F-12.2.7 | R-12.2.7 |
| Comments annotate which graph node produced each section | F-12.2.7 | R-12.2.7 |
| HLSL Tools IntelliSense works on generated files | F-12.2.7 | R-12.2.7 |
| Only reachable variants are generated | F-12.2.7 | R-12.2.7 |

## US-12.2.3 Author Custom Shader Nodes as HLSL Snippets

**As a** technical artist, **I want** to write custom shader graph nodes as HLSL function
snippets with typed inputs and outputs, **so that** I can extend the material system with
custom effects without modifying the engine, and my snippets appear as reusable nodes in
the shader graph palette.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Custom node is an HLSL function with typed parameters | F-12.2.7 | R-12.2.7 |
| Custom node appears in the graph editor node palette | F-12.2.7, F-15.8.10 | R-12.2.7 |
| Graph compiler calls the function with resolved inputs | F-12.2.7 | R-12.2.7 |
| Generated HLSL includes the custom function definition | F-12.2.7 | R-12.2.7 |

## US-12.2.4 Compile Shaders to All GPU Backends

**As a** DevOps engineer, **I want** the build pipeline to automatically compile generated
HLSL to DXIL (D3D12), SPIR-V (Vulkan), and MSL (Metal) using DXC and Metal Shader Converter,
**so that** every platform gets optimized shader bytecode without manual per-platform steps.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| DXC compiles HLSL to DXIL for D3D12 | F-12.2.9 | R-12.2.9 |
| DXC compiles HLSL to SPIR-V for Vulkan | F-12.2.9 | R-12.2.9 |
| Metal Shader Converter translates DXIL to MSL | F-12.2.9 | R-12.2.9 |
| Compiled bytecode is cached by HLSL source hash | F-12.2.9 | R-12.2.9, R-15.11.2 |
| Both compilers accessed via cxx.rs FFI | F-12.2.9 | R-12.2.9 |

## US-12.2.5 Navigate from Shader Errors to Graph Nodes

**As a** technical artist, **I want** shader compilation errors to show both the HLSL line
number and the graph node that generated the offending code, **so that** I can click the error
in the editor and navigate directly to the visual graph node to fix it.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compilation error includes HLSL line number | F-12.2.9 | R-12.2.9 |
| Error maps to the originating graph node ID | F-12.2.9 | R-12.2.9 |
| Clicking the error in the editor selects the graph node | F-12.2.9, F-15.8.11 | R-12.2.9 |

## US-12.2.6 Track Asset Dependencies for Incremental Builds

**As a** DevOps engineer, **I want** the asset pipeline to track dependencies between all
assets and only rebuild what changed, **so that** CI builds complete in minutes instead of
hours when only a few assets are modified.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| DAG tracks all asset cross-references | F-12.2.8 | R-12.2.8 |
| Modifying one texture rebuilds only dependent materials | F-12.2.8 | R-12.2.8 |
| Circular references are detected and reported | F-12.2.8 | R-12.2.8 |
| Incremental builds are 10x+ faster than full rebuilds | F-12.2.8 | R-X.14.1 |
