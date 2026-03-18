# R-2.7 — Environment and Atmosphere Requirements

## Sky and Atmosphere

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-2.7.1 | [F-2.7.1](../../features/rendering/environment.md) |
| R-2.7.2 | [F-2.7.2](../../features/rendering/environment.md) |
| R-2.7.3 | [F-2.7.3](../../features/rendering/environment.md) |
| R-2.7.4 | [F-2.7.4](../../features/rendering/environment.md) |
| R-2.7.5 | [F-2.7.5](../../features/rendering/environment.md) |

1. **R-2.7.1** — The engine **SHALL** render a physically-based procedural sky using precomputed
   atmosphere LUTs (transmittance, multi-scattering, sky view, aerial perspective) with configurable
   sun position and Rayleigh/Mie scattering parameters that support continuous time-of-day
   transitions without visible popping or banding.
   - **Rationale:** A physically-based sky model provides consistent, realistic outdoor lighting
     that responds to time-of-day changes.
   - **Verification:** Sweep sun elevation from sunrise to sunset and verify smooth color
     transitions with no visible banding; compare LUT-based output against a reference
     Bruneton/Hillaire implementation to confirm spectral accuracy.
2. **R-2.7.2** — The engine **SHALL** render volumetric fog using a frustum-aligned voxel (froxel)
   grid that encodes in-scattered light and extinction per cell, resolved via a ray-march pass from
   the camera through the grid and composited over the scene as a multiply-accumulate operation.
   - **Rationale:** Froxel-based volumetric fog provides physically-motivated light scattering with
     depth-aware density variation, essential for atmospheric environments.
   - **Verification:** Place a directional light in a fog-filled scene and verify visible volumetric
     scattering with correct depth attenuation; compare froxel resolution settings and confirm no
     visible voxel grid artifacts at the default resolution.
3. **R-2.7.3** — The engine **SHALL** render volumetric clouds via ray-marching through an
   altitude-bounded layer using 3D noise textures, with temporal reprojection amortizing the
   ray-march cost across frames while maintaining stable cloud appearance during camera movement.
   The cloud simulation data is shared with the sky-atmosphere domain (R-3.5.3); this requirement
   covers the rendering pass only.
   - **Rationale:** Volumetric clouds are a major visual element for outdoor scenes; temporal
     reprojection is necessary to keep ray-march cost within frame budget.
   - **Verification:** Render a sky with volumetric clouds and verify cloud density and lighting are
     physically plausible; move the camera rapidly and confirm temporal reprojection produces no
     visible ghosting or shimmer.
4. **R-2.7.4** — The engine **SHALL** render light shaft (god ray) effects from bright occluded
   sources using either a screen-space radial blur mode or a volumetric mode that reuses the froxel
   grid (R-2.7.2), with both modes producing visible directional light shafts.
   - **Rationale:** God rays enhance atmospheric depth and direct the viewer's attention toward
     light sources; reusing the froxel grid avoids redundant volumetric computation.
   - **Verification:** Place an occluded directional light behind geometry and confirm visible light
     shafts in both screen-space and volumetric modes; verify the volumetric mode produces
     consistent results with the froxel fog pass.
5. **R-2.7.5** — The engine **SHALL** provide analytical fog (exponential and exponential-squared
   models) with height falloff, usable standalone or combined with the volumetric froxel system,
   with configurable density, height, and falloff parameters.
   - **Rationale:** Distance and height fog provide cost-effective atmospheric depth cues for scenes
     that do not require full volumetric fog.
   - **Verification:** Enable distance fog and verify objects fade to the fog color with increasing
     distance following exponential falloff; enable height fog and verify density decreases with
     altitude; combine with froxel fog and confirm correct compositing.

## Water and Ocean

| ID      | Derived From                                       |
|---------|----------------------------------------------------|
| R-2.7.6 | [F-2.7.6](../../features/rendering/environment.md) |
| R-2.7.7 | [F-2.7.7](../../features/rendering/environment.md) |
| R-2.7.8 | [F-2.7.8](../../features/rendering/environment.md) |
| R-2.7.9 | [F-2.7.9](../../features/rendering/environment.md) |

1. **R-2.7.6** — The engine **SHALL** simulate an ocean surface using FFT-based compute stages
   (spectrum initialization, IFFT, normal map generation), rendered via the mesh shader pipeline
   with LOD based on camera distance, supporting reflections (SSR or RT), Fresnel blending, foam,
   and optional underwater effects. The FFT ocean wave computation is shared with the geometry-world
   water domain (R-3.4.1); this requirement covers the rendering integration only.
   - **Rationale:** An FFT ocean provides physically-based wave spectra with controllable sea
     states, required for any game featuring large bodies of water.
   - **Verification:** Initialize the ocean with a known wave spectrum and verify surface
     displacement matches expected wave heights; confirm LOD transitions produce no visible popping;
     verify Fresnel reflections, foam, and underwater effects are active and visually correct.
