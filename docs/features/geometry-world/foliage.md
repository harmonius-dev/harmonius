# 3.3 — Foliage & Vegetation

## Instanced Foliage Rendering

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.1 | GPU-Driven Instanced Foliage | All foliage is rendered via GPU-driven hardware instancing. Instance transforms, per-instance random seeds, and LOD indices are stored in a large GPU buffer managed by a hierarchical spatial tree. A compute-shader culling pass performs frustum, distance, and occlusion tests on instance clusters, compacting survivors into indirect draw arguments. This supports millions of vegetation instances across MMO-scale forests with minimal CPU overhead. | R-3.3.1 | F-3.1.3 | Instance count caps scale per tier: mobile 50K-100K, Switch 200K, desktop 1M+. Culling cluster granularity coarsened on mobile to reduce dispatch cost. |

## Procedural Placement

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.2 | Density Map and Rule-Based Procedural Placement | A compute-shader-based procedural placement system populates terrain tiles with foliage instances at runtime. Placement is driven by density maps, biome classification, slope/altitude constraints, and artist-defined rule graphs. Evaluation occurs per visible tile and feeds results directly into the instanced rendering pipeline, eliminating the need to store per-instance data on disk for vast open worlds. | R-3.3.2 | F-3.2.1, F-3.3.1 | Density multiplier scales per tier: mobile 0.25x-0.5x, Switch 0.5x, desktop 1.0x. Placement evaluation radius reduced on mobile. |

## Foliage LOD

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.3 | Billboard and Impostor LOD | Distant foliage transitions through discrete LOD levels: full 3D mesh, simplified mesh, and finally a camera-facing billboard or impostor card. Impostors are pre-rendered sprite sheets capturing the asset from multiple angles with full PBR attributes. Crossfade dithering over a configurable screen-space range eliminates visible LOD pop. For dense MMO forests, impostor rendering reduces per-instance triangle count by orders of magnitude at distance. | R-3.3.3 | F-3.3.1 | Billboard/impostor transition distance shorter on mobile (earlier switch to impostors). Impostor atlas resolution halved on mobile. |

## Wind Animation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.4 | GPU Vertex Shader Wind Animation | Hierarchical wind deformation computed entirely in the vertex or mesh shader. Foliage vertex shaders sample wind velocity from the shared wind field texture generated from `WindSource` ECS entities (F-4.7.5) and apply procedural oscillation. Per-instance phase offsets drive three-layer animation: trunk sway (low frequency), branch oscillation (medium frequency), and leaf flutter (high frequency). Wind gusts propagate spatially as wave fronts across the world. The system supports artist-tunable per-species wind response curves. | R-3.3.4 | F-3.3.1, F-4.7.5 (Wind Field Texture) | Wind animation layers scale per tier: mobile trunk sway only (1 layer), Switch trunk + branch (2 layers), desktop all 3 layers. Wind field texture resolution halved on mobile. |

## Foliage Collision / Interaction

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.5 | Character-Vegetation Interaction | Plants react to character movement, projectiles, and environmental forces. Interaction impulses are written to a screen-space or world-space interaction buffer by gameplay systems. The foliage vertex shader reads this buffer to apply persistent displacement (e.g., grass parting around the player, bushes bending from impacts). Displacement decays over configurable time constants. In MMO scenarios, interaction is evaluated for nearby players only to bound cost. | R-3.3.5 | F-3.3.1, F-3.3.4 | Interaction buffer resolution and range reduced on mobile. Decay rate increased on mobile to limit active displacement samples. |

## Grass Rendering

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.6 | Procedural Grass Blade Rendering | Dense grass fields are rendered as procedurally generated blade geometry in a compute or mesh shader. Blade shape, height, curvature, and color variation are driven by terrain material layers and noise functions. Grass density scales with distance, transitioning to a ground-cover texture at far range. The system targets hundreds of thousands of visible blades per frame for lush meadow and savannah environments. | R-3.3.6 | F-3.2.1, F-3.3.2 | Visible blade count scales per tier: mobile 10K-30K, Switch 50K, desktop 200K+. Blade geometry simplified on mobile (fewer segments per blade). Grass render distance reduced on mobile. |

## Tree Systems

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-3.3.7 | Tree Rendering System | A dedicated tree rendering pipeline handling trunk, branch, and leaf canopy as separate submeshes with distinct shading models. Trunks and branches use standard PBR with bark detail maps. Leaf canopies use two-sided foliage shading with subsurface light transmission. Each tree species defines a wind skeleton for physically-based sway animation. The system supports artist-authored and procedurally generated trees with seamless LOD integration. | R-3.3.7 | F-3.3.1, F-3.3.3, F-3.3.4 | Subsurface leaf transmission disabled on mobile. Canopy mesh LOD more aggressive on mobile. Wind skeleton simplified to trunk-only on mobile. |
