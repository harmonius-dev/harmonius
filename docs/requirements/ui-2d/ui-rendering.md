# R-10.4 -- UI Rendering Requirements

## R-10.4.1 Batched Quad Rendering

The engine **SHALL** batch UI textured quads that share the same texture atlas page and blend state
into single indirect draw calls, packing per-quad instance data (position, size, UV rect, tint,
corner radius, clip rect) into a GPU buffer dispatched through the mesh shader pipeline.

- **Derived from:** [F-10.4.1](../../features/ui-2d/ui-rendering.md)
- **Rationale:** Merging same-atlas, same-blend-state quads into indirect dispatches minimizes
  draw call overhead, which is critical for complex UIs containing hundreds of visible widgets.
- **Verification:** Render a UI scene with 500 visible widgets spanning four atlas pages and two
  blend states. Verify that total draw calls equal the number of distinct (atlas page, blend
  state) combinations. Confirm per-quad instance data is readable in a GPU capture and that all
  quads render with correct position, tint, and UV mapping.

## R-10.4.2 SDF Text Rendering

The engine **SHALL** render text glyphs using multi-channel signed distance field (MSDF) textures
generated at asset build time, supporting subpixel positioning, configurable hinting, and
per-glyph outline and shadow effects while sustaining at least 5 000 visible glyphs per frame.

- **Derived from:** [F-10.4.2](../../features/ui-2d/ui-rendering.md)
- **Rationale:** MSDF text remains sharp at arbitrary scale and rotation without regenerating
  glyph bitmaps, and avoids dependence on platform-specific subpixel layouts.
- **Verification:** Render a scrollable chat panel containing 5 000 MSDF glyphs at 100%, 200%,
  and 300% UI scale. Verify glyph edges remain sharp (no blurring or rasterization artifacts)
  at every scale via image comparison against a reference render. Confirm outline and shadow
  effects appear correctly on a subset of glyphs. Measure frame time stays below 4 ms for the
  text pass alone.

## R-10.4.3 Vector Graphics Rendering

The engine **SHALL** provide GPU-accelerated vector path rendering supporting filled and stroked
paths with Bezier curves, arcs, and linear, radial, and conical gradients, tessellated via a
compute pipeline into coverage masks or signed distance fields.

- **Derived from:** [F-10.4.3](../../features/ui-2d/ui-rendering.md)
- **Rationale:** Resolution-independent vector paths eliminate bitmap assets for shapes such as
  minimap overlays, cooldown sweeps, radial menus, and custom health bars.
- **Verification:** Render a radial cooldown sweep, a stroked Bezier path, and a conical gradient
  fill at 100% and 300% UI scale. Verify all paths render without aliasing or tessellation
  artifacts at both scales. Confirm the compute tessellation pass produces identical visual
  output to a CPU reference rasterizer within a per-pixel tolerance of 1/255.

## R-10.4.4 UI Atlas and Nine-Slice Rendering

The engine **SHALL** pack UI textures into runtime-managed atlases and support nine-slice rendering
that stretches panel backgrounds and borders without distorting corners or edges, rebuilding atlas
pages incrementally when new assets stream in without causing full-repack stalls.

- **Derived from:** [F-10.4.4](../../features/ui-2d/ui-rendering.md)
- **Rationale:** Atlas packing maximizes batching by reducing texture binds, and incremental
  rebuilds prevent frame hitches when assets arrive asynchronously during gameplay.
- **Verification:** Stream in 20 new UI textures during gameplay and verify atlas pages update
  incrementally with no frame exceeding a 2 ms repack cost. Render a nine-slice panel at three
  different sizes and verify corner regions remain undistorted via pixel comparison against a
  reference image.

## R-10.4.5 Render-to-Texture for 3D-in-UI

The engine **SHALL** render 3D scenes into offscreen render targets composited into UI panels as
texture quads, supporting camera orbit, zoom, and animation playback, and rendering multiple
simultaneous 3D previews at reduced resolution with upscaling.

- **Derived from:** [F-10.4.5](../../features/ui-2d/ui-rendering.md)
- **Rationale:** Offscreen 3D renders at reduced resolution enable character portraits and item
  previews without burdening the main scene's GPU budget.
- **Verification:** Open two side-by-side 3D item preview panels each rendering an animated
  character model. Verify both panels display correct geometry, lighting, and animation
  playback. Confirm each preview renders at half the panel's native resolution and is upscaled.
  Measure combined GPU cost of both previews stays below 1 ms on target hardware.

## R-10.4.6 World-Space and Diegetic UI

The engine **SHALL** render UI elements as textured quads in 3D world space subject to perspective
projection, lighting, and depth testing, supporting both diegetic UI (in-world screens, signs)
and semi-diegetic UI (camera-facing nameplates) through the batched quad pipeline injected into
the 3D render pass.

- **Derived from:** [F-10.4.6](../../features/ui-2d/ui-rendering.md)
- **Rationale:** Sharing the batched quad pipeline for world-space UI avoids a separate rendering
  path and ensures consistent batching, while depth testing integrates UI naturally into the 3D
  scene.
- **Verification:** Place a diegetic in-world screen and a semi-diegetic nameplate in a 3D scene.
  Verify the screen is occluded by geometry in front of it and the nameplate billboard-faces the
  camera. Confirm both elements appear in the 3D render pass (not the screen-space overlay) via
  GPU capture inspection. Verify lighting affects the diegetic surface correctly.

## R-10.4.7 Anti-Aliased Edges and Clipping

The engine **SHALL** apply SDF-based alpha-blended anti-aliasing to UI element edges (rounded
rectangles, circles, vector paths) and provide hardware clip rect or stencil-based clipping that
confines child widgets to parent bounds, maintaining sharp edges at UI scale factors from 100%
through 300%.

- **Derived from:** [F-10.4.7](../../features/ui-2d/ui-rendering.md)
- **Rationale:** SDF-based edge anti-aliasing remains resolution-independent across all scale
  factors, and parent-bounds clipping is essential for scroll views and masked containers.
- **Verification:** Render a rounded rectangle, a circle, and a vector path at 100%, 200%, and
  300% UI scale. Verify edges show smooth anti-aliasing with no staircasing at any scale via
  image comparison. Place a child widget partially outside a scroll-view parent and verify
  clipped pixels are not visible. Confirm no blurring of adjacent elements.
