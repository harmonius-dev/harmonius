# R-2.11 -- Stylized and Gameplay Effects Requirements

## Outlines and Highlights

1. **R-2.11.1** — The engine **SHALL** render per-entity outlines using screen-space edge detection
   and jump-flood algorithms with configurable color, width, style, and priority, falling back to
   Sobel on devices without compute.
   - **Rationale:** Multi-technique outlines provide selection feedback across all hardware tiers.
   - **Verification:** Select an entity. Verify outline appears with configured color and width.
     Verify Sobel fallback on a compute-free device. Verify priority ordering when two outlines
     overlap.

2. **R-2.11.2** — The engine **SHALL** render emissive highlight overlays with pulsing, fresnel rim,
   and inner glow modes driven by per-entity ECS components, with stencil-masked Gaussian blur
   compositing.
   - **Rationale:** Gameplay-driven highlights make interactive objects visually prominent.
   - **Verification:** Set a HighlightState component. Verify the entity glows. Verify pulsing
     modulates intensity. Verify flat-color fallback on mobile.

## Toon Shading

3. **R-2.11.3** — The engine **SHALL** provide a configurable cel-shading pipeline with
   artist-controlled band counts, ramp textures, shaped specular highlights, and rim lighting,
   scaling from 2-3 bands on mobile to full hatching on desktop.
   - **Rationale:** Cel shading enables stylized games with full creative control over lighting
     bands.
   - **Verification:** Set 3 bands with a ramp texture. Verify discrete light/dark transitions.
     Verify shaped specular. Verify hatching pattern in shadow regions on desktop.

## Gameplay Visibility

4. **R-2.11.4** — The engine **SHALL** automatically fade, dissolve, or remove occluding geometry
   above the player via volume-based, ray-based, or layer-based modes with configurable dissolve
   patterns and speed.
   - **Rationale:** Occlusion removal is essential for isometric and top-down games.
   - **Verification:** Move the camera beneath a roof. Verify the roof fades. Verify dither dissolve
     on mobile. Verify smooth alpha on desktop.

5. **R-2.11.5** — The engine **SHALL** render occluded entities as colored silhouettes through walls
   using a separate depth comparison, with priority levels controlling which entities show through.
   - **Rationale:** X-ray visibility enables ally and enemy tracking through cover in team games.
   - **Verification:** Place an entity behind a wall with XRayVisible component. Verify silhouette
     visible. Verify priority controls which entities appear.
