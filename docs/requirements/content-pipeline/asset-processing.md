# R-12.2 Asset Processing

## R-12.2.1 Texture Compression (BC, ASTC, ETC)
The engine **SHALL** compress imported textures offline into GPU-native block-compressed formats
(BC7/BC6H for desktop and console, ASTC for mobile and Apple Silicon, ETC2 as mobile fallback) with
quality levels and format selection driven by import presets and per-platform override tables.
- **Derived from:** [F-12.2.1](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Platform-appropriate block compression minimizes GPU memory and bandwidth while
  maintaining visual quality across all target hardware.
- **Verification:** Compress a test texture for each target platform and confirm the output format
  matches the platform override table; compare PSNR against the quality threshold for each preset.

## R-12.2.2 LOD Generation
The engine **SHALL** generate automatic discrete LOD chains using edge-collapse mesh simplification,
with each LOD level targeting a configurable triangle-count ratio and error threshold to preserve
silhouette fidelity, requiring no manual artist intervention.
- **Derived from:** [F-12.2.2](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Fully automated LOD generation is required for worlds containing millions of unique
  meshes where manual LOD authoring is infeasible.
- **Verification:** Generate an LOD chain for a reference mesh and confirm each level meets its
  triangle-count ratio within the error threshold; confirm silhouette deviation stays below the
  configured limit.

## R-12.2.3 Meshlet Building
The engine **SHALL** partition each LOD mesh into fixed-size meshlets (max 64 vertices, 124
triangles) and precompute per-meshlet bounds (AABB and normal cone) for GPU-driven culling.
- **Derived from:** [F-12.2.3](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Meshlet partitioning is a prerequisite for the GPU-driven mesh shader rendering
  pipeline, and precomputed bounds enable efficient per-meshlet culling on the GPU.
- **Verification:** Build meshlets from a test mesh and confirm no meshlet exceeds 64 vertices or
  124 triangles; confirm each meshlet has a valid AABB and normal cone; render with mesh shaders and
  confirm correct visual output.

## R-12.2.4 Vertex Cache and Overdraw Optimization
The engine **SHALL** reorder triangles and vertices within each meshlet to maximize post-transform
vertex cache hit rates and minimize overdraw.
- **Derived from:** [F-12.2.4](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Cache-optimized vertex ordering reduces GPU vertex processing cost, and overdraw
  optimization reduces fragment shading waste.
- **Verification:** Measure ACMR (average cache miss ratio) before and after optimization and
  confirm improvement; measure overdraw ratio and confirm reduction compared to unoptimized order.

## R-12.2.5 Lightmap UV Generation
The engine **SHALL** automatically generate non-overlapping lightmap UV atlases for static geometry
with minimal chart stretching, uniform texel density, and atlas packing grouped by lightmap page.
- **Derived from:** [F-12.2.5](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Automated lightmap UV unwrapping eliminates manual artist work and ensures
  consistent texel density for baked global illumination.
- **Verification:** Generate lightmap UVs for a set of static meshes and confirm no chart overlap,
  stretching below threshold, and texel density variance within tolerance.

## R-12.2.6 Audio Encoding
The engine **SHALL** encode imported audio into runtime formats — Opus for voice and music, ADPCM
for short sound effects, and raw PCM for latency-critical audio — with encoding parameters
controlled by import presets.
- **Derived from:** [F-12.2.6](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Multiple encoding formats balance compression ratio, decode latency, and audio
  fidelity across different use cases (music, SFX, voice).
- **Verification:** Encode test audio with each preset and confirm correct output format; decode
  each output and confirm audio quality meets the preset's SNR threshold.

## R-12.2.7 Shader Graph to HLSL Code Generation
The engine **SHALL** compile visual shader graph assets into clean, human-readable HLSL source
files. Each graph node **SHALL** emit an HLSL function; the compiler **SHALL** resolve input
connections, generate typed local variables, and compose node functions into complete shader entry
points. Custom nodes **SHALL** be user-authored HLSL snippets with function signatures called with
resolved inputs. Generated .hlsl files **SHALL** contain no template markers, preprocessor hacks, or
non-standard extensions — they **SHALL** be fully compatible with HLSL syntax highlighting and HLSL
Tools features (IntelliSense, error squiggles, go-to-definition) in any IDE. Comments **SHALL**
annotate which graph node produced each code section. Only reachable shader variants identified
through static analysis **SHALL** be generated.
- **Derived from:** [F-12.2.7](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Human-readable HLSL output enables debugging, manual inspection, and IDE tooling.
  Pure HLSL with no template remnants ensures compatibility with standard HLSL tools.
- **Verification:** Generate HLSL from a shader graph with 20+ nodes; open in Visual Studio with
  HLSL Tools; verify syntax highlighting, IntelliSense, and error squiggles work correctly. Verify
  no template markers ({{, %%, <%>) appear in the output. Verify comments trace each section to its
  graph node.

## R-12.2.8 Asset Dependency Graphs
The engine **SHALL** track directed acyclic dependency graphs between all assets, drive incremental
rebuilds from the dependency graph, detect circular references, and enable impact analysis when a
shared asset changes.
- **Derived from:** [F-12.2.8](../../features/content-pipeline/asset-processing.md)
- **Rationale:** Dependency tracking ensures correct build ordering and enables incremental
  rebuilds, which are critical for large-scale asset libraries.
- **Verification:** Modify a shared texture and confirm only dependent materials and prefabs are
  rebuilt; introduce a circular reference and confirm it is detected and reported as an error.

## R-12.2.9 DXC and Metal Shader Converter Pipeline
The engine **SHALL** compile generated HLSL source files (R-12.2.7) into platform-native bytecode
using DXC for HLSL-to-DXIL and HLSL-to-SPIR-V, and Metal Shader Converter for DXIL-to-MSL. DXC
**SHALL** perform validation, optimization (dead code elimination, constant folding), and reflection
(binding layouts, push constant ranges, workgroup sizes). Both DXC and Metal Shader Converter are
C++ libraries accessed via cxx.rs FFI. Compiled bytecode **SHALL** be cached by HLSL source hash in
the shared build cache (F-15.11.2). Compilation errors **SHALL** report the original HLSL line
number AND the graph node that generated it, enabling click-to-navigate from compiler errors to the
visual graph node in the editor.
- **Derived from:** [F-12.2.9](../../features/content-pipeline/asset-processing.md)
- **Rationale:** DXC and Metal Shader Converter are industry-standard, vendor-supported shader
  compilers that produce optimized output with full reflection data for all target APIs.
- **Verification:** Compile a shader with dead code and constants, confirm dead code is removed and
  constants are folded; confirm reflected binding layouts match the source HLSL; validate each
  output format on its target API.
