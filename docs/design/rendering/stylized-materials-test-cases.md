# Stylized Rendering and Advanced Materials Test Cases

Companion test cases for [stylized-materials.md](stylized-materials.md).

## Unit Tests

### TC-2.12.9.1 Graph Add Remove Nodes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add 100 nodes, remove 50 | Graph integrity preserved | R-2.12.9 |

### TC-2.12.9.2 Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Material graph with cycle | `CycleDetected` error | R-2.12.9 |

### TC-2.12.9.3 Graph Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Float output connected to Texture2D input | `TypeMismatch` error | R-2.12.9 |

### TC-2.12.9.4 Graph Dead Node Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Nodes not connected to any output | Excluded from compiled HLSL | R-2.12.9 |

### TC-2.12.9.5 Graph Constant Folding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two constants through Add node | Single constant in compiled HLSL | R-2.12.9 |

### TC-2.12.9.6 Tier Budget Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 65-node graph, Mobile tier | `TierBudgetExceeded` error | R-2.12.9 |

### TC-2.12.9.7 Tier Budget Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 129-node graph, Switch tier | `TierBudgetExceeded` error | R-2.12.9 |

### TC-2.12.9.8 Tier Budget Desktop Unlimited

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-node graph, Desktop tier | Compiles successfully | R-2.12.9 |

### TC-2.12.9.9 Permutation Key Hash Unique

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 random permutation keys | Zero hash collisions | R-2.12.9 |

### TC-2.12.9.10 Permutation Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile material, query same key | Cache hit, no recompilation | R-2.12.9 |

### TC-2.12.9.11 Permutation Cache Invalidate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile, invalidate, query same key | Recompilation occurs | R-2.12.9 |

### TC-2.11.3.1 Toon Band Quantization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | NdotL = [0.0, 0.25, 0.5, 0.75, 1.0], 4 bands | Output matches expected band thresholds | R-2.11.3 |

### TC-2.11.3.2 Toon Specular Shapes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Circular specular shape | Circular mask geometry | R-2.11.3 |
| 2 | Star specular shape | Star mask geometry | R-2.11.3 |
| 3 | Cross specular shape | Cross mask geometry | R-2.11.3 |

### TC-2.11.1.1 Outline Sobel Edge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Depth+normal buffers with known edge | Sobel detects edge | R-2.11.1 |

### TC-2.11.1.2 Outline JFA Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Seeded silhouette, JFA passes | Per-pixel distance within tolerance of ground truth | R-2.11.1 |

### TC-2.11.1.3 Outline Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two outlines, different priorities | Higher priority wins | R-2.11.1 |

### TC-2.11.1.4 Outline Multi-Layer Compose

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Selection + team + quest outlines | Additive composition | R-2.11.1 |

### TC-2.11.2.1 Highlight Pulse Sinusoidal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | pulse speed=2.0, amplitude=0.5, t=PI/4 | Intensity = sin(2.0 * PI/4) * 0.5 | R-2.11.2 |

### TC-2.11.2.2 Highlight Mode Inner Glow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | InnerGlow mode, opacity=0.7 | Flat color fill at 0.7 opacity | R-2.11.2 |

### TC-2.12.1.1 Glass IOR Refraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Glass surface, IOR=1.5, normal incidence | UV distortion matches Snell's law | R-2.12.1 |

### TC-2.12.1.2 Glass Thin vs Thick

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thin-surface mode | Single refraction event | R-2.12.1 |
| 2 | Thick-surface mode | Double refraction (entry + exit) | R-2.12.1 |

### TC-2.12.1.3 Glass Fresnel Grazing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grazing angle view | Reflection intensity matches Schlick approximation | R-2.12.1 |

### TC-2.12.5.1 Fabric Sheen BRDF

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Charlie/Ashikhmin sheen BRDF evaluation | Energy conserved (output <= input) | R-2.12.5 |

### TC-2.12.6.1 Metal Anisotropy Direction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Direction map on brushed metal | Specular stretches along tangent | R-2.12.6 |

### TC-2.12.8.1 Clearcoat Dual Specular

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Clearcoated material render | Two distinct specular highlights | R-2.12.8 |

### TC-2.12.8.2 Layer Blend Height Based

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rust over metal via height blend | Smooth transition at height boundary | R-2.12.8 |

