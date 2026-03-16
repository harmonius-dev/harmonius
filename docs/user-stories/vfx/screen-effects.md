# User Stories -- 11.3 Screen Effects

## US-11.3.1.1 Feel Explosions Through Camera Shake

**As a** player (P-23), **I want** procedural camera shake driven by Perlin noise with configurable
frequency, amplitude, and decay from nearby explosions, **so that** impacts feel powerful and
visceral.

## US-11.3.1.2 Layer Multiple Shake Sources With Amplitude Clamping

**As a** effects artist (P-12), **I want** additive shake from multiple sources (explosions,
impacts, spells) with total amplitude clamping, **so that** overlapping shake events do not
accumulate to nausea-inducing levels.

## US-11.3.1.3 Validate Reduced-Motion Accessibility Option

**As an** engine tester (P-27), **I want** to enable the reduced-motion accessibility setting and
verify that screen shake amplitude is significantly reduced or disabled, **so that** players
sensitive to motion sickness can play comfortably.

## US-11.3.2.1 See Per-Object Motion Blur During Fast Combat

**As a** player (P-23), **I want** per-object and camera motion blur computed from velocity buffers,
**so that** fast sword swings and dodge rolls blur naturally with consistent blur width regardless
of frame rate.

## US-11.3.2.2 Validate Motion Blur Scaling Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile disables motion blur entirely,
Switch uses camera-only at half-res, and desktop runs full per-pixel blur, **so that** motion blur
cost is appropriate per platform.

## US-11.3.3.1 See Lens Flare From Bright Lights and Emissive Surfaces

**As a** player (P-23), **I want** screen-space lens flare with ghosts, halos, and starbursts from
bright sources that smooth on partial occlusion, **so that** looking toward bright lights produces
convincing optical artifacts.

## US-11.3.3.2 Author Custom Flare Element Templates

**As a** effects artist (P-12), **I want** artist-authored flare element templates controlling
shape, color shift, and radial falloff per light, **so that** different light types (sun, lantern,
muzzle flash) produce distinct flare styles.

## US-11.3.3.3 Validate Flare Element Count on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile limits flare elements to 2 ghosts
with no starburst and disables flares entirely on low-end mobile GPUs, **so that** flare cost scales
per device capability.

## US-11.3.4.1 Add Cinematic Post-Process Polish With CA and Film Grain

**As a** effects artist (P-12), **I want** chromatic aberration with radial and event-driven pulse
modes combined with animated blue-noise film grain and vignette, **so that** the final image has
cinematic lens character.

## US-11.3.4.2 Validate CA and Grain Disabled by Default on Mobile

**As an** engine tester (P-27), **I want** to verify that chromatic aberration and film grain are
disabled by default on mobile to save fill rate, with vignette retained as a cheap overlay,
**so that** post-process cost is minimal on mobile.

## US-11.3.5.1 Create Heat Shimmer Above Fire Sources

**As a** effects artist (P-12), **I want** screen-space distortion driven by scrolling normal maps
for heat haze above fire, magical portals, and shockwave ripples, **so that** thermal and magical
effects distort the view convincingly.

## US-11.3.5.2 Validate Distortion Buffer Resolution Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses quarter-resolution distortion
buffer, Switch and desktop use half-resolution, and the effect is disabled entirely on low-end
mobile GPUs, **so that** distortion cost scales per platform.

## US-11.3.6.1 Flash the Screen Red When Damaged

**As a** player (P-23), **I want** directional color flash and textured damage overlays (blood
spatter, frost, cracked glass) triggered by gameplay events with configurable color, intensity, and
decay, **so that** taking damage provides immediate visual feedback.

## US-11.3.6.2 Author Custom Damage Overlay Textures With Dissolve Masks

**As a** effects artist (P-12), **I want** artist-authored screen-space overlay textures with
dissolve masks that composite additively with independent lifecycle timers, **so that** each damage
type (fire, ice, poison) has a distinct screen effect.

## US-11.3.6.3 Validate Concurrent Overlay Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile limits concurrent overlays to 2 and
desktop to 4 with clamped total opacity, **so that** overlay overdraw is controlled per platform.
