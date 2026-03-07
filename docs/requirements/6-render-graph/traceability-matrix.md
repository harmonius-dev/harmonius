# Traceability Matrix

## Forward Mapping: High-Level → Render Graph

| Source Req | Title | RG Requirements |
|---|---|---|
| R-2.1.1 | Add Entities to Scene | No graph impact — CPU-side only |
| R-2.1.2 | Configure Perspective Camera | No graph impact — CPU-side only |
| R-2.1.3 | Configure Orthographic Camera | No graph impact — CPU-side only |
| R-2.1.4 | Set Entity Transforms | RG-2.3, RG-3.4, RG-4.1, RG-4.3, RG-4.4, RG-10.5, RG-11.1, RG-11.7, RG-14.1, RG-14.2 |
| R-2.1.5 | Enable Multi-Camera Rendering | RG-2.11, RG-5.2, RG-9.1, RG-9.2, RG-9.3, RG-10.1, RG-10.3, RG-10.7, RG-14.3 |
| R-2.1.6 | Configure Render-to-Texture | RG-1.1, RG-2.1, RG-3.1, RG-3.3, RG-5.1, RG-8.2 |
| R-2.1.7 | Configure Cubemap Captures | RG-1.5, RG-1.6, RG-2.2, RG-2.6, RG-2.11, RG-3.2, RG-3.3, RG-5.2, RG-5.3, RG-9.1, RG-9.3, RG-9.4, RG-10.1, RG-10.3, RG-13.8, RG-14.3, RG-14.5, RG-14.6 |
| R-2.1.8 | Select Lighting Model | RG-1.1, RG-1.4, RG-2.1, RG-2.7, RG-8.1, RG-8.2, RG-13.1, RG-13.5, RG-13.7 |
| R-2.1.9 | Configure GPU Culling | RG-4.1, RG-6.1, RG-7.3, RG-12.2, RG-12.5, RG-13.3 |
| R-2.1.10 | Configure Scene Capture | RG-2.1, RG-3.1, RG-3.3, RG-5.1, RG-5.2, RG-9.1, RG-9.2, RG-9.3, RG-14.3 |
| R-2.1.11 | Configure Dynamic Resolution | RG-2.5, RG-7.1, RG-7.4, RG-7.6, RG-12.1, RG-12.5, RG-14.1, RG-14.4 |
| R-2.1.12 | Configure Post-Processing Stack | RG-1.1, RG-1.3, RG-1.6, RG-3.1, RG-5.1, RG-5.3, RG-7.3, RG-7.6, RG-8.1, RG-13.1, RG-13.2, RG-13.3, RG-13.5, RG-14.5 |
| R-2.2.1 | Create Point Lights | No graph impact — CPU-side only |
| R-2.2.2 | Create Spot Lights | No graph impact — CPU-side only |
| R-2.2.3 | Create Directional Lights | No graph impact — CPU-side only |
| R-2.2.4 | Create Area Lights | No graph impact — CPU-side only |
| R-2.2.5 | Configure Sky Light (IBL) | RG-1.5, RG-2.6, RG-3.1, RG-3.4, RG-4.1, RG-4.2, RG-4.4 |
| R-2.2.6 | Assign IES Light Profiles | No graph impact — CPU-side only |
| R-2.2.7 | Configure Light Functions | No graph impact — CPU-side only |
| R-2.2.8 | Configure Forward+ Tile Size | RG-5.2, RG-9.1 |
| R-2.2.9 | Observe Active Light Count | RG-12.2, RG-12.5 |
| R-2.2.10 | Configure DDGI Probe Grid | RG-2.6, RG-2.23 |
| R-2.2.11 | Toggle DDGI at Runtime | RG-1.6, RG-6.3 |
| R-2.2.12 | Configure RT Indirect Lighting | No graph impact — CPU-side only |
| R-2.2.13 | Configure God Rays | RG-9.1 |
| R-2.2.14 | Adjust Light Priority | No graph impact — CPU-side only |
| R-2.2.15 | Enable Stochastic Many-Light Sampling | RG-5.2, RG-9.1, RG-9.3 |
| R-2.3.1 | Enable Cascaded Shadow Maps | RG-1.5, RG-1.9, RG-2.6, RG-5.2, RG-9.1, RG-9.3, RG-9.4, RG-10.3 |
| R-2.3.2 | Configure Shadow Atlas | RG-1.9, RG-2.8, RG-2.17 |
| R-2.3.3 | Select Soft Shadow Quality | RG-1.4, RG-2.7, RG-6.1, RG-6.2, RG-6.3, RG-7.3 |
| R-2.3.4 | Configure Shadow Bias | RG-14.2 |
| R-2.3.5 | Select Ambient Occlusion Tier | RG-1.1, RG-1.4, RG-2.7, RG-6.1, RG-6.2, RG-6.3, RG-7.3 |
| R-2.3.6 | Configure AO Parameters | RG-14.2 |
| R-2.3.7 | Enable Per-Light Shadows | RG-1.6, RG-1.10, RG-14.5 |
| R-2.3.8 | Configure Occlusion Culling | RG-1.1, RG-3.1, RG-5.1, RG-12.2 |
| R-2.3.9 | Configure Quality Tier Fallbacks | RG-6.3, RG-6.6, RG-7.1 |
| R-2.3.10 | Enable Virtual Shadow Maps | RG-1.11, RG-2.9, RG-2.17 |
| R-2.3.11 | Enable Contact Shadows | RG-1.1, RG-1.10 |
| R-2.3.12 | Enable Distance Field Shadows | RG-1.1, RG-6.1 |
| R-2.3.13 | Enable Capsule Shadows | RG-1.1 |
| R-2.3.14 | Enable Order-Independent Transparency | RG-1.1, RG-1.3, RG-1.4, RG-2.1, RG-5.3 |
| R-2.3.15 | Enable Volumetric Shadow Maps | RG-1.1, RG-2.1, RG-3.1 |
| R-2.4.1 | Create PBR Materials | No graph impact — CPU-side only |
| R-2.4.2 | Configure Extended BSDF Layers | No graph impact — CPU-side only |
| R-2.4.3 | Configure SSS Profiles | RG-1.4, RG-6.3 |
| R-2.4.4 | Set Alpha Mode | RG-1.6, RG-13.5 |
| R-2.4.5 | Enable Alpha Mask Cutouts | RG-14.5 |
| R-2.4.6 | Configure Transparency Sorting | RG-5.3 |
| R-2.4.7 | Assign Materials via Shader Graphs | No graph impact — CPU-side only |
| R-2.4.8 | Override Material Parameters at Runtime | RG-14.2 |
| R-2.4.9 | Configure LOD Transition Style | No graph impact — CPU-side only |
| R-2.4.10 | Create Material Instances | No graph impact — CPU-side only |
| R-2.4.11 | Configure Material Layers | No graph impact — CPU-side only |
| R-2.4.12 | Place Decals | RG-1.1, RG-1.3, RG-3.1, RG-5.3 |
| R-2.4.13 | Select Shading Model Variant | RG-1.4, RG-6.1, RG-6.3 |
| R-2.5.1 | Query RT Hardware Availability | RG-6.1, RG-6.4 |
| R-2.5.2 | Enable RT Acceleration Structures | RG-1.1, RG-2.18, RG-3.1, RG-3.3, RG-3.4, RG-4.1 |
| R-2.5.3 | Configure RT Reflections | RG-1.1, RG-1.4, RG-3.1, RG-3.3 |
| R-2.5.4 | Select Reflection Fallback | RG-6.3 |
| R-2.5.5 | Configure RT Sample Counts | RG-5.1, RG-5.3 |
| R-2.5.6 | Configure Denoiser Strength | RG-1.3, RG-2.4 |
| R-2.5.7 | Mark Entities for RT Participation | No graph impact — CPU-side only |
| R-2.5.8 | Enable RT Global Illumination | RG-1.3, RG-1.6, RG-2.4, RG-2.6 |
| R-2.5.9 | Enable Path Tracing | RG-1.4, RG-2.4, RG-2.19, RG-13.5 |
| R-2.5.10 | Enable RT Subsurface Transmission | RG-1.1, RG-3.6 |
| R-2.5.11 | Enable Surfel-Based GI | RG-1.1, RG-2.2, RG-2.4, RG-3.1, RG-4.2 |
| R-2.5.12 | Enable ReSTIR Sampling | RG-1.1, RG-2.4, RG-3.1 |
| R-2.5.13 | Enable Real-Time Production Path Tracing | RG-1.1, RG-1.4, RG-2.4, RG-2.18, RG-2.19, RG-3.1 |
| R-2.5.14 | Enable Opacity Micromaps | RG-2.18, RG-2.25 |
| R-2.5.15 | Enable Shader Execution Reordering | No graph impact — GPU pipeline config only |
| R-2.5.16 | Configure Neural Denoising | RG-1.3, RG-1.4, RG-2.4 |
| R-2.5.17 | Enable RT Lens Flare | RG-1.1, RG-2.18, RG-3.1 |
| R-2.5.18 | Enable Voxel-Based GI | RG-1.1, RG-2.2, RG-3.1, RG-4.2 |
| R-2.5.19 | Enable Neural Radiance Cache | RG-1.1, RG-2.2, RG-2.4, RG-4.2 |
| R-2.5.20 | Enable Stochastic SSR | RG-1.1, RG-2.4, RG-3.1 |
| R-2.6.1 | Configure Procedural Sky | RG-1.1, RG-14.2 |
| R-2.6.2 | Configure Atmosphere LUT Resolution | RG-2.20 |
| R-2.6.3 | Enable Volumetric Fog | RG-1.1, RG-1.6, RG-6.1 |
| R-2.6.4 | Place Fog Volumes | No graph impact — CPU-side only |
| R-2.6.5 | Enable Procedural Clouds | RG-1.1, RG-1.6, RG-2.4, RG-2.19, RG-7.3, RG-14.5 |
| R-2.6.6 | Configure FFT Ocean | RG-1.3, RG-2.2, RG-4.2, RG-4.4, RG-5.3 |
| R-2.6.7 | Select Ocean Reflection Mode | RG-1.4, RG-6.3, RG-13.5 |
| R-2.6.8 | Configure God Ray Source | RG-1.4, RG-6.3 |
| R-2.6.9 | Set Distance Fog Parameters | RG-1.1 |
| R-2.6.10 | Animate Environment Parameters | RG-14.2 |
| R-2.6.11 | Import Heterogeneous Volumes | RG-1.1 |
| R-2.6.12 | Enable Voxel-Based Volumetric Clouds | RG-1.1, RG-1.6, RG-2.4, RG-2.5, RG-2.19 |
| R-2.6.13 | Configure Breaking Waves | RG-1.1, RG-1.3, RG-2.2 |
| R-2.6.14 | Configure Weather System | No graph impact — CPU-side only |
| R-2.7.1 | Add Mesh Entities | RG-1.1, RG-6.1, RG-6.2, RG-6.4 |
| R-2.7.2 | Configure Meshlet LOD | No graph impact — CPU-side only |
| R-2.7.3 | Instance Mesh Entities | RG-1.1, RG-2.13 |
| R-2.7.4 | Create GPU Splines | No graph impact — CPU-side only |
| R-2.7.5 | Submit Procedural Geometry Jobs | RG-2.2, RG-2.13, RG-2.14, RG-3.4, RG-4.1, RG-4.2, RG-4.3, RG-4.4, RG-10.4, RG-10.6, RG-13.4 |
| R-2.7.6 | Configure Mesh Shader Debug Viz | RG-1.6, RG-1.8, RG-7.3, RG-7.6, RG-12.6, RG-13.3, RG-14.5 |
| R-2.7.7 | Query Meshlet Statistics | RG-1.8, RG-12.1, RG-12.2, RG-12.7 |
| R-2.7.8 | Configure Instanced Draw Batching | RG-1.8, RG-12.2, RG-12.7 |
| R-2.7.9 | Configure Dynamic Tessellation | No graph impact — CPU-side only |
| R-2.7.10 | Enable Visibility Buffer | RG-1.1, RG-1.3, RG-2.1, RG-2.16, RG-3.1, RG-3.3, RG-3.6, RG-5.1, RG-6.1, RG-6.2, RG-6.4, RG-8.1, RG-8.2 |
| R-2.7.11 | Configure Variable Rate Shading | RG-1.1, RG-2.1, RG-2.12, RG-3.1, RG-3.3, RG-3.6, RG-5.1, RG-6.1, RG-6.2, RG-6.4, RG-8.1, RG-8.2, RG-13.4 |
| R-2.7.12 | Enable Meshlet Compression | No graph impact — GPU pipeline config only |
| R-2.7.13 | Enable GPU Work Graphs | RG-1.1, RG-1.13 |
| R-2.7.14 | Enable Concurrent Binary Tree Tessellation | RG-1.1, RG-2.2, RG-2.13 |
| R-2.7.15 | Configure LOD Generation | No graph impact — CPU-side only |
| R-2.7.16 | Configure LOD Blending | No graph impact — CPU-side only |
| R-2.7.17 | Configure Skinned Mesh LOD | No graph impact — CPU-side only |
| R-2.7.18 | Configure Static Mesh Merging | RG-2.2, RG-2.13 |
| R-2.8.1 | Configure Streaming World | RG-2.8, RG-2.9, RG-11.1, RG-11.5, RG-13.6 |
| R-2.8.2 | Register World Chunk Sources | RG-11.1, RG-11.4 |
| R-2.8.3 | Configure Voxel World | RG-2.8, RG-2.9, RG-11.5, RG-13.6 |
| R-2.8.4 | Modify Voxels at Runtime | RG-1.6, RG-3.1, RG-11.1, RG-11.4 |
| R-2.8.5 | Configure Heightmap Terrain | RG-2.2, RG-2.8, RG-2.9 |
| R-2.8.6 | Stream Terrain Tiles | RG-11.1, RG-11.4, RG-11.5, RG-13.6 |
| R-2.8.7 | Configure Virtual Texture Pages | RG-2.8, RG-2.9, RG-2.23 |
| R-2.8.8 | Monitor Streaming Residency | RG-12.3 |
| R-2.8.9 | Configure HLOD Generation | RG-2.2, RG-4.2 |
| R-2.8.10 | Configure Landscape Material Layers | RG-2.6, RG-14.2 |
| R-2.8.11 | Configure Virtual Heightfield Mesh | RG-1.1, RG-6.1, RG-6.3 |
| R-2.8.12 | Enable Procedural Terrain Splatting | RG-1.1 |
| R-2.8.13 | Enable Surface Deformation | RG-1.1, RG-2.2 |
| R-2.8.14 | Enable GPU-Driven Procedural Placement | RG-1.1, RG-2.2, RG-2.13 |
| R-2.8.15 | Configure Clipmap Terrain | RG-1.1, RG-2.2 |
| R-2.8.16 | Configure Voxel World LOD | No graph impact — CPU-side only |
| R-2.9.1 | Attach Skeleton to Mesh | RG-1.1, RG-2.2, RG-3.1, RG-14.2 |
| R-2.9.2 | Play Animation Clips | No graph impact — CPU-side only |
| R-2.9.3 | Blend Animation Clips | No graph impact — CPU-side only |
| R-2.9.4 | Define Animation State Machines | No graph impact — CPU-side only |
| R-2.9.5 | Drive State Machine Parameters | No graph impact — CPU-side only |
| R-2.9.6 | Add Morph Targets | RG-5.3, RG-14.2 |
| R-2.9.7 | Configure IK Chains | RG-1.3, RG-5.3 |
| R-2.9.8 | Enable Ragdoll Simulation | RG-1.1, RG-1.6, RG-4.2, RG-12.5 |
| R-2.9.9 | Animate Instanced Crowds | RG-1.1, RG-2.2, RG-2.13, RG-14.2 |
| R-2.9.10 | Define Custom Animation Curves | No graph impact — CPU-side only |
| R-2.9.11 | Register Custom Animation Evaluators | No graph impact — CPU-side only |
| R-2.9.12 | Enable Cloth Simulation | RG-1.3, RG-1.6, RG-4.2, RG-5.3 |
| R-2.9.13 | Enable Hair/Fur Simulation | RG-1.6, RG-4.2 |
| R-2.9.14 | Enable Motion Matching | No graph impact — CPU-side only |
| R-2.9.15 | Enable Muscle Deformation | RG-1.1 |
| R-2.10.1 | Create Vector UI Elements | RG-1.1, RG-14.5 |
| R-2.10.2 | Create Bitmap UI Elements | RG-2.3, RG-14.2 |
| R-2.10.3 | Layout UI with Flexbox | No graph impact — CPU-side only |
| R-2.10.4 | Update UI Incrementally | No graph impact — CPU-side only |
| R-2.10.5 | Create Sprite Instances | RG-1.1, RG-2.3, RG-14.2 |
| R-2.10.6 | Create Tilemaps | RG-1.1, RG-1.3, RG-5.3, RG-14.2 |
| R-2.10.7 | Configure Isometric Rendering | No graph impact — CPU-side only |
| R-2.10.8 | Configure Parallax Scrolling | RG-1.3, RG-5.3, RG-14.5 |
| R-2.11.1 | Author Shader Graphs | No graph impact — CPU-side only |
| R-2.11.2 | Compile Shader Graphs | No graph impact — CPU-side only |
| R-2.11.3 | Register Custom Shader Nodes | No graph impact — CPU-side only |
| R-2.11.4 | Import glTF Assets | RG-2.3, RG-2.15, RG-3.4, RG-4.3, RG-4.4, RG-11.1, RG-11.2 |
| R-2.11.5 | Readback GPU Data to CPU | RG-2.3, RG-2.10, RG-3.1, RG-8.4, RG-8.5, RG-11.2, RG-12.5, RG-14.1, RG-14.2 |
| R-2.11.6 | Declare Custom Render Passes | RG-1.1, RG-1.2, RG-3.1, RG-4.1, RG-5.1, RG-5.7, RG-13.1, RG-13.2, RG-13.4 |
| R-2.11.7 | Extend Material Models | No graph impact — CPU-side only |
| R-2.11.8 | Validate Shader Graphs | No graph impact — CPU-side only |
| R-2.11.9 | Author Domain-Specific Shader Language | No graph impact — CPU-side only |
| R-2.11.10 | Enable Neural Texture Compression | No graph impact — GPU pipeline config only |
| R-2.11.11 | Enable Neural Materials | No graph impact — GPU pipeline config only |
| R-2.12.1 | Configure Streaming Priorities | RG-5.5, RG-7.2, RG-7.3, RG-11.6, RG-13.2, RG-13.3 |
| R-2.12.2 | Configure Streaming Pool Budgets | RG-2.2, RG-2.8, RG-7.2, RG-7.3, RG-7.5, RG-8.2, RG-8.3, RG-11.5, RG-13.3 |
| R-2.12.3 | Enable Tile Streaming | RG-1.1, RG-1.7, RG-2.4, RG-2.9, RG-2.10, RG-3.3, RG-4.3, RG-4.6, RG-5.5, RG-6.1, RG-6.3, RG-6.4, RG-7.3, RG-8.4, RG-8.5, RG-10.2, RG-10.5, RG-11.1, RG-11.2, RG-11.3, RG-11.4, RG-11.6, RG-13.3, RG-13.6, RG-14.1, RG-14.2, RG-14.6, RG-14.7, RG-14.8 |
| R-2.12.4 | Enable Voxel Streaming | RG-1.7, RG-2.4, RG-2.9, RG-2.10, RG-3.3, RG-4.3, RG-6.1, RG-6.3, RG-8.4, RG-10.2, RG-10.5, RG-11.1, RG-11.2, RG-11.3, RG-11.4, RG-13.6, RG-14.2, RG-14.6, RG-14.7, RG-14.8 |
| R-2.12.5 | Configure Async Compute Jobs | RG-1.1, RG-2.10, RG-4.1, RG-4.2, RG-4.4, RG-4.5, RG-4.6, RG-5.4, RG-6.4, RG-6.5, RG-8.4, RG-10.2, RG-10.5, RG-10.6, RG-14.6 |
| R-2.12.6 | Synchronize Cross-Queue Resources | RG-3.4, RG-4.1, RG-4.3, RG-4.4, RG-4.5, RG-4.6, RG-6.5, RG-10.2, RG-10.6, RG-11.2, RG-13.4 |
| R-2.12.7 | Configure Split-Screen Views | RG-2.11, RG-3.5, RG-5.2, RG-5.6, RG-9.1, RG-9.2, RG-9.3, RG-9.5, RG-10.1, RG-10.3, RG-10.4, RG-10.7, RG-13.1, RG-13.4, RG-14.3 |
| R-2.12.8 | Monitor IO Throughput | RG-1.8, RG-8.6, RG-12.1, RG-12.3, RG-12.4, RG-12.7 |
| R-2.12.9 | Enable GPU Decompression | RG-1.1, RG-2.10, RG-4.2 |
| R-2.12.10 | Configure Bindless Resource Management | RG-2.15 |
| R-2.13.1 | Configure Bloom | RG-1.1, RG-1.3, RG-1.6, RG-2.5, RG-2.22, RG-3.1, RG-3.3, RG-3.6, RG-5.1, RG-8.1, RG-8.2, RG-13.3, RG-14.5 |
| R-2.13.2 | Configure Depth of Field | RG-1.1, RG-1.3, RG-1.6, RG-2.5, RG-3.1, RG-8.1, RG-8.2, RG-14.5 |
| R-2.13.3 | Configure Motion Blur | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.13.4 | Configure Auto Exposure | RG-1.1, RG-2.4, RG-3.1 |
| R-2.13.5 | Configure Tonemapping and Color Grading | RG-1.1, RG-1.6, RG-3.1, RG-4.2, RG-14.5 |
| R-2.13.6 | Configure Film Grain | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.13.7 | Configure Chromatic Aberration | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.13.8 | Configure Lens Flare | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.13.9 | Configure Vignette | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.13.10 | Create Post-Process Materials | RG-1.1, RG-1.2 |
| R-2.13.11 | Configure Local Exposure | RG-1.1, RG-1.6, RG-2.4, RG-2.5, RG-5.1, RG-5.3, RG-13.2 |
| R-2.13.12 | Configure Panini Projection | RG-1.1, RG-1.6, RG-3.1, RG-14.5 |
| R-2.14.1 | Configure TAA | RG-1.4, RG-1.6, RG-2.4, RG-2.19, RG-3.1, RG-13.5, RG-14.2, RG-14.6 |
| R-2.14.2 | Configure Temporal Super Resolution | RG-2.4, RG-2.5, RG-2.19, RG-2.20, RG-14.2 |
| R-2.14.3 | Enable FXAA Fallback | RG-1.4, RG-1.6, RG-6.3 |
| R-2.14.4 | Enable Forward-Path MSAA | RG-1.1, RG-1.4, RG-2.21, RG-3.1, RG-3.3, RG-6.1, RG-6.7 |
| R-2.14.5 | Select Anti-Aliasing Mode | RG-1.4, RG-1.12, RG-2.7, RG-6.3, RG-13.7 |
| R-2.14.6 | Enable Checkerboard Rendering | RG-1.14, RG-2.4, RG-2.5 |
| R-2.14.7 | Integrate Third-Party Upscalers | RG-1.4, RG-2.4, RG-2.5, RG-2.20 |
| R-2.14.8 | Enable Frame Generation | RG-2.4, RG-2.24 |
| R-2.14.9 | Enable Latency Reduction | No graph impact — API-level only |
| R-2.15.1 | Configure Strand Hair | RG-6.1, RG-6.4 |
| R-2.15.2 | Configure Card-Based Hair | No graph impact — CPU-side only |
| R-2.15.3 | Configure Hair LOD | RG-1.4, RG-6.3, RG-13.5 |
| R-2.15.4 | Configure Eye Rendering | No graph impact — CPU-side only |
| R-2.15.5 | Configure Cloth Shading | No graph impact — CPU-side only |
| R-2.15.6 | Configure Skin Rendering | RG-1.1, RG-3.1 |
| R-2.15.7 | Assign Deep Opacity Maps | RG-1.1, RG-1.6, RG-1.10, RG-2.6, RG-3.1, RG-7.3, RG-9.1, RG-14.5 |
| R-2.15.8 | Select Character Shading Quality | RG-1.4, RG-6.3, RG-13.5 |
| R-2.15.9 | Enable Compute Software Rasterized Hair | RG-1.1, RG-2.1, RG-3.1 |
| R-2.15.10 | Enable Peach Fuzz Rendering | RG-1.1, RG-2.1 |
| R-2.15.11 | Configure Biometric Skin Model | No graph impact — CPU-side only |
| R-2.16.1 | Configure Hierarchical Instanced Foliage | RG-1.1, RG-2.2, RG-2.13, RG-3.1, RG-4.4 |
| R-2.16.2 | Configure Foliage Wind | RG-2.2, RG-14.2 |
| R-2.16.3 | Configure Per-Instance Fade | No graph impact — CPU-side only |
| R-2.16.4 | Enable Two-Sided Foliage Transmission | No graph impact — CPU-side only |
| R-2.16.5 | Configure Foliage LOD | RG-1.3, RG-1.6, RG-2.13, RG-14.5 |
| R-2.16.6 | Enable GPU Vegetation Skinning | RG-1.1, RG-2.2 |
| R-2.16.7 | Enable Interactive Vegetation | RG-1.1, RG-2.2 |
| R-2.16.8 | Enable Deferred Texturing for Vegetation | RG-1.1, RG-2.1 |
| R-2.17.1 | Create GPU Particle Emitters | RG-1.1, RG-1.6, RG-2.2, RG-2.13, RG-3.1, RG-4.2, RG-4.4, RG-7.3, RG-14.2, RG-14.5 |
| R-2.17.2 | Configure Sprite Particles | No graph impact — CPU-side only |
| R-2.17.3 | Configure Mesh Particles | No graph impact — CPU-side only |
| R-2.17.4 | Configure Ribbon/Trail Particles | No graph impact — CPU-side only |
| R-2.17.5 | Enable Particle Light Emission | RG-5.3 |
| R-2.17.6 | Configure GPU Fluid Simulation | RG-1.1, RG-1.3, RG-1.6, RG-2.2, RG-2.4, RG-4.2, RG-7.3 |
| R-2.17.7 | Configure Particle Collision | RG-3.1 |
| R-2.17.8 | Configure Particle Data Interfaces | No graph impact — CPU-side only |
| R-2.17.9 | Configure Particle Warm-Up | No graph impact — CPU-side only |
| R-2.17.10 | Configure Particle LOD | RG-1.4, RG-1.6, RG-6.3, RG-14.5 |
| R-2.17.11 | Enable Destruction System | RG-1.1, RG-1.6, RG-2.2 |

