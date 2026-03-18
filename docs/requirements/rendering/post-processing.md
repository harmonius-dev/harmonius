# R-2.9 — Post-Processing Requirements

## Core Post-Processing Effects

| ID      | Derived From                                           |
|---------|--------------------------------------------------------|
| R-2.9.1 | [F-2.9.1](../../features/rendering/post-processing.md) |
| R-2.9.2 | [F-2.9.2](../../features/rendering/post-processing.md) |
| R-2.9.3 | [F-2.9.3](../../features/rendering/post-processing.md) |
| R-2.9.4 | [F-2.9.4](../../features/rendering/post-processing.md) |
| R-2.9.5 | [F-2.9.5](../../features/rendering/post-processing.md) |

1. **R-2.9.1** — The engine **SHALL** render bloom by extracting pixels above a configurable
   luminance threshold and applying a multi-pass downscale/upscale blur chain (Gaussian or
   dual-kawase), composited additively over the scene, producing visible glow around bright sources.
   - **Rationale:** Bloom simulates camera lens diffraction and scattering, adding perceptual
     brightness and cinematic quality to HDR content.
   - **Verification:** Place a bright emissive object in a dark scene and verify a visible glow halo
     appears around it; adjust the luminance threshold and confirm the bloom extent changes
     accordingly; verify the blur chain produces no visible banding between mip levels.
2. **R-2.9.2** — The engine **SHALL** render cinematic depth of field driven by real-world camera
   parameters (aperture, focal length, focus distance) using a gather-based circular bokeh filter
   that produces near and far field blur with proper occlusion handling and configurable bokeh shape
   and size.
   - **Rationale:** Cinematic DOF is essential for cutscenes, photo modes, and focal emphasis in
     gameplay cameras.
   - **Verification:** Set a shallow depth of field with a foreground and background object; verify
     the focused object is sharp while near and far objects exhibit circular bokeh; change aperture
     and focal length and confirm blur radius responds correctly; verify no bleeding of background
     blur into sharp foreground silhouettes.
3. **R-2.9.3** — The engine **SHALL** render per-pixel velocity-based motion blur using a
   full-screen velocity buffer, sampling along each pixel's motion vector to simulate camera shutter
   exposure, with support for both camera motion and per-object motion contribution.
   - **Rationale:** Motion blur enhances the perception of speed and cinematic quality during camera
     and object movement.
   - **Verification:** Move the camera rapidly and verify directional blur along the motion
     direction; move individual objects while the camera is stationary and confirm per-object motion
     blur is applied independently of camera blur; verify stationary objects remain sharp.
4. **R-2.9.4** — The engine **SHALL** automatically adjust scene brightness by building a luminance
   histogram via a compute pass and temporally smoothing the exposure value (EV100) between
   configurable minimum and maximum bounds, simulating human eye adaptation.
   - **Rationale:** Auto exposure prevents over- or under-exposed scenes as the camera moves between
     bright and dark areas, matching human visual adaptation behavior.
   - **Verification:** Move the camera from a dark interior to a bright exterior and verify exposure
     smoothly adapts over time without abrupt jumps; configure min/max EV bounds and confirm the
     exposure stays within the configured range.
5. **R-2.9.5** — The engine **SHALL** provide HDR-to-LDR tone mapping with an ACES-compliant filmic
   curve and color grading controls including white balance, lift/gamma/gain for
   shadows/midtones/highlights, saturation, contrast, and color lookup table (LUT) application, with
   support for HDR display output.
   - **Rationale:** Tonemapping and color grading are the final image quality controls that
     establish the visual identity of a game.
   - **Verification:** Apply the ACES tone mapper and verify HDR scene values map to visible LDR
     range with no clipping of bright highlights; apply a known color LUT and confirm the output
     matches the expected color transformation; adjust each grading control independently and verify
     it affects only its target tonal range; output to an HDR display and verify extended brightness
     range.

## Cinematic and Lens Effects

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-2.9.6  | [F-2.9.6](../../features/rendering/post-processing.md)  |
| R-2.9.7  | [F-2.9.7](../../features/rendering/post-processing.md)  |
| R-2.9.8  | [F-2.9.8](../../features/rendering/post-processing.md)  |
| R-2.9.9  | [F-2.9.9](../../features/rendering/post-processing.md)  |
| R-2.9.12 | [F-2.9.12](../../features/rendering/post-processing.md) |

