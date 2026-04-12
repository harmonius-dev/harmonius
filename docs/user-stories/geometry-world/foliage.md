# User Stories -- 3.3 Foliage & Vegetation

## Instanced Foliage Rendering

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.1.1 | environment artist (P-8)   |
| US-3.3.1.2 | engine developer (P-26)    |
| US-3.3.1.3 | level designer (P-6)       |
| US-3.3.1.4 | technical artist (P-13)    |
| US-3.3.1.5 | engine developer (P-26)    |

1. **US-3.3.1.1** -- **As a** environment artist (P-8), **I want** foliage rendered via GPU-driven
   instancing with compute-shader culling, **so that** millions of vegetation instances display
   across forests without CPU bottlenecks.
2. **US-3.3.1.2** -- **As a** engine developer (P-26), **I want** instance transforms stored in a
   GPU buffer managed by a hierarchical spatial tree, **so that** culling operates entirely on the
   GPU without CPU readback.
3. **US-3.3.1.3** -- **As a** level designer (P-6), **I want** foliage instances culled by frustum,
   distance, and occlusion in a single compute pass, **so that** only visible vegetation consumes
   rendering budget.
4. **US-3.3.1.4** -- **As a** technical artist (P-13), **I want** per-instance random seeds and LOD
   indices available in the GPU buffer, **so that** each instance can vary visually without
   additional draw calls.
5. **US-3.3.1.5** -- **As a** engine developer (P-26), **I want** culled survivors compacted into
   indirect draw arguments, **so that** the CPU issues a fixed number of draw calls regardless of
   visible instance count.

## Procedural Placement

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.2.1 | environment artist (P-8)   |
| US-3.3.2.2 | level designer (P-6)       |
| US-3.3.2.3 | technical artist (P-13)    |
| US-3.3.2.4 | engine developer (P-26)    |

1. **US-3.3.2.1** -- **As a** environment artist (P-8), **I want** to define density maps and biome
   rules that populate terrain with foliage at runtime, **so that** I control vegetation
   distribution without placing instances manually.
2. **US-3.3.2.2** -- **As a** level designer (P-6), **I want** slope and altitude constraints to
   filter placement automatically, **so that** foliage only appears on appropriate terrain surfaces.
3. **US-3.3.2.3** -- **As a** technical artist (P-13), **I want** placement evaluation to occur per
   visible tile on the GPU, **so that** no per-instance data is stored on disk for open worlds.
4. **US-3.3.2.4** -- **As a** engine developer (P-26), **I want** procedural placement to feed
   directly into the instanced rendering pipeline, **so that** generated instances avoid a CPU
   staging step.

## Foliage LOD

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.3.1 | environment artist (P-8)   |
| US-3.3.3.2 | technical artist (P-13)    |
| US-3.3.3.3 | level designer (P-6)       |
| US-3.3.3.4 | engine developer (P-26)    |

1. **US-3.3.3.1** -- **As a** environment artist (P-8), **I want** distant foliage to transition
   through full mesh, simplified mesh, and impostor LODs, **so that** dense forests remain
   performant at distance.
2. **US-3.3.3.2** -- **As a** technical artist (P-13), **I want** impostors pre-rendered as sprite
   sheets with full PBR attributes, **so that** distant vegetation retains correct lighting
   response.
3. **US-3.3.3.3** -- **As a** level designer (P-6), **I want** crossfade dithering over a
   configurable range at LOD transitions, **so that** there is no visible pop when LOD levels
   switch.
4. **US-3.3.3.4** -- **As a** engine developer (P-26), **I want** impostor atlas resolution
   configurable per platform tier, **so that** mobile targets use less VRAM for distant vegetation.

## Wind Animation

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.4.1 | environment artist (P-8)   |
| US-3.3.4.2 | technical artist (P-13)    |
| US-3.3.4.3 | level designer (P-6)       |
| US-3.3.4.4 | engine developer (P-26)    |

1. **US-3.3.4.1** -- **As a** environment artist (P-8), **I want** foliage to sample a shared wind
   field texture with three-layer animation (trunk sway, branch oscillation, leaf flutter),
   **so that** vegetation responds naturally to wind.
