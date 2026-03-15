# R-11.3 — Screen Effect Requirements

## R-11.3.1 Screen Shake

The engine **SHALL** apply procedural camera shake driven by Perlin noise with configurable
frequency, amplitude, decay, and directional bias, supporting additive layering from multiple
simultaneous sources with amplitude clamping and accessibility options for reduced motion.

- **Derived from:** [F-11.3.1](../../features/vfx/screen-effects.md)
- **Rationale:** Screen shake communicates impact and intensity of gameplay events such as
  explosions and heavy hits while accessibility options prevent motion sickness.
- **Verification:** Trigger a shake event and verify camera displacement follows a noise-based
  pattern with visible decay over time; trigger multiple simultaneous shake sources and
  confirm additive layering with clamped total amplitude; enable reduced motion accessibility
  and verify shake intensity is attenuated or disabled.

## R-11.3.2 Motion Blur

The engine **SHALL** render per-object and camera motion blur from velocity buffers, computing
per-pixel velocity from previous-frame transform deltas for objects and camera angular and
translational velocity for camera blur, using tile-based reconstruction with intensity scaling
by frame rate.

- **Derived from:** [F-11.3.2](../../features/vfx/screen-effects.md)
- **Rationale:** Motion blur enhances perceived speed and cinematic quality during rapid camera
  and object movement.
- **Verification:** Move an object while the camera is stationary and verify per-object blur
  along its motion vector; move the camera with stationary objects and confirm directional
  camera blur; verify stationary objects remain sharp; change frame rate and confirm blur
  width scales to maintain consistent visual appearance.

## R-11.3.3 Lens Flare

The engine **SHALL** render screen-space lens flares triggered by bright light sources,
generating procedural ghosts, halos, and starbursts from the light's screen position and
occluded luminance via depth queries, with artist-authored flare element templates and
temporal smoothing on partial occlusion.

- **Derived from:** [F-11.3.3](../../features/vfx/screen-effects.md)
- **Rationale:** Lens flares simulate physical camera optics, enhancing perceived brightness
  of strong light sources and adding cinematic polish.
- **Verification:** Point the camera at a bright light and verify visible ghost, halo, and
  starburst elements appear at the correct screen positions; partially occlude the light and
  confirm temporal smoothing prevents popping; apply a custom flare template and verify
  shape, color shift, and radial falloff match the authored parameters.

## R-11.3.4 Chromatic Aberration and Film Grain

The engine **SHALL** render chromatic aberration as a three-tap color-channel-offset
post-process with radial intensity increasing toward screen edges and event-driven pulses,
combined with animated blue-noise film grain and radial vignette as a final-pass overlay
with intensity parameters exposed to gameplay triggers.

- **Derived from:** [F-11.3.4](../../features/vfx/screen-effects.md)
- **Rationale:** Chromatic aberration and film grain add cinematic lens character and visual
  texture to the final image, with gameplay-driven pulses enhancing impact feedback.
- **Verification:** Enable chromatic aberration and verify visible RGB separation increasing
  toward screen edges; trigger an event-driven pulse and confirm a transient aberration
  burst at the impact point; enable film grain and verify animated noise overlay; adjust
  vignette intensity and confirm radial darkening at screen edges.

## R-11.3.5 Heat Haze and Refraction

The engine **SHALL** render screen-space distortion by sampling the color buffer with per-pixel
UV offsets driven by scrolling normal maps or particle distortion fields, accumulating
distortion vectors into a half-resolution buffer to bound cost when multiple refraction
sources overlap.

- **Derived from:** [F-11.3.5](../../features/vfx/screen-effects.md)
- **Rationale:** Screen-space refraction distortion is required for heat shimmer, magical
  portals, and shockwave effects common in combat and environmental VFX.
- **Verification:** Place a heat haze emitter above a fire source and verify visible
  screen-space distortion; spawn multiple overlapping refraction sources and confirm the
  half-resolution accumulation buffer bounds the rendering cost; verify distortion vectors
  from scrolling normal maps produce smooth, continuous animation without tearing.

## R-11.3.6 Damage Overlays and Screen Flash

The engine **SHALL** render full-screen and directional color flashes triggered by gameplay
events with parameterized color, intensity, and decay curve per event type, compositing
textured screen-space overlays with artist-authored dissolve masks and independent lifecycle
timers at clamped total opacity.

- **Derived from:** [F-11.3.6](../../features/vfx/screen-effects.md)
- **Rationale:** Damage overlays and screen flashes provide immediate visual feedback for
  health changes and status effects, reinforcing gameplay readability.
- **Verification:** Trigger a damage event and verify a directional color flash appears with
  correct color, intensity, and decay; trigger a healing event and confirm a distinct flash
  response; apply a textured overlay with a dissolve mask and verify the dissolve animates
  correctly; trigger multiple simultaneous overlays and confirm composite opacity is clamped.
