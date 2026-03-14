# Renderer Requirements

These requirements do not apply to the core framework. They would apply to a renderer built using
this framework. They help us understand what features renderers would actually consider building, and
what requirements would apply to users building renderers with the framework.

## Scene and Cameras

- **R-2.1.1** Add Entities to Scene
- **R-2.1.2** Configure Perspective Camera
- **R-2.1.3** Configure Orthographic Camera
- **R-2.1.4** Set Entity Transforms
- **R-2.1.5** Enable Multi-Camera Rendering
- **R-2.1.6** Configure Render-to-Texture
- **R-2.1.7** Configure Cubemap Captures
- **R-2.1.8** Select Lighting Model
- **R-2.1.9** Configure GPU Culling
- **R-2.1.10** Configure Scene Capture
- **R-2.1.11** Configure Dynamic Resolution
- **R-2.1.12** Configure Post-Processing Stack

## Lighting

- **R-2.2.1** Create Point Lights
- **R-2.2.2** Create Spot Lights
- **R-2.2.3** Create Directional Lights
- **R-2.2.4** Create Area Lights
- **R-2.2.5** Configure Sky Light (IBL)
- **R-2.2.6** Assign IES Light Profiles
- **R-2.2.7** Configure Light Functions
- **R-2.2.8** Configure Forward+ Tile Size
- **R-2.2.9** Observe Active Light Count
- **R-2.2.10** Configure DDGI Probe Grid
- **R-2.2.11** Toggle DDGI at Runtime
- **R-2.2.12** Configure RT Indirect Lighting
- **R-2.2.13** Configure God Rays
- **R-2.2.14** Adjust Light Priority
- **R-2.2.15** Enable Stochastic Many-Light Sampling

## Shadows and Occlusion

- **R-2.3.1** Enable Cascaded Shadow Maps
- **R-2.3.2** Configure Shadow Atlas
- **R-2.3.3** Select Soft Shadow Quality
- **R-2.3.4** Configure Shadow Bias
- **R-2.3.5** Select Ambient Occlusion Tier
- **R-2.3.6** Configure AO Parameters
- **R-2.3.7** Enable Per-Light Shadows
- **R-2.3.8** Configure Occlusion Culling
- **R-2.3.9** Configure Quality Tier Fallbacks
- **R-2.3.10** Enable Virtual Shadow Maps
- **R-2.3.11** Enable Contact Shadows
- **R-2.3.12** Enable Distance Field Shadows
- **R-2.3.13** Enable Capsule Shadows
- **R-2.3.14** Enable Order-Independent Transparency
- **R-2.3.15** Enable Volumetric Shadow Maps

## Materials and Shading

- **R-2.4.1** Create PBR Materials
- **R-2.4.2** Configure Extended BSDF Layers
- **R-2.4.3** Configure SSS Profiles
- **R-2.4.4** Set Alpha Mode
- **R-2.4.5** Enable Alpha Mask Cutouts
- **R-2.4.6** Configure Transparency Sorting
- **R-2.4.7** Assign Materials via Shader Graphs
- **R-2.4.8** Override Material Parameters at Runtime
- **R-2.4.9** Configure LOD Transition Style
- **R-2.4.10** Create Material Instances
- **R-2.4.11** Configure Material Layers
- **R-2.4.12** Place Decals
- **R-2.4.13** Select Shading Model Variant

## Ray Tracing

- **R-2.5.1** Query RT Hardware Availability
- **R-2.5.2** Enable RT Acceleration Structures
- **R-2.5.3** Configure RT Reflections
- **R-2.5.4** Select Reflection Fallback
- **R-2.5.5** Configure RT Sample Counts
- **R-2.5.6** Configure Denoiser Strength
- **R-2.5.7** Mark Entities for RT Participation
- **R-2.5.8** Enable RT Global Illumination
- **R-2.5.9** Enable Path Tracing
- **R-2.5.10** Enable RT Subsurface Transmission
- **R-2.5.11** Enable Surfel-Based GI
- **R-2.5.12** Enable ReSTIR Sampling
- **R-2.5.13** Enable Real-Time Production Path Tracing
- **R-2.5.14** Enable Opacity Micromaps
- **R-2.5.15** Enable Shader Execution Reordering
- **R-2.5.16** Configure Neural Denoising
- **R-2.5.17** Enable RT Lens Flare
- **R-2.5.18** Enable Voxel-Based GI
- **R-2.5.19** Enable Neural Radiance Cache
- **R-2.5.20** Enable Stochastic SSR

