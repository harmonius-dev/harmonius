# R-10.4 — UI Rendering Requirements

## Batching and Geometry

1. **R-10.4.1** — The engine **SHALL** batch UI textured quads sharing the same atlas page and blend
   state into indirect draw calls, packing per-quad instance data into a GPU buffer.
   - **Rationale:** Merging same-atlas, same-blend quads minimizes draw call overhead for complex
     UIs.
   - **Verification:** Render 500 widgets across 4 atlas pages and 2 blend states. Assert draw calls
     equal the number of distinct (page, state) combinations.

2. **R-10.4.2** — The engine **SHALL** render text glyphs using MSDF textures with subpixel
   positioning, configurable hinting, and per-glyph outline/shadow, sustaining 5000+ visible glyphs
   per frame.
   - **Rationale:** MSDF text remains sharp at arbitrary scale without regenerating bitmaps.
   - **Verification:** Render 5000 glyphs at 100%, 200%, 300% scale. Assert sharp edges via image
     comparison. Assert text pass under 4 ms.

3. **R-10.4.3** — The engine **SHALL** provide GPU-accelerated vector path rendering with filled and
   stroked Bezier curves, arcs, and gradients, via compute tessellation into coverage masks or SDFs.
   - **Rationale:** Resolution-independent vector paths eliminate bitmap assets for shapes.
   - **Verification:** Render cooldown sweep, stroked Bezier, and conical gradient at 100% and 300%.
     Assert no aliasing or tessellation artifacts.

## Atlas and Texture

4. **R-10.4.4** — The engine **SHALL** pack UI textures into runtime-managed atlases with nine-slice
   rendering and incremental atlas page rebuilds without full-repack stalls.
   - **Rationale:** Atlas packing maximizes batching; incremental rebuilds prevent frame hitches.
   - **Verification:** Stream 20 new textures. Assert incremental repack under 2 ms. Assert
     nine-slice corners undistorted.

5. **R-10.4.5** — The engine **SHALL** render 3D scenes into offscreen targets composited into UI
   panels, supporting orbit, zoom, animation, and multiple simultaneous previews at reduced
   resolution.
   - **Rationale:** Reduced-resolution 3D renders enable character and item previews without GPU
     burden.
   - **Verification:** Open two 3D preview panels. Assert correct geometry and animation. Assert
     combined GPU cost under 1 ms.

## Compositing

6. **R-10.4.6** — The engine **SHALL** render UI elements as textured quads in 3D world space with
   perspective, lighting, and depth testing, for both diegetic and semi-diegetic UI.
   - **Rationale:** Sharing the batched quad pipeline for world-space UI avoids a separate rendering
     path.
   - **Verification:** Place diegetic screen and nameplate. Assert screen occluded by geometry.
     Assert nameplate billboard-faces camera. Verify both in 3D render pass via GPU capture.

7. **R-10.4.7** — The engine **SHALL** apply SDF-based anti-aliasing to UI edges and provide
   stencil-based clipping confining children to parent bounds, sharp at 100%-300% scale.
   - **Rationale:** SDF edges remain resolution-independent across scales; clipping is essential for
     scroll views.
   - **Verification:** Render rounded rect, circle, and vector path at 100%, 200%, 300%. Assert
     smooth edges. Assert child partially outside scroll view is clipped.

## UI Render Graph Integration

8. **R-10.4.8** — The engine **SHALL** integrate UI rendering into the render graph as named passes
   ordered relative to the 3D pipeline (world-space after opaque, screen-space after tonemapping,
   post-process UI last).
   - **Rationale:** Correct pass ordering ensures world-space UI participates in lighting while
     screen-space UI renders over the final composited image.
   - **Verification:** Render a scene with world-space and screen-space UI. Assert world-space UI
     appears after the opaque pass via GPU capture. Assert screen-space UI renders after
     tonemapping. Assert post-process UI is the final pass.
