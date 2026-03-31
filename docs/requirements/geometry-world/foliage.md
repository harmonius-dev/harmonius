# R-3.3 -- Foliage & Vegetation Requirements

## Instanced Rendering

1. **R-3.3.1** -- The engine **SHALL** render all foliage via GPU-driven hardware instancing with a
   compute-shader culling pass (frustum, distance, occlusion) that compacts survivors into indirect
   draw arguments.
   - **Rationale:** GPU-driven instancing with cluster culling supports millions of instances with
     minimal CPU overhead.
   - **Verification:** Render 1M foliage instances. Assert CPU draw-call count is constant. Assert
     occluded clusters produce zero rasterizer work.

## Procedural Placement

2. **R-3.3.2** -- The engine **SHALL** procedurally place foliage at runtime via GPU compute driven
   by density maps, biome classification, slope/altitude constraints, and artist-defined rule
   graphs.
   - **Rationale:** Runtime placement eliminates per-instance disk storage for vast open worlds.
   - **Verification:** Configure placement rules. Assert instances appear respecting all
     constraints. Assert no per-instance data is stored on disk.

## Billboard and Impostor LOD

3. **R-3.3.3** -- The engine **SHALL** transition foliage through full mesh, simplified mesh, and
   billboard/impostor LODs with crossfade dithering over a configurable range.
   - **Rationale:** Impostor rendering reduces per-instance triangle count by orders of magnitude at
     distance.
   - **Verification:** Observe foliage at increasing distance. Assert no visible LOD pop. Assert
     impostor rendering activates beyond the configured threshold.

## Wind Animation

4. **R-3.3.4** -- The engine **SHALL** compute hierarchical wind deformation in the vertex shader by
   sampling a shared wind field texture, applying trunk sway, branch oscillation, and leaf flutter
   with per-instance phase offsets.
   - **Rationale:** GPU-side wind animation avoids CPU per-instance state while producing natural
     motion.
   - **Verification:** Place foliage in a wind field. Assert three animation layers are visible.
     Assert per-species response curves produce distinct motion.

## Character-Vegetation Interaction

5. **R-3.3.5** -- The engine **SHALL** displace foliage in response to character movement and
   impacts via an interaction buffer read by the vertex shader, with displacement decaying over
   configurable time constants.
   - **Rationale:** Reactive vegetation increases immersion without CPU per-instance tracking.
   - **Verification:** Walk a character through grass. Assert grass parts around the character.
     Assert displacement decays to rest within the configured time.

## Procedural Grass

6. **R-3.3.6** -- The engine **SHALL** render dense grass fields as procedurally generated blade
   geometry in a compute or mesh shader, with blade shape driven by terrain material layers and
   noise functions.
   - **Rationale:** Procedural grass requires no CPU vertex data and scales density with distance.
   - **Verification:** Render a meadow. Assert blade count matches the per-tier target. Assert
     blades match the underlying terrain material.

## Tree Rendering

7. **R-3.3.7** -- The engine **SHALL** provide a dedicated tree rendering pipeline with separate
   trunk, branch, and canopy shading, two-sided leaf transmission, and per-species wind skeletons.
   - **Rationale:** Trees require specialized shading for bark, leaf transparency, and
     physically-based sway.
   - **Verification:** Render a tree with subsurface leaf transmission. Assert backlit leaves
     transmit light. Assert wind skeleton drives distinct per-species sway.
