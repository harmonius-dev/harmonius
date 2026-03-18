# R-11.2 — Decal Requirements

## R-11.2.1–R-11.2.6 Decals

| ID       | Derived From                             |
|----------|------------------------------------------|
| R-11.2.1 | [F-11.2.1](../../features/vfx/decals.md) |
| R-11.2.2 | [F-11.2.2](../../features/vfx/decals.md) |
| R-11.2.3 | [F-11.2.3](../../features/vfx/decals.md) |
| R-11.2.4 | [F-11.2.4](../../features/vfx/decals.md) |
| R-11.2.5 | [F-11.2.5](../../features/vfx/decals.md) |
| R-11.2.6 | [F-11.2.6](../../features/vfx/decals.md) |

1. **R-11.2.1** — The engine **SHALL** render screen-space deferred decals by rasterizing an
   oriented bounding box against G-buffer depth, independently modifying albedo, normal, roughness,
   and metallic channels with per-axis fade, angle-based attenuation, and triplanar projection for
   complex geometry intersections.
   - **Rationale:** Deferred decals provide flexible, mesh-independent surface detailing across
     terrain and architecture without modifying underlying geometry or materials.
   - **Verification:** Place a decal across a mesh/terrain boundary and verify seamless blending;
     confirm each G-buffer channel can be independently enabled or disabled per decal; rotate a
     decal to a grazing angle and verify angle-based attenuation prevents stretching; verify
     triplanar projection handles convex geometry intersections without seams.
2. **R-11.2.2** — The engine **SHALL** render geometry-based mesh decals projected onto static
   surfaces by clipping decal geometry to receiving triangles and storing overlay meshes with
   independent UVs and correct tangent-space normals.
   - **Rationale:** Mesh decals deliver sub-pixel accuracy for persistent markings such as murals
     and signage where tangent-space normal correctness is required.
   - **Verification:** Project a mesh decal onto a curved static mesh and verify the overlay
     geometry conforms to the surface with no z-fighting; confirm tangent-space normals produce
     correct lighting response; verify UVs are independent from the receiving mesh.
3. **R-11.2.3** — The engine **SHALL** pack decal textures into a runtime texture atlas, batch
   decals sharing an atlas page into a single indirect draw call, and manage atlas residency with
   LRU eviction under memory pressure.
   - **Rationale:** Atlas batching reduces texture bind and draw call overhead when hundreds of
     decals are active simultaneously during large-scale encounters.
   - **Verification:** Spawn 500 decals using 50 unique textures and verify they are packed into
     atlas pages; confirm draw call count is proportional to atlas page count, not individual decal
     count; exceed the atlas memory budget and verify LRU eviction removes the least-recently-used
     entries without visual corruption.
4. **R-11.2.4** — The engine **SHALL** resolve overlapping decals using a priority value and layer
   mask per decal, support alpha, multiply, and additive blend modes, provide time-based fade-in,
   sustain, and dissolve-out lifecycle with noise-threshold breakup, and reclaim the oldest
   low-priority decals first when the global pool is exhausted.
   - **Rationale:** Priority-based stacking and lifecycle management prevent visual conflicts
     between overlapping decals and bound memory usage under sustained gameplay.
   - **Verification:** Place two decals at different priorities on the same surface and verify the
     higher-priority decal overwrites the lower per-channel; test each blend mode and confirm
     correct compositing; spawn a decal and verify fade-in, sustain, and dissolve-out timing matches
     configuration; exhaust the pool and confirm the oldest low-priority decals are reclaimed first.
5. **R-11.2.5** — The engine **SHALL** spawn procedural damage decals from hit events with variation
   driven by weapon type, impact angle, and surface material, using velocity-directed projection and
   randomized atlas selection for blood splatter, persisting on surfaces and characters.
   - **Rationale:** Procedural damage decals provide immediate visual feedback for combat
     interactions and enhance the readability of melee and ranged encounters.
   - **Verification:** Trigger hit events with different weapon types and verify distinct decal
     variations spawn; change impact angle and confirm projection direction responds correctly;
     spawn multiple blood splatters and verify atlas selection avoids repeated patterns; confirm
     decals persist on both static surfaces and character meshes.
6. **R-11.2.6** — The engine **SHALL** spawn surface-aware deformation decals from character
   locomotion and vehicle movement, varying footprint shape, depth, and material response by terrain
   material layer, and rendering tire tracks as ribbon-style UV projections matching wheel contact
   patch width.
   - **Rationale:** Surface-reactive footprints and tire tracks ground characters and vehicles in
     the world by reflecting terrain material properties in locomotion marks.
   - **Verification:** Walk a character across mud, snow, and sand terrain layers and verify
     footprint shape and material response differ per surface type; drive a vehicle and confirm tire
     track width matches the wheel contact patch; verify ribbon UV projection produces continuous
     tracks along the vehicle path without gaps or distortion.