## Reverse Mapping: Render Graph → High-Level

| RG Req | Title | Derived From |
|---|---|---|
| RG-1.1 | Typed Pass Descriptors | R-2.1.6, R-2.1.8, R-2.1.12, R-2.3.5, R-2.3.8, R-2.3.11, R-2.3.12, R-2.3.13, R-2.4.12, R-2.5.2, R-2.5.3, R-2.5.10, R-2.6.1, R-2.6.3, R-2.6.5, R-2.6.9, R-2.6.11, R-2.7.1, R-2.7.3, R-2.7.10, R-2.7.11, R-2.8.11, R-2.9.1, R-2.9.8, R-2.9.9, R-2.10.1, R-2.10.5, R-2.10.6, R-2.11.6, R-2.12.3, R-2.12.5, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.4, R-2.13.5, R-2.13.6, R-2.13.7, R-2.13.8, R-2.13.9, R-2.13.10, R-2.13.11, R-2.13.12, R-2.14.4, R-2.15.6, R-2.15.7, R-2.16.1, R-2.17.1, R-2.17.6 |
| RG-1.2 | User-Declared Custom Pass Registration | R-2.11.6, R-2.13.10 |
| RG-1.3 | Ordered Pass Chain Declaration | R-2.1.12, R-2.4.12, R-2.5.6, R-2.5.8, R-2.6.6, R-2.7.10, R-2.9.7, R-2.9.12, R-2.10.6, R-2.10.8, R-2.13.1, R-2.13.2, R-2.16.5, R-2.17.6 |
| RG-1.4 | Compile-Time Pass Variant Selection | R-2.1.8, R-2.3.3, R-2.3.5, R-2.4.3, R-2.4.13, R-2.5.3, R-2.5.9, R-2.6.7, R-2.6.8, R-2.14.1, R-2.14.3, R-2.14.4, R-2.14.5, R-2.15.3, R-2.15.8, R-2.17.10 |
| RG-1.5 | Array-Slice-Targeted Pass Instances | R-2.1.7, R-2.2.5, R-2.3.1 |
| RG-1.6 | Conditional Pass Declaration | R-2.1.7, R-2.1.12, R-2.2.11, R-2.3.7, R-2.4.4, R-2.5.8, R-2.6.3, R-2.6.5, R-2.7.6, R-2.8.4, R-2.9.8, R-2.9.12, R-2.9.13, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.5, R-2.13.6, R-2.13.7, R-2.13.8, R-2.13.9, R-2.13.11, R-2.13.12, R-2.14.1, R-2.14.3, R-2.15.7, R-2.16.5, R-2.17.1, R-2.17.6 |
| RG-1.7 | Host Callback Pass | R-2.12.3, R-2.12.4 |
| RG-1.8 | Per-Pass Debug Metadata | R-2.7.6, R-2.7.7, R-2.7.8, R-2.12.8 |
| RG-1.9 | Per-Pass Render Area Constraint | R-2.3.1, R-2.3.2 |
| RG-1.10 | Per-Instance Conditional Enable on Sub-Graph Instances | R-2.3.7, R-2.3.11, R-2.15.7 |
| RG-1.11 | Variable-Count Sub-Graph Instantiation | R-2.3.10 |
| RG-1.12 | Per-Instance Variant Parameter on Sub-Graph | R-2.14.5 |
| RG-1.13 | GPU Work Graph Pass | R-2.7.13 |
| RG-1.14 | Checkerboard Resolve Pass | R-2.14.6 |
| RG-2.1 | Transient Resource Declaration | R-2.1.6, R-2.1.8, R-2.1.10, R-2.7.10, R-2.7.11 |
| RG-2.2 | Persistent Resource Declaration | R-2.1.7, R-2.6.6, R-2.7.5, R-2.7.18, R-2.8.5, R-2.8.9, R-2.9.1, R-2.9.9, R-2.12.2, R-2.16.1, R-2.16.2, R-2.17.1, R-2.17.6 |
| RG-2.3 | Imported External Resource Declaration | R-2.1.4, R-2.10.2, R-2.10.5, R-2.11.4, R-2.11.5 |
| RG-2.4 | History Resource Declaration | R-2.5.6, R-2.5.8, R-2.5.9, R-2.6.5, R-2.12.3, R-2.12.4, R-2.13.4, R-2.13.11, R-2.14.1, R-2.14.2, R-2.17.6 |
| RG-2.5 | Resolution-Scaled Resource Dimensions | R-2.1.11, R-2.13.1, R-2.13.2, R-2.13.11, R-2.14.2 |
| RG-2.6 | Texture Array Resource Declaration | R-2.1.7, R-2.2.5, R-2.2.10, R-2.3.1, R-2.5.8, R-2.8.10, R-2.15.7 |
| RG-2.7 | Variant-Conditional Resource Sets | R-2.1.8, R-2.3.3, R-2.3.5, R-2.14.5 |
| RG-2.8 | Pool-Backed Fixed-Capacity Resource Pools | R-2.3.2, R-2.8.1, R-2.8.3, R-2.8.5, R-2.8.7, R-2.12.2 |
| RG-2.9 | Sparse Texture Resource Declaration | R-2.3.10, R-2.8.1, R-2.8.3, R-2.8.5, R-2.8.7, R-2.12.3, R-2.12.4 |
| RG-2.10 | Staging Buffer Resource Declaration | R-2.11.5, R-2.12.3, R-2.12.4, R-2.12.5 |
| RG-2.11 | Shared vs. Exclusive Resource Annotation | R-2.1.5, R-2.1.7, R-2.12.7 |
| RG-2.12 | Shading Rate Image Resource | R-2.7.11 |
| RG-2.13 | Indirect Argument Buffer Resource | R-2.7.3, R-2.7.5, R-2.7.18, R-2.9.9, R-2.16.1, R-2.16.5, R-2.17.1 |
| RG-2.14 | Ring Buffer Resource Declaration | R-2.7.5 |
| RG-2.15 | Bindless Heap Resource Registration | R-2.11.4 |
| RG-2.16 | 64-Bit-Per-Pixel Render Target Format | R-2.7.10 |
| RG-2.17 | Atlas Sub-Region Resource Declaration | R-2.3.2, R-2.3.10 |
| RG-2.18 | Acceleration Structure Resource Type | R-2.5.2 |
| RG-2.19 | Conditional History Invalidation | R-2.5.9, R-2.6.5, R-2.14.1, R-2.14.2 |
| RG-2.20 | Multiple Independent Named Resolution Parameters | R-2.6.2, R-2.14.2 |
| RG-2.21 | Multi-Sample Render Target Resource Declaration | R-2.14.4 |
| RG-2.22 | Mip-Level Sub-Resource Targeting | R-2.13.1 |
| RG-2.23 | Fixed-Capacity Persistent Texture with Runtime Active Extent | R-2.2.10, R-2.8.7 |
| RG-2.24 | Multi-Frame History Chain | R-2.14.8 |
| RG-2.25 | Opacity Micromap Resource Annotation | R-2.5.14 |
| RG-3.1 | Automatic Read-After-Write Barriers | R-2.1.6, R-2.1.10, R-2.1.12, R-2.2.5, R-2.3.8, R-2.4.12, R-2.5.2, R-2.5.3, R-2.7.10, R-2.7.11, R-2.8.4, R-2.9.1, R-2.11.5, R-2.11.6, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.4, R-2.13.5, R-2.13.6, R-2.13.7, R-2.13.8, R-2.13.9, R-2.13.12, R-2.14.1, R-2.14.4, R-2.15.6, R-2.15.7, R-2.16.1, R-2.17.1, R-2.17.7 |
| RG-3.2 | Automatic Write-After-Write Barriers | R-2.1.7 |
| RG-3.3 | Automatic Layout Transition Tracking | R-2.1.6, R-2.1.7, R-2.1.10, R-2.5.2, R-2.5.3, R-2.7.10, R-2.7.11, R-2.12.3, R-2.12.4, R-2.13.1, R-2.14.4 |
| RG-3.4 | Cross-Queue Ownership Transfer Barriers | R-2.1.4, R-2.2.5, R-2.5.2, R-2.7.5, R-2.11.4, R-2.12.6 |
| RG-3.5 | Single-Writer Invariant Enforcement | R-2.12.7 |
| RG-3.6 | Barrier Merging and Split Barriers | R-2.5.10, R-2.7.10, R-2.7.11, R-2.13.1 |
| RG-4.1 | Per-Pass Queue Affinity Declaration | R-2.1.4, R-2.1.9, R-2.2.5, R-2.5.2, R-2.7.5, R-2.11.6, R-2.12.5, R-2.12.6 |
| RG-4.2 | Dedicated Async Compute Queue | R-2.2.5, R-2.6.6, R-2.7.5, R-2.8.9, R-2.9.8, R-2.9.12, R-2.9.13, R-2.12.5, R-2.13.5, R-2.17.1, R-2.17.6 |
| RG-4.3 | Dedicated Transfer Queue | R-2.1.4, R-2.7.5, R-2.11.4, R-2.12.3, R-2.12.4, R-2.12.6 |
| RG-4.4 | Cross-Queue Dependency Declaration | R-2.1.4, R-2.2.5, R-2.6.6, R-2.7.5, R-2.11.4, R-2.12.5, R-2.12.6, R-2.16.1, R-2.17.1 |
| RG-4.5 | Queue Capability Fallback | R-2.12.5, R-2.12.6 |
| RG-4.6 | Queue-Specific Command Buffer Allocation | R-2.12.3, R-2.12.5, R-2.12.6 |
| RG-5.1 | Topological Execution Order | R-2.1.6, R-2.1.10, R-2.1.12, R-2.3.8, R-2.5.5, R-2.7.10, R-2.7.11, R-2.11.6, R-2.13.11 |
| RG-5.2 | Parameterized Sub-Graph Instantiation | R-2.1.5, R-2.1.7, R-2.1.10, R-2.2.8, R-2.2.15, R-2.3.1, R-2.12.7 |
| RG-5.3 | Explicit Ordering Constraints | R-2.1.7, R-2.1.12, R-2.4.6, R-2.4.12, R-2.5.5, R-2.6.6, R-2.9.6, R-2.9.7, R-2.9.12, R-2.10.6, R-2.10.8, R-2.13.11, R-2.17.5 |
| RG-5.4 | Multi-Frame Pass Dependencies | R-2.12.5 |
| RG-5.5 | Priority-Ordered Transfer Scheduling | R-2.12.1, R-2.12.3 |
| RG-5.6 | Deterministic Ordering | R-2.12.7 |
| RG-5.7 | Cycle Detection | R-2.11.6 |
| RG-6.1 | Static Capability Gate on Passes | R-2.1.9, R-2.3.3, R-2.3.5, R-2.3.12, R-2.4.13, R-2.5.1, R-2.6.3, R-2.7.1, R-2.7.10, R-2.7.11, R-2.8.11, R-2.12.3, R-2.12.4, R-2.14.4, R-2.15.1 |
| RG-6.2 | Hard vs. Soft Capability Gates | R-2.3.3, R-2.3.5, R-2.7.1, R-2.7.10, R-2.7.11 |
| RG-6.3 | Fallback Chain Declaration | R-2.2.11, R-2.3.3, R-2.3.5, R-2.3.9, R-2.4.3, R-2.4.13, R-2.5.4, R-2.6.7, R-2.6.8, R-2.8.11, R-2.12.3, R-2.12.4, R-2.14.3, R-2.14.5, R-2.15.3, R-2.15.8, R-2.17.10 |
| RG-6.4 | Capability Descriptor | R-2.5.1, R-2.7.1, R-2.7.10, R-2.7.11, R-2.12.3, R-2.12.5, R-2.15.1 |
| RG-6.5 | Queue Capability Fallback Gate | R-2.12.5, R-2.12.6 |
| RG-6.6 | Composite Capability-and-Budget Fallback Gate | R-2.3.9 |
| RG-6.7 | Path-Conditioned Variant Gate | R-2.14.4 |
| RG-7.1 | GPU Timing Feedback Gate | R-2.1.11, R-2.3.9 |
| RG-7.2 | Per-Pass Cost and Priority Annotations | R-2.12.1, R-2.12.2 |
| RG-7.3 | Cascading Dead-Pass Elimination | R-2.1.9, R-2.1.12, R-2.3.3, R-2.3.5, R-2.6.5, R-2.7.6, R-2.12.1, R-2.12.2, R-2.12.3, R-2.15.7, R-2.17.1, R-2.17.6 |
| RG-7.4 | Resolution Scale Parameter | R-2.1.11 |
| RG-7.5 | Per-Pool Utilization Budget Gate | R-2.12.2 |
| RG-7.6 | Runtime Parameter Mutation Without Recompilation | R-2.1.11, R-2.1.12, R-2.7.6 |
| RG-8.1 | Lifetime Interval Computation | R-2.1.8, R-2.1.12, R-2.7.10, R-2.7.11, R-2.13.1, R-2.13.2 |
| RG-8.2 | Aliased Heap Allocation | R-2.1.6, R-2.1.8, R-2.7.10, R-2.7.11, R-2.12.2, R-2.13.1, R-2.13.2 |
| RG-8.3 | Pool-Based Aliasing Domain | R-2.12.2 |
| RG-8.4 | Staging Buffer Ring Aliasing | R-2.11.5, R-2.12.3, R-2.12.4, R-2.12.5 |
| RG-8.5 | Heap Type Selection | R-2.11.5, R-2.12.3 |
| RG-8.6 | Memory Usage Diagnostics | R-2.12.8 |
| RG-9.1 | Parameterized Sub-Graph Templates | R-2.1.5, R-2.1.7, R-2.1.10, R-2.2.8, R-2.2.13, R-2.2.15, R-2.3.1, R-2.12.7, R-2.15.7 |
| RG-9.2 | Per-Instance Exclusive Resource Binding | R-2.1.5, R-2.1.10, R-2.12.7 |
| RG-9.3 | Shared Read-Only Resources Across Instances | R-2.1.5, R-2.1.7, R-2.1.10, R-2.2.15, R-2.3.1, R-2.12.7 |
| RG-9.4 | Array-Layer Instance Targeting | R-2.1.7, R-2.3.1 |
| RG-9.5 | Multi-Instance Sub-Graph Compilation | R-2.12.7 |
| RG-10.1 | Independent Command Buffers Per Pass | R-2.1.5, R-2.1.7, R-2.12.7 |
| RG-10.2 | Thread-Safe Command Buffer Pool | R-2.12.3, R-2.12.4, R-2.12.5, R-2.12.6 |
| RG-10.3 | Sub-Graph Instance Parallel Encoding | R-2.1.5, R-2.1.7, R-2.3.1, R-2.12.7 |
| RG-10.4 | Encoding Dependency Graph | R-2.7.5, R-2.12.7 |
| RG-10.5 | Per-Frame Ring Buffer Allocation | R-2.1.4, R-2.12.3, R-2.12.4, R-2.12.5 |
| RG-10.6 | Timeline Fence Coordination | R-2.7.5, R-2.12.5, R-2.12.6 |
| RG-10.7 | Submission Ordering Separate from Encoding Order | R-2.1.5, R-2.12.7 |
| RG-11.1 | Transfer Queue Upload Pass | R-2.1.4, R-2.8.1, R-2.8.2, R-2.8.4, R-2.8.6, R-2.11.4, R-2.12.3, R-2.12.4 |
| RG-11.2 | Completion Fence Per Transfer Pass | R-2.11.4, R-2.11.5, R-2.12.3, R-2.12.4, R-2.12.6 |
| RG-11.3 | Residency Tracking Resource | R-2.12.3, R-2.12.4 |
| RG-11.4 | Fault-Driven Transfer Insertion | R-2.8.2, R-2.8.4, R-2.8.6, R-2.12.3, R-2.12.4 |
| RG-11.5 | LRU Eviction Callback | R-2.8.1, R-2.8.3, R-2.8.6, R-2.12.2 |
| RG-11.6 | Transfer Pass Priority Scheduling | R-2.12.1, R-2.12.3 |
| RG-11.7 | Frame-Boundary Resource Hand-Off | R-2.1.4 |
| RG-12.1 | Per-Pass GPU Timestamp Queries | R-2.1.11, R-2.7.7, R-2.12.8 |
| RG-12.2 | Per-Pass Pipeline Statistics Queries | R-2.1.9, R-2.2.9, R-2.3.8, R-2.7.7, R-2.7.8 |
| RG-12.3 | Transfer Throughput Statistics | R-2.8.8, R-2.12.8 |
| RG-12.4 | Queue Depth Counter | R-2.12.8 |
| RG-12.5 | GPU Readback Pass | R-2.1.9, R-2.1.11, R-2.2.9, R-2.9.8, R-2.11.5 |
| RG-12.6 | Conditional Debug Overlay Passes | R-2.7.6 |
| RG-12.7 | Zero-Overhead Diagnostic Opt-Out | R-2.7.7, R-2.7.8, R-2.12.8 |
| RG-13.1 | DAG Compilation to Execution Plan | R-2.1.8, R-2.1.12, R-2.11.6, R-2.12.7 |
| RG-13.2 | Dead-Pass Elimination | R-2.1.12, R-2.11.6, R-2.12.1, R-2.13.11 |
| RG-13.3 | Transitive Dead-Pass Elimination | R-2.1.9, R-2.1.12, R-2.7.6, R-2.12.1, R-2.12.2, R-2.12.3, R-2.13.1 |
| RG-13.4 | Compile-Time Validation | R-2.7.5, R-2.7.11, R-2.11.6, R-2.12.6, R-2.12.7 |
| RG-13.5 | Recompilation Triggers | R-2.1.8, R-2.1.12, R-2.4.4, R-2.5.9, R-2.6.7, R-2.14.1, R-2.15.3, R-2.15.8 |
| RG-13.6 | Incremental Recompilation on Residency Change | R-2.8.1, R-2.8.3, R-2.8.6, R-2.12.3, R-2.12.4 |
| RG-13.7 | Variant Selection Validation | R-2.1.8, R-2.14.5 |
| RG-13.8 | Sub-Graph Instance Count Validation | R-2.1.7 |
| RG-14.1 | Topology-Data Separation | R-2.1.4, R-2.1.11, R-2.11.5, R-2.12.3 |
| RG-14.2 | Per-Frame Buffer and Texture Binding | R-2.1.4, R-2.3.4, R-2.3.6, R-2.4.8, R-2.6.1, R-2.6.10, R-2.8.10, R-2.9.1, R-2.9.6, R-2.9.9, R-2.10.2, R-2.10.5, R-2.10.6, R-2.11.5, R-2.12.3, R-2.12.4, R-2.14.1, R-2.14.2, R-2.16.2, R-2.17.1 |
| RG-14.3 | Per-Frame Sub-Graph Parameter Update | R-2.1.5, R-2.1.7, R-2.1.10, R-2.12.7 |
| RG-14.4 | Dynamic Resolution Parameter Update | R-2.1.11 |
| RG-14.5 | Per-Frame Pass Activation Flags | R-2.1.7, R-2.1.12, R-2.3.7, R-2.4.5, R-2.6.5, R-2.7.6, R-2.10.1, R-2.10.8, R-2.13.1, R-2.13.2, R-2.13.3, R-2.13.5, R-2.13.6, R-2.13.7, R-2.13.8, R-2.13.9, R-2.13.12, R-2.15.7, R-2.16.5, R-2.17.1, R-2.17.10 |
| RG-14.6 | Frame Index Propagation | R-2.1.7, R-2.12.3, R-2.12.4, R-2.12.5, R-2.14.1 |
| RG-14.7 | Dynamic Transfer Pass Injection | R-2.12.3, R-2.12.4 |
| RG-14.8 | Per-Frame Residency Map Binding | R-2.12.3, R-2.12.4 |

