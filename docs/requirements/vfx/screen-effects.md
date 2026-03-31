# R-11.3 -- Screen Effect Requirements

## Camera Effects

1. **R-11.3.1** — The engine **SHALL** apply procedural camera shake driven by Perlin noise with
   configurable frequency, amplitude, decay, and directional bias, supporting additive layering from
   multiple sources with amplitude clamping and accessibility options for reduced motion.
   - **Rationale:** Screen shake communicates impact intensity while accessibility options prevent
     motion sickness.
   - **Verification:** Trigger shake and verify noise-based displacement with decay. Trigger
     multiple sources and confirm additive layering with clamped total. Enable reduced motion and
     verify attenuation or disable.

2. **R-11.3.2** — The engine **SHALL** render per-object and camera motion blur from velocity
   buffers, computing per-pixel velocity from previous-frame deltas, using tile-based reconstruction
   with intensity scaling by frame rate.
   - **Rationale:** Motion blur enhances perceived speed and cinematic quality during rapid
     movement.
   - **Verification:** Move an object while camera is stationary and verify per-object blur. Move
     camera with stationary objects and confirm directional blur. Change frame rate and verify width
     scales consistently.

3. **R-11.3.3** — The engine **SHALL** render screen-space lens flares from bright sources,
   generating procedural ghosts, halos, and starbursts via depth-queried luminance, with
   artist-authored templates and temporal smoothing.
   - **Rationale:** Lens flares simulate physical camera optics, enhancing perceived brightness and
     cinematic polish.
   - **Verification:** Point camera at a bright light and verify ghost, halo, and starburst
     elements. Partially occlude and confirm temporal smoothing. Apply custom template and verify
     shape, color, and falloff.

## Post-Process Distortions

4. **R-11.3.4** — The engine **SHALL** render chromatic aberration as three-tap color-channel offset
   with radial intensity and event-driven pulses, combined with animated film grain and radial
   vignette, with intensity exposed to gameplay triggers.
   - **Rationale:** Chromatic aberration and film grain add cinematic lens character with
     gameplay-driven pulses.
   - **Verification:** Enable CA and verify RGB separation increasing toward edges. Trigger a pulse
     and confirm transient burst. Enable grain and verify animated overlay. Adjust vignette and
     confirm radial darkening.

5. **R-11.3.5** — The engine **SHALL** render screen-space distortion by sampling the color buffer
   with per-pixel UV offsets from scrolling normal maps or particle fields, accumulating into a
   half-resolution buffer.
   - **Rationale:** Screen-space distortion is needed for heat shimmer, magical portals, and
     shockwave effects.
   - **Verification:** Place heat haze above fire and verify distortion. Spawn overlapping sources
     and confirm half-resolution buffer bounds cost. Verify smooth animation from scrolling normals.

## Gameplay Overlays

6. **R-11.3.6** — The engine **SHALL** render full-screen and directional color flashes with
   parameterized color, intensity, and decay per event type, compositing textured overlays with
   dissolve masks and independent lifecycle timers at clamped total opacity.
   - **Rationale:** Damage overlays and flashes provide immediate feedback for health changes and
     status effects.
   - **Verification:** Trigger damage and verify directional flash with correct parameters. Apply
     textured overlay and verify dissolve animation. Trigger multiple simultaneous overlays and
     confirm clamped composite opacity.
