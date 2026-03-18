# Stylized Rendering and Advanced Materials Test Cases

Companion test cases for [stylized-materials.md](stylized-materials.md).

## Unit Tests

### TC-2.12.9.1 Graph Add Remove Nodes

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Add 100 nodes, remove 50
   - **Expected:** Graph integrity preserved

### TC-2.12.9.2 Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Material graph with cycle
   - **Expected:** `CycleDetected` error

### TC-2.12.9.3 Graph Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Float output connected to Texture2D input
   - **Expected:** `TypeMismatch` error

### TC-2.12.9.4 Graph Dead Node Elimination

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Nodes not connected to any output
   - **Expected:** Excluded from compiled HLSL

### TC-2.12.9.5 Graph Constant Folding

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Two constants through Add node
   - **Expected:** Single constant in compiled HLSL

### TC-2.12.9.6 Tier Budget Mobile

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 65-node graph, Mobile tier
   - **Expected:** `TierBudgetExceeded` error

### TC-2.12.9.7 Tier Budget Switch

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 129-node graph, Switch tier
   - **Expected:** `TierBudgetExceeded` error

### TC-2.12.9.8 Tier Budget Desktop Unlimited

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 1000-node graph, Desktop tier
   - **Expected:** Compiles successfully

### TC-2.12.9.9 Permutation Key Hash Unique

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 10,000 random permutation keys
   - **Expected:** Zero hash collisions

### TC-2.12.9.10 Permutation Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Compile material, query same key
   - **Expected:** Cache hit, no recompilation

### TC-2.12.9.11 Permutation Cache Invalidate

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Compile, invalidate, query same key
   - **Expected:** Recompilation occurs

### TC-2.11.3.1 Toon Band Quantization

| # | Requirement |
|---|-------------|
| 1 | R-2.11.3    |

1. **#1** — NdotL = [0.0, 0.25, 0.5, 0.75, 1.0], 4 bands
   - **Expected:** Output matches expected band thresholds

### TC-2.11.3.2 Toon Specular Shapes

| # | Requirement |
|---|-------------|
| 1 | R-2.11.3    |
| 2 | R-2.11.3    |
| 3 | R-2.11.3    |

1. **#1** — Circular specular shape
   - **Expected:** Circular mask geometry
2. **#2** — Star specular shape
   - **Expected:** Star mask geometry
3. **#3** — Cross specular shape
   - **Expected:** Cross mask geometry

### TC-2.11.1.1 Outline Sobel Edge

| # | Requirement |
|---|-------------|
| 1 | R-2.11.1    |

1. **#1** — Depth+normal buffers with known edge
   - **Expected:** Sobel detects edge

### TC-2.11.1.2 Outline JFA Distance

| # | Requirement |
|---|-------------|
| 1 | R-2.11.1    |

1. **#1** — Seeded silhouette, JFA passes
   - **Expected:** Per-pixel distance within tolerance of ground truth

### TC-2.11.1.3 Outline Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-2.11.1    |

1. **#1** — Two outlines, different priorities
   - **Expected:** Higher priority wins

### TC-2.11.1.4 Outline Multi-Layer Compose

| # | Requirement |
|---|-------------|
| 1 | R-2.11.1    |

1. **#1** — Selection + team + quest outlines
   - **Expected:** Additive composition

### TC-2.11.2.1 Highlight Pulse Sinusoidal

| # | Requirement |
|---|-------------|
| 1 | R-2.11.2    |

1. **#1** — pulse speed=2.0, amplitude=0.5, t=PI/4
   - **Expected:** Intensity = sin(2.0 * PI/4) * 0.5

### TC-2.11.2.2 Highlight Mode Inner Glow

| # | Requirement |
|---|-------------|
| 1 | R-2.11.2    |

1. **#1** — InnerGlow mode, opacity=0.7
   - **Expected:** Flat color fill at 0.7 opacity

### TC-2.12.1.1 Glass IOR Refraction

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |

1. **#1** — Glass surface, IOR=1.5, normal incidence
   - **Expected:** UV distortion matches Snell's law

### TC-2.12.1.2 Glass Thin vs Thick

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |
| 2 | R-2.12.1    |

1. **#1** — Thin-surface mode
   - **Expected:** Single refraction event