2. **R-2.7.7** — The engine **SHALL** render heterogeneous volumetric media (fire, smoke,
   explosions, clouds) from OpenVDB data stored as sparse volume textures, ray-marched with full
   lighting and shadow integration, supporting emission, absorption, and anisotropic scattering via
   a volumetric BSDF.
   - **Rationale:** OpenVDB volumes enable high-fidelity volumetric effects authored in DCC tools
     and rendered with physically-based volumetric lighting.
   - **Verification:** Import an OpenVDB smoke volume and render with a directional light; verify
     self-shadowing, emission, absorption, and anisotropic scattering are visible and physically
     plausible; confirm sparse storage uses less memory than a dense 3D texture of equivalent
     resolution.
3. **R-2.7.8** — The engine **SHALL** render production-quality volumetric clouds using a full 3D
   voxel representation with compressed signed distance fields for ray-march acceleration,
   supporting ground-level and high-altitude fly-through gameplay, fluid simulation-based modeling,
   and temporal reprojection over multiple frames with full frame reconstruction.
   - **Rationale:** Voxel-based clouds supersede noise-only ray marching for production environments
     requiring fly-through and advanced lighting (dark edges, inner glow).
   - **Verification:** Fly a camera through and around voxel clouds and verify no visual artifacts
     at close range; compare rendering cost against noise-only clouds and confirm SDF acceleration
     reduces ray-march sample count; verify temporal reprojection stability during rapid camera
     movement.
4. **R-2.7.9** — The engine **SHALL** render art-directable breaking waves using offline-simulated
   deformation textures combined with a runtime compute shader generating vertex buffers, with
   wavefront shapes evaluated via Coons surface interpolation and artist-controlled guide curves
   defining wave shape, timing, and animation.
   - **Rationale:** Breaking waves require both physical plausibility and artistic control; the
     hybrid offline-simulation plus runtime-evaluation approach provides both.
   - **Verification:** Load a breaking wave asset with guide curves and verify smooth wave
     deformation along the wavefront; modify guide curves and confirm wave shape responds to artist
     input; verify Coons surface interpolation produces smooth evaluation without crease artifacts.

## Weather

| ID       | Derived From                                        |
|----------|-----------------------------------------------------|
| R-2.7.10 | [F-2.7.10](../../features/rendering/environment.md) |

1. **R-2.7.10** — The engine **SHALL** provide a dynamic weather state machine driving volumetric
   clouds, atmospheric scattering, fog density, precipitation particles, wind fields, and lighting
   conditions, with at least six weather states (clear, overcast, rain, thunderstorm, snow, dust
   storm) that interpolate over configurable transition periods and influence material wetness,
   puddle rendering, and vegetation animation intensity.
   - **Rationale:** A unified weather system ensures all environmental subsystems respond coherently
     to weather state changes, preventing visual inconsistency.
   - **Verification:** Trigger each weather state transition and verify all driven systems (clouds,
     fog, particles, lighting, material wetness, puddles, vegetation sway) respond correctly;
     measure transition interpolation and confirm it completes within the configured period without
     discontinuities.

## Non-Functional Requirements

| ID        |
|-----------|
| NFR-2.7.1 |
| NFR-2.7.2 |
| NFR-2.7.3 |

1. **NFR-2.7.1** — The froxel-based volumetric fog pass **SHALL** complete in under 2.0 ms at 1080p
   on target hardware, including the ray-march resolve and compositing steps.
   - **Rationale:** Volumetric fog runs every frame in outdoor scenes; its cost must be bounded to
     preserve frame budget.
   - **Verification:** Profile the volumetric fog pass at 1080p in a fog-heavy scene and verify
     total GPU time is below 2.0 ms.
2. **NFR-2.7.2** — The volumetric cloud rendering pass (noise-based or voxel-based) **SHALL**
   complete in under 3.0 ms per frame at 1080p with temporal reprojection enabled.
   - **Rationale:** Cloud rendering is one of the most expensive atmospheric effects; budgeting
     prevents it from dominating the frame.
   - **Verification:** Profile the cloud pass at 1080p with temporal reprojection and verify GPU
     time is below 3.0 ms.
3. **NFR-2.7.3** — The FFT ocean simulation (spectrum + IFFT + normals) and rendering (mesh +
   reflections + foam) **SHALL** complete in under 3.0 ms combined at 1080p on target hardware.
   - **Rationale:** Ocean rendering involves multiple compute and rasterization stages; the combined
     cost must fit within the frame budget for scenes with water.
   - **Verification:** Profile all ocean compute and rendering passes at 1080p and verify combined
     GPU time is below 3.0 ms.