2. **US-3.3.4.2** -- **As a** technical artist (P-13), **I want** per-species wind response curves
   that I can tune, **so that** different tree and plant types react distinctly to the same wind
   conditions.
3. **US-3.3.4.3** -- **As a** level designer (P-6), **I want** wind gusts to propagate spatially as
   wave fronts, **so that** wind feels directional across the world rather than uniform.
4. **US-3.3.4.4** -- **As a** engine developer (P-26), **I want** wind animation computed entirely
   in the vertex shader, **so that** no CPU-side per-instance animation state is required.

## Foliage Collision / Interaction

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.5.1 | level designer (P-6)       |
| US-3.3.5.2 | environment artist (P-8)   |
| US-3.3.5.3 | engine developer (P-26)    |
| US-3.3.5.4 | technical artist (P-13)    |

1. **US-3.3.5.1** -- **As a** level designer (P-6), **I want** grass to part around characters and
   bushes to bend from impacts, **so that** the world feels physically reactive to movement.
2. **US-3.3.5.2** -- **As a** environment artist (P-8), **I want** displacement to decay over
   configurable time constants, **so that** bent foliage returns to rest naturally.
3. **US-3.3.5.3** -- **As a** engine developer (P-26), **I want** interaction impulses written to an
   interaction buffer read by the vertex shader, **so that** displacement requires no per-instance
   CPU state.
4. **US-3.3.5.4** -- **As a** technical artist (P-13), **I want** interaction range and buffer
   resolution configurable per platform tier, **so that** mobile targets limit displacement cost.

## Grass Rendering

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.6.1 | environment artist (P-8)   |
| US-3.3.6.2 | level designer (P-6)       |
| US-3.3.6.3 | technical artist (P-13)    |
| US-3.3.6.4 | engine developer (P-26)    |

1. **US-3.3.6.1** -- **As a** environment artist (P-8), **I want** dense grass fields rendered as
   procedurally generated blade geometry, **so that** meadows and savannahs look lush without
   hand-placed assets.
2. **US-3.3.6.2** -- **As a** level designer (P-6), **I want** grass density to scale with distance
   and transition to a ground-cover texture at far range, **so that** performance stays within
   budget.
3. **US-3.3.6.3** -- **As a** technical artist (P-13), **I want** blade shape, height, curvature,
   and color driven by terrain material layers and noise, **so that** grass matches the underlying
   biome.
4. **US-3.3.6.4** -- **As a** engine developer (P-26), **I want** blade geometry generated in a
   compute or mesh shader, **so that** no CPU vertex data is required for grass.

## Tree Systems

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.7.1 | environment artist (P-8)   |
| US-3.3.7.2 | technical artist (P-13)    |
| US-3.3.7.3 | level designer (P-6)       |
| US-3.3.7.4 | engine developer (P-26)    |

1. **US-3.3.7.1** -- **As a** environment artist (P-8), **I want** a dedicated tree pipeline with
   separate trunk, branch, and canopy shading models, **so that** trees render with correct bark and
   leaf materials.
2. **US-3.3.7.2** -- **As a** technical artist (P-13), **I want** leaf canopies to use two-sided
   foliage shading with subsurface light transmission, **so that** sunlight filters through leaves
   realistically.
3. **US-3.3.7.3** -- **As a** level designer (P-6), **I want** each tree species to define a wind
   skeleton for sway animation, **so that** different trees move distinctly in wind.
4. **US-3.3.7.4** -- **As a** engine developer (P-26), **I want** artist-authored and procedurally
   generated trees to integrate seamlessly with the LOD pipeline, **so that** both tree sources
   share the same rendering path.

## Meshlet Foliage Path

| ID         | Persona                    |
|------------|----------------------------|
| US-3.3.8.1 | environment artist (P-8)   |
| US-3.3.9.1 | engine developer (P-26)    |

1. **US-3.3.8.1** -- **As an** environment artist (P-8), **I want** trees and bushes rendered
   through the meshlet cluster DAG with voxelized distant clusters, **so that** forests maintain
   detail and volume at any distance without per-asset LOD chains.
2. **US-3.3.9.1** -- **As an** engine developer (P-26), **I want** meshlet foliage wind driven by
   skeletal bone chains rather than vertex World Position Offset, **so that** meshlet cluster AABBs
   stay stable for correct cluster culling.