## Environment and Atmosphere

- **R-2.6.1** Configure Procedural Sky
- **R-2.6.2** Configure Atmosphere LUT Resolution
- **R-2.6.3** Enable Volumetric Fog
- **R-2.6.4** Place Fog Volumes
- **R-2.6.5** Enable Procedural Clouds
- **R-2.6.6** Configure FFT Ocean
- **R-2.6.7** Select Ocean Reflection Mode
- **R-2.6.8** Configure God Ray Source
- **R-2.6.9** Set Distance Fog Parameters
- **R-2.6.10** Animate Environment Parameters
- **R-2.6.11** Import Heterogeneous Volumes
- **R-2.6.12** Enable Voxel-Based Volumetric Clouds
- **R-2.6.13** Configure Breaking Waves
- **R-2.6.14** Configure Weather System

## Geometry and Meshes

- **R-2.7.1** Add Mesh Entities
- **R-2.7.2** Configure Meshlet LOD
- **R-2.7.3** Instance Mesh Entities
- **R-2.7.4** Create GPU Splines
- **R-2.7.5** Submit Procedural Geometry Jobs
- **R-2.7.6** Configure Mesh Shader Debug Viz
- **R-2.7.7** Query Meshlet Statistics
- **R-2.7.8** Configure Instanced Draw Batching
- **R-2.7.9** Configure Dynamic Tessellation
- **R-2.7.10** Enable Visibility Buffer
- **R-2.7.11** Configure Variable Rate Shading
- **R-2.7.12** Enable Meshlet Compression
- **R-2.7.13** Enable GPU Work Graphs
- **R-2.7.14** Enable Concurrent Binary Tree Tessellation
- **R-2.7.15** Configure LOD Generation
- **R-2.7.16** Configure LOD Blending
- **R-2.7.17** Configure Skinned Mesh LOD
- **R-2.7.18** Configure Static Mesh Merging

## Worlds and Terrain

- **R-2.8.1** Configure Streaming World
- **R-2.8.2** Register World Chunk Sources
- **R-2.8.3** Configure Voxel World
- **R-2.8.4** Modify Voxels at Runtime
- **R-2.8.5** Configure Heightmap Terrain
- **R-2.8.6** Stream Terrain Tiles
- **R-2.8.7** Configure Virtual Texture Pages
- **R-2.8.8** Monitor Streaming Residency
- **R-2.8.9** Configure HLOD Generation
- **R-2.8.10** Configure Landscape Material Layers
- **R-2.8.11** Configure Virtual Heightfield Mesh
- **R-2.8.12** Enable Procedural Terrain Splatting
- **R-2.8.13** Enable Surface Deformation
- **R-2.8.14** Enable GPU-Driven Procedural Placement
- **R-2.8.15** Configure Clipmap Terrain
- **R-2.8.16** Configure Voxel World LOD

## Animation

- **R-2.9.1** through **R-2.9.15** (see animation domain)

## UI and 2D

- **R-2.10.1** through **R-2.10.8** (see ui-2d domain)

## Shader and Assets

- **R-2.11.1** through **R-2.11.11** (see content-pipeline domain)

## Streaming and IO

- **R-2.12.1** through **R-2.12.10** (see content-pipeline domain)

## Post-Processing

- **R-2.13.1** through **R-2.13.12** (see rendering post-processing)

## Anti-Aliasing and Upscaling

- **R-2.14.1** through **R-2.14.9** (see rendering anti-aliasing)

## Hair and Character Rendering

- **R-2.15.1** through **R-2.15.11** (see rendering character-rendering)

## Foliage and Vegetation

- **R-2.16.1** through **R-2.16.8** (see geometry-world foliage)

## VFX and Particles

- **R-2.17.1** through **R-2.17.11** (see vfx domain)

For full requirement details, see the original files in
[docs/requirements/2-renderer/](../2-renderer/).