## Summary

| Metric | Count |
|---|---|
| Total high-level requirements (R-2.x.y) | 218 |
| Requirements with graph impact | 159 |
| Requirements without graph impact (CPU-only) | 59 |
| Total render graph requirements (RG-x.y) | 119 |
| Average deduplication ratio (high-level with impact : RG) | 1.33:1 |

### CPU-Only Requirements (No Graph Impact)

| Req | Title |
|---|---|
| R-2.1.1 | Add Entities to Scene |
| R-2.1.2 | Configure Perspective Camera |
| R-2.1.3 | Configure Orthographic Camera |
| R-2.2.1 | Create Point Lights |
| R-2.2.2 | Create Spot Lights |
| R-2.2.3 | Create Directional Lights |
| R-2.2.4 | Create Area Lights |
| R-2.2.6 | Assign IES Light Profiles |
| R-2.2.7 | Configure Light Functions |
| R-2.2.12 | Configure RT Indirect Lighting |
| R-2.2.14 | Adjust Light Priority |
| R-2.4.1 | Create PBR Materials |
| R-2.4.2 | Configure Extended BSDF Layers |
| R-2.4.7 | Assign Materials via Shader Graphs |
| R-2.4.9 | Configure LOD Transition Style |
| R-2.4.10 | Create Material Instances |
| R-2.4.11 | Configure Material Layers |
| R-2.5.7 | Mark Entities for RT Participation |
| R-2.5.15 | Enable Shader Execution Reordering |
| R-2.6.4 | Place Fog Volumes |
| R-2.6.14 | Configure Weather System |
| R-2.7.2 | Configure Meshlet LOD |
| R-2.7.4 | Create GPU Splines |
| R-2.7.9 | Configure Dynamic Tessellation |
| R-2.7.12 | Enable Meshlet Compression |
| R-2.7.15 | Configure LOD Generation |
| R-2.7.16 | Configure LOD Blending |
| R-2.7.17 | Configure Skinned Mesh LOD |
| R-2.8.16 | Configure Voxel World LOD |
| R-2.9.2 | Play Animation Clips |
| R-2.9.3 | Blend Animation Clips |
| R-2.9.4 | Define Animation State Machines |
| R-2.9.5 | Drive State Machine Parameters |
| R-2.9.10 | Define Custom Animation Curves |
| R-2.9.11 | Register Custom Animation Evaluators |
| R-2.9.14 | Enable Motion Matching |
| R-2.10.3 | Layout UI with Flexbox |
| R-2.10.4 | Update UI Incrementally |
| R-2.10.7 | Configure Isometric Rendering |
| R-2.11.1 | Author Shader Graphs |
| R-2.11.2 | Compile Shader Graphs |
| R-2.11.3 | Register Custom Shader Nodes |
| R-2.11.7 | Extend Material Models |
| R-2.11.8 | Validate Shader Graphs |
| R-2.11.9 | Author Domain-Specific Shader Language |
| R-2.11.10 | Enable Neural Texture Compression |
| R-2.11.11 | Enable Neural Materials |
| R-2.14.9 | Enable Latency Reduction |
| R-2.15.2 | Configure Card-Based Hair |
| R-2.15.4 | Configure Eye Rendering |
| R-2.15.5 | Configure Cloth Shading |
| R-2.15.11 | Configure Biometric Skin Model |
| R-2.16.3 | Configure Per-Instance Fade |
| R-2.16.4 | Enable Two-Sided Foliage Transmission |
| R-2.17.2 | Configure Sprite Particles |
| R-2.17.3 | Configure Mesh Particles |
| R-2.17.4 | Configure Ribbon/Trail Particles |
| R-2.17.8 | Configure Particle Data Interfaces |
| R-2.17.9 | Configure Particle Warm-Up |
