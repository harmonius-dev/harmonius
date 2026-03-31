# User Stories -- 11.3 Screen Effects

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-11.3.1.1 | effects artist (P-12)   |
| US-11.3.1.2 | effects artist (P-12)   |
| US-11.3.1.3 | game designer (P-5)     |
| US-11.3.2.1 | effects artist (P-12)   |
| US-11.3.2.2 | technical artist (P-13) |
| US-11.3.2.3 | engine developer (P-26) |
| US-11.3.3.1 | effects artist (P-12)   |
| US-11.3.3.2 | effects artist (P-12)   |
| US-11.3.3.3 | engine developer (P-26) |
| US-11.3.4.1 | effects artist (P-12)   |
| US-11.3.4.2 | effects artist (P-12)   |
| US-11.3.4.3 | engine developer (P-26) |
| US-11.3.5.1 | effects artist (P-12)   |
| US-11.3.5.2 | technical artist (P-13) |
| US-11.3.5.3 | engine developer (P-26) |
| US-11.3.6.1 | effects artist (P-12)   |
| US-11.3.6.2 | effects artist (P-12)   |
| US-11.3.6.3 | game designer (P-5)     |

1. **US-11.3.1.1** — **As a** effects artist (P-12), **I want** procedural camera shake driven by
   Perlin noise with configurable frequency, amplitude, and decay, **so that** explosions and
   impacts feel visceral.

2. **US-11.3.1.2** — **As a** effects artist (P-12), **I want** additive shake from multiple sources
   with total amplitude clamping, **so that** overlapping events do not accumulate to
   nausea-inducing levels.

3. **US-11.3.1.3** — **As a** game designer (P-5), **I want** a reduced-motion accessibility setting
   that attenuates or disables screen shake, **so that** players sensitive to motion sickness can
   play comfortably.

4. **US-11.3.2.1** — **As a** effects artist (P-12), **I want** per-object and camera motion blur
   from velocity buffers with tile-based reconstruction, **so that** fast sword swings and dodge
   rolls blur naturally.

5. **US-11.3.2.2** — **As a** technical artist (P-13), **I want** blur intensity scaling with frame
   rate, **so that** consistent blur width is maintained regardless of performance.

6. **US-11.3.2.3** — **As a** engine developer (P-26), **I want** mobile to disable motion blur,
   Switch to use camera-only at half-res, and desktop to run full per-pixel, **so that** blur cost
   is appropriate per platform.

7. **US-11.3.3.1** — **As a** effects artist (P-12), **I want** screen-space lens flare with
   procedural ghosts, halos, and starbursts from bright sources with temporal smoothing on partial
   occlusion, **so that** bright lights produce convincing optical artifacts.

8. **US-11.3.3.2** — **As a** effects artist (P-12), **I want** artist-authored flare element
   templates controlling shape, color shift, and radial falloff, **so that** different light types
   produce distinct flare styles.

9. **US-11.3.3.3** — **As a** engine developer (P-26), **I want** mobile limited to 2 flare ghosts
   with no starburst and flares disabled on low-end GPUs, **so that** flare cost scales per device
   capability.

10. **US-11.3.4.1** — **As a** effects artist (P-12), **I want** chromatic aberration with radial
    and event-driven pulse modes, **so that** impacts and cinematic moments have lens character.

11. **US-11.3.4.2** — **As a** effects artist (P-12), **I want** animated blue-noise film grain and
    radial vignette as a final-pass overlay with gameplay-driven intensity, **so that** the image
    has cinematic texture.

12. **US-11.3.4.3** — **As a** engine developer (P-26), **I want** chromatic aberration and film
    grain disabled by default on mobile with vignette retained, **so that** post-process cost is
    minimal on mobile.

13. **US-11.3.5.1** — **As a** effects artist (P-12), **I want** screen-space distortion driven by
    scrolling normal maps for heat haze, magical portals, and shockwave ripples, **so that** thermal
    and magical effects distort the view.

14. **US-11.3.5.2** — **As a** technical artist (P-13), **I want** distortion vectors accumulated
    into a half-resolution buffer, **so that** cost is bounded when many refraction sources overlap.

15. **US-11.3.5.3** — **As a** engine developer (P-26), **I want** mobile to use quarter-resolution
    distortion and the effect disabled on low-end GPUs, **so that** distortion cost scales per
    platform.

16. **US-11.3.6.1** — **As a** effects artist (P-12), **I want** full-screen and directional color
    flashes triggered by gameplay events with parameterized color, intensity, and decay, **so that**
    damage provides immediate visual feedback.

17. **US-11.3.6.2** — **As a** effects artist (P-12), **I want** textured screen-space overlays with
    dissolve masks and independent lifecycle timers per damage type, **so that** fire, ice, and
    poison each have distinct effects.

18. **US-11.3.6.3** — **As a** game designer (P-5), **I want** concurrent overlays limited (2 on
    mobile, 4 on desktop) with clamped total opacity, **so that** overlay overdraw is controlled per
    platform.
