# 2.3 — Core Rendering

### F-2.3.1 Direct Lighting

Point, spot, and directional light evaluation with physically-based attenuation. All light types
contribute to the same unified light buffer consumed by both forward and deferred paths.

- **Requirements:** R-2.3.1
- **Dependencies:** None
- **Platform notes:** Mobile: max 16 dynamic lights per tile; simplified attenuation. Switch: max 32
  lights per tile; handheld reduces to 16. Desktop: full quality, up to 256 lights per tile.
  High-end: unlimited lights per tile with stochastic sampling.

### F-2.3.2 GPU Frustum Culling

Meshlet-level frustum culling on the GPU via a compute pass. Each meshlet's AABB is tested against
the six camera frustum planes. Meshlets outside the frustum are excluded from the indirect draw
buffer.

- **Requirements:** R-2.3.2
- **Dependencies:** F-2.1.1
- **Platform notes:** Mobile: runs at reduced dispatch granularity to fit tile-based GPU budgets.
  Switch: full GPU culling in docked mode; handheld may batch fewer meshlets per dispatch.
  Desktop/High-end: full quality, no restrictions.

### F-2.3.3 Backface Culling

Meshlet-level normal cone culling on the GPU. Meshlets whose triangles all face away from the camera
(determined by the cone test against camera position) are culled before rasterization.

- **Requirements:** R-2.3.3
- **Dependencies:** None
- **Platform notes:** All platforms: full quality, no degradation. Lightweight compute pass with
  minimal bandwidth cost on tile-based mobile GPUs.

### F-2.3.4 Occlusion Culling (HZB Two-Phase)

Two-phase hierarchical Z-buffer occlusion culling. Phase 1 tests meshlets against the previous
frame's HZB (conservative). Phase 2 re-tests phase-1 rejects against the current frame's HZB. This
avoids missing newly-revealed geometry.

- **Requirements:** R-2.3.4
- **Dependencies:** F-2.3.2
- **Platform notes:** Mobile: HZB at quarter resolution; phase 2 may be skipped under frame budget
  pressure. Switch: half-resolution HZB; both phases active. Desktop/High-end: full-resolution HZB
  with both phases.

### F-2.3.5 Orthographic Projection

Orthographic camera projection for top-down views, 2D game rendering, and shadow map generation.

- **Requirements:** R-2.3.5
- **Dependencies:** None
- **Platform notes:** All platforms: full quality, no degradation.

### F-2.3.6 Perspective Projection (Reverse-Z)

Perspective camera projection with reverse-Z and optional infinite far plane for maximum depth
precision at distance.

- **Requirements:** R-2.3.6
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Reverse-Z requires clearing depth to 0 instead of
  1, supported on Metal, D3D12, and Vulkan. Older mobile Vulkan drivers may need viewport transform
  workaround.

### F-2.3.7 GPU-Driven Instancing

GPU-side instance compaction and indirect dispatch. After culling, surviving opaque meshlet
instances are batched by shared material and material instance into contiguous draw buffers and
dispatched via a single indirect call per material batch. Opaque meshlets are not depth-sorted; draw
order within a batch is arbitrary. Transparent objects are not batched -- each transparent object is
drawn individually in back-to-front sorted order (see F-2.4.5).

- **Requirements:** R-2.3.7
- **Dependencies:** F-2.3.2, F-2.3.3, F-2.3.4
- **Platform notes:** Mobile: instance buffer sized for lower object counts (~10k meshlets);
  indirect draw requires Vulkan 1.1+ or Metal family GPU 3+. Switch: full indirect draw support;
  handheld caps meshlet budget. Desktop/High-end: no limits.

### F-2.3.8 Render-to-Texture

Ability to render any pass to an off-screen texture for use by subsequent passes. The render graph
compiler automatically inserts barriers between write and read.

- **Requirements:** R-2.3.8
- **Dependencies:** F-2.2.1
- **Platform notes:** Mobile: off-screen targets should use memoryless storage where possible to
  avoid bandwidth on tile-based GPUs. All platforms: full support.

### F-2.3.9 Cubemaps

Static and dynamic cubemap rendering for environment maps, reflection probes, and IBL prefiltering.
Dynamic cubemaps re-render specified faces per frame.

- **Requirements:** R-2.3.9
- **Dependencies:** F-2.3.8
- **Platform notes:** Mobile: static cubemaps only; max 128x128 per face; ASTC compressed. Switch:
  one dynamic face update per frame; 256x256 per face. Desktop: full dynamic cubemaps at 512x512.
  High-end: 1024x1024 per face, multiple dynamic cubemaps per frame.

### F-2.3.10 Scene Capture

Rendering the scene from an arbitrary camera into a texture target for use as a material input.
Supports 2D planar capture and omnidirectional cubemap capture for security cameras, mirrors,
portals, and minimap rendering.

- **Requirements:** R-2.3.10
- **Dependencies:** F-2.3.8
- **Platform notes:** Mobile: max 1 scene capture active; quarter-resolution; simplified lighting
  (forward only, no shadows). Switch: max 2 captures; half-resolution. Desktop: configurable count
  and resolution. High-end: unlimited at full resolution.

### F-2.3.11 Dynamic Resolution

Runtime resolution scaling that adjusts the internal render resolution to maintain a target frame
budget. A GPU timing feedback loop drives the screen percentage between configurable min/max bounds.

- **Requirements:** R-2.3.11
- **Dependencies:** None
- **Platform notes:** Mobile: aggressive range 50-75% of native; targets 30 fps. Switch: 50-100%
  handheld (720p native), 60-100% docked (1080p native). Desktop: 67-100% with upscaling to native.
  High-end: 50-100% with TSR/DLSS to 4K.

### F-2.3.12 Subsurface Scattering

Screen-space and ray-traced subsurface scattering for skin, wax, marble, and other translucent
materials. Driven by per-material SSS profiles defining scatter radius and extinction.

- **Requirements:** R-2.3.12
- **Dependencies:** F-2.4.3
- **Platform notes:** Mobile: preintegrated SSS LUT only (no screen-space blur); no RT SSS. Switch:
  screen-space SSS at half resolution. Desktop: full screen-space SSS. High-end: RT subsurface
  scattering with multi-sample profiles.

### F-2.3.13 Alpha Mask Cutouts

Alpha-tested geometry for vegetation, fences, and decals. Material flags control the alpha test
threshold. Cutout geometry participates in shadow map rendering.

- **Requirements:** R-2.3.13
- **Dependencies:** None
- **Platform notes:** Mobile: alpha test breaks tile-based hidden surface removal on some GPUs; use
  sparingly and prefer opaque proxies at distance. All other platforms: full quality, no
  restrictions.
