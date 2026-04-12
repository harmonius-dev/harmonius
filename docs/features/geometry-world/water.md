# 3.4 — Water

## Ocean Simulation

| ID      | Feature |
|---------|--------------------------- |
| F-3.4.1 | FFT Ocean Wave Simulation |

1. **F-3.4.1** — Open-ocean surface displacement computed via inverse FFT on a GPU compute shader.
   Multiple spectral cascades (Phillips, JONSWAP, or TMA spectrum) at different resolutions capture
   waves from large swells down to capillary ripples. The simulation runs at a fixed timestep and
   writes displacement, normal, and fold maps to tiled textures that seamlessly repeat across the
   infinite ocean grid required for MMO naval traversal. Analytical Gerstner waves can be layered on
   top for artist-directed swell control during storms or cinematic sequences. The visual ocean
   surface integrates with the physics water simulation (F-4.8.5) for buoyancy and wave interaction.
   - **Deps:** F-4.8.5 (Water Surface Simulation)
   - **Platform:** FFT resolution scales per tier: mobile 64x64 (1 cascade), Switch 128x128 (2
     cascades), desktop 256x256+ (3-4 cascades). Mobile falls back to Gerstner-only waves when
     compute budget is exceeded.

## Shoreline Blending

| ID      | Feature |
|---------|------------------------------------ |
| F-3.4.2 | Shoreline and Depth-Based Blending |

1. **F-3.4.2** — Water surfaces blend smoothly with terrain and objects at shorelines using scene
   depth comparisons. A soft intersection zone fades water opacity and adjusts wave amplitude based
   on water depth, producing natural beach wash and rocky shore effects. A shoreline foam mask is
   generated from the depth gradient and animated with scrolling noise to simulate wave breaking and
   surf.
   - **Deps:** F-3.4.1, F-3.2.1
   - **Platform:** Foam mask resolution halved on mobile. Shoreline blend width reduced on mobile to
     simplify the depth comparison pass.

## Underwater Rendering

| ID      | Feature |
|---------|----------------------------------------- |
| F-3.4.3 | Underwater Rendering and Volume Effects |

1. **F-3.4.3** — When the camera submerges below the water surface, the renderer switches to an
   underwater mode with depth-based fog, light attenuation using Beer-Lambert absorption, and color
   shift toward the water body's absorption spectrum. A refracted surface view from below uses a
   distorted screen-space lookup. God rays from the surface are rendered as volumetric light shafts,
   enabling immersive underwater zones in the MMO world.
   - **Deps:** F-3.4.1
   - **Platform:** Volumetric god rays disabled on mobile; uses screen-space approximation.
     Underwater fog uses simpler depth curve on mobile.

## Caustics

| ID      | Feature |
|---------|--------------------------- |
| F-3.4.4 | Water Caustics Projection |

1. **F-3.4.4** — Caustic light patterns are projected onto underwater surfaces and the seabed.
   Caustics are computed from the ocean normal map by tracing refracted light paths into a caustics
   texture, or approximated via animated tiling caustics maps. The caustic texture is projected in
   world space and modulates the lighting contribution on receiving surfaces, adding visual richness
   to shallow water and underwater environments.
   - **Deps:** F-3.4.1
   - **Platform:** Mobile uses pre-baked tiling caustics maps only. Desktop uses real-time refracted
     caustics from the ocean normal map.

## Water Refraction / Reflection

| ID      | Feature |
|---------|--------------------------------- |
| F-3.4.5 | Water Reflection and Refraction |

1. **F-3.4.5** — Water surfaces combine reflection and refraction using Fresnel-weighted blending.
   Reflections use a hierarchical approach: screen-space reflections (SSR) for nearby objects, a sky
   environment cubemap for distant sky, and an optional planar reflection pass for high-quality
   mirror-like lakes. Refraction distorts the below-surface scene using the water normal map as an
   offset into the screen-space color buffer.
   - **Deps:** F-3.4.1
   - **Platform:** Reflection quality scales per tier: mobile uses cubemap-only reflections, Switch
     adds SSR at half-res, desktop uses full-res SSR + optional planar reflection pass for hero
     water bodies.

## River Flow

| ID      | Feature |
|---------|--------------------------- |
| F-3.4.6 | Flow Map River Simulation |

1. **F-3.4.6** — Rivers and streams use artist-painted flow maps that define surface velocity
   direction and magnitude per texel. The flow map drives UV animation of normal and foam textures,
   creating the appearance of directional water movement along river channels. Flow speed modulates
   wave amplitude and foam intensity. River spline meshes connect seamlessly with the ocean system
   at estuary points.
   - **Platform:** Flow map resolution consistent across platforms. Normal map animation layers
     reduced on mobile (1 layer vs 2-3 on desktop).

## Foam Generation

| ID      | Feature |
|---------|------------------------- |
| F-3.4.7 | Dynamic Foam Generation |

1. **F-3.4.7** — Foam is generated dynamically from wave folding (Jacobian of the FFT displacement),
   shoreline depth, flow map turbulence, and object wake interactions. A foam coverage map
   accumulates contributions from these sources and decays over time. The foam map modulates surface
   albedo, roughness, and opacity in the water material shader, producing realistic whitecap, surf,
   and wake foam across oceans, rivers, and lakes.
   - **Deps:** F-3.4.1, F-3.4.2, F-3.4.6
   - **Platform:** Foam coverage map resolution halved on mobile. Object wake foam disabled on
     mobile. Whitecap generation from FFT Jacobian simplified on Switch.

## Clipmap Mesh Generation

| ID      | Feature |
|---------|-------------------------------- |
| F-3.4.8 | Water Clipmap Ring Mesh Generation |

1. **F-3.4.8** — Ocean and large-water surface meshes are generated as camera-centred concentric
   clipmap rings with per-ring tessellation factor. FFT displacement (F-3.4.1) is applied in the
   vertex shader rather than baked into vertex buffers. Rings regenerate when the camera exceeds a
   configurable movement threshold, so near-camera water retains high tessellation while the surface
   scales to infinite extent.
   - **Deps:** F-3.4.1
   - **Platform:** Ring count and tessellation factor scale per tier: mobile 3 rings, Switch 5,
     desktop 7+, high-end PC 9.
