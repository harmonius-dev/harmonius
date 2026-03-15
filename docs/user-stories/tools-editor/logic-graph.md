# User Stories: Visual Logic Graph

## US-15.8.1 No-Code Gameplay Authoring

**As a** designer, **I want** to author all gameplay logic -- abilities, AI behavior, quest
conditions, and dialogue branching -- through visual graphs with typed pins and live
debugging, **so that** I can build and iterate on game mechanics without writing code.

## US-15.8.2 Graph Search and Refactoring

**As a** designer, **I want** to find all uses of a node type across the entire project and
rename it with automatic propagation to every referencing graph, **so that** I can
refactor large no-code projects safely without manually updating hundreds of graphs.

## US-15.8.3 Author Shader Logic Visually

**As a** technical artist, **I want** to author vertex, fragment, and compute shaders using
visual graph nodes for math, texture sampling, and buffer access, **so that** I can create
shader logic without writing HLSL code.

## US-15.8.4 Compile Shader Graphs to All Target Platforms

**As a** technical artist, **I want** my shader graph to compile to HLSL and then to DXIL,
SPIR-V, or MSL per target platform, with errors mapped back to the originating node, **so
that** I can target all platforms from a single visual graph.

## US-15.8.5 Create PBR Materials With Live Preview

**As an** artist, **I want** to author PBR materials using a specialized graph variant with
base color, metallic, roughness, normal, and emissive inputs and see live viewport preview,
**so that** I can iterate on material appearance in real time.