1. **R-2.9.6** — The engine **SHALL** render procedural or texture-based film grain overlay with
   configurable grain intensity, size, and luminance response, producing visible photographic noise.
   - **Rationale:** Film grain adds cinematic texture and visual character to the final image.
   - **Verification:** Enable film grain and verify visible noise overlay; increase intensity and
     confirm grain becomes more pronounced; verify grain response to luminance (brighter or darker
     areas produce different grain density as configured).
2. **R-2.9.7** — The engine **SHALL** simulate chromatic aberration by applying per-channel RGB UV
   offsets radially from screen center with configurable intensity and start offset, producing
   visible color channel separation at image edges.
   - **Rationale:** Chromatic aberration simulates real lens imperfections, adding cinematic
     character and realism to the rendered image.
   - **Verification:** Enable chromatic aberration and verify visible RGB separation at screen
     edges; confirm the effect increases radially from center; set intensity to zero and verify no
     visible effect.
3. **R-2.9.8** — The engine **SHALL** render image-based lens flare effects from bright sources by
   extracting pixels above a brightness threshold and generating ghost and halo artifacts via scaled
   and rotated resampling to simulate internal lens reflections.
   - **Rationale:** Lens flare enhances the perceived brightness of light sources and adds cinematic
     lens simulation to the final image.
   - **Verification:** Point the camera at a bright light source and verify visible ghost and halo
     artifacts appear; move the camera and confirm flare elements track the source position
     correctly; verify no flare is generated from sources below the brightness threshold.
4. **R-2.9.9** — The engine **SHALL** render a radial darkening vignette from screen center to edges
   using a configurable power curve, simulating optical lens falloff.
   - **Rationale:** Vignetting draws viewer attention toward the frame center and simulates physical
     lens characteristics.
   - **Verification:** Enable vignette and verify visible darkening at frame edges and corners;
     adjust the power curve and confirm the falloff profile changes; verify screen center remains
     unaffected.
5. **R-2.9.12** — The engine **SHALL** provide a post-process panini projection pass that remaps
   pixels to reduce straight-line warping at wide field-of-view angles while preserving center
   fidelity.
   - **Rationale:** Wide FOV perspective projection produces distracting edge distortion; panini
     projection corrects this for ultra-wide camera setups.
   - **Verification:** Render a scene at 120-degree FOV with and without panini projection; verify
     straight lines near screen edges are less warped with panini enabled; confirm center region
     geometry is unchanged between both modes.

## Advanced Post-Processing

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-2.9.10 | [F-2.9.10](../../features/rendering/post-processing.md) |
| R-2.9.11 | [F-2.9.11](../../features/rendering/post-processing.md) |
| R-2.9.13 | [F-2.9.13](../../features/rendering/post-processing.md) |
| R-2.9.14 | [F-2.9.14](../../features/rendering/post-processing.md) |

1. **R-2.9.10** — The engine **SHALL** support user-defined fullscreen shader effects executed as
   post-process passes, with read access to scene textures (depth, stencil, G-buffer channels,
   velocity) and arbitrary color output, enabling custom effects such as outlines, distortion, and
   stylization without engine source modification.
   - **Rationale:** Custom post-process materials allow game-specific visual effects without
     modifying the engine's built-in post-processing pipeline.
   - **Verification:** Create a custom post-process material that reads scene depth and outputs a
     depth-based color gradient; apply it as a post-process pass and verify the output matches
     expected depth visualization; confirm access to all declared scene textures.
2. **R-2.9.11** — The engine **SHALL** provide per-region exposure adjustment using a downsampled
   luminance grid that drives per-tile exposure compensation, preventing detail loss in
   high-contrast scenes with simultaneous bright and dark regions.
   - **Rationale:** Global auto exposure cannot preserve detail in both bright and dark areas of
     high-contrast scenes; local exposure addresses this per-region.
   - **Verification:** Render a scene with a bright exterior visible through a dark interior window;
     verify local exposure preserves detail in both bright and dark regions simultaneously; compare
     against global-only exposure and confirm reduced clipping in high-contrast areas.
