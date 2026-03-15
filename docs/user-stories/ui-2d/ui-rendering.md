# User Stories — 10.4 UI Rendering

## US-10.4.1 Render UI With Minimal Draw Call Overhead

**As an** engine developer (P-26), **I want** UI elements rendered as batched textured quads merged
by atlas page and blend state into indirect dispatch draw calls, **so that** complex UIs with
hundreds of visible widgets render in a handful of dispatches per frame.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Quads sharing atlas page and blend state merged into single draws | F-10.4.1 | R-10.4.1 |
| Per-quad instance data packed into GPU buffer | F-10.4.1 | R-10.4.1 |
| Indirect dispatch for batched rendering | F-10.4.1 | R-10.4.1 |

## US-10.4.2 Verify Batched Rendering Stays Within Draw Call Budget

**As an** engine tester (P-27), **I want** to benchmark UI rendering to verify the full HUD stays
under 50 draw calls and 2ms GPU time, **so that** the rendering pipeline meets its performance
budget on minimum-spec hardware.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Full HUD renders under 50 draw calls | F-10.4.1 | R-X.12.1 |
| GPU time under 2ms for complete HUD | F-10.4.1 | R-X.12.1 |
| Batching reduces call count proportional to shared atlases | F-10.4.1 | R-10.4.1 |

## US-10.4.3 See Sharp Text at Any Scale Without Blurring

**As a** player (P-23), **I want** text that remains sharp and readable at any UI scale from 100% to
300% and at any rotation, **so that** chat, tooltips, nameplates, and damage numbers are always
crisp regardless of my display settings.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| MSDF text sharp at all scale factors 100%-300% | F-10.4.2 | R-10.4.2 |
| Subpixel positioning for accurate glyph placement | F-10.4.2 | R-10.4.2 |
| Per-glyph outline and shadow effects | F-10.4.2 | R-10.4.2 |
| 5000+ glyphs rendered per frame efficiently | F-10.4.2 | R-10.4.2 |

## US-10.4.4 Implement MSDF Text Rendering Pipeline

**As an** engine developer (P-26), **I want** to implement multi-channel signed distance field text
rendering with MSDF atlases generated at asset build time, **so that** text is
resolution-independent and avoids reliance on platform-specific subpixel layout.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| MSDF atlases generated at build time per font face/size | F-10.4.2 | R-10.4.2 |
| Configurable hinting and subpixel positioning | F-10.4.2 | R-10.4.2 |
| Consistent rendering quality across RGB/BGR displays | F-10.4.2 | R-10.4.2 |

## US-10.4.5 Create Resolution-Independent Shapes and Overlays

**As a** designer (P-5), **I want** GPU-accelerated vector graphics with filled paths, stroked
paths, Bezier curves, gradients, and resolution-independent shapes, **so that** I can create crisp
minimap overlays, cooldown sweeps, radial menus, and custom health bars without bitmap assets.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Filled and stroked paths with Bezier curves and arcs | F-10.4.3 | R-10.4.3 |
| Linear, radial, and conical gradients | F-10.4.3 | R-10.4.3 |
| Compute-driven tessellation pipeline | F-10.4.3 | R-10.4.3 |
| Resolution-independent at all DPI levels | F-10.4.3 | R-10.4.3 |

## US-10.4.6 Implement GPU-Accelerated Vector Path Rendering

**As an** engine developer (P-26), **I want** to implement a compute-driven pipeline that
tessellates vector paths into coverage masks or signed distance fields, **so that** vector graphics
render on the GPU without CPU-side rasterization overhead.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compute shader tessellation of vector paths | F-10.4.3 | R-10.4.3 |
| SDF or coverage mask output for anti-aliased edges | F-10.4.3 | R-10.4.3 |
| Gradient fill support in compute pipeline | F-10.4.3 | R-10.4.3 |

## US-10.4.7 Author Nine-Slice Panel Backgrounds Without Distortion

**As an** artist (P-8), **I want** UI textures packed into runtime-managed atlases with nine-slice
rendering that stretches backgrounds without distorting corners or edges, **so that** panel
backgrounds and borders scale cleanly at any size with a single atlas entry per visual style.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| UI textures packed into runtime-managed atlases | F-10.4.4 | R-10.4.4 |
| Nine-slice rendering preserves corners and edges | F-10.4.4 | R-10.4.4 |
| Incremental atlas rebuild when new assets stream in | F-10.4.4 | R-10.4.4 |
| Atlas page size 2048x2048 mobile, 4096x4096 desktop | F-10.4.4 | R-10.4.4 |

