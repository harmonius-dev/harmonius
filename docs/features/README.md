# Harmonius Game Engine -- Features

Complete feature specifications for the Harmonius game engine, a general-purpose, all-genre engine
supporting MMO, RPG, FPS, RTS, 2D, VR, co-op, and local multiplayer games. Each feature has a unique
identifier (e.g., `F-2.3.1` is the first feature in category 2.3). Features reference requirements
from [docs/requirements/](../requirements/).

## Summary

| # | Module | Files | Features |
|---|--------|-------|----------|
| 1 | [Core Runtime](core-runtime/) | 9 | 100 |
| 2 | [Rendering](rendering/) | 12 | 142 |
| 3 | [Geometry & World](geometry-world/) | 6 | 106 |
| 4 | [Physics](physics/) | 8 | 62 |
| 5 | [Audio](audio/) | 5 | 38 |
| 6 | [Input](input/) | 5 | 31 |
| 7 | [AI](ai/) | 8 | 67 |
| 8 | [Networking](networking/) | 8 | 52 |
| 9 | [Animation](animation/) | 6 | 46 |
| 10 | [UI & 2D](ui-2d/) | 6 | 75 |
| 11 | [VFX](vfx/) | 6 | 38 |
| 12 | [Content Pipeline](content-pipeline/) | 7 | 75 |
| 13 | [Game Framework](game-framework/) | 28 | 347 |
| 14 | [Platform](platform/) | 6 | 42 |
| 15 | [Tools & Editor](tools-editor/) | 19 | 160 |
| | **Total** | **139** | **1381** |

## Complete Feature Index

