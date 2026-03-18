# User Stories — 10.4 UI Rendering

## Batched Quad Rendering

| ID        | Persona                 | Features | Requirements       |
|-----------|-------------------------|----------|--------------------|
| US-10.4.1 | Engine developer (P-26) | F-10.4.1 | R-10.4.1           |
| US-10.4.2 | Engine tester (P-27)    | F-10.4.1 | R-X.12.1, R-10.4.1 |

1. **US-10.4.1** — I want UI elements rendered as batched textured quads merged by atlas page and
   blend state into indirect dispatch draw calls, so that complex UIs with hundreds of visible
   widgets render in a handful of dispatches per frame.
   - **Acceptance:** Quads sharing atlas page and blend state merged into single draws; per-quad
     instance data packed into GPU buffer; indirect dispatch for batched rendering
2. **US-10.4.2** — I want to benchmark UI rendering to verify the full HUD stays under 50 draw calls
   and 2ms GPU time, so that the rendering pipeline meets its performance budget on minimum-spec
   hardware.
   - **Acceptance:** Full HUD renders under 50 draw calls; GPU time under 2ms for complete HUD;
     batching reduces call count proportional to shared atlases

## SDF Text Rendering

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-10.4.3 | Player (P-23)           | F-10.4.2 | R-10.4.2     |
| US-10.4.4 | Engine developer (P-26) | F-10.4.2 | R-10.4.2     |

1. **US-10.4.3** — I want text that remains sharp and readable at any UI scale from 100% to 300% and
   at any rotation, so that chat, tooltips, nameplates, and damage numbers are always crisp
   regardless of my display settings.
   - **Acceptance:** MSDF text sharp at all scale factors 100%-300%; subpixel positioning for
     accurate glyph placement; per-glyph outline and shadow effects; 5000+ glyphs rendered per frame
     efficiently
2. **US-10.4.4** — I want to implement multi-channel signed distance field text rendering with MSDF
   atlases generated at asset build time, so that text is resolution-independent and avoids reliance
   on platform-specific subpixel layout.
   - **Acceptance:** MSDF atlases generated at build time per font face/size; configurable hinting
     and subpixel positioning; consistent rendering quality across RGB/BGR displays

## Vector Graphics Rendering

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-10.4.5 | Designer (P-5)          | F-10.4.3 | R-10.4.3     |
| US-10.4.6 | Engine developer (P-26) | F-10.4.3 | R-10.4.3     |

1. **US-10.4.5** — I want GPU-accelerated vector graphics with filled paths, stroked paths, Bezier
   curves, gradients, and resolution-independent shapes, so that I can create crisp minimap
   overlays, cooldown sweeps, radial menus, and custom health bars without bitmap assets.
   - **Acceptance:** Filled and stroked paths with Bezier curves and arcs; linear, radial, and
     conical gradients; compute-driven tessellation pipeline; resolution-independent at all DPI
     levels
2. **US-10.4.6** — I want to implement a compute-driven pipeline that tessellates vector paths into
   coverage masks or signed distance fields, so that vector graphics render on the GPU without
   CPU-side rasterization overhead.
   - **Acceptance:** Compute shader tessellation of vector paths; SDF or coverage mask output for
     anti-aliased edges; gradient fill support in compute pipeline

## UI Atlas and Nine-Slice Rendering

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-10.4.7 | Artist (P-8)         | F-10.4.4 | R-10.4.4     |
| US-10.4.8 | Engine tester (P-27) | F-10.4.4 | R-10.4.4     |

1. **US-10.4.7** — I want UI textures packed into runtime-managed atlases with nine-slice rendering
   that stretches backgrounds without distorting corners or edges, so that panel backgrounds and
   borders scale cleanly at any size with a single atlas entry per visual style.
   - **Acceptance:** UI textures packed into runtime-managed atlases; nine-slice rendering preserves
     corners and edges; incremental atlas rebuild when new assets stream in; atlas page size
     2048x2048 mobile, 4096x4096 desktop
2. **US-10.4.8** — I want to verify that atlas pages rebuild incrementally when new assets stream in
   without causing full repack stalls, so that streaming in new UI textures does not cause frame
   hitches.
   - **Acceptance:** Incremental repack, no full atlas rebuild on new assets; no frame hitches
     during atlas update; texture binds minimized by atlas batching

## Render-to-Texture for 3D-in-UI

| ID         | Persona        | Features | Requirements |
|------------|----------------|----------|--------------|
| US-10.4.9  | Player (P-23)  | F-10.4.5 | R-10.4.5     |
| US-10.4.10 | Designer (P-5) | F-10.4.5 | R-10.4.5     |

1. **US-10.4.9** — I want 3D character portraits, item previews, and dressing room views rendered
   inside UI panels with camera orbit and zoom, so that I can inspect my character and equipment
   from any angle before making decisions.
   - **Acceptance:** 3D scenes rendered to offscreen targets and composited into UI; camera orbit,
     zoom, and animation playback in preview; multiple simultaneous previews for side-by-side
     comparison
2. **US-10.4.10** — I want 3D-in-UI preview panels with simplified lighting setup that render at
   reduced resolution and upscale, so that item and character previews look good without excessive
   GPU cost.
   - **Acceptance:** Simplified lighting for 3D preview renders; reduced resolution rendering with
     upscale; mobile limits to one simultaneous preview at quarter res; desktop supports multiple
     previews at half res

## World-Space and Diegetic UI

| ID         | Persona            | Features | Requirements |
|------------|--------------------|----------|--------------|
| US-10.4.11 | Developer (P-15)   | F-10.4.6 | R-10.4.6     |
| US-10.4.12 | QA engineer (P-19) | F-10.4.6 | R-10.4.6     |

1. **US-10.4.11** — I want UI elements rendered as textured quads in 3D world space with
   perspective, lighting, and depth testing, so that nameplates, holographic displays, and in-world
   signs exist naturally in the game world.
   - **Acceptance:** UI quads placed in 3D world with perspective; subject to lighting and depth
     testing; semi-diegetic UI (nameplates) faces camera; shares batched quad pipeline with
     screen-space UI
2. **US-10.4.12** — I want to verify that world-space UI elements depth-sort correctly against 3D
   scene geometry, so that nameplates and signs are occluded by walls and rendered in front of
   transparent objects as expected.
   - **Acceptance:** World-space UI occluded by opaque geometry; depth testing correct for
     overlapping world-space elements; camera-facing UI does not z-fight with scene geometry

## Anti-Aliased Edges and Clipping

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-10.4.13 | Player (P-23)           | F-10.4.7 | R-10.4.7     |
| US-10.4.14 | Engine developer (P-26) | F-10.4.7 | R-10.4.7     |

1. **US-10.4.13** — I want UI element edges to be anti-aliased and sharp at all scale factors
   without blurring adjacent elements, so that rounded buttons, circles, and vector paths look clean
   at any DPI.
   - **Acceptance:** SDF-based alpha blending for rounded rectangles and circles; hardware clip
     rects or stencil-based clipping for scroll views; anti-aliasing sharp at 100%-300% UI scale; no
     blurring of adjacent elements
2. **US-10.4.14** — I want to implement SDF-based anti-aliasing for UI edges and stencil-based
   clipping for masked containers, so that child widgets are confined to parent bounds in scroll
   views with clean anti-aliased edges.
   - **Acceptance:** SDF anti-aliasing for all vector path edges; stencil clipping confines children
     to parent bounds; clipping works for nested scroll views
