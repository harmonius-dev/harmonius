# 11.2 — Decals

## Projection and Rendering

### F-11.2.1 Deferred and Projected Decals

Screen-space deferred decals rendered by rasterizing an oriented bounding box and sampling the
G-buffer depth to reconstruct world position. Each decal modifies albedo, normal, roughness, and
metallic channels independently. Box-projected decals blend across multiple meshes and terrain
patches with per-axis fade and angle-based attenuation to prevent stretching. Triplanar projection
handles complex geometry intersections common in fortress walls and cave environments.

- **Requirements:** R-11.2.1
- **Dependencies:** None
- **Platform notes:** Requires deferred rendering pipeline; forward path falls back to mesh decals

### F-11.2.2 Mesh Decals

Geometry-based decals projected onto static mesh surfaces at bake time or on demand. Decal
geometry is clipped to the receiving triangle soup and stored as an overlay mesh with its own UVs.
Used for high-quality persistent markings — murals, graffiti, signage — where sub-pixel accuracy
and correct tangent-space normals matter more than runtime flexibility.

- **Requirements:** R-11.2.2
- **Dependencies:** None
- **Platform notes:** Bake-time mesh decals run on all platforms. On-demand mesh decal
  generation is disabled on mobile due to CPU cost; uses deferred decals instead.

## Management and Performance

### F-11.2.3 Decal Atlasing and Batching

Runtime decal texture atlas that packs decal textures into shared atlas pages to minimize texture
binds and reduce draw call overhead. Decals sharing an atlas page are batched into a single
indirect draw. Atlas residency is managed by a streaming budget, evicting least-recently-used
entries when memory pressure rises during large siege battles.

- **Requirements:** R-11.2.3
- **Dependencies:** F-11.2.1
- **Platform notes:** Mobile uses smaller atlas pages (1024x1024 vs. 2048x2048 on desktop)
  and a lower streaming budget to fit within GPU memory constraints.

### F-11.2.4 Decal Priority, Layering, and Lifecycle

Priority-based decal stacking system that resolves overlap between multiple decals on the same
surface. Each decal carries a priority value and a layer mask; higher-priority decals overwrite
lower ones per-channel. Blend modes (alpha, multiply, additive) control compositing. Time-based
lifecycle provides configurable fade-in, sustain, and dissolve-out with noise-threshold breakup.
A global budget reclaims the oldest low-priority decals first when the pool is exhausted.

- **Requirements:** R-11.2.4
- **Dependencies:** F-11.2.1
- **Platform notes:** Mobile global decal pool is 64 (vs. 256 on desktop). Shorter default
  lifecycle durations on mobile to recycle slots faster.

## Gameplay Decals

### F-11.2.5 Blood and Damage Decals

Procedural damage decals spawned from hit events with variation driven by weapon type, impact
angle, and surface material. Blood splatter uses velocity-directed projection and randomized atlas
selection to avoid repetition. Damage decals persist on surfaces and characters, providing visual
feedback in melee combat and raid encounters.

- **Requirements:** R-11.2.5
- **Dependencies:** F-11.2.1, F-11.2.4
- **Platform notes:** Mobile uses fewer atlas variants and shorter persistence. Blood decals
  may be disabled by platform content rating (region-specific).

### F-11.2.6 Footprints and Tire Tracks

Surface-aware deformation decals spawned by character locomotion and vehicle movement. Footprint
shape, depth, and material response (mud displacement, snow compression, sand scattering) are
driven by the underlying terrain material layer. Tire tracks use ribbon-style UV projection along
the vehicle path with width matching the wheel contact patch.

- **Requirements:** R-11.2.6
- **Dependencies:** F-11.2.1, F-11.2.4
- **Platform notes:** Mobile spawns footprints at reduced frequency (every 4th step vs. every
  step on desktop). Deformation response uses simplified textures on mobile.
