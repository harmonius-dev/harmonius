# R-12.2 Asset Processing

## Texture Compression

1. **R-12.2.1** — The engine **SHALL** compress imported textures offline into GPU-native
   block-compressed formats (BC7/BC6H for desktop, ASTC for Apple Silicon and mobile, ETC2 as mobile
   fallback) with quality levels and format selection driven by import presets and per-platform
   override tables.
   - **Rationale:** Platform-appropriate block compression minimizes GPU memory and bandwidth while
     maintaining visual quality across all target hardware.
   - **Verification:** Compress a test texture for each target platform and confirm the output
     format matches the override table; compare PSNR against the quality threshold for each preset.

## LOD Generation

2. **R-12.2.2** — The engine **SHALL** generate automatic discrete LOD chains using edge-collapse
   mesh simplification, with each LOD level targeting a configurable triangle-count ratio and error
   threshold to preserve silhouette fidelity, requiring no manual artist intervention.
   - **Rationale:** Fully automated LOD generation is needed for worlds with millions of unique
     meshes where manual LOD authoring is infeasible.
   - **Verification:** Generate an LOD chain for a reference mesh and confirm each level meets its
     triangle-count ratio; confirm silhouette deviation stays below the configured limit.

## Meshlet Building

3. **R-12.2.3** — The engine **SHALL** partition each LOD mesh into fixed-size meshlets (max 64
   vertices, 124 triangles) and precompute per-meshlet bounds (AABB and normal cone) for GPU-driven
   culling.
   - **Rationale:** Meshlet partitioning is a prerequisite for the GPU-driven mesh shader rendering
     pipeline, and precomputed bounds enable per-meshlet culling on the GPU.
   - **Verification:** Build meshlets from a test mesh and confirm no meshlet exceeds 64 vertices or
     124 triangles; confirm each meshlet has a valid AABB and normal cone.

## Vertex Cache Optimization

4. **R-12.2.4** — The engine **SHALL** reorder triangles and vertices within each meshlet to
   maximize post-transform vertex cache hit rates and minimize overdraw.
   - **Rationale:** Cache-optimized vertex ordering reduces GPU vertex processing cost, and overdraw
     optimization reduces fragment shading waste.
   - **Verification:** Measure ACMR before and after optimization and confirm improvement; measure
     overdraw ratio and confirm reduction.

## Lightmap UV Generation

5. **R-12.2.5** — The engine **SHALL** automatically generate non-overlapping lightmap UV atlases
   for static geometry with minimal chart stretching, uniform texel density, and atlas packing
   grouped by lightmap page.
   - **Rationale:** Automated lightmap UV unwrapping eliminates manual artist work and ensures
     consistent texel density for baked global illumination.
   - **Verification:** Generate lightmap UVs for a set of static meshes and confirm no chart
     overlap, stretching below threshold, and texel density variance within tolerance.

## Audio Encoding

6. **R-12.2.6** — The engine **SHALL** encode imported audio into runtime formats: Opus for voice
   and music, ADPCM for short sound effects, and raw PCM for latency-critical audio, with encoding
   parameters controlled by import presets.
   - **Rationale:** Multiple encoding formats balance compression ratio, decode latency, and
     fidelity across different audio use cases.
   - **Verification:** Encode test audio with each preset and confirm correct output format; decode
     each output and confirm quality meets the preset SNR threshold.

## Shader Graph Compilation

7. **R-12.2.7** — The engine **SHALL** compile visual shader graph assets into clean, human-readable
   GLSL source files where each graph node emits an GLSL function. The compiler **SHALL** resolve
   input connections, generate typed local variables, and compose node functions into complete
   shader entry points. Generated .glsl files **SHALL** contain no template markers or non-standard
   extensions and **SHALL** be fully compatible with GLSL Tools (IntelliSense, error squiggles,
   go-to-definition). Only reachable shader variants identified through static analysis **SHALL** be
   generated.
   - **Rationale:** Human-readable GLSL output enables debugging, manual inspection, and IDE tooling
     without template remnants.
   - **Verification:** Generate GLSL from a shader graph with 20+ nodes; open in VS with GLSL Tools;
     verify syntax highlighting and IntelliSense work; verify no template markers appear; verify
     comments trace each section to its graph node.

## Dependency Tracking

8. **R-12.2.8** — The engine **SHALL** track directed acyclic dependency graphs between all assets,
   drive incremental rebuilds from the dependency graph, detect circular references, and enable
   impact analysis when a shared asset changes.
   - **Rationale:** Dependency tracking ensures correct build ordering and enables incremental
     rebuilds critical for large-scale asset libraries.
   - **Verification:** Modify a shared texture and confirm only dependent materials and entity
     templates are rebuilt; introduce a circular reference and confirm it is detected and reported
     as an error.

## Shader Bytecode Compilation

9. **R-12.2.9** — The engine **SHALL** compile generated GLSL source files into platform-native
   bytecode using glslc for GLSL-to-SPIR-V and GLSL-to-SPIR-V, and glslc for SPIR-V-to-SPIR-V. glslc
   **SHALL** perform validation, optimization (dead code elimination, constant folding), and
   reflection (binding layouts, push constant ranges, workgroup sizes). Compiled bytecode **SHALL**
   be cached by GLSL source hash in the shared build cache (F-15.11.2). Compilation errors **SHALL**
   report the original GLSL line number and the graph node that generated it.
   - **Rationale:** glslc are industry-standard compilers producing
     optimized output with full reflection data for all target APIs.
   - **Verification:** Compile a shader with dead code and constants; confirm dead code is removed
     and constants are folded; confirm reflected binding layouts match the source; validate each
     output on its target API.
