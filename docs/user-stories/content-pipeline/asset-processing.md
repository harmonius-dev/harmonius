# User Stories -- 12.2 Asset Processing

## Texture Compression

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.2.1 | technical artist (P-13)    |
| US-12.2.2 | build engineer (P-16)      |

1. **US-12.2.1** — **As a** technical artist (P-13), **I want** imported textures compressed into
   platform-native formats (BC7/BC6H, ASTC, ETC2) driven by import presets, **so that** each target
   platform gets optimal GPU texture formats.
2. **US-12.2.2** — **As a** build engineer (P-16), **I want** per-platform override tables for
   texture compression format selection, **so that** CI builds produce correct formats for every
   shipping target without manual intervention.

## LOD Generation

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.2.3 | environment artist (P-8)   |
| US-12.2.4 | technical artist (P-13)    |

1. **US-12.2.3** — **As an** environment artist (P-8), **I want** automatic LOD chain generation
   from my imported meshes with no manual authoring, **so that** every mesh has proper distance
   fallbacks for open-world scenes.
2. **US-12.2.4** — **As a** technical artist (P-13), **I want** configurable triangle-count ratios
   and error thresholds per LOD level, **so that** I can control the quality-performance trade-off
   for different asset categories.

## Meshlet Building

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.2.5 | engine developer (P-26)    |
| US-12.2.6 | technical artist (P-13)    |

1. **US-12.2.5** — **As an** engine developer (P-26), **I want** each LOD mesh partitioned into
   fixed-size meshlets with precomputed AABB and normal cone bounds, **so that** the GPU-driven
   rendering pipeline can cull per meshlet.
2. **US-12.2.6** — **As a** technical artist (P-13), **I want** vertex and triangle ordering within
   meshlets optimized for cache hit rates and minimal overdraw, **so that** GPU vertex processing
   cost is minimized.

## Lightmap UV Generation

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.2.7 | environment artist (P-8)   |

1. **US-12.2.7** — **As an** environment artist (P-8), **I want** automatic non-overlapping lightmap
   UV atlases generated for static geometry, **so that** baked global illumination has consistent
   texel density without manual UV work.

## Audio Encoding

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.2.8 | audio designer (P-14)      |
| US-12.2.9 | technical artist (P-13)    |

1. **US-12.2.8** — **As an** audio designer (P-14), **I want** imported audio encoded into Opus,
   ADPCM, or PCM based on import presets, **so that** each audio category uses the right codec for
   its latency and compression needs.
2. **US-12.2.9** — **As a** technical artist (P-13), **I want** encoding parameters configurable per
   import preset, **so that** I can tune compression quality for voice, music, and SFX
   independently.

## Shader Graph Compilation

| ID         | Persona                    |
|------------|----------------------------|
| US-12.2.10 | technical artist (P-13)    |
| US-12.2.11 | engine developer (P-26)    |
| US-12.2.12 | technical artist (P-13)    |

1. **US-12.2.10** — **As a** technical artist (P-13), **I want** shader graphs compiled into clean,
   human-readable HLSL with comments tracing each section to its graph node, **so that** I can
   inspect and debug generated shaders in any IDE.
2. **US-12.2.11** — **As an** engine developer (P-26), **I want** only reachable shader variants
   compiled via static analysis, **so that** permutation explosion does not waste build time on dead
   code paths.
3. **US-12.2.12** — **As a** technical artist (P-13), **I want** generated HLSL fully compatible
   with HLSL Tools features like IntelliSense and go-to-definition, **so that** I can use standard
   IDE tooling on shader output.

## Dependency Tracking

| ID         | Persona                    |
|------------|----------------------------|
| US-12.2.13 | build engineer (P-16)      |
| US-12.2.14 | technical artist (P-13)    |

1. **US-12.2.13** — **As a** build engineer (P-16), **I want** directed acyclic dependency graphs
   tracked for all assets, **so that** incremental rebuilds process only changed assets and their
   dependents.
2. **US-12.2.14** — **As a** technical artist (P-13), **I want** circular dependency references
   detected and reported as errors, **so that** broken dependency chains are caught before they
   cause build failures.

## Shader Bytecode Compilation

| ID         | Persona                    |
|------------|----------------------------|
| US-12.2.15 | engine developer (P-26)    |
| US-12.2.16 | technical artist (P-13)    |
| US-12.2.17 | build engineer (P-16)      |

1. **US-12.2.15** — **As an** engine developer (P-26), **I want** generated HLSL compiled to DXIL,
   SPIR-V, and MSL via DXC and Metal Shader Converter with full reflection data, **so that** binding
   layouts and push constant ranges are extracted automatically.
2. **US-12.2.16** — **As a** technical artist (P-13), **I want** shader compilation errors to report
   both the HLSL line number and the originating graph node, **so that** I can click through from an
   error to the visual node that caused it.
3. **US-12.2.17** — **As a** build engineer (P-16), **I want** compiled shader bytecode cached by
   HLSL source hash in the shared build cache, **so that** unchanged shaders are not recompiled
   across CI runs.

## Collision Shape Generation

| ID         | Persona                    |
|------------|----------------------------|
| US-12.2.18 | game designer (P-5)        |
| US-12.2.19 | technical artist (P-13)    |
| US-12.2.20 | environment artist (P-8)   |

1. **US-12.2.18** — **As a** game designer (P-5), **I want** collision shapes generated
   automatically from mesh sources at import time, **so that** I do not have to author physics
   colliders for every prop by hand.
2. **US-12.2.19** — **As a** technical artist (P-13), **I want** to choose between V-HACD convex
   decomposition, quickhull, and analytic primitives (AABB, sphere, capsule) per mesh in the import
   settings, **so that** each asset gets the right fidelity-performance tradeoff.
3. **US-12.2.20** — **As an** environment artist (P-8), **I want** generated collision shapes stored
   alongside the render mesh in the processed asset, **so that** runtime physics bindings work
   without extra wiring.