| Module | Area | ID | Title |
|--------|------|----|-------|
| core-runtime | entity-component-system | F-1.1.1 | Archetype-Based Dense Storage |
| core-runtime | entity-component-system | F-1.1.2 | Sparse Component Storage |
| core-runtime | entity-component-system | F-1.1.3 | Archetype Graph and Edge Caching |
| core-runtime | entity-component-system | F-1.1.4 | Static and Dynamic Component Registration |
| core-runtime | entity-component-system | F-1.1.5 | Tag Components (Zero-Size) |
| core-runtime | entity-component-system | F-1.1.6 | Shared Components |
| core-runtime | entity-component-system | F-1.1.7 | Buffer Components (Dynamic Arrays) |
| core-runtime | entity-component-system | F-1.1.8 | Enableable Components |
| core-runtime | entity-component-system | F-1.1.9 | Component Hooks |
| core-runtime | entity-component-system | F-1.1.10 | Component Bundles and Required Components |
| core-runtime | entity-component-system | F-1.1.11 | Entity Lifecycle with Generational Indices |
| core-runtime | entity-component-system | F-1.1.12 | Cleanup Components and Deferred Destruction |
| core-runtime | entity-component-system | F-1.1.13 | Entity Names and Path Lookup |
| core-runtime | entity-component-system | F-1.1.14 | Entity Relationships (Pairs) |
| core-runtime | entity-component-system | F-1.1.15 | Relationship Properties |
| core-runtime | entity-component-system | F-1.1.16 | Built-In Parent-Child Hierarchy |
| core-runtime | entity-component-system | F-1.1.17 | Composable Archetype Queries |
| core-runtime | entity-component-system | F-1.1.18 | Query Sorting and Grouping |
| core-runtime | entity-component-system | F-1.1.19 | Query Variables and Pattern Matching |
| core-runtime | entity-component-system | F-1.1.20 | Automatic Parallel Iteration |
| core-runtime | entity-component-system | F-1.1.21 | Component Aspects |
| core-runtime | entity-component-system | F-1.1.22 | Tick-Based Change Detection |
| core-runtime | entity-component-system | F-1.1.23 | World Resources |
| core-runtime | entity-component-system | F-1.1.24 | Non-Send Resources |
| core-runtime | entity-component-system | F-1.1.25 | Dependency Resolution and Topological Ordering |
| core-runtime | entity-component-system | F-1.1.26 | System Groups and Phases |
| core-runtime | entity-component-system | F-1.1.27 | System Run Criteria and Conditions |
| core-runtime | entity-component-system | F-1.1.28 | Ambiguity Detection |
| core-runtime | entity-component-system | F-1.1.29 | Exclusive Systems |
| core-runtime | entity-component-system | F-1.1.30 | Event-Triggered Observers |
| core-runtime | entity-component-system | F-1.1.31 | Custom Entity Events |
| core-runtime | entity-component-system | F-1.1.32 | Deferred Structural Changes via Command Buffers |
| core-runtime | entity-component-system | F-1.1.33 | Parallel Command Recording |
| core-runtime | entity-component-system | F-1.1.34 | Multiple World Support |
| core-runtime | entity-component-system | F-1.1.35 | Entity Migration Between Worlds |
| core-runtime | entity-component-system | F-1.1.36 | Prefab Entities with Inheritance |
| core-runtime | entity-component-system | F-1.1.37 | Prefab Children and Nested Prefabs |
| core-runtime | entity-component-system | F-1.1.38 | ECS-Integrated State Machine |
| core-runtime | scene-and-transforms | F-1.2.1 | Entity-Based Scene Hierarchy |
| core-runtime | scene-and-transforms | F-1.2.2 | Hierarchy Traversal Iterators |
| core-runtime | scene-and-transforms | F-1.2.3 | Cascading Lifecycle Propagation |
| core-runtime | scene-and-transforms | F-1.2.4 | Hierarchical Transform Propagation |
| core-runtime | scene-and-transforms | F-1.2.5 | Transform Dirty Tracking |
| core-runtime | scene-and-transforms | F-1.2.6 | Spatial Partitioning Index |
| core-runtime | scene-and-transforms | F-1.2.7 | Spatial Scene Queries |
| core-runtime | reflection-and-type-system | F-1.3.1 | Global Type Registry |
| core-runtime | reflection-and-type-system | F-1.3.2 | Structured Type Descriptors |
| core-runtime | reflection-and-type-system | F-1.3.3 | Reflective Property Access |
| core-runtime | reflection-and-type-system | F-1.3.4 | Collection Reflection |
| core-runtime | reflection-and-type-system | F-1.3.5 | Type-Erased Dynamic Values |
| core-runtime | reflection-and-type-system | F-1.3.6 | Custom Type and Field Attributes |
| core-runtime | reflection-and-type-system | F-1.3.7 | Trait Object Registration and Dispatch |
| core-runtime | serialization | F-1.4.1 | Compact Binary Serialization Format |
| core-runtime | serialization | F-1.4.2 | Zero-Copy Deserialization for Read-Only Data |
| core-runtime | serialization | F-1.4.3 | Human-Readable Text Serialization |
| core-runtime | serialization | F-1.4.4 | Schema Versioning with Semantic Version Tags |
| core-runtime | serialization | F-1.4.5 | Data Migration Pipeline |
| core-runtime | serialization | F-1.4.6 | Asset-Aware Serialization with Handle Resolution |
| core-runtime | serialization | F-1.4.7 | Full Scene Serialization and Deserialization |
| core-runtime | events-and-messaging | F-1.5.1 | Typed Double-Buffered Event Channels |
| core-runtime | events-and-messaging | F-1.5.2 | Persistent Event Streams with Cursor-Based Reading |
| core-runtime | events-and-messaging | F-1.5.3 | Component Lifecycle Observers |
| core-runtime | events-and-messaging | F-1.5.4 | Deferred Command Buffers |
| core-runtime | events-and-messaging | F-1.5.5 | Reactive Query Notifications |
| core-runtime | events-and-messaging | F-1.5.6 | Typed Singleton Resources for Shared State |
| core-runtime | events-and-messaging | F-1.5.7 | Cross-World Event Bridges |
| core-runtime | plugin-system | F-1.6.1 | Declarative Plugin Registration |
| core-runtime | plugin-system | F-1.6.2 | Plugin Groups and Presets |
| core-runtime | plugin-system | F-1.6.3 | Explicit Plugin Dependency Declaration |
| core-runtime | plugin-system | F-1.6.4 | Plugin Load Order Resolution |
| core-runtime | plugin-system | F-1.6.5 | Hot Reload of Gameplay Plugins |
| core-runtime | plugin-system | F-1.6.6 | Semantic Versioning and ABI Stability Metadata |
| core-runtime | plugin-system | F-1.6.7 | Capability Negotiation for Optional Features |
| core-runtime | memory-management | F-1.7.1 | Per-Frame Arena Allocator |
| core-runtime | memory-management | F-1.7.2 | Scoped Arena Allocator with Nested Lifetimes |
| core-runtime | memory-management | F-1.7.3 | Typed Pool Allocator |
| core-runtime | memory-management | F-1.7.4 | Generational Index Handles |
| core-runtime | memory-management | F-1.7.5 | Slot Map Container |
| core-runtime | memory-management | F-1.7.6 | Per-Subsystem Memory Budgets |
| core-runtime | memory-management | F-1.7.7 | Memory Profiling and Telemetry Hooks |
| core-runtime | memory-management | F-1.7.8 | Allocation Tagging and Categorization |
| core-runtime | memory-management | F-1.7.9 | Arbitrary Precision Numeric Types |
| core-runtime | async-io | F-1.8.1 | Platform I/O Backend Abstraction |
| core-runtime | async-io | F-1.8.2 | Completion-Based Async Execution Model |
| core-runtime | async-io | F-1.8.3 | Async File I/O Operations |
| core-runtime | async-io | F-1.8.4 | Async Network Socket I/O |
| core-runtime | async-io | F-1.8.5 | Async Audio Stream I/O |
| core-runtime | async-io | F-1.8.6 | Scatter-Gather and Vectored I/O |
| core-runtime | async-io | F-1.8.7 | I/O Priority and Deadline Scheduling |
| core-runtime | async-io | F-1.8.8 | Cooperative I/O Cancellation |
| core-runtime | async-io | F-1.8.9 | Registered Buffer Pools and Zero-Copy Transfers |
| core-runtime | spatial-indexing | F-1.9.1 | Shared BVH Spatial Index |
| core-runtime | spatial-indexing | F-1.9.2 | Incremental BVH Updates |
| core-runtime | spatial-indexing | F-1.9.3 | Hierarchical Grid / Octree Index |
| core-runtime | spatial-indexing | F-1.9.4 | Unified Spatial Query API |
| core-runtime | spatial-indexing | F-1.9.5 | Batch and Parallel Spatial Queries |
| core-runtime | spatial-indexing | F-1.9.6 | Physics Broadphase Integration |
| core-runtime | spatial-indexing | F-1.9.7 | Rendering Culling Integration |
| core-runtime | spatial-indexing | F-1.9.8 | Network Interest Management Integration |
| core-runtime | spatial-indexing | F-1.9.9 | AI Perception and Gameplay Integration |
| rendering | gpu-abstraction-layer | F-2.1.1 | GPU Backend Trait |
| rendering | gpu-abstraction-layer | F-2.1.2 | Command Buffer Abstraction |
| rendering | gpu-abstraction-layer | F-2.1.3 | Pipeline State Abstraction |
| rendering | gpu-abstraction-layer | F-2.1.4 | Metal Backend (Swift-to-C-to-Bindgen) |
| rendering | gpu-abstraction-layer | F-2.1.5 | D3D12 Backend (COM-to-Bindgen) |
| rendering | gpu-abstraction-layer | F-2.1.6 | Vulkan Backend (C-to-Bindgen) |
| rendering | gpu-abstraction-layer | F-2.1.7 | Memory Sub-Allocation |
| rendering | gpu-abstraction-layer | F-2.1.8 | GPU State Tracking |
| rendering | gpu-abstraction-layer | F-2.1.9 | Barrier Optimization |
| rendering | gpu-abstraction-layer | F-2.1.10 | Work Graph Support |
| rendering | gpu-abstraction-layer | F-2.1.11 | Cross-Backend Feature Emulation |
| rendering | gpu-abstraction-layer | F-2.1.12 | GPU Performance Queries and Profiling |
| rendering | render-graph | F-2.2.1 | Declarative Pass Registration |
| rendering | render-graph | F-2.2.2 | Capability Gating |
| rendering | render-graph | F-2.2.3 | Transient Resource Declaration |
| rendering | render-graph | F-2.2.4 | Resource Aliasing |
| rendering | render-graph | F-2.2.5 | Automatic Barrier Insertion |
| rendering | render-graph | F-2.2.6 | Multi-Queue Scheduling |
| rendering | render-graph | F-2.2.7 | Pass Ordering and Topological Sort |
| rendering | render-graph | F-2.2.8 | Budget Culling |
| rendering | render-graph | F-2.2.9 | Multi-View Execution |
| rendering | render-graph | F-2.2.10 | Parallel Command Encoding |
| rendering | render-graph | F-2.2.11 | Streaming Integration |
| rendering | render-graph | F-2.2.12 | Graph Compilation and Caching |
| rendering | render-graph | F-2.2.13 | Render Graph Diagnostics |
| rendering | core-rendering | F-2.3.1 | Direct Lighting |
| rendering | core-rendering | F-2.3.2 | GPU Frustum Culling |
| rendering | core-rendering | F-2.3.3 | Backface Culling |
| rendering | core-rendering | F-2.3.4 | Occlusion Culling (HZB Two-Phase) |
| rendering | core-rendering | F-2.3.5 | Orthographic Projection |
| rendering | core-rendering | F-2.3.6 | Perspective Projection (Reverse-Z) |
| rendering | core-rendering | F-2.3.7 | GPU-Driven Instancing |
| rendering | core-rendering | F-2.3.8 | Render-to-Texture |
| rendering | core-rendering | F-2.3.9 | Cubemaps |
| rendering | core-rendering | F-2.3.10 | Scene Capture |
| rendering | core-rendering | F-2.3.11 | Dynamic Resolution |
| rendering | core-rendering | F-2.3.12 | Subsurface Scattering |
| rendering | core-rendering | F-2.3.13 | Alpha Mask Cutouts |
| rendering | lighting | F-2.4.1 | Forward+ Lighting (Tiled/Clustered) |
| rendering | lighting | F-2.4.2 | Deferred Lighting (G-Buffer) |
| rendering | lighting | F-2.4.3 | PBR Materials (Cook-Torrance BRDF) |
| rendering | lighting | F-2.4.4 | Extended BSDF Materials |
| rendering | lighting | F-2.4.5 | Transparent Objects |
| rendering | lighting | F-2.4.6 | Material Instances |
| rendering | lighting | F-2.4.7 | Material Layers and Blending |
| rendering | lighting | F-2.4.8 | Decal Rendering |
| rendering | lighting | F-2.4.9 | Shading Model Variants |
| rendering | lighting | F-2.4.10 | Stochastic Many-Light Sampling |
| rendering | lighting | F-2.4.11 | Cascaded Shadow Maps |
| rendering | lighting | F-2.4.12 | Soft Shadows (PCF / PCSS / RT) |
| rendering | lighting | F-2.4.13 | Ambient Occlusion (SSAO / GTAO / RT AO) |
| rendering | lighting | F-2.4.14 | Virtual Shadow Maps |
| rendering | lighting | F-2.4.15 | Contact Shadows |
| rendering | lighting | F-2.4.16 | Distance Field Shadows |
| rendering | lighting | F-2.4.17 | Capsule Shadows |
| rendering | lighting | F-2.4.18 | Order-Independent Transparency (OIT) |
| rendering | lighting | F-2.4.19 | Volumetric Shadow Maps |
| rendering | lighting | F-2.4.20 | Area Lights (Rect/Sphere) |
| rendering | lighting | F-2.4.21 | Sky Light (IBL) |
| rendering | lighting | F-2.4.22 | IES Light Profiles |
| rendering | lighting | F-2.4.23 | Light Functions |
| rendering | advanced-rendering | F-2.5.1 | Acceleration Structure Management (BLAS/TLAS) |
| rendering | advanced-rendering | F-2.5.2 | Ray Traced Reflections (Hybrid SSR + RT) |
| rendering | advanced-rendering | F-2.5.3 | Ray Traced Indirect Lighting |
| rendering | advanced-rendering | F-2.5.4 | Real-Time Global Illumination (DDGI) |
| rendering | advanced-rendering | F-2.5.5 | Path Tracing (Reference Renderer) |
| rendering | advanced-rendering | F-2.5.6 | Ray Traced Subsurface Transmission |
| rendering | advanced-rendering | F-2.5.7 | Surfel-Based Global Illumination |
| rendering | advanced-rendering | F-2.5.8 | ReSTIR Sampling Framework |
| rendering | advanced-rendering | F-2.5.9 | Real-Time Production Path Tracing |
| rendering | advanced-rendering | F-2.5.10 | Opacity Micromaps |
| rendering | advanced-rendering | F-2.5.11 | Shader Execution Reordering |
| rendering | advanced-rendering | F-2.5.12 | Neural Denoising (Ray Reconstruction) |
| rendering | advanced-rendering | F-2.5.13 | RT Lens Flare |
| rendering | advanced-rendering | F-2.5.14 | Voxel-Based Global Illumination |
| rendering | advanced-rendering | F-2.5.15 | Neural Radiance Cache |
| rendering | advanced-rendering | F-2.5.16 | Stochastic Screen-Space Reflections |
| rendering | anti-aliasing-upscaling | F-2.6.1 | Temporal Anti-Aliasing (TAA) |
| rendering | anti-aliasing-upscaling | F-2.6.2 | Temporal Super Resolution (TSR) |
| rendering | anti-aliasing-upscaling | F-2.6.3 | FXAA (Fast Approximate Anti-Aliasing) |
| rendering | anti-aliasing-upscaling | F-2.6.4 | MSAA (Multi-Sample Anti-Aliasing) |
| rendering | anti-aliasing-upscaling | F-2.6.5 | Checkerboard Rendering |
| rendering | anti-aliasing-upscaling | F-2.6.6 | Third-Party Upscaler Integration |
| rendering | anti-aliasing-upscaling | F-2.6.7 | Frame Generation |
| rendering | anti-aliasing-upscaling | F-2.6.8 | Latency Reduction |
| rendering | environment | F-2.7.1 | Procedural Sky (Bruneton/Hillaire Atmosphere) |
| rendering | environment | F-2.7.2 | Ray-Marched Volumetric Fog (Froxels) |
| rendering | environment | F-2.7.3 | Procedural Volumetric Clouds |
| rendering | environment | F-2.7.4 | God Rays |
| rendering | environment | F-2.7.5 | Distance and Height Fog |
| rendering | environment | F-2.7.6 | Water Simulation (FFT Ocean) |
| rendering | environment | F-2.7.7 | Heterogeneous Volumes (OpenVDB) |
| rendering | environment | F-2.7.8 | Voxel-Based Volumetric Clouds |
| rendering | environment | F-2.7.9 | Art-Directable Breaking Waves |
| rendering | environment | F-2.7.10 | Weather System |
| rendering | character-rendering | F-2.8.1 | Strand-Based Hair Rendering |
| rendering | character-rendering | F-2.8.2 | Card-Based Hair Rendering |
| rendering | character-rendering | F-2.8.3 | Hair LOD System |
| rendering | character-rendering | F-2.8.4 | Eye Rendering |
| rendering | character-rendering | F-2.8.5 | Cloth Rendering |
| rendering | character-rendering | F-2.8.6 | Skin Rendering (Subsurface Profiles) |
| rendering | character-rendering | F-2.8.7 | Compute Software Rasterized Hair |
| rendering | character-rendering | F-2.8.8 | Peach Fuzz (Vellus Hair) |
| rendering | character-rendering | F-2.8.9 | Biometric Skin Model |
| rendering | post-processing | F-2.9.1 | Bloom |
| rendering | post-processing | F-2.9.2 | Depth of Field (Cinematic) |
| rendering | post-processing | F-2.9.3 | Motion Blur |
| rendering | post-processing | F-2.9.4 | Auto Exposure / Eye Adaptation |
| rendering | post-processing | F-2.9.5 | Tonemapping and Color Grading |
| rendering | post-processing | F-2.9.6 | Film Grain |
| rendering | post-processing | F-2.9.7 | Chromatic Aberration |
| rendering | post-processing | F-2.9.8 | Lens Flare |
| rendering | post-processing | F-2.9.9 | Vignette |
| rendering | post-processing | F-2.9.10 | Post-Process Materials |
| rendering | post-processing | F-2.9.11 | Local Exposure |
| rendering | post-processing | F-2.9.12 | Panini Projection |
| rendering | post-processing | F-2.9.13 | Screen-Space Cavity and Curvature |
| rendering | post-processing | F-2.9.14 | Post-Process Graph Editor |
| rendering | scene-rendering-pipeline | F-2.10.1 | Render Proxy Extraction |
| rendering | scene-rendering-pipeline | F-2.10.2 | Render Component System |
| rendering | scene-rendering-pipeline | F-2.10.3 | Change Detection and Incremental Update |
| rendering | scene-rendering-pipeline | F-2.10.4 | View and Camera Setup |
| rendering | scene-rendering-pipeline | F-2.10.5 | Multi-View Rendering |
| rendering | scene-rendering-pipeline | F-2.10.6 | Draw List Construction |
| rendering | scene-rendering-pipeline | F-2.10.7 | GPU-Driven Batch Compaction |
| rendering | scene-rendering-pipeline | F-2.10.8 | Material Parameter Binding |
| rendering | scene-rendering-pipeline | F-2.10.9 | Debug Visualization and Gizmos |
| rendering | scene-rendering-pipeline | F-2.10.10 | Buffer Visualization Modes |
| rendering | stylized-effects | F-2.11.1 | 2D and 3D Outline Rendering |
| rendering | stylized-effects | F-2.11.2 | Highlight and Glow Effect |
| rendering | stylized-effects | F-2.11.3 | Advanced Toon Shading |
| rendering | stylized-effects | F-2.11.4 | Cut-Through Visibility and Roof Fading |
| rendering | stylized-effects | F-2.11.5 | X-Ray and See-Through Silhouette Rendering |
| rendering | advanced-materials | F-2.12.1 | Transparent Glass and Crystal Rendering |
| rendering | advanced-materials | F-2.12.2 | Ocean Reflection and Refraction |
| rendering | advanced-materials | F-2.12.3 | Emission Maps and Emissive Materials |
| rendering | advanced-materials | F-2.12.4 | Heightmap Tessellation and Parallax |
| rendering | advanced-materials | F-2.12.5 | Fabric and Cloth Materials |
| rendering | advanced-materials | F-2.12.6 | Metal, Wood, Stone, and Natural Materials |
| rendering | advanced-materials | F-2.12.7 | Rubber, Wax, and Soft Surface Materials |
| rendering | advanced-materials | F-2.12.8 | Clearcoat and Multi-Layer Materials |
| rendering | advanced-materials | F-2.12.9 | Fully Custom Material Graphs |
| geometry-world | meshlet-pipeline | F-3.1.1 | Meshlet Decomposition and Hierarchy |
| geometry-world | meshlet-pipeline | F-3.1.2 | Two-Phase Occlusion Culling |
| geometry-world | meshlet-pipeline | F-3.1.3 | Cluster and Triangle Culling |
| geometry-world | meshlet-pipeline | F-3.1.4 | Mesh Shader Pipeline with Indirect Draw Fallback |
| geometry-world | meshlet-pipeline | F-3.1.5 | Screen-Space Error LOD Selection |
| geometry-world | meshlet-pipeline | F-3.1.6 | On-Demand Meshlet Page Streaming |
| geometry-world | meshlet-pipeline | F-3.1.7 | Visibility Buffer Rendering |
| geometry-world | terrain | F-3.2.1 | Heightfield Terrain with Tile-Based Streaming |
| geometry-world | terrain | F-3.2.2 | Virtual Texture Clipmap |
| geometry-world | terrain | F-3.2.3 | CDLOD / Geometry Clipmap LOD |
| geometry-world | terrain | F-3.2.4 | Terrain Hole Masking |
| geometry-world | terrain | F-3.2.5 | Splatmap Material Blending |
| geometry-world | terrain | F-3.2.6 | Terrain Physics Collision |
| geometry-world | terrain | F-3.2.7 | Large World Coordinate Support |
| geometry-world | terrain | F-3.2.8 | Indoor Environments and Portal Visibility |
| geometry-world | terrain | F-3.2.9 | Voxel Volume Representation |
| geometry-world | terrain | F-3.2.10 | Hybrid Heightmap-Voxel Terrain |
| geometry-world | terrain | F-3.2.11 | Planetary-Scale Voxel Sphere |
| geometry-world | terrain | F-3.2.12 | Voxel Meshing Pipeline |
| geometry-world | terrain | F-3.2.13 | Runtime Voxel Editing and Deformation |
| geometry-world | terrain | F-3.2.14 | Voxel LOD and Streaming |
| geometry-world | foliage | F-3.3.1 | GPU-Driven Instanced Foliage |
| geometry-world | foliage | F-3.3.2 | Density Map and Rule-Based Procedural Placement |
| geometry-world | foliage | F-3.3.3 | Billboard and Impostor LOD |
| geometry-world | foliage | F-3.3.4 | GPU Vertex Shader Wind Animation |
| geometry-world | foliage | F-3.3.5 | Character-Vegetation Interaction |
| geometry-world | foliage | F-3.3.6 | Procedural Grass Blade Rendering |
| geometry-world | foliage | F-3.3.7 | Tree Rendering System |
| geometry-world | water | F-3.4.1 | FFT Ocean Wave Simulation |
| geometry-world | water | F-3.4.2 | Shoreline and Depth-Based Blending |
| geometry-world | water | F-3.4.3 | Underwater Rendering and Volume Effects |
| geometry-world | water | F-3.4.4 | Water Caustics Projection |
| geometry-world | water | F-3.4.5 | Water Reflection and Refraction |
| geometry-world | water | F-3.4.6 | Flow Map River Simulation |
| geometry-world | water | F-3.4.7 | Dynamic Foam Generation |
| geometry-world | sky-atmosphere | F-3.5.1 | Procedural Sky Model |
| geometry-world | sky-atmosphere | F-3.5.2 | Multi-Scattering Atmosphere with Aerial Perspective |
| geometry-world | sky-atmosphere | F-3.5.3 | Ray-Marched Volumetric Clouds |
| geometry-world | sky-atmosphere | F-3.5.4 | Cloud Shadow Map |
| geometry-world | sky-atmosphere | F-3.5.5 | Dynamic Time-of-Day System |
| geometry-world | sky-atmosphere | F-3.5.6 | Celestial Body Rendering |
| geometry-world | sky-atmosphere | F-3.5.7 | Environment Cubemap Capture |
| geometry-world | procedural-generation | F-3.6.1 | Node-Based Procedural Content Graph |
| geometry-world | procedural-generation | F-3.6.2 | Point Generation Nodes |
| geometry-world | procedural-generation | F-3.6.3 | Point Filtering and Transformation |
| geometry-world | procedural-generation | F-3.6.4 | Mesh and Instance Spawning from Points |
| geometry-world | procedural-generation | F-3.6.5 | Deterministic Seeding |
| geometry-world | procedural-generation | F-3.6.6 | Point Attributes and Metadata |
| geometry-world | procedural-generation | F-3.6.7 | Point Set Operations |
| geometry-world | procedural-generation | F-3.6.8 | Graph Control Flow (Loops, Branches, Subgraphs) |
| geometry-world | procedural-generation | F-3.6.9 | Non-Destructive Terrain Stamp System |
| geometry-world | procedural-generation | F-3.6.10 | Terrain Texture Stamps |
| geometry-world | procedural-generation | F-3.6.11 | Biome Distribution System |
| geometry-world | procedural-generation | F-3.6.12 | Rule-Based Vegetation Placement |
| geometry-world | procedural-generation | F-3.6.13 | Vegetation Clearing Along Splines |
| geometry-world | procedural-generation | F-3.6.14 | Spline-Based Road Generation |
| geometry-world | procedural-generation | F-3.6.15 | Road Network Generation |
| geometry-world | procedural-generation | F-3.6.16 | Spline SDF Optimization |
| geometry-world | procedural-generation | F-3.6.17 | Road Intersections and Junctions |
| geometry-world | procedural-generation | F-3.6.18 | Shape Grammar Building Generator |
| geometry-world | procedural-generation | F-3.6.19 | Modular Building Assembly |
| geometry-world | procedural-generation | F-3.6.20 | 2D Tile-Based WFC |
| geometry-world | procedural-generation | F-3.6.21 | 3D Voxel WFC |
| geometry-world | procedural-generation | F-3.6.22 | WFC Constraint Painting |
| geometry-world | procedural-generation | F-3.6.23 | Socket-Based Modular Assembly Engine |
| geometry-world | procedural-generation | F-3.6.24 | Procedural Object Generation Rules |
| geometry-world | procedural-generation | F-3.6.25 | Houdini Engine Procedural Object Pipeline |
| geometry-world | procedural-generation | F-3.6.26 | Hierarchical Modular Composition |
| geometry-world | procedural-generation | F-3.6.27 | Interactive PCG Authoring Tools |
| geometry-world | procedural-generation | F-3.6.28 | Artist-Guided Constraint Authoring |
| geometry-world | procedural-generation | F-3.6.29 | AI-Driven Content Generation |
| geometry-world | procedural-generation | F-3.6.30 | Constraint Satisfaction Solver |
| geometry-world | procedural-generation | F-3.6.31 | Runtime Chunk-Based Procedural Generation |
| geometry-world | procedural-generation | F-3.6.32 | GPU Compute Procedural Generation |
| geometry-world | procedural-generation | F-3.6.33 | Noise Function Library |
| geometry-world | procedural-generation | F-3.6.34 | Planetary Terrain Generation |
| geometry-world | procedural-generation | F-3.6.35 | City and Settlement Generation |
| geometry-world | procedural-generation | F-3.6.36 | Faction and Civilization Generation |
| geometry-world | procedural-generation | F-3.6.37 | Procedural Quest Generation |
| geometry-world | procedural-generation | F-3.6.38 | Dynamic Ecosystem Simulation |
| geometry-world | procedural-generation | F-3.6.39 | Civilization Time-Scale Simulation |
| geometry-world | procedural-generation | F-3.6.40 | Procedural Enemy and Creature Placement |
| geometry-world | procedural-generation | F-3.6.41 | Procedural Loot and Economy Distribution |
| geometry-world | procedural-generation | F-3.6.42 | Plate Tectonics and Geological Simulation |
| geometry-world | procedural-generation | F-3.6.43 | Climate and Atmospheric Simulation |
| geometry-world | procedural-generation | F-3.6.44 | Biome Classification and Distribution |
| geometry-world | procedural-generation | F-3.6.45 | Hydrological System and Water Body Generation |
| geometry-world | procedural-generation | F-3.6.46 | Geological Landform Generation |
| geometry-world | procedural-generation | F-3.6.47 | Earth Import and GIS Data Integration |
| geometry-world | procedural-generation | F-3.6.48 | Configurable Planet Parameters |
| geometry-world | procedural-generation | F-3.6.49 | Star System Generation and Stellar Lifecycle |
| geometry-world | procedural-generation | F-3.6.50 | Protoplanetary Disk and Accretion Simulation |
| geometry-world | procedural-generation | F-3.6.51 | Planetary Collision and Giant Impact Simulation |
| geometry-world | procedural-generation | F-3.6.52 | Gas Giant and Non-Terrestrial Planet Generation |
| geometry-world | procedural-generation | F-3.6.53 | Moon and Ring System Generation |
| geometry-world | procedural-generation | F-3.6.54 | Automatic Planet Type Classification |
| geometry-world | procedural-generation | F-3.6.55 | Galaxy Structure Generation |
| geometry-world | procedural-generation | F-3.6.56 | Supermassive Black Hole and Galactic Core |
| geometry-world | procedural-generation | F-3.6.57 | Dark Matter Halo and Large-Scale Structure |
| geometry-world | procedural-generation | F-3.6.58 | Stellar Collision and Merger Simulation |
| geometry-world | procedural-generation | F-3.6.59 | Black Hole Formation and Collision |
| geometry-world | procedural-generation | F-3.6.60 | Universe Generation Pipeline |
| geometry-world | procedural-generation | F-3.6.61 | Planetary Mineralogy and Resource Distribution |
| geometry-world | procedural-generation | F-3.6.62 | Server-Side Universe Generation and Sharding |
| geometry-world | procedural-generation | F-3.6.63 | Sparse Cosmic Data Storage |
| geometry-world | procedural-generation | F-3.6.64 | On-Demand Hierarchical Detail Resolution |
| physics | rigid-body-dynamics | F-4.1.1 | Deterministic Fixed-Timestep Integration |
| physics | rigid-body-dynamics | F-4.1.2 | Simulation Substeps |
| physics | rigid-body-dynamics | F-4.1.3 | Contact Resolution with Restitution and Friction |
| physics | rigid-body-dynamics | F-4.1.4 | Continuous Collision Detection |
| physics | rigid-body-dynamics | F-4.1.5 | Simulation Islands |
| physics | rigid-body-dynamics | F-4.1.6 | Body Sleeping and Wake-Up |
| physics | rigid-body-dynamics | F-4.1.7 | Cross-Zone Physics Continuity |
| physics | rigid-body-dynamics | F-4.1.8 | Character Controller |
| physics | rigid-body-dynamics | F-4.1.9 | Moving Platform System |
| physics | rigid-body-dynamics | F-4.1.10 | Surface Smoothing and Ground Conformance |
| physics | collision-detection | F-4.2.1 | Broadphase Acceleration Structures |
| physics | collision-detection | F-4.2.2 | Narrowphase Contact Generation |
| physics | collision-detection | F-4.2.3 | Primitive and Convex Collision Shapes |
| physics | collision-detection | F-4.2.4 | Triangle Mesh and Heightfield Shapes |
| physics | collision-detection | F-4.2.5 | Compound Shapes |
| physics | collision-detection | F-4.2.6 | Collision Filtering and Layers |
| physics | collision-detection | F-4.2.7 | Collision Events |
| physics | collision-detection | F-4.2.8 | Trigger Volumes |
| physics | collision-detection | F-4.2.9 | Physics Material Assets |
| physics | constraints-and-joints | F-4.3.1 | Core Joint Types |
| physics | constraints-and-joints | F-4.3.2 | Advanced Joint Types |
| physics | constraints-and-joints | F-4.3.3 | Joint Motors and Limits |
| physics | constraints-and-joints | F-4.3.4 | Breakable Joints |
| physics | constraints-and-joints | F-4.3.5 | Ragdoll Configuration |
| physics | constraints-and-joints | F-4.3.6 | Joint Chains and Ropes |
| physics | constraints-and-joints | F-4.3.7 | Constraint Solvers |
| physics | constraints-and-joints | F-4.3.8 | Limb Severance and Joint Destruction |
| physics | constraints-and-joints | F-4.3.9 | Prosthetic and Limb Replacement |
| physics | spatial-queries | F-4.4.1 | Ray Casting |
| physics | spatial-queries | F-4.4.2 | Shape Casting (Sweep Tests) |
| physics | spatial-queries | F-4.4.3 | Overlap Tests |
| physics | spatial-queries | F-4.4.4 | Closest Point Queries |
| physics | spatial-queries | F-4.4.5 | Batch Query Execution |
| physics | spatial-queries | F-4.4.6 | Query Filtering and Custom Predicates |
| physics | vehicle-physics | F-4.5.1 | Wheel Suspension Model |
| physics | vehicle-physics | F-4.5.2 | Tire Friction Model |
| physics | vehicle-physics | F-4.5.3 | Drivetrain Simulation |
| physics | vehicle-physics | F-4.5.4 | Anti-Roll Bars and Stability Control |
| physics | vehicle-physics | F-4.5.5 | Tracked Vehicle Simulation |
| physics | vehicle-physics | F-4.5.6 | Hover Vehicle Simulation |
| physics | vehicle-physics | F-4.5.7 | Server-Authoritative Vehicle Replication |
| physics | destruction-and-fracture | F-4.6.1 | Voronoi Fracture Generation |
| physics | destruction-and-fracture | F-4.6.2 | Pre-Fractured Mesh Authoring |
| physics | destruction-and-fracture | F-4.6.3 | Runtime Fracture and Activation |
| physics | destruction-and-fracture | F-4.6.4 | Progressive Damage Model |
| physics | destruction-and-fracture | F-4.6.5 | Stress Propagation and Structural Collapse |
| physics | destruction-and-fracture | F-4.6.6 | Debris Simulation and Lifecycle |
| physics | destruction-and-fracture | F-4.6.7 | Debris Pooling and LOD |
| physics | soft-body-and-cloth | F-4.7.1 | Position-Based Dynamics Solver |
| physics | soft-body-and-cloth | F-4.7.2 | Cloth Simulation |
| physics | soft-body-and-cloth | F-4.7.3 | Cloth Self-Collision |
| physics | soft-body-and-cloth | F-4.7.4 | Two-Way Rigid Body Coupling |
| physics | soft-body-and-cloth | F-4.7.5 | Wind Interaction |
| physics | soft-body-and-cloth | F-4.7.6 | Cloth Tearing |
| physics | soft-body-and-cloth | F-4.7.7 | Cloth Level of Detail |
| physics | fluid-simulation | F-4.8.1 | SPH Fluid Simulation |
| physics | fluid-simulation | F-4.8.2 | FLIP/PIC Hybrid Simulation |
| physics | fluid-simulation | F-4.8.3 | Eulerian Grid Fluid Solver |
| physics | fluid-simulation | F-4.8.4 | Fluid Surface Reconstruction |
| physics | fluid-simulation | F-4.8.5 | Water Surface Simulation |
| physics | fluid-simulation | F-4.8.6 | Buoyancy and Drag Forces |
| physics | fluid-simulation | F-4.8.7 | Two-Way Fluid-Rigid Body Coupling |
| audio | audio-engine | F-5.1.1 | Sound Source Component |
| audio | audio-engine | F-5.1.2 | Listener Component |
| audio | audio-engine | F-5.1.3 | Hierarchical Mixer Bus Graph |
| audio | audio-engine | F-5.1.4 | Voice Management and Priority System |
| audio | audio-engine | F-5.1.5 | Streaming Playback |
| audio | audio-engine | F-5.1.6 | Sample-Accurate Scheduling |
| audio | audio-engine | F-5.1.7 | Audio Format and Codec Support |
| audio | spatial-audio | F-5.2.1 | 3D Sound Positioning and Doppler |
| audio | spatial-audio | F-5.2.2 | Distance Attenuation Curves |
| audio | spatial-audio | F-5.2.3 | HRTF Binaural Rendering |
| audio | spatial-audio | F-5.2.4 | Ambisonics Encoding and Decoding |
| audio | spatial-audio | F-5.2.5 | Occlusion and Obstruction Filtering |
| audio | spatial-audio | F-5.2.6 | Sound Propagation via Portals and Rays |
| audio | spatial-audio | F-5.2.7 | Reverb Zones and Early Reflections |
| audio | dsp-and-effects | F-5.3.1 | Parametric Filters |
| audio | dsp-and-effects | F-5.3.2 | Parametric Equalizer |
| audio | dsp-and-effects | F-5.3.3 | Algorithmic Reverb |
| audio | dsp-and-effects | F-5.3.4 | Convolution Reverb |
| audio | dsp-and-effects | F-5.3.5 | Compressor, Limiter, and Dynamics Processing |
| audio | dsp-and-effects | F-5.3.6 | Delay, Chorus, and Flanger |
| audio | dsp-and-effects | F-5.3.7 | Pitch Shifting |
| audio | dsp-and-effects | F-5.3.8 | Custom DSP Node Chains |
| audio | music-system | F-5.4.1 | Vertical Re-Mixing (Layered Stems) |
| audio | music-system | F-5.4.2 | Horizontal Re-Sequencing |
| audio | music-system | F-5.4.3 | Transition Rules (Crossfade and Beat-Sync) |
| audio | music-system | F-5.4.4 | Tempo and Beat Clock |
| audio | music-system | F-5.4.5 | Stinger Playback |
| audio | music-system | F-5.4.6 | Playlists and Weighted Randomization |
| audio | music-system | F-5.4.7 | Dynamic Intensity Parameter |
| audio | voice-and-speech | F-5.5.1 | Voice Chat Codec and Transport |
| audio | voice-and-speech | F-5.5.2 | Jitter Buffer and Packet Loss Concealment |
| audio | voice-and-speech | F-5.5.3 | Voice Activity Detection and Noise Suppression |
| audio | voice-and-speech | F-5.5.4 | Text-to-Speech Integration |
| audio | voice-and-speech | F-5.5.5 | Viseme Generation for Lip Sync |
| audio | voice-and-speech | F-5.5.6 | Dialogue Playback and Queuing |
| audio | voice-and-speech | F-5.5.7 | Branching Dialogue Graph |
| audio | voice-and-speech | F-5.5.8 | Voice Chat Channel Management |
| audio | voice-and-speech | F-5.5.9 | Acoustic Echo Cancellation |
| input | device-abstraction | F-6.1.1 | Keyboard Input Capture |
| input | device-abstraction | F-6.1.2 | Mouse Button, Motion, and Scroll Input |
| input | device-abstraction | F-6.1.3 | Unified Gamepad Abstraction |
| input | device-abstraction | F-6.1.4 | Multi-Touch and Pen Input |
| input | device-abstraction | F-6.1.5 | Device Hot-Plug Detection and Enumeration |
| input | input-actions-and-mapping | F-6.2.1 | Typed Input Actions |
| input | input-actions-and-mapping | F-6.2.2 | Input Mapping Contexts with Priority Stacking |
| input | input-actions-and-mapping | F-6.2.3 | Input Value Modifiers |
| input | input-actions-and-mapping | F-6.2.4 | Input Trigger Conditions |
| input | input-actions-and-mapping | F-6.2.5 | Runtime Key Rebinding with Conflict Detection |
| input | input-actions-and-mapping | F-6.2.6 | Platform-Aware Button Glyph Resolution |
| input | input-actions-and-mapping | F-6.2.7 | Input Recording and Playback |
| input | input-actions-and-mapping | F-6.2.8 | Combo Input Trees and Directional Sequences |
| input | input-actions-and-mapping | F-6.2.9 | Input Buffering and Ability Cancel Windows |
| input | input-actions-and-mapping | F-6.2.10 | Input Smoothing, Acceleration, and Aim Assist |
| input | input-actions-and-mapping | F-6.2.11 | Controller-Driven UI Interaction |
| input | gestures | F-6.3.1 | Tap, Multi-Tap, and Long Press Recognition |
| input | gestures | F-6.3.2 | Swipe Direction Recognition |
| input | gestures | F-6.3.3 | Pinch, Rotate, and Pan Gestures |
| input | gestures | F-6.3.4 | Gesture State Machines with Conflict Resolution |
| input | gestures | F-6.3.5 | Custom Gesture Definition |
| input | haptics-and-feedback | F-6.4.1 | Dual-Motor Rumble with Pattern Sequencing |
| input | haptics-and-feedback | F-6.4.2 | DualSense Adaptive Trigger Effects |
| input | haptics-and-feedback | F-6.4.3 | High-Definition Haptic Playback |
| input | haptics-and-feedback | F-6.4.4 | Audio-Driven Haptic Generation |
| input | haptics-and-feedback | F-6.4.5 | Custom Force Feedback Profiles |
| input | vr-input | F-6.5.1 | Head-Mounted Display Tracking |
| input | vr-input | F-6.5.2 | Motion Controller Input |
| input | vr-input | F-6.5.3 | Hand Tracking and Skeletal Input |
| input | vr-input | F-6.5.4 | Eye Tracking and Gaze Input |
| input | vr-input | F-6.5.5 | VR Controller Haptics |
| ai | navigation | F-7.1.1 | Recast-Style NavMesh Generation |
| ai | navigation | F-7.1.2 | NavMesh Tiling & Streaming |
| ai | navigation | F-7.1.3 | A* Pathfinding on NavMesh |
| ai | navigation | F-7.1.4 | String Pulling (Funnel Algorithm) |
| ai | navigation | F-7.1.5 | Path Smoothing |
| ai | navigation | F-7.1.6 | Dynamic Obstacle Carving |
| ai | navigation | F-7.1.7 | NavMesh Links (Off-Mesh Connections) |
| ai | navigation | F-7.1.8 | Incremental Tile Rebuild |
| ai | navigation | F-7.1.9 | Background NavMesh Generation |
| ai | navigation | F-7.1.10 | Destruction-Triggered NavMesh Updates |
| ai | navigation | F-7.1.11 | Player-Built Structure Integration |
| ai | navigation | F-7.1.12 | Multi-Size Agent NavMeshes |
| ai | navigation | F-7.1.13 | Dynamic Area Cost Modification |
| ai | navigation | F-7.1.14 | Hierarchical Pathfinding (HPA*) |
| ai | navigation | F-7.1.15 | NavMesh Runtime Visualization |
| ai | steering-avoidance | F-7.2.1 | RVO/ORCA Local Avoidance |
| ai | steering-avoidance | F-7.2.2 | Obstacle Avoidance (Static Geometry) |
| ai | steering-avoidance | F-7.2.3 | Core Steering Behaviors |
| ai | steering-avoidance | F-7.2.4 | Steering Behavior Blending & Priority |
| ai | steering-avoidance | F-7.2.5 | Formation Movement |
| ai | steering-avoidance | F-7.2.6 | Group Steering & Cohesion |
| ai | behavior-trees | F-7.3.1 | Core Composite & Leaf Nodes |
| ai | behavior-trees | F-7.3.2 | Decorator Nodes |
| ai | behavior-trees | F-7.3.3 | Conditional Aborts |
| ai | behavior-trees | F-7.3.4 | Blackboard System |
| ai | behavior-trees | F-7.3.5 | Behavior Tree Assets & Serialization |
| ai | behavior-trees | F-7.3.6 | Subtree References & Reuse |
| ai | behavior-trees | F-7.3.7 | Runtime Debugging & Visualization |
| ai | utility-ai | F-7.4.1 | Scoring Functions & Response Curves |
| ai | utility-ai | F-7.4.2 | Action Selection & Compensation |
| ai | utility-ai | F-7.4.3 | Considerations & Input Axes |
| ai | utility-ai | F-7.4.4 | Dual Utility System |
| ai | utility-ai | F-7.4.5 | Context-Based Reasoning |
| ai | goap | F-7.5.1 | World State Representation |
| ai | goap | F-7.5.2 | GOAP Forward-Search Planner |
| ai | goap | F-7.5.3 | Action Preconditions & Effects |
| ai | goap | F-7.5.4 | Plan Caching & Reuse |
| ai | goap | F-7.5.5 | Replanning Triggers |
| ai | goap | F-7.5.6 | Goal Prioritization |
| ai | perception | F-7.6.1 | Sight Sense (Cone + Line of Sight) |
| ai | perception | F-7.6.2 | Hearing Sense (Radius + Attenuation) |
| ai | perception | F-7.6.3 | Damage Sense |
| ai | perception | F-7.6.4 | Team & Faction Awareness |
| ai | perception | F-7.6.5 | Stimuli Registration & Expiration |
| ai | perception | F-7.6.6 | Sense Aging & Memory Decay |
| ai | perception | F-7.6.7 | Custom Senses & Perception Priority |
| ai | perception | F-7.6.8 | Smell Sense and Scent Trails |
| ai | perception | F-7.6.9 | Environmental Evidence and Footprint Detection |
| ai | perception | F-7.6.10 | AI Investigation Behavior |
| ai | perception | F-7.6.11 | Multi-Sense Tracking and Pursuit |
| ai | crowd-simulation | F-7.7.1 | Flocking Behaviors (Separation, Alignment, Cohesion) |
| ai | crowd-simulation | F-7.7.2 | Flow Field Navigation |
| ai | crowd-simulation | F-7.7.3 | Flow Field Streaming & Caching |
| ai | crowd-simulation | F-7.7.4 | Mass Entity Simulation |
| ai | crowd-simulation | F-7.7.5 | AI Level of Detail (Processing Budget) |
| ai | crowd-simulation | F-7.7.6 | Density Management |
| ai | crowd-simulation | F-7.7.7 | Knowledge Sharing and Event Propagation |
| ai | crowd-simulation | F-7.7.8 | Shared Awareness and Synchronized Group Reactions |
| ai | crowd-simulation | F-7.7.9 | Faction-Based Behavioral Relationships |
| ai | crowd-simulation | F-7.7.10 | Threat Table and Aggro Targeting |
| ai | crowd-simulation | F-7.7.11 | Animal Tracking and Hunting Behaviors |
| ai | tactical-combat | F-7.8.1 | Cover Evaluation and Scoring |
| ai | tactical-combat | F-7.8.2 | Flanking and Pincer Behavior |
| ai | tactical-combat | F-7.8.3 | Squad Formation and Communication |
| ai | tactical-combat | F-7.8.4 | Suppressive Fire and Pinning |
| ai | tactical-combat | F-7.8.5 | Search and Investigation Patterns |
| ai | tactical-combat | F-7.8.6 | Retreat and Fallback Behavior |
| networking | transport-layer | F-8.1.1 | Connection Handshake and Authentication |
| networking | transport-layer | F-8.1.2 | Connection Lifecycle Management |
| networking | transport-layer | F-8.1.3 | Reliable Ordered Channel |
| networking | transport-layer | F-8.1.4 | Unreliable and Unordered Channels |
| networking | transport-layer | F-8.1.5 | DTLS Encryption |
| networking | transport-layer | F-8.1.6 | Packet Fragmentation, Reassembly, and MTU Discovery |
| networking | transport-layer | F-8.1.7 | Bandwidth Estimation and Congestion Control |
| networking | transport-layer | F-8.1.8 | Network Diagnostics and Quality Indicators |
| networking | state-replication | F-8.2.1 | Delta-Compressed Property Replication |
| networking | state-replication | F-8.2.2 | Component Replication with Schema Versioning |
| networking | state-replication | F-8.2.3 | Area-of-Interest Filtering |
| networking | state-replication | F-8.2.4 | Conditional and Tiered Replication |
| networking | state-replication | F-8.2.5 | Priority Scheduling and Bandwidth Budgeting |
| networking | state-replication | F-8.2.6 | Entity Dormancy |
| networking | remote-procedure-calls | F-8.3.1 | Server RPC (Client-to-Server) |
| networking | remote-procedure-calls | F-8.3.2 | Client RPC (Server-to-Client) |
| networking | remote-procedure-calls | F-8.3.3 | Multicast RPC (Server-to-Group) |
| networking | remote-procedure-calls | F-8.3.4 | RPC Reliability Modes |
| networking | remote-procedure-calls | F-8.3.5 | Parameter Serialization and Validation |
| networking | prediction-and-rollback | F-8.4.1 | Input Prediction and Server Reconciliation |
| networking | prediction-and-rollback | F-8.4.2 | Input Buffering and Redundant Transmission |
| networking | prediction-and-rollback | F-8.4.3 | Snapshot Interpolation |
| networking | prediction-and-rollback | F-8.4.4 | Entity Extrapolation with Error Correction |
| networking | prediction-and-rollback | F-8.4.5 | Server-Side Lag Compensation (Hitbox Rewinding) |
| networking | prediction-and-rollback | F-8.4.6 | Jitter Buffer and Adaptive Tick Alignment |
| networking | session-management | F-8.5.1 | Login and Authentication Services |
| networking | session-management | F-8.5.2 | Skill-Based and Region-Based Matchmaking |
| networking | session-management | F-8.5.3 | Lobby and Party System |
| networking | session-management | F-8.5.4 | Dedicated Server Cluster Management |
| networking | session-management | F-8.5.5 | Session Discovery and Reconnection |
| networking | session-management | F-8.5.6 | Cross-Play Matchmaking and Account Linking |
| networking | session-management | F-8.5.7 | Login Queue and Capacity Management |
| networking | session-management | F-8.5.8 | Headless Dedicated Game Server |
| networking | session-management | F-8.5.9 | Skill-Based Matchmaking Service |
| networking | replay-system | F-8.6.1 | State Recording with Snapshots and Deltas |
| networking | replay-system | F-8.6.2 | Deterministic Playback |
| networking | replay-system | F-8.6.3 | Seek, Fast-Forward, and Slow Motion |
| networking | replay-system | F-8.6.4 | Live Spectator Mode |
| networking | replay-system | F-8.6.5 | Kill Cam and Highlight Extraction |
| networking | mmo-infrastructure | F-8.7.1 | World Sharding and Instancing |
| networking | mmo-infrastructure | F-8.7.2 | Seamless Zone Transitions |
| networking | mmo-infrastructure | F-8.7.3 | Dynamic Server Mesh |
| networking | mmo-infrastructure | F-8.7.4 | Player Migration Between Servers |
| networking | mmo-infrastructure | F-8.7.5 | Persistent World State and Database Integration |
| networking | mmo-infrastructure | F-8.7.6 | Load Balancing and Auto-Scaling |
| networking | mmo-infrastructure | F-8.7.7 | Cross-Shard Services |
| networking | mmo-infrastructure | F-8.7.8 | Inter-Server Communication Bus |
| networking | anti-cheat | F-8.8.1 | Server-Side Cheat Detection |
| networking | anti-cheat | F-8.8.2 | Client Integrity Verification |
| networking | anti-cheat | F-8.8.3 | Behavioral Analysis and Anomaly Detection |
| networking | anti-cheat | F-8.8.4 | Economy Exploit Prevention |
| networking | anti-cheat | F-8.8.5 | Rate Limiting and Abuse Prevention |
| animation | skeletal | F-9.1.1 | GPU Compute Skinning |
| animation | skeletal | F-9.1.2 | GPU Keyframe Evaluation |
| animation | skeletal | F-9.1.3 | Animation Blending (Linear and Cubic) |
| animation | skeletal | F-9.1.4 | Animation Layers and Additive Blending |
| animation | skeletal | F-9.1.5 | Instanced Skeletal Animation |
| animation | skeletal | F-9.1.6 | Root Motion Extraction |
| animation | skeletal | F-9.1.7 | Animation Compression |
| animation | skeletal | F-9.1.8 | Animation Retargeting |
| animation | skeletal | F-9.1.9 | Animation Events and Notifies |
| animation | skeletal | F-9.1.10 | Animation Level of Detail |
| animation | morph | F-9.2.1 | GPU Blend Shape Accumulation |
| animation | morph | F-9.2.2 | Corrective Blend Shapes |
| animation | morph | F-9.2.3 | Facial Animation System |
| animation | morph | F-9.2.4 | Per-Vertex Animation Textures |
| animation | morph | F-9.2.5 | Morph Target Streaming |
| animation | procedural | F-9.3.1 | Two-Bone IK Solver |
| animation | procedural | F-9.3.2 | CCD IK Solver |
| animation | procedural | F-9.3.3 | FABRIK IK Solver |
| animation | procedural | F-9.3.4 | Ragdoll Physics (Partial and Full) |
| animation | procedural | F-9.3.5 | Look-At and Aim Constraints |
| animation | procedural | F-9.3.6 | Motion Matching |
| animation | procedural | F-9.3.7 | Foot Placement and Procedural Locomotion |
| animation | procedural | F-9.3.8 | Multi-Skeleton Procedural Locomotion |
| animation | procedural | F-9.3.9 | Physics-Based Locomotion |
| animation | procedural | F-9.3.10 | Procedural Attachment and Dismemberment |
| animation | procedural | F-9.3.11 | Locomotion Diagnostics and Visualization |
| animation | state-machine | F-9.4.1 | Animation State Graph |
| animation | state-machine | F-9.4.2 | Transitions with Blend Profiles and Sync Markers |
| animation | state-machine | F-9.4.3 | Sub-State Machines |
| animation | state-machine | F-9.4.4 | State Machine Animation Layers |
| animation | state-machine | F-9.4.5 | State Variables and Conditions |
| animation | state-machine | F-9.4.6 | Sync Groups |
| animation | state-machine | F-9.4.7 | Animation Montages |
| animation | state-machine | F-9.4.8 | 1D and 2D Blend Spaces |
| animation | state-machine | F-9.4.9 | Aim Offset and Additive Aim Layers |
| animation | state-machine | F-9.4.10 | AI Animation Integration |
| animation | cloth-hair | F-9.5.1 | GPU Cloth Simulation |
| animation | cloth-hair | F-9.5.2 | Strand-Based Hair Simulation |
| animation | cloth-hair | F-9.5.3 | Card-Based Hair Rendering |
| animation | cloth-hair | F-9.5.4 | Hair LOD System |
| animation | cloth-hair | F-9.5.5 | Cloth-Body Interaction |
| animation | cloth-hair | F-9.5.6 | Hair Wind Response |
| animation | first-person | F-9.6.1 | First-Person Camera Controller |
| animation | first-person | F-9.6.2 | Procedural Weapon Sway and Bob |
| animation | first-person | F-9.6.3 | Procedural Recoil and ADS Animation |
| animation | first-person | F-9.6.4 | Weapon Equip, Inspect, and Dual Wield |
| ui-2d | widget-framework | F-10.1.1 | Declarative Retained Widget Tree |
| ui-2d | widget-framework | F-10.1.2 | Declarative UI Asset Format |
| ui-2d | widget-framework | F-10.1.3 | Widget Pooling and Recycling |
| ui-2d | widget-framework | F-10.1.4 | Flexbox and Grid Layout |
| ui-2d | widget-framework | F-10.1.5 | Anchor and Constraint Layout |
| ui-2d | widget-framework | F-10.1.6 | CSS-like Styling and Themes |
| ui-2d | widget-framework | F-10.1.7 | Reactive Data Binding |
| ui-2d | widget-framework | F-10.1.8 | Focus and Navigation System |
| ui-2d | widget-framework | F-10.1.9 | Localization Hooks |
| ui-2d | widget-framework | F-10.1.10 | World-Space UI Panels |
| ui-2d | widget-framework | F-10.1.11 | VR-Optimized UI Interaction |
| ui-2d | widget-framework | F-10.1.12 | Automatic Minimal Tree Diffing |
| ui-2d | widget-framework | F-10.1.13 | Widget Animation System |
| ui-2d | widget-framework | F-10.1.14 | UI Audio Feedback |
| ui-2d | common-widgets | F-10.2.1 | Rich Text and Text Shaping |
| ui-2d | common-widgets | F-10.2.2 | Text Input and Editing |
| ui-2d | common-widgets | F-10.2.3 | Buttons, Sliders, and Toggle Controls |
| ui-2d | common-widgets | F-10.2.4 | Combo Boxes and Dropdown Menus |
| ui-2d | common-widgets | F-10.2.5 | Scroll Views and Virtualized List Views |
| ui-2d | common-widgets | F-10.2.6 | Tooltips, Context Menus, and Modal Dialogs |
| ui-2d | common-widgets | F-10.2.7 | Drag and Drop |
| ui-2d | common-widgets | F-10.2.8 | Progress Bars and Loading Indicators |
| ui-2d | hud-and-game-ui | F-10.3.1 | Health, Resource, and Cast Bars |
| ui-2d | hud-and-game-ui | F-10.3.2 | Buff and Debuff Icons |
| ui-2d | hud-and-game-ui | F-10.3.3 | Action Bars and Cooldown Indicators |
| ui-2d | hud-and-game-ui | F-10.3.4 | Nameplates and World-Space Labels |
| ui-2d | hud-and-game-ui | F-10.3.5 | Floating Combat Text and Damage Numbers |
| ui-2d | hud-and-game-ui | F-10.3.6 | Minimap and World Map Overlays |
| ui-2d | hud-and-game-ui | F-10.3.7 | Quest Tracker and Objective HUD |
| ui-2d | hud-and-game-ui | F-10.3.8 | Chat System |
| ui-2d | hud-and-game-ui | F-10.3.9 | Inventory Grids and Container Management |
| ui-2d | hud-and-game-ui | F-10.3.10 | Compass Bar HUD |
| ui-2d | hud-and-game-ui | F-10.3.11 | Off-Screen Objective Indicators |
| ui-2d | hud-and-game-ui | F-10.3.12 | Procedural Minimap Generation |
| ui-2d | hud-and-game-ui | F-10.3.13 | World Map Generation and Rendering |
| ui-2d | hud-and-game-ui | F-10.3.14 | Artist-Authored Map Overlays and Post-Processing |
| ui-2d | hud-and-game-ui | F-10.3.15 | Dynamic Map Markers and Quest Integration |
| ui-2d | ui-rendering | F-10.4.1 | Batched Quad Rendering |
| ui-2d | ui-rendering | F-10.4.2 | SDF Text Rendering |
| ui-2d | ui-rendering | F-10.4.3 | Vector Graphics Rendering |
| ui-2d | ui-rendering | F-10.4.4 | UI Atlas and Nine-Slice Rendering |
| ui-2d | ui-rendering | F-10.4.5 | Render-to-Texture for 3D-in-UI |
| ui-2d | ui-rendering | F-10.4.6 | World-Space and Diegetic UI |
| ui-2d | ui-rendering | F-10.4.7 | Anti-Aliased Edges and Clipping |
| ui-2d | 2d-games | F-10.5.1 | Sprite Rendering and Sprite Sheets |
| ui-2d | 2d-games | F-10.5.2 | Frame-Based Sprite Animation |
| ui-2d | 2d-games | F-10.5.3 | 2D Skeletal Animation |
| ui-2d | 2d-games | F-10.5.4 | Vector-Based 2D Rendering |
| ui-2d | 2d-games | F-10.5.5 | Vector Skeletal Animation |
| ui-2d | 2d-games | F-10.5.6 | Tilemap Rendering |
| ui-2d | 2d-games | F-10.5.7 | Isometric and Hex Tilemap Support |
| ui-2d | 2d-games | F-10.5.8 | Procedural 2D Tilemap Generation |
| ui-2d | 2d-games | F-10.5.9 | 2D Camera System |
| ui-2d | 2d-games | F-10.5.10 | 2D Rigid Body Physics |
| ui-2d | 2d-games | F-10.5.11 | 2D Collision Shapes and Tilemap Colliders |
| ui-2d | 2d-games | F-10.5.12 | 2D Joints and Constraints |
| ui-2d | 2d-games | F-10.5.13 | 2D Spatial Queries |
| ui-2d | 2d-games | F-10.5.14 | 2D Dynamic Lighting |
| ui-2d | 2d-games | F-10.5.15 | 2D Particle Effects |
| ui-2d | 2d-games | F-10.5.16 | On-Screen Virtual Controls |
| ui-2d | 2d-games | F-10.5.17 | Mobile Gesture Integration for 2D Games |
| ui-2d | 2d-games | F-10.5.18 | 2D State Replication |
| ui-2d | 2d-games | F-10.5.19 | 2D Client Prediction and Rollback |
| ui-2d | 2d-games | F-10.5.20 | Procedural 2D World Generation |
| ui-2d | 2d-games | F-10.5.21 | 2D Room and Dungeon Generation |
| ui-2d | 2d-games | F-10.5.22 | 2.5D Layer Composition |
| ui-2d | 2d-games | F-10.5.23 | Perspective 3D with 2D Physics (Octopath-Style) |
| ui-2d | 2d-games | F-10.5.24 | Dynamic 2D/3D Asset Layering |
| ui-2d | accessibility | F-10.6.1 | Screen Reader Support |
| ui-2d | accessibility | F-10.6.2 | Subtitle and Caption System |
| ui-2d | accessibility | F-10.6.3 | Colorblind Modes |
| ui-2d | accessibility | F-10.6.4 | High Contrast and Scalable UI |
| ui-2d | accessibility | F-10.6.5 | Input Remapping for Accessibility |
| ui-2d | accessibility | F-10.6.6 | Text-to-Speech for Chat |
| ui-2d | accessibility | F-10.6.7 | WCAG Compliance Targets |
| vfx | particles | F-11.1.1 | Compute Shader Particle Simulation |
| vfx | particles | F-11.1.2 | Particle Simulation Modules |
| vfx | particles | F-11.1.3 | Particle Rendering Modes |
| vfx | particles | F-11.1.4 | Particle LOD, Sorting, and Budget Culling |
| vfx | particles | F-11.1.5 | Sub-Emitters and Events |
| vfx | particles | F-11.1.6 | Particle Light Emission |
| vfx | particles | F-11.1.7 | GPU Fluid Simulation |
| vfx | decals | F-11.2.1 | Deferred and Projected Decals |
| vfx | decals | F-11.2.2 | Mesh Decals |
| vfx | decals | F-11.2.3 | Decal Atlasing and Batching |
| vfx | decals | F-11.2.4 | Decal Priority, Layering, and Lifecycle |
| vfx | decals | F-11.2.5 | Blood and Damage Decals |
| vfx | decals | F-11.2.6 | Footprints and Tire Tracks |
| vfx | screen-effects | F-11.3.1 | Screen Shake |
| vfx | screen-effects | F-11.3.2 | Motion Blur |
| vfx | screen-effects | F-11.3.3 | Lens Flare |
| vfx | screen-effects | F-11.3.4 | Chromatic Aberration and Film Grain |
| vfx | screen-effects | F-11.3.5 | Heat Haze and Refraction |
| vfx | screen-effects | F-11.3.6 | Damage Overlays and Screen Flash |
| vfx | weather-environmental | F-11.4.1 | Rain Particle System and Screen Droplets |
| vfx | weather-environmental | F-11.4.2 | Rain Puddles and Wet Surfaces |
| vfx | weather-environmental | F-11.4.3 | Snow Accumulation and Interaction |
| vfx | weather-environmental | F-11.4.4 | Fog Volumes |
| vfx | weather-environmental | F-11.4.5 | Lightning |
| vfx | weather-environmental | F-11.4.6 | Wind Visualization and Dust Storms |
| vfx | weather-environmental | F-11.4.7 | Underwater Caustics and Depth Fog |
| vfx | destruction | F-11.5.1 | Debris Spawning |
| vfx | destruction | F-11.5.2 | Dust Clouds and Smoke Plumes |
| vfx | destruction | F-11.5.3 | Sparks and Embers |
| vfx | destruction | F-11.5.4 | Structural Cracking Overlays |
| vfx | destruction | F-11.5.5 | Persistent Scorch Marks |
| vfx | destruction | F-11.5.6 | Explosion Shockwaves |
| vfx | destruction | F-11.5.7 | Fire Propagation Visuals |
| vfx | effect-graph | F-11.6.1 | Node-Based Effect Graph Editor |
| vfx | effect-graph | F-11.6.2 | Custom Effect Graph Nodes |
| vfx | effect-graph | F-11.6.3 | Effect Graph Parameter System |
| vfx | effect-graph | F-11.6.4 | Event-Driven Effect Spawning |
| vfx | effect-graph | F-11.6.5 | VFX LOD and Performance Budget |
| content-pipeline | asset-import | F-12.1.1 | Native Asset Ingestion |
| content-pipeline | asset-import | F-12.1.2 | Texture Source Import |
| content-pipeline | asset-import | F-12.1.3 | Audio Source Import |
| content-pipeline | asset-import | F-12.1.4 | Import Validation and Error Reporting |
| content-pipeline | asset-import | F-12.1.5 | Batch Import with Progress Tracking |
| content-pipeline | asset-processing | F-12.2.1 | Texture Compression (BC, ASTC, ETC) |
| content-pipeline | asset-processing | F-12.2.2 | LOD Generation |
| content-pipeline | asset-processing | F-12.2.3 | Meshlet Building |
| content-pipeline | asset-processing | F-12.2.4 | Vertex Cache and Overdraw Optimization |
| content-pipeline | asset-processing | F-12.2.5 | Lightmap UV Generation |
| content-pipeline | asset-processing | F-12.2.6 | Audio Encoding |
| content-pipeline | asset-processing | F-12.2.7 | Shader Graph to HLSL Code Generation |
| content-pipeline | asset-processing | F-12.2.8 | Asset Dependency Graphs |
| content-pipeline | asset-processing | F-12.2.9 | DXC and Metal Shader Converter Pipeline |
| content-pipeline | asset-database | F-12.3.1 | Content-Addressable Storage |
| content-pipeline | asset-database | F-12.3.2 | Asset Metadata Store |
| content-pipeline | asset-database | F-12.3.3 | Hash-Based Import Caching |
| content-pipeline | asset-database | F-12.3.4 | Incremental Builds |
| content-pipeline | asset-database | F-12.3.5 | Asset Search and Tagging |
| content-pipeline | asset-database | F-12.3.6 | Asset Thumbnail Generation |
| content-pipeline | asset-database | F-12.3.7 | AI-Driven Auto-Categorization |
| content-pipeline | asset-database | F-12.3.8 | LLM-Based Semantic Asset Search |
| content-pipeline | asset-database | F-12.3.9 | Smart Collections and Recommendations |
| content-pipeline | asset-database | F-12.3.10 | Asset Versioning |
| content-pipeline | hot-reload | F-12.4.1 | File Watcher |
| content-pipeline | hot-reload | F-12.4.2 | Asset Hot Reload |
| content-pipeline | hot-reload | F-12.4.3 | Shader Hot Reload |
| content-pipeline | hot-reload | F-12.4.4 | Logic Graph Hot Reload |
| content-pipeline | hot-reload | F-12.4.5 | UI Hot Reload |
| content-pipeline | hot-reload | F-12.4.6 | Partial Re-Import |
| content-pipeline | hot-reload | F-12.4.7 | Editor-Runtime Synchronization |
| content-pipeline | streaming-io | F-12.5.1 | Virtual File System |
| content-pipeline | streaming-io | F-12.5.2 | Platform-Native Async I/O |
| content-pipeline | streaming-io | F-12.5.3 | GPU Direct Storage |
| content-pipeline | streaming-io | F-12.5.4 | Texture Streaming |
| content-pipeline | streaming-io | F-12.5.5 | Mesh Streaming |
| content-pipeline | streaming-io | F-12.5.6 | Streaming Priority Queues |
| content-pipeline | streaming-io | F-12.5.7 | Memory Pressure Response |
| content-pipeline | streaming-io | F-12.5.8 | Pak / Archive Files |
| content-pipeline | streaming-io | F-12.5.9 | Compression (LZ4, Zstd) |
| content-pipeline | streaming-io | F-12.5.10 | Download-on-Demand Patching |
| content-pipeline | dcc-plugins | F-12.6.1 | Asset Pipeline Plugin SDK |
| content-pipeline | dcc-plugins | F-12.6.2 | Live Link Connection |
| content-pipeline | dcc-plugins | F-12.6.3 | Houdini Engine Integration |
| content-pipeline | dcc-plugins | F-12.6.4 | Houdini Mesh and Scene Export |
| content-pipeline | dcc-plugins | F-12.6.5 | Houdini Procedural Placement Pipeline |
| content-pipeline | dcc-plugins | F-12.6.6 | Maya Direct Export Plugin |
| content-pipeline | dcc-plugins | F-12.6.7 | Maya Animation and Rigging Export |
| content-pipeline | dcc-plugins | F-12.6.8 | Blender Direct Export Addon |
| content-pipeline | dcc-plugins | F-12.6.9 | Blender Material to Engine Material Conversion |
| content-pipeline | dcc-plugins | F-12.6.10 | Marvelous Designer Clothing Export |
| content-pipeline | dcc-plugins | F-12.6.11 | Garment-to-Character Fitting |
| content-pipeline | dcc-plugins | F-12.6.12 | Substance Material Import |
| content-pipeline | dcc-plugins | F-12.6.13 | Substance Painter Project Import |
| content-pipeline | dcc-plugins | F-12.6.14 | Runtime Substance Material Evaluation |
| content-pipeline | dcc-plugins | F-12.6.15 | Photoshop Direct Export Plugin |
| content-pipeline | dcc-plugins | F-12.6.16 | Photoshop Layer-to-UI Asset Pipeline |
| content-pipeline | dcc-plugins | F-12.6.17 | Illustrator Vector Asset Export |
| content-pipeline | dcc-plugins | F-12.6.18 | ZBrush High-Poly to Engine Pipeline |
| content-pipeline | dcc-plugins | F-12.6.19 | ZBrush Decimation and LOD Generation |
| content-pipeline | dcc-plugins | F-12.6.20 | MotionBuilder Animation Export |
| content-pipeline | dcc-plugins | F-12.6.21 | MotionBuilder Live Mocap Streaming |
| content-pipeline | dcc-plugins | F-12.6.22 | DCC Plugin Git LFS Lock Workflow |
| content-pipeline | dcc-plugins | F-12.6.23 | DCC Plugin Review Comment Viewer |
| content-pipeline | dcc-plugins | F-12.6.24 | DCC Plugin Asset Status Dashboard |
| content-pipeline | dcc-plugins | F-12.6.25 | DCC-Agnostic Material Mapping |
| content-pipeline | dcc-plugins | F-12.6.26 | Batch Export and CI Integration |
| content-pipeline | asset-versioning | F-12.7.1 | Universal Binary Asset Format |
| content-pipeline | asset-versioning | F-12.7.2 | Compressed Asset Bundles |
| content-pipeline | asset-versioning | F-12.7.3 | Structural Asset Diffing |
| content-pipeline | asset-versioning | F-12.7.4 | Three-Way Asset Merge |
| content-pipeline | asset-versioning | F-12.7.5 | Automatic Merge Conflict Resolution |
| content-pipeline | asset-versioning | F-12.7.6 | Spreadsheet-Style Data Table Editor |
| content-pipeline | asset-versioning | F-12.7.7 | Universal Asset Inspector |
| content-pipeline | asset-versioning | F-12.7.8 | Git LFS and Custom Merge Driver Integration |
| game-framework | gameplay-primitives | F-13.1.1 | Game Mode State Machine |
| game-framework | gameplay-primitives | F-13.1.2 | Game State Manager |
| game-framework | gameplay-primitives | F-13.1.3 | Player Controller |
| game-framework | gameplay-primitives | F-13.1.4 | Pawn and Character System |
| game-framework | gameplay-primitives | F-13.1.5 | Gameplay Ability System |
| game-framework | gameplay-primitives | F-13.1.6 | Gameplay Effect System |
| game-framework | gameplay-primitives | F-13.1.7 | Damage Model |
| game-framework | gameplay-primitives | F-13.1.8 | Death, Respawn, and Encounter Reset |
| game-framework | gameplay-primitives | F-13.1.9 | Modular System Registration |
| game-framework | gameplay-primitives | F-13.1.10 | Rust Plugin API for Developers |
| game-framework | world-management | F-13.2.1 | Level Streaming |
| game-framework | world-management | F-13.2.2 | Grid-Based World Partitioning |
| game-framework | world-management | F-13.2.3 | Hierarchical Spatial Partitioning |
| game-framework | world-management | F-13.2.4 | Sub-Level Composition |
| game-framework | world-management | F-13.2.5 | Persistent and Transient Actors |
| game-framework | world-management | F-13.2.6 | Day/Night Cycle |
| game-framework | world-management | F-13.2.7 | Weather System Integration |
| game-framework | save-system | F-13.3.1 | Save Game Serialization |
| game-framework | save-system | F-13.3.2 | Save Data Migration and Versioning |
| game-framework | save-system | F-13.3.3 | Checkpoint and Autosave |
| game-framework | save-system | F-13.3.4 | Save Slots and Management |
| game-framework | save-system | F-13.3.5 | Cloud Save with Platform APIs |
| game-framework | save-system | F-13.3.6 | Async Save I/O Pipeline |
| game-framework | scripting | F-13.4.1 | Gameplay Logic Graph Integration |
| game-framework | scripting | F-13.4.2 | Logic Graph Debugging for Gameplay |
| game-framework | scripting | F-13.4.3 | Logic Graph Hot Reload |
| game-framework | cinematics | F-13.5.1 | Sequencer and Timeline |
| game-framework | cinematics | F-13.5.2 | Cutscene Camera System |
| game-framework | cinematics | F-13.5.3 | Camera Rails and Splines |
| game-framework | cinematics | F-13.5.4 | Actor Animation Blending in Cinematics |
| game-framework | cinematics | F-13.5.5 | Dialogue Triggers and Subtitles |
| game-framework | cinematics | F-13.5.6a | Cutscene Skip System |
| game-framework | cinematics | F-13.5.6b | Cutscene Playback Speed |
| game-framework | cinematics | F-13.5.6c | Cutscene Pause |
| game-framework | cinematics | F-13.5.7 | Letterboxing and Cinematic Framing |
| game-framework | quest-dialogue | F-13.6.1 | Quest Graph System |
| game-framework | quest-dialogue | F-13.6.2 | Quest Prerequisites and Gating |
| game-framework | quest-dialogue | F-13.6.3 | Quest Tracking and Journal |
| game-framework | quest-dialogue | F-13.6.4 | Dialogue Tree System |
| game-framework | quest-dialogue | F-13.6.5a | Conversation Camera and Framing |
| game-framework | quest-dialogue | F-13.6.5b | Conversation Gameplay State |
| game-framework | quest-dialogue | F-13.6.5c | Conversation Interruption and Resumption |
| game-framework | quest-dialogue | F-13.6.6 | Quest Rewards and Economy Hooks |
| game-framework | quest-dialogue | F-13.6.7a | Server-Driven World Events |
| game-framework | quest-dialogue | F-13.6.7b | Quest Phasing System |
| game-framework | gameplay-databases | F-13.7.1 | Typed Table Schema Definition |
| game-framework | gameplay-databases | F-13.7.2 | Row-Based Data Tables |
| game-framework | gameplay-databases | F-13.7.3 | Curve and Formula Definitions |
| game-framework | gameplay-databases | F-13.7.4 | Visual Formula Nodes |
| game-framework | gameplay-databases | F-13.7.5 | Row Inheritance and Prototype Chains |
| game-framework | gameplay-databases | F-13.7.6 | Currency and Resource Definitions |
| game-framework | gameplay-databases | F-13.7.7 | Crafting Recipe Tables |
| game-framework | gameplay-databases | F-13.7.8 | Loot Tables with Weighted Random |
| game-framework | gameplay-databases | F-13.7.9 | Stat and Attribute Tables |
| game-framework | gameplay-databases | F-13.7.10 | Asset List Tables |
| game-framework | gameplay-databases | F-13.7.11 | Indexed Lookup and Filtering |
| game-framework | gameplay-databases | F-13.7.12 | ECS Component Binding |
| game-framework | gameplay-databases | F-13.7.13 | Hot Reload and Versioned Patching |
| game-framework | gameplay-databases | F-13.7.14 | Data Validation and Constraint Checking |
| game-framework | character-customization | F-13.8.1 | Parametric Facial Feature System |
| game-framework | character-customization | F-13.8.2 | Preset Blending and Templates |
| game-framework | character-customization | F-13.8.3 | Parametric Body Shape System |
| game-framework | character-customization | F-13.8.4 | Body Morph Propagation to Equipment |
| game-framework | character-customization | F-13.8.5 | Skin Material System |
| game-framework | character-customization | F-13.8.6 | Makeup and Face Paint Layer System |
| game-framework | character-customization | F-13.8.7 | Eye Customization |
| game-framework | character-customization | F-13.8.8 | Hair Customization System |
| game-framework | character-customization | F-13.8.9 | Modular Mesh Part System |
| game-framework | character-customization | F-13.8.10 | Equipment Attachment and Socket System |
| game-framework | character-customization | F-13.8.11 | Transmog and Appearance Override |
| game-framework | character-customization | F-13.8.12 | Multi-Race Base Mesh Support |
| game-framework | character-customization | F-13.8.13 | Character LOD and Crowd Optimization |
| game-framework | character-customization | F-13.8.14 | Mesh Merging and Draw Call Reduction |
| game-framework | character-customization | F-13.8.15 | Character Appearance Serialization |
| game-framework | inventory | F-13.9.1 | ECS-Based Inventory Containers |
| game-framework | inventory | F-13.9.2 | Grid-Based Inventory Layout |
| game-framework | inventory | F-13.9.3 | Item Stacking and Splitting |
| game-framework | inventory | F-13.9.4 | Per-Instance Item Properties |
| game-framework | inventory | F-13.9.5 | Item Socket and Augmentation System |
| game-framework | inventory | F-13.9.6 | Inventory Transfer and Drag-Drop |
| game-framework | inventory | F-13.9.7 | Loot Distribution |
| game-framework | inventory | F-13.9.8 | Merchant and Trading |
| game-framework | inventory | F-13.9.9 | Equipment Slot Binding |
| game-framework | inventory | F-13.9.10 | Inventory Serialization and Persistence |
| game-framework | abilities | F-13.10.1 | Ability Definition and Composition |
| game-framework | abilities | F-13.10.2 | Ability Activation and Input Integration |
| game-framework | abilities | F-13.10.3 | Gameplay Effect System |
| game-framework | abilities | F-13.10.4 | Melee Combat System |
| game-framework | abilities | F-13.10.5 | Ranged Combat and Projectile System |
| game-framework | abilities | F-13.10.6 | Hitbox and Hurtbox System |
| game-framework | selection-system | F-13.11.1 | 3D World Picking |
| game-framework | selection-system | F-13.11.2 | 2D Screen-Space Picking |
| game-framework | selection-system | F-13.11.3 | Selection State Management |
| game-framework | selection-system | F-13.11.4a | RTS Selection Preset |
| game-framework | selection-system | F-13.11.4b | RPG Selection Preset |
| game-framework | selection-system | F-13.11.4c | Action Selection Preset |
| game-framework | selection-system | F-13.11.4d | Builder/Sandbox Selection Preset |
| game-framework | selection-system | F-13.11.5 | Runtime Selection Groups |
| game-framework | selection-system | F-13.11.6a | Basic Command Dispatch |
| game-framework | selection-system | F-13.11.6b | Formation Movement |
| game-framework | selection-system | F-13.11.6c | Split and Subgroup Commands |
| game-framework | selection-system | F-13.11.6d | Command History and Undo |
| game-framework | selection-system | F-13.11.7 | 2D Selection Box (Marquee Select) |
| game-framework | selection-system | F-13.11.8 | Selection Indicators and Visual Feedback |
| game-framework | progression | F-13.12.1a | Race Definition |
| game-framework | progression | F-13.12.1b | Class Definition |
| game-framework | progression | F-13.12.1c | Multi-Class and Job Change |
| game-framework | progression | F-13.12.1d | Prestige and Rebirth System |
| game-framework | progression | F-13.12.2a | Talent Tree Data Model |
| game-framework | progression | F-13.12.2b | Talent Allocation and Respec |
| game-framework | progression | F-13.12.2c | Talent Tree Visual Editor |
| game-framework | progression | F-13.12.3a | Profession Data Model |
| game-framework | progression | F-13.12.3b | Gathering Profession Integration |
| game-framework | progression | F-13.12.3c | Crafting Profession Integration |
| game-framework | progression | F-13.12.4 | Crafting Station Interaction |
| game-framework | progression | F-13.12.5 | Reputation and Faction Standing |
| game-framework | progression | F-13.12.6a | Achievement Definition and Tracking |
| game-framework | progression | F-13.12.6b | Achievement Rewards and Display |
| game-framework | progression | F-13.12.6c | Platform Achievement Sync |
| game-framework | progression | F-13.12.7 | Item Enhancement and Enchanting |
| game-framework | progression | F-13.12.8a | Item Rarity Tier System |
| game-framework | progression | F-13.12.8b | Affix System |
| game-framework | progression | F-13.12.8c | Stat Re-Rolling Mechanics |
| game-framework | progression | F-13.12.9 | Item Set Bonuses |
| game-framework | progression | F-13.12.10 | Item Durability and Repair |
| game-framework | social | F-13.13.1a | Guild CRUD and Membership |
| game-framework | social | F-13.13.1b | Guild Rank and Permission System |
| game-framework | social | F-13.13.1c | Guild Leveling and Perks |
| game-framework | social | F-13.13.1d | Guild Roster UI |
| game-framework | social | F-13.13.2 | Guild Storage and Bank |
| game-framework | social | F-13.13.3a | Territory Claim System |
| game-framework | social | F-13.13.3b | Guild War Declaration and PvP Rules |
| game-framework | social | F-13.13.3c | Siege Mechanics |
| game-framework | social | F-13.13.3d | Guild Rankings and Leaderboards |
| game-framework | social | F-13.13.4 | Friends List and Social Graph |
| game-framework | social | F-13.13.5a | Core Mail Send/Receive |
| game-framework | social | F-13.13.5b | Mail Attachments |
| game-framework | social | F-13.13.5c | System Mail |
| game-framework | social | F-13.13.6a | Core Chat Infrastructure |
| game-framework | social | F-13.13.6b | Chat Content Features |
| game-framework | social | F-13.13.6c | Custom Player-Created Channels |
| game-framework | social | F-13.13.7 | Emote and Social Action System |
| game-framework | social | F-13.13.8 | Player Inspection |
| game-framework | social | F-13.13.9 | Dungeon and Group Finder |
| game-framework | social | F-13.13.10a | Arena System |
| game-framework | social | F-13.13.10b | Battleground System |
| game-framework | social | F-13.13.10c | PvP Rating and Seasonal Rewards |
| game-framework | social | F-13.13.10d | PvP Stat Normalization |
| game-framework | building-survival | F-13.14.1 | Modular Building Placement System |
| game-framework | building-survival | F-13.14.2 | Construction Phase and Progress |
| game-framework | building-survival | F-13.14.3 | Structural Integrity |
| game-framework | building-survival | F-13.14.4 | Building Upgrade and Repair |
| game-framework | building-survival | F-13.14.5a | Housing Plot and Instance System |
| game-framework | building-survival | F-13.14.5b | Furniture Placement |
| game-framework | building-survival | F-13.14.5c | Functional Furniture Effects |
| game-framework | building-survival | F-13.14.6a | Hunger and Thirst System |
| game-framework | building-survival | F-13.14.6b | Temperature and Warmth System |
| game-framework | building-survival | F-13.14.6c | Stamina and Fatigue System |
| game-framework | building-survival | F-13.14.6d | Vital Debuff System |
| game-framework | building-survival | F-13.14.7a | Resource Node Definition |
| game-framework | building-survival | F-13.14.7b | Gathering Interaction Loop |
| game-framework | building-survival | F-13.14.7c | Resource Node Procedural Distribution |
| game-framework | building-survival | F-13.14.8 | Farming and Crop System |
| game-framework | building-survival | F-13.14.9a | Animal Needs and Happiness |
| game-framework | building-survival | F-13.14.9b | Animal Housing |
| game-framework | building-survival | F-13.14.9c | Animal Breeding |
| game-framework | pets-mounts | F-13.15.1 | Companion AI Framework |
| game-framework | pets-mounts | F-13.15.2 | Pet Needs and Mood |
| game-framework | pets-mounts | F-13.15.3a | Mount Summoning and Dismissal |
| game-framework | pets-mounts | F-13.15.3b | Mounted Locomotion |
| game-framework | pets-mounts | F-13.15.3c | Mounted Combat |
| game-framework | pets-mounts | F-13.15.3d | Mount Type Variants |
| game-framework | pets-mounts | F-13.15.4 | Creature Taming |
| game-framework | pets-mounts | F-13.15.5a | Pet Life Stages |
| game-framework | pets-mounts | F-13.15.5b | Pet Evolution Branching |
| game-framework | pets-mounts | F-13.15.5c | Pet Breeding System |
| game-framework | weapon-system | F-13.16.1 | Weapon Fire Mode System |
| game-framework | weapon-system | F-13.16.2a | Magazine and Ammo Management |
| game-framework | weapon-system | F-13.16.2b | Reload Mechanics |
| game-framework | weapon-system | F-13.16.2c | Ammo Type System |
| game-framework | weapon-system | F-13.16.3 | Recoil Pattern and Weapon Spread |
| game-framework | weapon-system | F-13.16.4a | Projectile Drop and Travel Time |
| game-framework | weapon-system | F-13.16.4b | Wind Deflection |
| game-framework | weapon-system | F-13.16.4c | Surface Penetration and Ricochet |
| game-framework | weapon-system | F-13.16.4d | Weapon Zeroing |
| game-framework | weapon-system | F-13.16.5a | Attachment Slot Model |
| game-framework | weapon-system | F-13.16.5b | Attachment Visual Integration |
| game-framework | weapon-system | F-13.16.5c | Attachment Customization UI |
| game-framework | weapon-system | F-13.16.6a | Surface Type Tag System |
| game-framework | weapon-system | F-13.16.6b | Impact VFX Response |
| game-framework | weapon-system | F-13.16.6c | Impact Audio Response |
| game-framework | weapon-system | F-13.16.6d | Impact Decal Response |
| game-framework | traversal-interaction | F-13.17.1 | World Object Interaction System |
| game-framework | traversal-interaction | F-13.17.2 | Door and Lock System |
| game-framework | traversal-interaction | F-13.17.3 | Physics Object Pickup and Throw |
| game-framework | traversal-interaction | F-13.17.4a | Traversal Detection System |
| game-framework | traversal-interaction | F-13.17.4b | Vault and Mantle Actions |
| game-framework | traversal-interaction | F-13.17.4c | Wall Run |
| game-framework | traversal-interaction | F-13.17.4d | Crouch Slide |
| game-framework | traversal-interaction | F-13.17.4e | Balance Beam |
| game-framework | traversal-interaction | F-13.17.5a | Free-Climb System |
| game-framework | traversal-interaction | F-13.17.5b | Ladder System |
| game-framework | traversal-interaction | F-13.17.5c | Ledge Grab and Shimmy |
| game-framework | traversal-interaction | F-13.17.6 | Swimming and Diving |
| game-framework | traversal-interaction | F-13.17.7 | Grappling Hook and Zipline |
| game-framework | stealth-cover | F-13.18.1 | Player Visibility and Stealth System |
| game-framework | stealth-cover | F-13.18.2 | AI Alert State Machine |
| game-framework | stealth-cover | F-13.18.3 | Noise Generation and Distraction |
| game-framework | stealth-cover | F-13.18.4 | Stealth Takedown System |
| game-framework | stealth-cover | F-13.18.5 | Cover Point Detection and Usage |
| game-framework | npc-simulation | F-13.19.1 | NPC Relationship and Affinity System |
| game-framework | npc-simulation | F-13.19.2 | NPC Personality and Emotion Model |
| game-framework | npc-simulation | F-13.19.3a | NPC Deed Memory |
| game-framework | npc-simulation | F-13.19.3b | Gossip Propagation Network |
| game-framework | npc-simulation | F-13.19.3c | Emergent Reputation Aggregation |
| game-framework | npc-simulation | F-13.19.4a | Schedule Data Model |
| game-framework | npc-simulation | F-13.19.4b | Schedule Execution |
| game-framework | npc-simulation | F-13.19.4c | Schedule-Gated Interactions |
| game-framework | npc-simulation | F-13.19.5 | Ambient Bark System |
| game-framework | npc-simulation | F-13.19.6 | Threat and Aggro Table System |
| game-framework | npc-simulation | F-13.19.7 | NPC-to-NPC Conversation System |
| game-framework | npc-simulation | F-13.19.8 | NPC Independent Memory System |
| game-framework | npc-simulation | F-13.19.9 | NPC Environmental Interaction |
| game-framework | npc-simulation | F-13.19.10 | Social-Cue Player Search |
| game-framework | npc-simulation | F-13.19.11 | Quest and Story State NPC Awareness |
| game-framework | npc-simulation | F-13.19.12 | Player-Witnessed NPC Social Behaviors |
| game-framework | fog-of-war | F-13.20.1 | Fog of War Grid System |
| game-framework | fog-of-war | F-13.20.2 | Vision Source and Sight Radius |
| game-framework | fog-of-war | F-13.20.3 | Vision Modifier Volumes |
| game-framework | fog-of-war | F-13.20.4 | Fog of War Memory |
| game-framework | turn-based | F-13.21.1 | Tactical Grid System |
| game-framework | turn-based | F-13.21.2 | Turn Manager and Initiative |
| game-framework | turn-based | F-13.21.3 | Action Point Movement and Abilities |
| game-framework | turn-based | F-13.21.4 | Grid Cover and Overwatch |
| game-framework | turn-based | F-13.21.5 | Hit Probability and Combat Resolution |
| game-framework | racing | F-13.22.1 | Track and Checkpoint System |
| game-framework | racing | F-13.22.2 | Race Mode Framework |
| game-framework | racing | F-13.22.3a | Racing AI Navigation |
| game-framework | racing | F-13.22.3b | Rubber-Banding Difficulty |
| game-framework | racing | F-13.22.3c | AI Racing Behavior |
| game-framework | racing | F-13.22.4 | Drift Scoring and Boost System |
| game-framework | racing | F-13.22.5 | Ghost Replay and Leaderboards |
| game-framework | monetization | F-13.23.1 | Battle Pass and Season System |
| game-framework | monetization | F-13.23.2 | Daily and Weekly Challenge System |
| game-framework | monetization | F-13.23.3a | Platform Purchase Abstraction |
| game-framework | monetization | F-13.23.3b | Server-Side Receipt Validation |
| game-framework | monetization | F-13.23.3c | Premium Currency System |
| game-framework | monetization | F-13.23.3d | Purchase History and Refund Tracking |
| game-framework | monetization | F-13.23.4 | Daily Login Reward Calendar |
| game-framework | monetization | F-13.23.5a | Subscription State Verification |
| game-framework | monetization | F-13.23.5b | Subscription Benefit Application |
| game-framework | monetization | F-13.23.5c | Subscription Management UI |
| game-framework | monetization | F-13.23.5d | Subscription Gifting |
| game-framework | monetization | F-13.23.6a | Timed Game Trial |
| game-framework | monetization | F-13.23.6b | Free Weekend Events |
| game-framework | monetization | F-13.23.6c | Content Trial |
| game-framework | monetization | F-13.23.7 | DLC and Expansion Purchasing |
| game-framework | monetization | F-13.23.8 | Cosmetic Store and Virtual Currency |
| game-framework | monetization | F-13.23.9a | Deceptive UI Prevention |
| game-framework | monetization | F-13.23.9b | Minor-Targeted Ad Blocking |
| game-framework | monetization | F-13.23.9c | Dark Pattern Prevention |
| game-framework | monetization | F-13.23.9d | Frequency Cap Enforcement |
| game-framework | game-modes-misc | F-13.24.1 | Wave Spawning System |
| game-framework | game-modes-misc | F-13.24.2 | Tower Targeting and Upgrade System |
| game-framework | game-modes-misc | F-13.24.3 | Score and Combo System |
| game-framework | game-modes-misc | F-13.24.4a | Feedback Stack Asset and Triggering |
| game-framework | game-modes-misc | F-13.24.4b | Hit-Stop and Time Scale Effects |
| game-framework | game-modes-misc | F-13.24.4c | Feedback Entry Types Library |
| game-framework | game-modes-misc | F-13.24.5 | Fast Travel Network |
| game-framework | game-modes-misc | F-13.24.6 | Respawn and Graveyard System |
| game-framework | game-modes-misc | F-13.24.7a | Tutorial Step System |
| game-framework | game-modes-misc | F-13.24.7b | Tutorial Visual Overlays |
| game-framework | game-modes-misc | F-13.24.7c | Toast Notification System |
| game-framework | game-modes-misc | F-13.24.8a | Free Camera Controller |
| game-framework | game-modes-misc | F-13.24.8b | Photo Mode Visual Controls |
| game-framework | game-modes-misc | F-13.24.8c | Photo Capture and Gallery |
| game-framework | camera-system | F-13.25.1 | Virtual Camera Entity and Priority System |
| game-framework | camera-system | F-13.25.2 | Camera Brain and Output Controller |
| game-framework | camera-system | F-13.25.3 | Follow (Fixed Offset) |
| game-framework | camera-system | F-13.25.4 | Orbital Follow |
| game-framework | camera-system | F-13.25.5 | Third-Person Follow with Shoulder Offset |
| game-framework | camera-system | F-13.25.6 | Hard Lock to Target |
| game-framework | camera-system | F-13.25.7 | Position Composer (Adaptive Framing) |
| game-framework | camera-system | F-13.25.8 | Spline Dolly |
| game-framework | camera-system | F-13.25.9 | Rotation Composer (Adaptive Aim) |
| game-framework | camera-system | F-13.25.10 | Hard Look At |
| game-framework | camera-system | F-13.25.11 | Pan and Tilt (Input-Driven Rotation) |
| game-framework | camera-system | F-13.25.12 | Rotate with Follow Target |
| game-framework | camera-system | F-13.25.13 | Spring Arm Component |
| game-framework | camera-system | F-13.25.14 | Camera Deoccluder (Line-of-Sight Preservation) |
| game-framework | camera-system | F-13.25.15 | Camera Decollider (Geometry Penetration Prevention) |
| game-framework | camera-system | F-13.25.16 | Camera Blend System |
| game-framework | camera-system | F-13.25.17 | Camera Mixing (Weighted Multi-Camera Blend) |
| game-framework | camera-system | F-13.25.18 | Multi-Channel Perlin Noise Profiles |
| game-framework | camera-system | F-13.25.19 | Camera Impulse System |
| game-framework | camera-system | F-13.25.20 | Wave Oscillation Shake |
| game-framework | camera-system | F-13.25.21 | Composite Shake Patterns |
| game-framework | camera-system | F-13.25.22 | Sequencer-Driven Camera Shake |
| game-framework | camera-system | F-13.25.23 | State-Driven Camera Switching |
| game-framework | camera-system | F-13.25.24 | Clear Shot (Automatic Best-Camera Selection) |
| game-framework | camera-system | F-13.25.25 | Shot Quality Evaluator |
| game-framework | camera-system | F-13.25.26 | Camera Sequencer (Timed Camera Playlist) |
| game-framework | camera-system | F-13.25.27 | Target Group (Multi-Target Aggregation) |
| game-framework | camera-system | F-13.25.28 | Group Framing Extension |
| game-framework | camera-system | F-13.25.29 | Camera Confiner 2D |
| game-framework | camera-system | F-13.25.30 | Camera Confiner 3D |
| game-framework | camera-system | F-13.25.31 | Follow Zoom (Constant Screen-Size Framing) |
| game-framework | camera-system | F-13.25.32 | Auto Focus |
| game-framework | camera-system | F-13.25.33 | Third-Person Aim Extension |
| game-framework | camera-system | F-13.25.34 | FreeLook Modifier |
| game-framework | camera-system | F-13.25.35 | Recomposer (Timeline Composition Override) |
| game-framework | camera-system | F-13.25.36 | Camera Modifier Stack |
| game-framework | camera-system | F-13.25.37 | Camera Input Axis Controller |
| game-framework | camera-system | F-13.25.38 | Cine Camera Properties |
| game-framework | camera-system | F-13.25.39 | Camera Rig Presets (Dolly, Crane, Jib) |
| game-framework | camera-system | F-13.25.40 | Picture-in-Picture |
| game-framework | minigames | F-13.26.1 | Minigame Session and Sandbox Context |
| game-framework | minigames | F-13.26.2 | Minigame World Presentation |
| game-framework | minigames | F-13.26.3 | Minigame Lifecycle and Result Contract |
| game-framework | minigames | F-13.26.4 | Timing and Rhythm Minigames |
| game-framework | minigames | F-13.26.5a | Grid/Board Engine |
| game-framework | minigames | F-13.26.5b | Match Detection Algorithms |
| game-framework | minigames | F-13.26.5c | Board Minigame AI |
| game-framework | minigames | F-13.26.5d | Board Piece Animation and Cascading |
| game-framework | minigames | F-13.26.6 | Physics Toy Minigames |
| game-framework | minigames | F-13.26.7 | Multiplayer Minigame Sessions |
| game-framework | minigames | F-13.26.8 | Minigame Library and Discovery |
| game-framework | block-voxel | F-13.27.1 | Block Type Registry and Properties |
| game-framework | block-voxel | F-13.27.2 | Block Placement and Destruction |
| game-framework | block-voxel | F-13.27.3 | Chunk-Based Block Storage |
| game-framework | block-voxel | F-13.27.4 | Block Chunk Meshing |
| game-framework | block-voxel | F-13.27.5 | Block Light Propagation |
| game-framework | block-voxel | F-13.27.6a | Gravity Block Physics |
| game-framework | block-voxel | F-13.27.6b | Fluid Flow Simulation |
| game-framework | block-voxel | F-13.27.6c | Fluid-Block Interactions |
| game-framework | block-voxel | F-13.27.7a | Signal Source and Wire Blocks |
| game-framework | block-voxel | F-13.27.7b | Logic Gate Blocks |
| game-framework | block-voxel | F-13.27.7c | Mechanism Blocks |
| game-framework | block-voxel | F-13.27.7d | Circuit Evaluation and Budget |
| game-framework | block-voxel | F-13.27.8a | Block Terrain Generation |
| game-framework | block-voxel | F-13.27.8b | Block Biome System |
| game-framework | block-voxel | F-13.27.8c | Block Ore Placement |
| game-framework | block-voxel | F-13.27.8d | Block Structure Generation |
| game-framework | advertising | F-13.28.1 | Rewarded Video Ads |
| game-framework | advertising | F-13.28.2 | Interstitial Ads |
| game-framework | advertising | F-13.28.3 | Banner Ads |
| game-framework | advertising | F-13.28.4 | Ad Mediation and Network Abstraction |
| platform | window-display | F-14.1.1 | Window Creation and Lifecycle |
| platform | window-display | F-14.1.2 | Fullscreen, Borderless, and Windowed Modes |
| platform | window-display | F-14.1.3 | Display Enumeration and Multi-Monitor Support |
| platform | window-display | F-14.1.4 | DPI Awareness and Display Scaling |
| platform | window-display | F-14.1.5 | VSync and Refresh Rate Management |
| platform | window-display | F-14.1.6 | HDR Output and Tone Mapping Handoff |
| platform | os-integration | F-14.2.1 | Clipboard Access |
| platform | os-integration | F-14.2.2 | Native File Dialogs |
| platform | os-integration | F-14.2.3 | System Notifications and Tray Icons |
| platform | os-integration | F-14.2.4 | Drag and Drop |
| platform | os-integration | F-14.2.5 | Platform Keyboard Layouts and Dead Keys |
| platform | os-integration | F-14.2.6 | Input Method Editor (IME) for CJK |
| platform | threading-async | F-14.3.1 | Work-Stealing Thread Pool |
| platform | threading-async | F-14.3.2 | Thread Affinity and Priority |
| platform | threading-async | F-14.3.3 | Task Graph Job System |
| platform | threading-async | F-14.3.4 | Fiber and Stackful Coroutine Support |
| platform | threading-async | F-14.3.5 | Platform Async I/O Integration |
| platform | crash-reporting | F-14.4.1 | Crash Handler and Minidump Generation |
| platform | crash-reporting | F-14.4.2 | Symbol Upload and Server-Side Symbolication |
| platform | crash-reporting | F-14.4.3 | Crash Aggregation and Alerting |
| platform | crash-reporting | F-14.4.4 | Structured Logging with Severity and Channels |
| platform | crash-reporting | F-14.4.5 | Performance Counters and Telemetry Hooks |
| platform | crash-reporting | F-14.4.6 | GPU Crash Breadcrumbs |
| platform | platform-services | F-14.5.1 | Cross-Platform Achievement System |
| platform | platform-services | F-14.5.2 | Leaderboards |
| platform | platform-services | F-14.5.3 | Rich Presence |
| platform | platform-services | F-14.5.4 | Platform Voice and Party Integration |
| platform | platform-services | F-14.5.5 | Platform Cloud Storage |
| platform | platform-services | F-14.5.6 | Entitlements, DLC, and Subscription Verification |
| platform | platform-services | F-14.5.7 | Console Certification Compliance |
| platform | filesystem | F-14.6.1 | Async File Open, Read, and Write |
| platform | filesystem | F-14.6.2 | Async File Create and Delete |
| platform | filesystem | F-14.6.3 | Async File Metadata and Stat |
| platform | filesystem | F-14.6.4 | Async Directory Enumeration |
| platform | filesystem | F-14.6.5 | Directory Change Notifications |
| platform | filesystem | F-14.6.6 | File Content Change Detection |
| platform | filesystem | F-14.6.7 | Canonical Path Resolution |
| tools-editor | editor-framework | F-15.1.1 | Dockable Panel Layout |
| tools-editor | editor-framework | F-15.1.2 | Multi-Viewport |
| tools-editor | editor-framework | F-15.1.3 | Undo/Redo System (Command Pattern) |
| tools-editor | editor-framework | F-15.1.4 | Selection System |
| tools-editor | editor-framework | F-15.1.5 | Transform Gizmos |
| tools-editor | editor-framework | F-15.1.6 | Bounds and Measurement Gizmos |
| tools-editor | editor-framework | F-15.1.7 | Editor Preferences and Configuration |
| tools-editor | editor-framework | F-15.1.8 | Editor Extensions and Plugin API |
| tools-editor | editor-framework | F-15.1.9 | VR Editor Mode |
| tools-editor | level-editor | F-15.2.1 | Entity Placement and Snapping |
| tools-editor | level-editor | F-15.2.2 | Prefab System with Nested Prefabs |
| tools-editor | level-editor | F-15.2.3 | Prefab Instance Overrides |
| tools-editor | level-editor | F-15.2.4 | Brush and CSG Tools |
| tools-editor | level-editor | F-15.2.5 | Spline Editing |
| tools-editor | level-editor | F-15.2.6 | Landscape Painting |
| tools-editor | level-editor | F-15.2.7 | Foliage Painting |
| tools-editor | material-editor | F-15.3.1 | Node-Based Material Graph |
| tools-editor | material-editor | F-15.3.2 | Material Functions and Subgraphs |
| tools-editor | material-editor | F-15.3.3 | Live Material Preview |
| tools-editor | material-editor | F-15.3.4 | Shader Parameter Tweaking |
| tools-editor | material-editor | F-15.3.5 | Material Instances |
| tools-editor | material-editor | F-15.3.6 | Material Library and Browser |
| tools-editor | animation-editor | F-15.4.1 | Animation Timeline |
| tools-editor | animation-editor | F-15.4.2 | Curve Editor |
| tools-editor | animation-editor | F-15.4.3 | Skeleton Viewer |
| tools-editor | animation-editor | F-15.4.4 | Animation Preview |
| tools-editor | animation-editor | F-15.4.5 | Blend Space Editor |
| tools-editor | animation-editor | F-15.4.6 | State Machine Editor |
| tools-editor | animation-editor | F-15.4.7 | Retargeting Setup |
| tools-editor | profiling-tools | F-15.5.1 | Frame Profiler (CPU Timeline) |
| tools-editor | profiling-tools | F-15.5.2 | GPU Profiler (Pass Timing and Occupancy) |
| tools-editor | profiling-tools | F-15.5.3 | Memory Profiler (Allocation Tracking) |
| tools-editor | profiling-tools | F-15.5.4 | Leak Detection |
| tools-editor | profiling-tools | F-15.5.5 | Network Profiler (Bandwidth and Packet Inspector) |
| tools-editor | profiling-tools | F-15.5.6 | Stat Overlays |
| tools-editor | profiling-tools | F-15.5.7 | Remote Profiling |
| tools-editor | world-building | F-15.6.1 | Terrain Sculpting Brushes |
| tools-editor | world-building | F-15.6.2 | Terrain Erosion |
| tools-editor | world-building | F-15.6.3 | Terrain Material Painting |
| tools-editor | world-building | F-15.6.4 | Water Body Placement |
| tools-editor | world-building | F-15.6.5 | Vegetation Painting with Density Rules |
| tools-editor | world-building | F-15.6.6 | Lighting Setup (Light Probes and Reflection Probes) |
| tools-editor | world-building | F-15.6.7 | Navmesh Preview |
| tools-editor | world-building | F-15.6.8 | World Partition Visualization |
| tools-editor | ai-governance | F-15.7.1 | AI Content Provenance Tagging |
| tools-editor | ai-governance | F-15.7.2 | Human Modification Tracking |
| tools-editor | ai-governance | F-15.7.3 | Generative AI Feature Toggle |
| tools-editor | ai-governance | F-15.7.4 | AI Assistance Toggle |
| tools-editor | ai-governance | F-15.7.5 | Enterprise Remote Administration |
| tools-editor | ai-governance | F-15.7.6 | AI Content Audit Trail |
| tools-editor | ai-governance | F-15.7.7 | AI Content Review Workflow |
| tools-editor | ai-governance | F-15.7.8 | Provenance Metadata in Packaged Builds |
| tools-editor | logic-graph | F-15.8.1 | Universal Logic Graph Runtime |
| tools-editor | logic-graph | F-15.8.2 | Static Type System |
| tools-editor | logic-graph | F-15.8.3 | Strict Validation Before Use |
| tools-editor | logic-graph | F-15.8.4 | Gameplay Logic Graphs |
| tools-editor | logic-graph | F-15.8.5a | Shader Graph Core |
| tools-editor | logic-graph | F-15.8.5b | Shader Graph to HLSL Compilation |
| tools-editor | logic-graph | F-15.8.5c | Material Graph Variant |
| tools-editor | logic-graph | F-15.8.6 | Render Graph Configuration |
| tools-editor | logic-graph | F-15.8.7 | Animation Logic Graphs |
| tools-editor | logic-graph | F-15.8.8 | Audio Logic Graphs |
| tools-editor | logic-graph | F-15.8.9 | Custom Tool Graphs |
| tools-editor | logic-graph | F-15.8.10 | Graph Node Library |
| tools-editor | logic-graph | F-15.8.11 | Graph Debugging |
| tools-editor | logic-graph | F-15.8.12 | Graph Compilation and Optimization |
| tools-editor | logic-graph | F-15.8.13 | Graph Diffing and Merge |
| tools-editor | logic-graph | F-15.8.14 | Graph Search and Refactoring |
| tools-editor | ai-assistant | F-15.9.1a | Speech-to-Text Pipeline |
| tools-editor | ai-assistant | F-15.9.1b | Voice Command Interpretation |
| tools-editor | ai-assistant | F-15.9.1c | Voice Activation Modes |
| tools-editor | ai-assistant | F-15.9.2 | AI Assistant Tool Interface |
| tools-editor | ai-assistant | F-15.9.3 | Visual and Graphical Tool Access |
| tools-editor | ai-assistant | F-15.9.4 | Keyboard Shortcut Recommendations |
| tools-editor | ai-assistant | F-15.9.5 | Contextual Action Reminders |
| tools-editor | ai-assistant | F-15.9.6a | Headless Editor API Layer |
| tools-editor | ai-assistant | F-15.9.6b | Agent Orchestration |
| tools-editor | ai-assistant | F-15.9.6c | CI/CD Agent Integration |
| tools-editor | ai-assistant | F-15.9.7 | Screenshot and Screen Recording Capture |
| tools-editor | ai-assistant | F-15.9.8 | UI Accessibility Tree Analysis |
| tools-editor | ai-assistant | F-15.9.9 | Multi-Modal Understanding |
| tools-editor | ai-assistant | F-15.9.10 | AI Assistance Administration |
| tools-editor | version-control | F-15.10.1 | Native Git Integration |
| tools-editor | version-control | F-15.10.2 | Git LFS Management |
| tools-editor | version-control | F-15.10.3 | Asset-Aware Merge Driver |
| tools-editor | version-control | F-15.10.4 | Branch-Per-Feature Workflow |
| tools-editor | version-control | F-15.10.5 | Collaborative Presence |
| tools-editor | version-control | F-15.10.6 | Partial Clone and Sparse Checkout |
| tools-editor | version-control | F-15.10.7 | Shelving and Local Stash |
| tools-editor | version-control | F-15.10.8 | Multi-Provider Git Hosting Support |
| tools-editor | shared-cache | F-15.11.1 | Centralized Compiled Asset Cache |
| tools-editor | shared-cache | F-15.11.2 | Shader Compilation Cache |
| tools-editor | shared-cache | F-15.11.3 | Logic Graph Compilation Cache |
| tools-editor | shared-cache | F-15.11.4 | New Developer Onboarding Acceleration |
| tools-editor | shared-cache | F-15.11.5 | Cache Invalidation and Garbage Collection |
| tools-editor | shared-cache | F-15.11.6 | Cache Transport and Storage |
| tools-editor | shared-cache | F-15.11.7 | CI/CD Cache Population |
| tools-editor | shared-cache | F-15.11.8 | Cache Hit Metrics and Monitoring |
| tools-editor | remote-editing | F-15.12.1 | Remote Desktop Optimized Rendering |
| tools-editor | remote-editing | F-15.12.2 | Remote Editor Protocol |
| tools-editor | remote-editing | F-15.12.3 | CRDT-Based Real-Time Collaborative Editing |
| tools-editor | remote-editing | F-15.12.4 | Remote GPU Server Support |
| tools-editor | remote-editing | F-15.12.5 | Session Handoff and Persistence |
| tools-editor | remote-editing | F-15.12.6 | Bandwidth Adaptation and Quality Tiers |
| tools-editor | remote-editing | F-15.12.7 | Collaboration Cloud Service |
| tools-editor | remote-editing | F-15.12.8 | CRDT Document Model for Engine Assets |
| tools-editor | remote-editing | F-15.12.9 | Collaboration Access Control and Permissions |
| tools-editor | remote-editing | F-15.12.10 | Integrated Voice and Text Chat |
| tools-editor | remote-editing | F-15.12.11 | Work Groups and Isolated Workspaces |
| tools-editor | remote-editing | F-15.12.12 | AI Agent Collaboration |
| tools-editor | remote-editing | F-15.12.13 | Asset and Scene Comments |
| tools-editor | remote-editing | F-15.12.14 | Pull Request Review in Editor |
| tools-editor | localization-editor | F-15.13.1 | String Table Editor |
| tools-editor | localization-editor | F-15.13.2 | Localization Preview and Validation |
| tools-editor | localization-editor | F-15.13.3 | Translation Workflow Integration |
| tools-editor | deployment | F-15.14.1 | Platform Build Packaging |
| tools-editor | deployment | F-15.14.2 | Deploy-to-Device Workflow |
| tools-editor | deployment | F-15.14.3 | Certification Compliance Checker |
| tools-editor | deployment | F-15.14.4 | Code Signing Pipeline |
| tools-editor | deployment | F-15.14.5 | Platform-Specific Installers and Distributions |
| tools-editor | deployment | F-15.14.6 | Asset Bundle and DLC Packaging |
| tools-editor | deployment | F-15.14.7 | Delta Patching System |
| tools-editor | deployment | F-15.14.8 | Store Distribution Pipeline |
| tools-editor | launcher | F-15.15.1 | Engine Version Management |
| tools-editor | launcher | F-15.15.2 | Automatic Project Upgrades |
| tools-editor | launcher | F-15.15.3 | Project Browser and Creation Wizard |
| tools-editor | launcher | F-15.15.4 | Project File Format and Association |
| tools-editor | launcher | F-15.15.5 | Cross-Game Preferences and Account Management |
| tools-editor | launcher | F-15.15.6 | Collaboration Setup Wizard |
| tools-editor | mod-support | F-15.16.1 | Mod SDK and Authoring Tools |
| tools-editor | mod-support | F-15.16.2 | Developer-Defined Mod Constraints |
| tools-editor | mod-support | F-15.16.3 | Mod Packaging and Distribution Format |
| tools-editor | mod-support | F-15.16.4 | Mod Loading and Sandboxing |
| tools-editor | mod-support | F-15.16.5 | Mod Workshop Integration |
| tools-editor | mod-support | F-15.16.6 | Mod Moderation and Review |
| tools-editor | asset-store | F-15.17.1 | Integrated Asset Store Browser |
| tools-editor | asset-store | F-15.17.2 | One-Click Asset Import and Project Integration |
| tools-editor | asset-store | F-15.17.3 | Asset Ratings, Reviews, and Curation |
| tools-editor | asset-store | F-15.17.4 | Publisher Account and Dashboard |
| tools-editor | asset-store | F-15.17.5 | Automated Compatibility Testing |
| tools-editor | asset-store | F-15.17.6 | Revenue Sharing and Payout |
| tools-editor | asset-store | F-15.17.7 | Asset Type Support |
| tools-editor | asset-store | F-15.17.8 | License Management and DRM |
| tools-editor | server-infrastructure | F-15.18.1 | AWS CDK Deployment Stacks |
| tools-editor | server-infrastructure | F-15.18.2 | Live Collaboration Server |
| tools-editor | server-infrastructure | F-15.18.3 | Git and Git LFS Hosting with Locking |
| tools-editor | server-infrastructure | F-15.18.4 | Asset and Shader Compilation Server |
| tools-editor | server-infrastructure | F-15.18.5 | Signing and Distribution Server |
| tools-editor | server-infrastructure | F-15.18.6 | Continuous Deployment Pipeline |
| tools-editor | server-infrastructure | F-15.18.7 | Test Runner Infrastructure |
| tools-editor | server-infrastructure | F-15.18.8 | Shared Cache and Database Services |
| tools-editor | server-infrastructure | F-15.18.9 | Backup and Disaster Recovery |
| tools-editor | server-infrastructure | F-15.18.10 | Enterprise Security Configuration |
| tools-editor | documentation | F-15.19.1 | Auto-Generated API Reference |
| tools-editor | documentation | F-15.19.2 | Logic Graph Node Documentation |
| tools-editor | documentation | F-15.19.3 | Interactive In-Editor Tutorials |
| tools-editor | documentation | F-15.19.4 | Video Tutorial Integration |
| tools-editor | documentation | F-15.19.5 | Contextual Help and Tooltip System |
| tools-editor | documentation | F-15.19.6 | Sample Projects and Template Library |
| tools-editor | documentation | F-15.19.7 | Inline Code Examples in Documentation |
