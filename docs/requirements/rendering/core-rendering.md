# R-2.3 -- Core Rendering Requirements

## Lighting and Culling

1. **R-2.3.1** — The engine **SHALL** evaluate point, spot, and directional lights with
   physically-based attenuation, writing all types into a unified light buffer consumed by both
   forward and deferred rendering paths.
   - **Rationale:** A unified light buffer ensures consistent lighting regardless of the active
     pipeline.
   - **Verification:** Render a scene with all three light types using both paths. Verify
     pixel-identical output within floating-point tolerance.

2. **R-2.3.2** — The engine **SHALL** perform meshlet-level GPU frustum culling via a compute pass
   testing each AABB against the six camera frustum planes, excluding off- screen meshlets from the
   indirect draw buffer.
   - **Rationale:** GPU-side culling avoids CPU-GPU round- trips and scales to millions of meshlets.
   - **Verification:** Position the camera so 50% of meshlets are off-screen. Verify the indirect
     draw buffer excludes them. Compare against a CPU reference culler.

3. **R-2.3.3** — The engine **SHALL** perform meshlet-level normal cone culling rejecting fully
   backfacing meshlets before rasterization.
   - **Rationale:** Normal cone culling eliminates backfacing meshlets, reducing vertex processing
     and overdraw.
   - **Verification:** Render a sphere. Verify approximately 50% of meshlets are culled. Rotate 180
     degrees and verify the culled set inverts.

4. **R-2.3.4** — The engine **SHALL** implement two-phase HZB occlusion culling where phase 1 tests
   against the previous frame's HZB and phase 2 re-tests rejects against the current frame's HZB.
   - **Rationale:** Two-phase HZB prevents disocclusion artifacts when objects move or the camera
     translates.
   - **Verification:** Occlude 100 meshlets behind a wall. Move the camera to reveal them. Verify
     phase 2 recovers them in the same frame with no pop-in.

## Projection and Instancing

5. **R-2.3.5** — The engine **SHALL** support orthographic projection for top-down views, 2D
   rendering, and shadow map generation.
   - **Rationale:** Orthographic projection is required for 2D games, isometric views, and shadow
     cascades.
   - **Verification:** Render a scene with orthographic and perspective cameras. Verify orthographic
     output has no foreshortening. Verify shadow maps use orthographic.

6. **R-2.3.6** — The engine **SHALL** support reverse-Z perspective projection with optional
   infinite far plane for maximum depth precision at distance.
   - **Rationale:** Reverse-Z with infinite far plane maximizes depth precision for distant
     geometry.
   - **Verification:** Render objects at 1 m and 10 km. Verify no z-fighting. Verify depth clears to
     0 on all backends.

7. **R-2.3.7** — The engine **SHALL** compact surviving opaque meshlet instances by material into
   contiguous indirect draw buffers, dispatching via indirect calls.
   - **Rationale:** GPU-driven instancing eliminates per-draw CPU dispatch overhead at high draw
     counts.
   - **Verification:** Render 10,000 material instances. Verify fewer than 100 draw calls. Verify
     transparent objects render individually in sorted order.

## Render Targets and Dynamic Resolution

8. **R-2.3.8** — The engine **SHALL** support render-to- texture with automatic barrier insertion
   between write and read passes in the render graph.
   - **Rationale:** Automatic barriers prevent manual sync errors in multi-pass effect chains.
   - **Verification:** Chain two passes sharing a texture. Verify a barrier is inserted. Verify no
     rendering artifacts from missing synchronization.

9. **R-2.3.9** — The engine **SHALL** render static and dynamic cubemaps for environment maps,
   reflection probes, and IBL prefiltering with seamless edge stitching.
   - **Rationale:** Cubemaps provide environment reflections and image-based lighting data.
   - **Verification:** Render all six faces. Verify no visible seams at face edges. Verify dynamic
     face updates reflect scene changes.

10. **R-2.3.10** — The engine **SHALL** render the scene from arbitrary cameras into texture targets
    for mirrors, portals, and minimap rendering, respecting per-platform resolution limits.
    - **Rationale:** Scene capture enables gameplay elements like security cameras and portals.
    - **Verification:** Capture at quarter-res on mobile. Verify output matches the full scene at
      reduced resolution. Verify platform limits are enforced.

11. **R-2.3.11** — The engine **SHALL** dynamically scale internal render resolution between
    configurable min/max bounds using a GPU timing feedback loop to maintain a target frame budget.
    - **Rationale:** Dynamic resolution maintains frame rate during performance-intensive scenes.
    - **Verification:** Run a stress test. Verify resolution converges to a stable percentage
      without oscillation. Verify resolution stays within configured bounds.

## Material Effects

12. **R-2.3.12** — The engine **SHALL** provide screen-space and ray-traced subsurface scattering
    driven by per- material profiles with scatter radius and extinction.
    - **Rationale:** SSS is required for realistic skin, wax, marble, and other translucent
      materials.
    - **Verification:** Render skin with SSS on desktop. Compare against preintegrated LUT on
      mobile. Verify both produce acceptable translucency.

13. **R-2.3.13** — The engine **SHALL** support alpha-tested geometry with configurable threshold
    for vegetation, fences, and decals, including shadow map participation.
    - **Rationale:** Alpha cutouts enable detailed vegetation and fence silhouettes without dense
      geometry.
    - **Verification:** Render alpha-tested foliage in shadow maps. Verify shadows match the cutout
      silhouette. Verify distant vegetation uses opaque proxies on mobile.
