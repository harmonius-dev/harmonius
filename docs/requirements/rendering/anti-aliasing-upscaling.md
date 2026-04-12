# R-2.6 -- Anti-Aliasing and Upscaling Requirements

## Anti-Aliasing

1. **R-2.6.1** — The engine **SHALL** provide temporal anti-aliasing blending jittered samples with
   reprojected history, with temporal weighting clamps rejecting disoccluded pixels to reduce
   ghosting.
   - **Rationale:** TAA reduces geometric aliasing and specular shimmer across frames while clamps
     prevent ghosting during fast motion.
   - **Verification:** Render a scene with specular surfaces. Verify shimmer is eliminated. Pan the
     camera quickly and verify no visible ghosting.

2. **R-2.6.2** — The engine **SHALL** provide FXAA as a lightweight single-pass AA fallback with no
   temporal history dependency.
   - **Rationale:** FXAA provides basic AA on hardware where TAA is too costly or temporal history
     is unavailable.
   - **Verification:** Enable FXAA alone. Verify edge smoothing with no temporal dependency. Measure
     cost below 0.5 ms on mobile.

3. **R-2.6.3** — The engine **SHALL** support hardware MSAA at 2x and 4x in the forward rendering
   path.
   - **Rationale:** MSAA provides sub-pixel sampling for scenarios where temporal AA is unsuitable.
   - **Verification:** Enable 4x MSAA. Verify sub-pixel edge quality. Verify MSAA is unavailable in
     the deferred path.

## Upscaling

4. **R-2.6.4** — The engine **SHALL** provide a temporal super-resolution upscaler rendering
   high-resolution output from lower internal resolution with quality, balanced, and performance
   modes.
   - **Rationale:** TSR achieves higher display resolution at lower rendering cost.
   - **Verification:** Render at 1080p internal to 4K output. Compare against native 4K. Verify PSNR
     above 35 dB in quality mode.

5. **R-2.6.5** — The engine **SHALL** provide checkerboard rendering at half resolution with
   temporal reconstruction.
   - **Rationale:** Checkerboard halves pixel count while maintaining output resolution on
     bandwidth-limited platforms.
   - **Verification:** Enable checkerboard at 1080p. Verify reconstructed output matches native
     within 30 dB PSNR.

6. **R-2.6.6** — The engine **SHALL** integrate DLSS, FSR, and XeSS through an abstraction layer
   with graceful fallback to built-in TSR when no vendor SDK is available.
   - **Rationale:** Vendor upscalers provide optimized results on their hardware; fallback ensures
     coverage.
   - **Verification:** Enable each vendor upscaler. Verify output quality. Remove vendor SDK. Verify
     automatic fallback to TSR.

## Frame Generation and Latency

7. **R-2.6.7** — The engine **SHALL** support AI frame generation synthesizing intermediate frames
   with latency compensation via input-to-display pipeline optimization.
   - **Rationale:** Frame generation multiplies effective frame rate without additional rendering
     cost.
   - **Verification:** Enable frame gen at 30 fps. Verify output reaches 60 fps. Measure input
     latency and verify it does not exceed non-frame-gen baseline by more than 5 ms.

8. **R-2.6.8** — The engine **SHALL** integrate Reflex and Anti-Lag for end-to-end render latency
   optimization, synchronizing CPU submission with GPU back-pressure.
   - **Rationale:** Latency reduction ensures frame gen does not increase perceived input lag.
   - **Verification:** Measure end-to-end latency with and without Reflex. Verify at least 30%
     reduction. Verify frame rate is not affected.

## SMAA

9. **R-2.6.9** — The engine **SHALL** provide SMAA as a multi-pass spatial anti-aliasing technique
   with configurable quality presets, serving as an intermediate option between FXAA and TAA.
   - **Rationale:** SMAA offers higher edge quality than FXAA without temporal history dependency,
     filling a gap between lightweight and temporal AA.
   - **Verification:** Enable SMAA. Verify edge detection, blending weight, and neighborhood
     blending passes execute. Compare edge quality against FXAA and verify measurable improvement on
     aliased geometry.