3. **R-2.9.13** — The engine **SHALL** render a screen-space cavity and curvature effect that
   darkens concavities and brightens convexities by sampling the normal buffer (or
   depth-reconstructed normals) at configurable pixel-radius (curvature) and world-radius (cavity)
   offsets, computing per-pixel normal differences, and compositing the resulting single-channel
   cavity map multiplicatively over the lit scene color with independent ridge brightness and valley
   darkness intensity controls.
   - **Rationale:** Screen-space cavity reveals micro-surface detail (creases, edges, crevices) that
     standard lighting misses, enhancing visual depth and readability of complex geometry without
     additional geometry or texture authoring. Used by Blender's viewport cavity, the Unity "Screen
     Space Cavity & Curvature" asset (Jolly Theory), and stylized renderers for surface detail
     enhancement.
   - **Verification:** Render a scene with detailed geometry (cloth folds, rocky surfaces,
     mechanical parts); enable curvature mode and verify ridges are brightened and valleys are
     darkened at small pixel-scale offsets; enable cavity mode and verify similar enhancement at
     larger world-space-scaled offsets; set ridge intensity to zero and confirm only valley
     darkening is visible; set valley intensity to zero and confirm only ridge brightening; enable
     distance fade and verify the effect attenuates beyond the configured start distance; disable
     the G-buffer normal texture and verify the system falls back to depth-reconstructed normals
     with visually comparable output.
4. **R-2.9.14** — The engine **SHALL** provide a visual node-based graph editor that composes
   full-screen image processing operations (input buffer reads, spatial filters, color operations,
   blend modes, UV transforms, mask generation, and output writes) as typed-edge-connected nodes in
   a directed acyclic graph, compiles the graph to an optimized sequence of GPU compute dispatches
   executed within the post-processing pipeline, supports multi-pass chains with intermediate render
   targets, and integrates with the universal logic graph runtime for parameter binding, hot reload,
   and asset management.
   - **Rationale:** A node-based post-process editor enables artists and technical designers to
     create custom full-screen effects (stylization, color grading presets, screen-space detail
     enhancement, artistic filters) without writing shader code, consistent with the engine's
     no-code philosophy. Comparable to Blender's GPU Compositor, Unreal Engine's Post Process
     Materials in the material graph, and Babylon.js's Node Material system.
   - **Verification:** Create a post-process graph that reads scene color and depth, applies a
     Gaussian blur masked by a depth threshold, blends the blurred result with the original using
     screen blend mode, and outputs the final color; compile the graph and verify it produces a
     valid GPU dispatch sequence; apply the graph in the viewport and verify the expected selective
     blur effect is visible; modify a parameter at runtime and verify the effect updates within one
     frame; insert the graph before tonemapping and verify it operates on HDR data; insert after
     tonemapping and verify it operates on LDR data; create a multi-pass graph with an intermediate
     target and verify correct resource allocation and barrier insertion through the render graph.

## Non-Functional Requirements

| ID        |
|-----------|
| NFR-2.9.1 |
| NFR-2.9.2 |
| NFR-2.9.3 |

1. **NFR-2.9.1** — The entire post-processing pipeline (bloom + DOF + motion blur + exposure +
   tonemapping + grain + chromatic aberration + vignette) **SHALL** complete in under 3.0 ms
   combined at 1080p on target hardware.
   - **Rationale:** Post-processing runs every frame as the final image stage; its combined cost
     must leave sufficient budget for scene rendering.
   - **Verification:** Enable all post-processing effects and profile total GPU time at 1080p.
     Verify combined cost is below 3.0 ms.
2. **NFR-2.9.2** — No single post-processing effect **SHALL** exceed 1.0 ms at 1080p on target
   hardware, with lightweight effects (film grain, vignette, chromatic aberration) completing in
   under 0.1 ms each.
   - **Rationale:** Individual effect budgets prevent any single effect from dominating the
     post-processing pipeline.
   - **Verification:** Profile each post-processing effect individually at 1080p and verify it is
     within its respective budget.
3. **NFR-2.9.3** — Tonemapping and color grading **SHALL** produce output compliant with HDR10 (PQ
   EOTF, BT.2020 color space) when HDR display output is enabled, with peak luminance configurable
   up to 10,000 nits.
   - **Rationale:** Correct HDR output requires standards-compliant signal encoding to avoid
     washed-out or clipped images on HDR displays.
   - **Verification:** Output to an HDR10 display and verify PQ curve encoding with correct BT.2020
     primaries using a calibrated measurement tool.
