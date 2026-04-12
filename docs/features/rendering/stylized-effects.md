# 2.11 — Stylized and Gameplay Effects

## Outline Effects

| ID       | Feature                     |
|----------|-----------------------------|
| F-2.11.1 | 2D and 3D Outline Rendering |
| F-2.11.2 | Highlight and Glow Effect   |

1. **F-2.11.1** — Per-object outline rendering using a multi-technique approach. Screen-space edge
   detection (Sobel/Roberts on depth and normal buffers) produces clean outlines at any resolution.
   For finer control, a jump-flood-based outline computes per-pixel distance to the nearest
   silhouette edge, enabling smooth variable-width outlines with configurable falloff. Object-space
   shell extrusion (inverted-hull method) provides view-consistent outlines on individual meshes.
   Outlines support per-entity color, width, style (solid, dashed, animated), and priority
   (selection outline overrides hover outline). Multiple outline layers compose additively — an
   entity can have a selection outline, a team-color outline, and a quest-indicator outline
   simultaneously.
   - **Deps:** F-2.9.1 (Post-Processing Pipeline), F-2.10.1 (ECS-to-Renderer Bridge)
   - **Platform:** Jump-flood algorithm requires compute shader support; falls back to Sobel on
     devices without compute.
2. **F-2.11.2** — Emissive highlight overlay that makes selected or interactive objects visually
   prominent. The highlight system renders affected entities into a stencil-masked buffer with a
   configurable glow color and intensity, then applies a Gaussian blur and composites the result
   over the scene using additive or screen blending. Supports pulsing animation (sinusoidal
   intensity modulation), fresnel-based rim glow (brighter at silhouette edges), and inner glow
   (solid color fill with opacity). The effect reads from a per-entity `HighlightState` ECS
   component, enabling gameplay systems to control highlight appearance — interactive objects glow
   on hover, loot glows by rarity tier, enemies flash on damage.
   - **Deps:** F-2.11.1, F-2.9.1 (Post-Processing Pipeline), F-2.10.1 (ECS-to-Renderer Bridge)
   - **Platform:** Mobile: flat color inner glow only (no Gaussian blur or fresnel rim); max 4
     highlighted entities. Switch: Gaussian blur at half-res; max 8 highlighted entities.
     Desktop/High-end: full glow with blur, fresnel, and pulsing; unlimited highlighted entities.

## Toon and Stylized Shading

| ID       | Feature               |
|----------|-----------------------|
| F-2.11.3 | Advanced Toon Shading |

1. **F-2.11.3** — A configurable cel-shading pipeline that replaces or augments PBR lighting with
   stylized rendering. Lighting is quantized into configurable band counts with artist-controlled
   thresholds and colors per band (e.g., 3-band: shadow, midtone, highlight with custom ramp
   textures). Specular highlights are rendered as hard-edged shapes (circular, star, cross) with
   size and threshold controls. Rim lighting is computed from view-normal angle with configurable
   width, color, and blend mode. Toon shading supports per-material ramp textures for complete
   artistic control, including hatching and stipple patterns for shadow regions. The system works
   alongside the outline renderer (F-2.11.1) for a complete toon aesthetic.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.10.5 (Material Parameter Binding)
   - **Platform:** Mobile: 2-3 bands; no hatching/stipple patterns; simplified rim lighting. Switch:
     full band count; hatching patterns on hero characters only. Desktop/High-end: full toon
     pipeline with all band, specular, and rim options.

## Gameplay Visibility Effects

| ID       | Feature                                    |
|----------|--------------------------------------------|
| F-2.11.4 | Cut-Through Visibility and Roof Fading     |
| F-2.11.5 | X-Ray and See-Through Silhouette Rendering |

1. **F-2.11.4** — Automatic or manual fading, dissolving, or removing of occluding geometry (roofs,
   ceilings, canopy, upper floors) when the camera or player character moves beneath them. The
   system operates in configurable modes: volume-based (geometry inside a vertical prism above the
   player fades), ray-based (geometry between camera and player target is dissolved), and
   layer-based (entire tagged layers fade when the camera enters a trigger zone). Fading uses
   dithered transparency, cross-hatch dissolve patterns, or smooth alpha with configurable fade
   distance and speed. Furniture and props on faded floors can optionally fade along with their
   parent structure or remain visible as silhouettes. Essential for isometric RPGs, CRPGs (Baldur's
   Gate 3 style), and top-down games where interior visibility requires roof removal.
   - **Deps:** F-2.10.1 (ECS-to-Renderer Bridge), F-1.9.4 (Unified Spatial Query)
   - **Platform:** Mobile: layer-based and volume-based modes only (no ray-based); dither-only
     dissolve (no smooth alpha). Switch: all modes; dither dissolve. Desktop/ High-end: all modes
     with smooth alpha and cross-hatch patterns.
2. **F-2.11.5** — Render occluded entities as colored silhouettes visible through walls and other
   geometry. When an entity with an `XRayVisible` component is behind opaque geometry, a
   depth-tested silhouette pass renders it using a flat color, team tint, or fresnel outline at
   reduced opacity. Priority levels control which entities show through (player characters always,
   allies on toggle, enemies within detection range). The silhouette pass uses a separate depth
   comparison against the scene depth buffer — fragments that fail the normal depth test are
   rendered with the silhouette material instead of being discarded. Used for ally visibility
   through walls in team shooters, enemy tracking through cover, and important object highlighting
   in puzzle games.
   - **Deps:** F-2.11.1 (Outline Rendering), F-2.10.1 (ECS-to-Renderer Bridge)
   - **Platform:** Mobile: flat color silhouettes only (no fresnel); max 8 x-ray entities. Switch:
     flat color + team tint; max 16 entities. Desktop/High-end: full silhouette modes with fresnel
     outline; unlimited entities.

## Painterly and Pixel Art Styles

| ID       | Feature                      |
|----------|------------------------------|
| F-2.11.6 | Painterly Rendering Style    |
| F-2.11.7 | Pixel Art Rendering Style    |

1. **F-2.11.6** — Non-photorealistic painterly rendering with brush-stroke simulation, edge
   darkening, and wet-edge effects for a hand-painted or watercolor aesthetic. A brush-radius
   parameter drives a screen-space Kuwahara-style filter that abstracts scene color into painterly
   regions. Edge darkening intensity adds ink-like contours around silhouettes and internal creases.
   Wet-edge intensity modulates rim accumulation where adjacent brushstrokes meet, reproducing
   pigment pooling. The style compiles as a post-process stage on top of the standard lit scene,
   preserving PBR lighting while remapping surface appearance.
   - **Deps:** F-2.9.1 (Post-Processing Pipeline), F-2.11.1 (Outline Rendering)
   - **Platform:** Mobile: disabled. Switch: reduced brush radius and no wet-edge. Desktop: full
     quality painterly stage at half resolution. High-end: full resolution with per-material brush
     parameters.
2. **F-2.11.7** — Pixel art rendering that quantizes the final image to a configurable pixel
   resolution, restricts color output to a palette with configurable maximum color count, and
   applies nearest-neighbor filtering during the final upscale to display resolution. An optional
   dithering pattern smooths color banding within palette constraints. Used to render
   high-resolution 3D assets in a retro pixel-art style, enabling pixel-perfect output on any
   display while authoring content at full fidelity.
   - **Deps:** F-2.9.1 (Post-Processing Pipeline)
   - **Platform:** Mobile: full quality; pixel quantization is lightweight. Switch: full quality.
     Desktop/High-end: full quality with highest palette sizes and finest pixel resolutions.
