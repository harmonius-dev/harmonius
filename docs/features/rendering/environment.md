# 2.7 — Environment and Atmosphere

## Features

| ID       | Feature                                       |
|----------|-----------------------------------------------|
| F-2.7.1  | Procedural Sky (Bruneton/Hillaire Atmosphere) |
| F-2.7.2  | Ray-Marched Volumetric Fog (Froxels)          |
| F-2.7.3  | Procedural Volumetric Clouds                  |
| F-2.7.4  | God Rays                                      |
| F-2.7.5  | Distance and Height Fog                       |
| F-2.7.6  | Water Simulation (FFT Ocean)                  |
| F-2.7.7  | Heterogeneous Volumes (OpenVDB)               |
| F-2.7.8  | Voxel-Based Volumetric Clouds                 |
| F-2.7.9  | Art-Directable Breaking Waves                 |
| F-2.7.10 | Weather System                                |

1. **F-2.7.1** — Physically-based sky rendering using precomputed atmosphere LUTs (transmittance,
   multi-scattering, sky view, aerial perspective). Supports time-of-day transitions with sun
   position and Rayleigh/Mie scattering parameters.
   - **Platform:** Mobile: precomputed LUTs only; no runtime recomputation; reduced LUT resolution
     (32x128). Switch: LUT recomputation on time-of-day change only. Desktop: full LUT suite at
     standard resolution. High-end: high-res LUTs with aerial perspective at full depth range.
2. **F-2.7.2** — Frustum-aligned voxel (froxel) grid encoding in-scattered light and extinction per
   cell. A resolve pass ray-marches from the camera through the grid to accumulate volumetric fog,
   applied as a multiply-accumulate over the scene.
   - **Platform:** Mobile: disabled; uses analytical distance/height fog fallback (F-2.7.5). Switch:
     64x36x32 froxel grid at half-res temporal reprojection. Desktop: 160x90x64 grid. High-end:
     160x90x128 grid with per-froxel scattering.
3. **F-2.7.3** — Ray-marched volumetric clouds through an altitude-bounded layer using 3D noise
   textures. Temporal reprojection blends frames to amortize the high ray-march cost.
   - **Platform:** Mobile: disabled; uses skybox cloud texture fallback. Switch: quarter-res ray
     march, 32 steps, 4-frame temporal reprojection. Desktop: half-res, 64 steps, 2-frame
     reprojection. High-end: full-res option, 128 steps, single-frame with temporal accumulation.
4. **F-2.7.4** — Screen-space radial blur or full volumetric integration of light shafts from bright
   occluded sources (sun, lamps). The screen-space mode is lightweight; the volumetric mode reuses
   the froxel grid.
   - **Deps:** F-2.7.2
   - **Platform:** Mobile: screen-space radial blur only at half-res; no volumetric mode. Switch:
     screen-space mode; volumetric mode available docked. Desktop: volumetric integration via froxel
     grid. High-end: full volumetric with colored shafts and multi-light support.
5. **F-2.7.5** — Analytical fog (exponential and exponential-squared) with height falloff. Can be
   used standalone or in combination with the volumetric froxel system.
   - **Platform:** Mobile: primary fog method (replaces volumetric fog); combined into lighting pass
     to avoid extra bandwidth. All other platforms: full quality as standalone or combined with
     froxel volumetrics.
6. **F-2.7.6** — FFT-based ocean surface simulation with compute stages for spectrum initialization,
   IFFT, and normal map generation. The ocean surface is rendered via the mesh shader pipeline with
   LOD based on camera distance. Supports reflections (SSR or RT), Fresnel blending, foam, and
   optional underwater effects.
   - **Platform:** Mobile: simplified Gerstner wave sum (no FFT); planar reflection disabled;
     Fresnel-only shading with scrolling normal map. Switch: 128-point FFT; planar reflection at
     quarter-res. Desktop: 256-point FFT; SSR reflections; foam and underwater. High-end: 512-point
     FFT; RT reflections; full underwater effects.
7. **F-2.7.7** — Volumetric rendering of fire, smoke, explosions, and clouds from sparse volume data
   (OpenVDB import). Volumes are stored as sparse volume textures and ray-marched with full lighting
   and shadow integration. Supports emission, absorption, and anisotropic scattering via a
   volumetric BSDF.
   - **Platform:** Mobile: disabled; sparse volume textures exceed VRAM budget. Switch:
     billboard/impostor fallback for distant volumes; simplified 32^3 grid for near volumes.
     Desktop: 64^3-128^3 sparse volumes with lighting. High-end: 256^3+ sparse volumes with full
     volumetric BSDF and shadow integration.
8. **F-2.7.8** — Advanced volumetric cloud system using full 3D voxel representation with compressed
   signed distance fields for ray march acceleration. Supports both ground-level and high-altitude
   gameplay (fly-through). Fluid simulation-based cloud modeling with cloud-specific lighting (dark
   edges, inner glow). Temporal reprojection over multiple frames with full frame reconstruction.
   Supersedes noise-only ray marching for production-quality cloud environments.
   - **Deps:** F-2.7.3
   - **Platform:** Mobile: disabled; uses skybox cloud fallback. Switch: disabled; uses noise-based
     volumetric clouds (F-2.7.3) as fallback. Desktop: enabled with reduced voxel resolution and
     4-frame temporal reprojection. High-end: full voxel resolution with SDF acceleration and
     single-frame reconstruction.
9. **F-2.7.9** — Hybrid wave system combining offline Houdini-simulated breaking wave deformation
   baked as 2D deformation textures with a compute shader generating vertex buffers at runtime.
   Wavefront shapes use Coons surface interpolation for smooth evaluation. Art-directable guide
   curves give artists full control over wave shape, timing, and animation.
   - **Deps:** F-2.7.6
   - **Platform:** Mobile: disabled; simplified scrolling foam texture on base water. Switch:
     reduced vertex density; max 2 active breaking wave instances. Desktop: full vertex density;
     configurable instance count. High-end: unlimited instances with high-density vertex generation.
10. **F-2.7.10** — Dynamic weather state machine driving volumetric clouds, atmospheric scattering,
    fog density, precipitation particles, wind fields, and lighting conditions. Weather states
    (clear, overcast, rain, thunderstorm, snow, dust storm) interpolate over configurable transition
    periods. Weather directly influences material wetness, puddle rendering, and vegetation
    animation intensity.
    - **Deps:** F-2.7.1, F-2.7.2, F-2.7.3
    - **Platform:** Mobile: weather transitions drive fog/lighting only; no volumetric clouds or
      precipitation particles; no material wetness. Switch: precipitation particles at reduced count
      (500 max); simplified puddle rendering. Desktop: full weather system. High-end: full quality
      with extended precipitation range.