## US-10.4.8 Verify Atlas Incremental Repack Avoids Stalls

**As an** engine tester (P-27), **I want** to verify that atlas pages rebuild incrementally when new
assets stream in without causing full repack stalls, **so that** streaming in new UI textures does
not cause frame hitches.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Incremental repack, no full atlas rebuild on new assets | F-10.4.4 | R-10.4.4 |
| No frame hitches during atlas update | F-10.4.4 | R-10.4.4 |
| Texture binds minimized by atlas batching | F-10.4.4 | R-10.4.4 |

## US-10.4.9 Preview 3D Characters and Items in UI Panels

**As a** player (P-23), **I want** 3D character portraits, item previews, and dressing room views
rendered inside UI panels with camera orbit and zoom, **so that** I can inspect my character and
equipment from any angle before making decisions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 3D scenes rendered to offscreen targets and composited into UI | F-10.4.5 | R-10.4.5 |
| Camera orbit, zoom, and animation playback in preview | F-10.4.5 | R-10.4.5 |
| Multiple simultaneous previews for side-by-side comparison | F-10.4.5 | R-10.4.5 |

## US-10.4.10 Design 3D Preview Panels With Simplified Lighting

**As a** designer (P-5), **I want** 3D-in-UI preview panels with simplified lighting setup that
render at reduced resolution and upscale, **so that** item and character previews look good without
excessive GPU cost.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Simplified lighting for 3D preview renders | F-10.4.5 | R-10.4.5 |
| Reduced resolution rendering with upscale | F-10.4.5 | R-10.4.5 |
| Mobile limits to one simultaneous preview at quarter res | F-10.4.5 | R-10.4.5 |
| Desktop supports multiple previews at half res | F-10.4.5 | R-10.4.5 |

## US-10.4.11 Render Nameplates and Signs in the 3D World

**As a** developer (P-15), **I want** UI elements rendered as textured quads in 3D world space with
perspective, lighting, and depth testing, **so that** nameplates, holographic displays, and in-world
signs exist naturally in the game world.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| UI quads placed in 3D world with perspective | F-10.4.6 | R-10.4.6 |
| Subject to lighting and depth testing | F-10.4.6 | R-10.4.6 |
| Semi-diegetic UI (nameplates) faces camera | F-10.4.6 | R-10.4.6 |
| Shares batched quad pipeline with screen-space UI | F-10.4.6 | R-10.4.6 |

## US-10.4.12 Verify World-Space UI Depth Sorts Correctly

**As a** QA engineer (P-19), **I want** to verify that world-space UI elements depth-sort correctly
against 3D scene geometry, **so that** nameplates and signs are occluded by walls and rendered in
front of transparent objects as expected.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| World-space UI occluded by opaque geometry | F-10.4.6 | R-10.4.6 |
| Depth testing correct for overlapping world-space elements | F-10.4.6 | R-10.4.6 |
| Camera-facing UI does not z-fight with scene geometry | F-10.4.6 | R-10.4.6 |

## US-10.4.13 See Anti-Aliased UI Edges at All Scale Factors

**As a** player (P-23), **I want** UI element edges to be anti-aliased and sharp at all scale
factors without blurring adjacent elements, **so that** rounded buttons, circles, and vector paths
look clean at any DPI.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| SDF-based alpha blending for rounded rectangles and circles | F-10.4.7 | R-10.4.7 |
| Hardware clip rects or stencil-based clipping for scroll views | F-10.4.7 | R-10.4.7 |
| Anti-aliasing sharp at 100%-300% UI scale | F-10.4.7 | R-10.4.7 |
| No blurring of adjacent elements | F-10.4.7 | R-10.4.7 |

## US-10.4.14 Implement SDF Edge Anti-Aliasing and Stencil Clipping

**As an** engine developer (P-26), **I want** to implement SDF-based anti-aliasing for UI edges and
stencil-based clipping for masked containers, **so that** child widgets are confined to parent
bounds in scroll views with clean anti-aliased edges.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| SDF anti-aliasing for all vector path edges | F-10.4.7 | R-10.4.7 |
| Stencil clipping confines children to parent bounds | F-10.4.7 | R-10.4.7 |
| Clipping works for nested scroll views | F-10.4.7 | R-10.4.7 |
