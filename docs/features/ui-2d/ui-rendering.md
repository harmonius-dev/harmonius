# 10.4 — UI Rendering

## Batching and Geometry

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.4.1 | Batched Quad Rendering | Render UI elements as batched textured quads submitted through the engine's mesh shader pipeline. Quads sharing the same texture atlas page and blend state are merged into single draw calls. Per-quad instance data (position, size, UV rect, tint, corner radius, clip rect) is packed into a GPU buffer and drawn with indirect dispatch. For complex MMO UIs with hundreds of visible widgets, batching reduces draw call overhead to a handful of dispatches per frame. | R-10.4.1 | None | Uses the bindless resource model from the rendering domain; fallback path for non-bindless hardware is not required per project hardware targets. |
| F-10.4.2 | SDF Text Rendering | Render text glyphs using multi-channel signed distance field (MSDF) textures for resolution-independent, sharp text at any scale and rotation. MSDF atlases are generated at asset build time for each font face and size range. The runtime rasterizer supports subpixel positioning, configurable hinting, and per-glyph outline/shadow effects. Must render thousands of glyphs per frame efficiently for chat history, tooltips, nameplates, and damage numbers. | R-10.4.2 | F-10.4.1 | Subpixel rendering quality varies by display technology; MSDF avoids reliance on platform-specific subpixel layout (RGB vs BGR). |
| F-10.4.3 | Vector Graphics Rendering | GPU-accelerated vector path rendering supporting filled and stroked paths with Bezier curves, arcs, gradients (linear, radial, conical), and resolution-independent shapes. Uses a compute-driven pipeline to tessellate paths into coverage masks or signed distance fields. Enables crisp minimap overlays, circular cooldown sweeps, radial menus, and custom-shaped health bars without bitmap assets. | R-10.4.3 | F-10.4.1 | None |

## Texture and Atlas Management

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.4.4 | UI Atlas and Nine-Slice Rendering | Pack UI textures (icons, panel backgrounds, button states, borders) into runtime-managed atlases to minimize texture binds and maximize batching. Nine-slice (9-patch) rendering stretches panel backgrounds and borders without distorting corners or edges, using a single atlas entry per visual style. Atlas pages are rebuilt incrementally when new assets stream in, avoiding full repack stalls. | R-10.4.4 | F-10.4.1 | Atlas page size 2048x2048 on mobile, 4096x4096 on desktop. Fewer atlas pages reduce texture binds on bandwidth-limited mobile GPUs. |
| F-10.4.5 | Render-to-Texture for 3D-in-UI | Render 3D scenes (character portraits, item previews, dressing room, mount previews) into offscreen render targets that are composited into UI panels as texture quads. The 3D render uses a simplified lighting setup and supports camera orbit, zoom, and animation playback. Must handle multiple simultaneous 3D previews (e.g., comparing two items side by side) without excessive GPU cost by rendering at reduced resolution and upscaling. | R-10.4.5 | F-10.4.1 | Mobile limits to one simultaneous 3D preview at reduced resolution (quarter res). Desktop supports multiple previews at half res. |

## Compositing

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.4.6 | World-Space and Diegetic UI | Render UI elements as textured quads placed in the 3D world, subject to perspective, lighting, and depth testing. Supports diegetic UI (in-world screens, holographic displays, signs) and semi-diegetic UI (nameplates that face the camera but exist in world space). World-space UI shares the batched quad pipeline but injects geometry into the 3D render pass rather than the screen-space overlay pass. | R-10.4.6 | F-10.4.1, F-10.3.4 | None |
| F-10.4.7 | Anti-Aliased Edges and Clipping | Apply anti-aliasing to UI element edges using SDF-based alpha blending for rounded rectangles, circles, and vector paths. Hardware clip rects or stencil-based clipping confine child widgets to their parent's bounds for scroll views and masked containers. Edge anti-aliasing must remain sharp at all UI scale factors (100% through 300%) without blurring adjacent elements. | R-10.4.7 | F-10.4.1, F-10.4.3 | None |