2. **#2** — Thick-surface mode
   - **Expected:** Double refraction (entry + exit)

### TC-2.12.1.3 Glass Fresnel Grazing

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |

1. **#1** — Grazing angle view
   - **Expected:** Reflection intensity matches Schlick approximation

### TC-2.12.5.1 Fabric Sheen BRDF

| # | Requirement |
|---|-------------|
| 1 | R-2.12.5    |

1. **#1** — Charlie/Ashikhmin sheen BRDF evaluation
   - **Expected:** Energy conserved (output <= input)

### TC-2.12.6.1 Metal Anisotropy Direction

| # | Requirement |
|---|-------------|
| 1 | R-2.12.6    |

1. **#1** — Direction map on brushed metal
   - **Expected:** Specular stretches along tangent

### TC-2.12.8.1 Clearcoat Dual Specular

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |

1. **#1** — Clearcoated material render
   - **Expected:** Two distinct specular highlights

### TC-2.12.8.2 Layer Blend Height Based

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |

1. **#1** — Rust over metal via height blend
   - **Expected:** Smooth transition at height boundary

### TC-2.12.8.3 Layer Count Limit Per Tier

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |

1. **#1** — 5 layers, Mobile tier
   - **Expected:** Compile error for excess layers

### TC-2.12.4.1 POM Self Shadow

| # | Requirement |
|---|-------------|
| 1 | R-2.12.4    |

1. **#1** — POM self-shadow enabled, heightmap with ridges
   - **Expected:** Shadow in occluded regions

### TC-2.12.3.1 Emissive Bloom Trigger

| # | Requirement |
|---|-------------|
| 1 | R-2.12.3    |

1. **#1** — HDR emission above bloom threshold
   - **Expected:** Bloom pass activated

### TC-2.12.3.2 Emissive Scroll UV

| # | Requirement |
|---|-------------|
| 1 | R-2.12.3    |

1. **#1** — ScrollUv animation, 60 frames
   - **Expected:** UV offset increases with time

### TC-2.12.6.2 Weathering Age Progression

| # | Requirement |
|---|-------------|
| 1 | R-2.12.6    |

1. **#1** — Age 0 to 1 in steps
   - **Expected:** Procedural overlay intensity increases monotonically

### TC-2.12.7.1 Wax Thickness Transmission

| # | Requirement |
|---|-------------|
| 1 | R-2.12.7    |
| 2 | R-2.12.7    |

1. **#1** — Thin wax, backlit
   - **Expected:** Transmission visible
2. **#2** — Thick wax, backlit
   - **Expected:** Opaque (no transmission)

### TC-2.11.4.1 Cut-Through Volume Mode

| # | Requirement |
|---|-------------|
| 1 | R-2.11.4    |

1. **#1** — Entity below occluder in volume mode
   - **Expected:** Fade begins within 1 frame

### TC-2.11.4.2 Cut-Through Ray Mode

| # | Requirement |
|---|-------------|
| 1 | R-2.11.4    |

1. **#1** — Occluder between camera and target
   - **Expected:** Dissolve in ray mode

### TC-2.11.5.1 X-Ray Depth Comparison

| # | Requirement |
|---|-------------|
| 1 | R-2.11.5    |

1. **#1** — XRayVisible entity behind wall
   - **Expected:** Silhouette rendered

### TC-2.11.5.2 X-Ray Priority Filter

| # | Requirement |
|---|-------------|
| 1 | R-2.11.5    |
| 2 | R-2.11.5    |

1. **#1** — High-priority entity, filter on
   - **Expected:** Silhouette shows
2. **#2** — Low-priority entity, filter toggled off
   - **Expected:** Silhouette hidden

### TC-2.12.9.12 Subgraph Composition

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Two sub-graphs composed into parent
   - **Expected:** Correct HLSL emission

## Integration Tests

### TC-2.11.3.I1 Full Toon Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-2.11.3    |

1. **#1** — Lit scene with toon material, outlines, highlights
   - **Expected:** Visual output matches reference

### TC-2.11.3.I2 Toon and PBR Mixed

| # | Requirement |
|---|-------------|
| 1 | R-2.11.3    |

1. **#1** — Scene with PBR and toon entities
   - **Expected:** Each shaded correctly by ShadingModelId

### TC-2.12.1.I1 Glass Screen-Space Fallback

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |

