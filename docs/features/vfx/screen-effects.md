# 11.3 — Screen Effects

## Camera Effects

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-11.3.1 | Screen Shake | Procedural camera shake system driven by Perlin noise with configurable frequency, amplitude, decay, and directional bias. Shake events are additive and layer from multiple sources (explosions, impacts, spells) with amplitude clamping to prevent nausea. Supports rotational and translational shake independently, with accessibility options for reduced motion. | R-11.3.1 | None | Lightweight CPU effect; runs identically on all platforms. Mobile default amplitude is lower to reduce motion sickness on handheld devices. |
| F-11.3.2 | Motion Blur | Per-object and camera motion blur computed from velocity buffers. Per-object blur uses the previous-frame transform delta to generate per-pixel velocity; camera blur applies a directional filter based on camera angular and translational velocity. Tile-based reconstruction minimizes sampling cost. Intensity scales with frame rate to maintain consistent blur width. | R-11.3.2 | None | Mobile skips per-object blur; camera-only blur at half resolution. Disabled entirely on low-end mobile GPUs. Switch uses reduced sample count. |
| F-11.3.3 | Lens Flare | Screen-space lens flare system triggered by bright light sources and emissive surfaces. Flare ghosts, halos, and starbursts are generated procedurally from a light's screen position and occluded luminance sampled via depth queries. Artist-authored flare element templates control shape, color shift, and radial falloff. Temporal smoothing prevents popping on partial occlusion. | R-11.3.3 | None | Mobile limits flare elements to 2 ghosts (vs. 6 on desktop) and skips starburst generation. Disabled on low-end mobile GPUs. |

## Post-Process Distortions

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-11.3.4 | Chromatic Aberration and Film Grain | Color-channel-offset post-process simulating lens dispersion, with radial aberration increasing toward screen edges and event-driven pulses from impact points. Uses three-tap separated sampling with artist-controlled intensity and maximum pixel offset. Combined with animated blue-noise film grain and radial vignette as a final-pass overlay, with intensity parameters exposed to gameplay triggers for cinematic polish. | R-11.3.4 | None | Mobile disables chromatic aberration and film grain by default to save fill rate. Vignette is retained as a single-sample overlay (cheap). |
| F-11.3.5 | Heat Haze and Refraction | Screen-space distortion effect sampling the color buffer with per-pixel UV offsets driven by scrolling normal maps or particle distortion fields. Used for heat shimmer above fire, magical portals, and shockwave ripples. Distortion vectors are accumulated into a half-resolution buffer to bound cost when many refraction sources overlap in raid encounters. | R-11.3.5 | None | Mobile uses quarter-resolution distortion buffer. Disabled entirely on low-end mobile GPUs. Switch uses half-resolution (same as desktop). |

## Gameplay Overlays

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-11.3.6 | Damage Overlays and Screen Flash | Full-screen or directional color flash triggered by gameplay events — damage taken, healing received, status effects — with parameterized color, intensity, and decay curve per event type. Textured screen-space overlays render blood spatter, cracked glass, frost, or corruption effects with artist-authored dissolve masks. Multiple overlays and flashes composite additively with independent lifecycle timers and clamped total opacity. | R-11.3.6 | None | Lightweight full-screen overlay; runs on all platforms. Mobile limits concurrent overlay count to 2 (vs. 4 on desktop) to control overdraw. |