### TC-2.12.8.3 Layer Count Limit Per Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 layers, Mobile tier | Compile error for excess layers | R-2.12.8 |

### TC-2.12.4.1 POM Self Shadow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | POM self-shadow enabled, heightmap with ridges | Shadow in occluded regions | R-2.12.4 |

### TC-2.12.3.1 Emissive Bloom Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HDR emission above bloom threshold | Bloom pass activated | R-2.12.3 |

### TC-2.12.3.2 Emissive Scroll UV

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ScrollUv animation, 60 frames | UV offset increases with time | R-2.12.3 |

### TC-2.12.6.2 Weathering Age Progression

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Age 0 to 1 in steps | Procedural overlay intensity increases monotonically | R-2.12.6 |

### TC-2.12.7.1 Wax Thickness Transmission

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thin wax, backlit | Transmission visible | R-2.12.7 |
| 2 | Thick wax, backlit | Opaque (no transmission) | R-2.12.7 |

### TC-2.11.4.1 Cut-Through Volume Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity below occluder in volume mode | Fade begins within 1 frame | R-2.11.4 |

### TC-2.11.4.2 Cut-Through Ray Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Occluder between camera and target | Dissolve in ray mode | R-2.11.4 |

### TC-2.11.5.1 X-Ray Depth Comparison

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | XRayVisible entity behind wall | Silhouette rendered | R-2.11.5 |

### TC-2.11.5.2 X-Ray Priority Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-priority entity, filter on | Silhouette shows | R-2.11.5 |
| 2 | Low-priority entity, filter toggled off | Silhouette hidden | R-2.11.5 |

### TC-2.12.9.12 Subgraph Composition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two sub-graphs composed into parent | Correct HLSL emission | R-2.12.9 |

## Integration Tests

### TC-2.11.3.I1 Full Toon Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lit scene with toon material, outlines, highlights | Visual output matches reference | R-2.11.3 |

### TC-2.11.3.I2 Toon and PBR Mixed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with PBR and toon entities | Each shaded correctly by ShadingModelId | R-2.11.3 |

### TC-2.12.1.I1 Glass Screen-Space Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RT disabled, glass material | Screen-space refraction fallback, no artifacts | R-2.12.1 |

### TC-2.12.1.I2 Glass RT Refraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RT enabled, crystal gemstone | Internal refraction paths visible | R-2.12.1 |

### TC-2.12.5.I1 Fabric Cloth Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fabric material on cloth-simulated mesh | Shading updates with deformation | R-2.12.5 |

### TC-2.12.8.I1 Multi-Layer Weathered Pipe

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-layer material: metal + rust + wet clearcoat | Correct blending, runtime parameter drive | R-2.12.8 |

### TC-2.12.8.I2 Clearcoat Rain Wetting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wetness driven over time | Clearcoat roughness and reflectivity change | R-2.12.8 |

### TC-2.12.4.I1 Parallax Tessellation vs POM

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same heightmap, tessellation vs POM | Both produce visible depth | R-2.12.4 |

### TC-2.12.3.I1 Emissive GI Contribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RT GI enabled, emissive surface near wall | Indirect colored light on wall | R-2.12.3 |

### TC-2.11.5.I1 X-Ray Through Wall

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall with XRayVisible ally behind it | Colored silhouette visible through wall | R-2.11.5 |

### TC-2.11.4.I1 Cut-Through Isometric

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Isometric camera, roofed building, player inside | Roof fades | R-2.11.4 |

### TC-2.11.1.I1 Outline JFA Fallback Sobel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compute capability disabled | JFA outlines fall back to Sobel | R-2.11.1 |

### TC-2.12.9.I1 Material Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify material graph at runtime | Live preview updates within 1 second | R-2.12.9 |

### TC-2.12.9.I2 Compile All Backends

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100-node graph compiled to D3D12, Vulkan, Metal | All succeed under 5 seconds | R-2.12.9 |

### TC-2.12.9.I3 Permutation Warm Parallel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 permutations compiled in parallel | All compile without errors | R-2.12.9 |

### TC-2.12.7.I1 Deformation Roughness Rubber

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stretch rubber entity via physics | Roughness increases in real time | R-2.12.7 |

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
