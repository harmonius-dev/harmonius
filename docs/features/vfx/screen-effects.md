# 11.3 — Screen Effects

## Camera Effects

### F-11.3.1 Screen Shake

Procedural camera shake system driven by Perlin noise with configurable frequency, amplitude,
decay, and directional bias. Shake events are additive and layer from multiple sources (explosions,
impacts, spells) with amplitude clamping to prevent nausea. Supports rotational and translational
shake independently, with accessibility options for reduced motion.

- **Requirements:** R-11.3.1
- **Dependencies:** None
- **Platform notes:** None

### F-11.3.2 Motion Blur

Per-object and camera motion blur computed from velocity buffers. Per-object blur uses the
previous-frame transform delta to generate per-pixel velocity; camera blur applies a directional
filter based on camera angular and translational velocity. Tile-based reconstruction minimizes
sampling cost. Intensity scales with frame rate to maintain consistent blur width.

- **Requirements:** R-11.3.2
- **Dependencies:** None
- **Platform notes:** None

### F-11.3.3 Lens Flare

Screen-space lens flare system triggered by bright light sources and emissive surfaces. Flare
ghosts, halos, and starbursts are generated procedurally from a light's screen position and
occluded luminance sampled via depth queries. Artist-authored flare element templates control
shape, color shift, and radial falloff. Temporal smoothing prevents popping on partial occlusion.

- **Requirements:** R-11.3.3
- **Dependencies:** None
- **Platform notes:** None

## Post-Process Distortions

### F-11.3.4 Chromatic Aberration and Film Grain

Color-channel-offset post-process simulating lens dispersion, with radial aberration increasing
toward screen edges and event-driven pulses from impact points. Uses three-tap separated sampling
with artist-controlled intensity and maximum pixel offset. Combined with animated blue-noise film
grain and radial vignette as a final-pass overlay, with intensity parameters exposed to gameplay
triggers for cinematic polish.

- **Requirements:** R-11.3.4
- **Dependencies:** None
- **Platform notes:** None

### F-11.3.5 Heat Haze and Refraction

Screen-space distortion effect sampling the color buffer with per-pixel UV offsets driven by
scrolling normal maps or particle distortion fields. Used for heat shimmer above fire, magical
portals, and shockwave ripples. Distortion vectors are accumulated into a half-resolution buffer
to bound cost when many refraction sources overlap in raid encounters.

- **Requirements:** R-11.3.5
- **Dependencies:** None
- **Platform notes:** None

## Gameplay Overlays

### F-11.3.6 Damage Overlays and Screen Flash

Full-screen or directional color flash triggered by gameplay events — damage taken, healing
received, status effects — with parameterized color, intensity, and decay curve per event type.
Textured screen-space overlays render blood spatter, cracked glass, frost, or corruption effects
with artist-authored dissolve masks. Multiple overlays and flashes composite additively with
independent lifecycle timers and clamped total opacity.

- **Requirements:** R-11.3.6
- **Dependencies:** None
- **Platform notes:** None
