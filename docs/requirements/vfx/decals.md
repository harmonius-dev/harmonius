# R-11.2 -- Decal Requirements

## Projection and Rendering

1. **R-11.2.1** — The engine **SHALL** render screen-space deferred decals by rasterizing an
   oriented bounding box against G-buffer depth, independently modifying albedo, normal, roughness,
   and metallic channels with per-axis fade, angle-based attenuation, and triplanar projection.
   - **Rationale:** Deferred decals provide flexible mesh-independent surface detailing across
     terrain and architecture without modifying underlying geometry.
   - **Verification:** Place a decal across a mesh/terrain boundary and verify seamless blending.
     Confirm each G-buffer channel is independently controllable. Verify triplanar handles convex
     geometry without seams.

2. **R-11.2.2** — The engine **SHALL** render geometry-based mesh decals projected onto static
   surfaces by clipping to receiving triangles, storing overlay meshes with independent UVs and
   correct tangent-space normals.
   - **Rationale:** Mesh decals deliver sub-pixel accuracy for persistent markings where
     tangent-space correctness matters.
   - **Verification:** Project onto a curved mesh and verify surface conformance with no z-fighting.
     Confirm tangent-space normals produce correct lighting.

## Management and Performance

3. **R-11.2.3** — The engine **SHALL** pack decal textures into a runtime atlas, batch decals
   sharing a page into a single indirect draw, and manage residency with LRU eviction.
   - **Rationale:** Atlas batching reduces texture bind and draw call overhead when hundreds of
     decals are active.
   - **Verification:** Spawn 500 decals with 50 textures and verify atlas packing. Confirm draw
     calls proportional to pages. Exceed memory and verify LRU eviction.

4. **R-11.2.4** — The engine **SHALL** resolve overlapping decals using per-decal priority and layer
   mask, support alpha/multiply/additive blend modes, provide fade-in/sustain/ dissolve-out
   lifecycle with noise breakup, and reclaim oldest low-priority decals first when the pool is
   exhausted.
   - **Rationale:** Priority stacking and lifecycle management prevent visual conflicts and bound
     memory under sustained gameplay.
   - **Verification:** Place two decals at different priorities and verify correct overwrite. Test
     each blend mode. Verify lifecycle timing. Exhaust pool and confirm reclamation.

## Gameplay Decals

5. **R-11.2.5** — The engine **SHALL** spawn procedural damage decals from hit events with variation
   by weapon type, impact angle, and surface material, using velocity-directed projection and
   randomized atlas selection, persisting on surfaces and characters.
   - **Rationale:** Procedural damage decals provide immediate visual feedback and enhance combat
     readability.
   - **Verification:** Trigger hits with different weapons and verify distinct variations. Change
     angle and confirm direction response. Verify atlas avoids repetition.

6. **R-11.2.6** — The engine **SHALL** spawn surface-aware deformation decals from locomotion and
   vehicles, varying shape, depth, and response by terrain material, rendering tire tracks as ribbon
   UV projections matching wheel width.
   - **Rationale:** Surface-reactive footprints and tire tracks ground characters and vehicles in
     the world.
   - **Verification:** Walk across mud, snow, and sand and verify distinct responses. Drive a
     vehicle and confirm track width matches wheel patch. Verify ribbon continuity.
