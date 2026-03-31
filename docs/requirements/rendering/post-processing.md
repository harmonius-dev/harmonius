# R-2.9 -- Post-Processing Requirements

## Core Effects

1. **R-2.9.1** — The engine **SHALL** provide bloom with configurable luminance threshold and
   multi-pass blur, scaling from dual-kawase on mobile to wide Gaussian on high-end.
   - **Rationale:** Bloom simulates camera diffraction from bright sources; tiered blur matches
     platform budgets.
   - **Verification:** Set threshold to 1.0. Verify only HDR pixels above 1.0 contribute. Compare
     mobile dual-kawase and desktop Gaussian output.

2. **R-2.9.2** — The engine **SHALL** provide cinematic depth of field with real camera parameters
   (aperture, focal length, focus distance) and configurable bokeh shape.
   - **Rationale:** DOF creates photographic focus effects for cutscenes and gameplay cinematics.
   - **Verification:** Set aperture f/1.4. Verify near and far blur. Verify circular bokeh shape.
     Verify Gaussian fallback on mobile.

3. **R-2.9.3** — The engine **SHALL** provide per-pixel motion blur using the velocity buffer with
   both camera and per-object motion contribution.
   - **Rationale:** Motion blur conveys speed and simulates camera shutter exposure.
   - **Verification:** Move the camera and a character. Verify blur direction matches motion
     vectors. Verify disabled on mobile.

## Exposure and Tone Mapping

4. **R-2.9.4** — The engine **SHALL** provide auto exposure via luminance histogram with
   configurable min/max EV100 bounds and temporal smoothing.
   - **Rationale:** Auto exposure simulates human eye adaptation between dark and bright areas.
   - **Verification:** Transition from dark interior to bright exterior. Verify exposure adapts
     smoothly within configured bounds.

5. **R-2.9.5** — The engine **SHALL** provide ACES tone mapping with lift/gamma/gain color grading,
   LUT application, and HDR display output.
   - **Rationale:** ACES provides an industry-standard filmic curve; HDR output uses display
     capability.
   - **Verification:** Apply a color LUT. Verify output matches LUT transformation. Verify HDR10
     output on supported displays.

## Camera Effects

6. **R-2.9.6** — The engine **SHALL** provide film grain, chromatic aberration, lens flare, and
   vignette as configurable lightweight post-process effects.
   - **Rationale:** Camera simulation effects add cinematic character at negligible GPU cost.
   - **Verification:** Enable each effect. Verify visual result. Verify combined cost below 0.3 ms
     on mobile.

## Custom and Advanced Effects

7. **R-2.9.7** — The engine **SHALL** support custom post-process material graphs reading scene
   textures and outputting arbitrary color transforms, chainable in unlimited passes on desktop.
   - **Rationale:** Custom post-process enables unique stylization without engine code changes.
   - **Verification:** Create a grayscale post-process graph. Verify correct output. Chain 3 custom
     passes. Verify correct compositing order.

8. **R-2.9.8** — The engine **SHALL** provide local exposure adjusting per-region brightness via a
   downsampled tile grid to preserve detail in high-contrast scenes.
   - **Rationale:** Local exposure prevents detail loss when both bright and dark regions are on
     screen.
   - **Verification:** Render a scene with bright sky and dark interior visible. Verify both regions
     preserve detail. Verify disabled on mobile.

9. **R-2.9.9** — The engine **SHALL** provide panini projection correcting perspective warping at
   wide FOV.
   - **Rationale:** Panini projection reduces straight-line distortion at screen edges for wide-FOV
     games.
   - **Verification:** Enable at 110 degree FOV. Verify reduced edge warping. Verify center fidelity
     preserved.

10. **R-2.9.10** — The engine **SHALL** provide screen-space cavity and curvature enhancement with
    curvature and cavity modes using normal-buffer sampling, scaling from curvature-only on mobile
    to both on desktop.
    - **Rationale:** Cavity and curvature reveal micro- surface detail invisible to standard
      lighting.
    - **Verification:** Render detailed geometry. Verify darkened concavities and brightened
      convexities. Verify curvature-only on mobile.

11. **R-2.9.11** — The engine **SHALL** provide a node-based post-process graph editor compiling to
    optimized GPU compute dispatches within the post-processing pipeline.
    - **Rationale:** Visual graph editing enables custom image processing without shader code
      authoring.
    - **Verification:** Create a 5-node graph. Verify it compiles and runs. Verify output matches a
      manually coded equivalent.