1. **#1** — RT disabled, glass material
   - **Expected:** Screen-space refraction fallback, no artifacts

### TC-2.12.1.I2 Glass RT Refraction

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |

1. **#1** — RT enabled, crystal gemstone
   - **Expected:** Internal refraction paths visible

### TC-2.12.5.I1 Fabric Cloth Integration

| # | Requirement |
|---|-------------|
| 1 | R-2.12.5    |

1. **#1** — Fabric material on cloth-simulated mesh
   - **Expected:** Shading updates with deformation

### TC-2.12.8.I1 Multi-Layer Weathered Pipe

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |

1. **#1** — 3-layer material: metal + rust + wet clearcoat
   - **Expected:** Correct blending, runtime parameter drive

### TC-2.12.8.I2 Clearcoat Rain Wetting

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |

1. **#1** — Wetness driven over time
   - **Expected:** Clearcoat roughness and reflectivity change

### TC-2.12.4.I1 Parallax Tessellation vs POM

| # | Requirement |
|---|-------------|
| 1 | R-2.12.4    |

1. **#1** — Same heightmap, tessellation vs POM
   - **Expected:** Both produce visible depth

### TC-2.12.3.I1 Emissive GI Contribution

| # | Requirement |
|---|-------------|
| 1 | R-2.12.3    |

1. **#1** — RT GI enabled, emissive surface near wall
   - **Expected:** Indirect colored light on wall

### TC-2.11.5.I1 X-Ray Through Wall

| # | Requirement |
|---|-------------|
| 1 | R-2.11.5    |

1. **#1** — Wall with XRayVisible ally behind it
   - **Expected:** Colored silhouette visible through wall

### TC-2.11.4.I1 Cut-Through Isometric

| # | Requirement |
|---|-------------|
| 1 | R-2.11.4    |

1. **#1** — Isometric camera, roofed building, player inside
   - **Expected:** Roof fades

### TC-2.11.1.I1 Outline JFA Fallback Sobel

| # | Requirement |
|---|-------------|
| 1 | R-2.11.1    |

1. **#1** — Compute capability disabled
   - **Expected:** JFA outlines fall back to Sobel

### TC-2.12.9.I1 Material Hot Reload

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — Modify material graph at runtime
   - **Expected:** Live preview updates within 1 second

### TC-2.12.9.I2 Compile All Backends

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 100-node graph compiled to D3D12, Vulkan, Metal
   - **Expected:** All succeed under 5 seconds

### TC-2.12.9.I3 Permutation Warm Parallel

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |

1. **#1** — 50 permutations compiled in parallel
   - **Expected:** All compile without errors

### TC-2.12.7.I1 Deformation Roughness Rubber

| # | Requirement |
|---|-------------|
| 1 | R-2.12.7    |

1. **#1** — Stretch rubber entity via physics
   - **Expected:** Roughness increases in real time

## Benchmarks

### TC-2.11.1.B1 Outline GPU Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 entities, 1080p | GPU time | < 1.0 ms | NFR-2.11.1 |

### TC-2.11.1.B2 JFA Outline

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 entities, 1080p | GPU time | < 0.5 ms | NFR-2.11.1 |

### TC-2.11.4.B1 Cut-Through Fade Response

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fade trigger | Response time | < 1 frame | NFR-2.11.2 |

### TC-2.12.9.B1 Material Graph Compile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 nodes, all backends | Compile time | < 5 s | NFR-2.12.1 |

### TC-2.12.9.B2 Incremental Recompile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1 node change | Compile time | < 1 s | NFR-2.12.1 |

### TC-2.12.1.B1 Refraction PSNR Screen-Space

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Screen-space vs RT reference | PSNR | >= 5 dB | NFR-2.12.2 |

### TC-2.12.1.B2 Refraction PSNR RT

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | RT vs ground truth reference | PSNR | >= 1 dB | NFR-2.12.2 |

### TC-2.12.8.B1 4-Layer Material Cost

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4-layer vs single PBR | Cost increase | < 50% | NFR-2.12.3 |

### TC-2.12.9.B3 Permutation Cache Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cache lookup | Latency | < 1 us | R-2.12.9 |

### TC-2.11.3.B1 Toon Lighting Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.3 ms | R-2.11.3 |
