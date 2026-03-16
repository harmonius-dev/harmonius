# User Stories -- 11.3 Screen Effects

## Camera Effects

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.3.1.1 | Player (P-23) | I want procedural camera shake driven by Perlin noise with configurable frequency, amplitude, and decay from nearby explosions, so that impacts feel powerful and visceral. | Camera displacement follows noise-based pattern with visible decay; shake intensity corresponds to distance from source | F-11.3.1 | R-11.3.1 |
| US-11.3.1.2 | Effects artist (P-12) | I want additive shake from multiple sources (explosions, impacts, spells) with total amplitude clamping, so that overlapping shake events do not accumulate to nausea-inducing levels. | Multiple simultaneous shakes layer additively; total amplitude clamped to configured maximum | F-11.3.1 | R-11.3.1 |
| US-11.3.1.3 | Engine tester (P-27) | I want to enable the reduced-motion accessibility setting and verify that screen shake amplitude is significantly reduced or disabled, so that players sensitive to motion sickness can play comfortably. | Reduced-motion setting attenuates or disables shake; no shake visible when fully disabled | F-11.3.1 | R-11.3.1 |
| US-11.3.2.1 | Player (P-23) | I want per-object and camera motion blur computed from velocity buffers, so that fast sword swings and dodge rolls blur naturally with consistent blur width regardless of frame rate. | Per-object blur along motion vector; camera blur directional; stationary objects remain sharp; blur width scales with frame rate | F-11.3.2 | R-11.3.2 |
| US-11.3.2.2 | Engine tester (P-27) | I want to verify that mobile disables motion blur entirely, Switch uses camera-only at half-res, and desktop runs full per-pixel blur, so that motion blur cost is appropriate per platform. | Mobile: disabled; Switch: camera-only half-res; desktop: full per-pixel blur | F-11.3.2 | R-11.3.2 |
| US-11.3.3.1 | Player (P-23) | I want screen-space lens flare with ghosts, halos, and starbursts from bright sources that smooth on partial occlusion, so that looking toward bright lights produces convincing optical artifacts. | Ghosts, halos, and starbursts at correct screen positions; temporal smoothing on partial occlusion prevents popping | F-11.3.3 | R-11.3.3 |
| US-11.3.3.2 | Effects artist (P-12) | I want artist-authored flare element templates controlling shape, color shift, and radial falloff per light, so that different light types (sun, lantern, muzzle flash) produce distinct flare styles. | Custom templates control shape, color, and falloff; distinct flare styles per light type | F-11.3.3 | R-11.3.3 |
| US-11.3.3.3 | Engine tester (P-27) | I want to verify that mobile limits flare elements to 2 ghosts with no starburst and disables flares entirely on low-end mobile GPUs, so that flare cost scales per device capability. | Mobile: 2 ghosts, no starburst; disabled on low-end mobile GPUs | F-11.3.3 | R-11.3.3 |

## Post-Process Distortions

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.3.4.1 | Effects artist (P-12) | I want chromatic aberration with radial and event-driven pulse modes combined with animated blue-noise film grain and vignette, so that the final image has cinematic lens character. | Visible RGB separation increasing toward edges; event-driven pulse at impact point; animated noise overlay; radial vignette darkening | F-11.3.4 | R-11.3.4 |
| US-11.3.4.2 | Engine tester (P-27) | I want to verify that chromatic aberration and film grain are disabled by default on mobile to save fill rate, with vignette retained as a cheap overlay, so that post-process cost is minimal on mobile. | Mobile: CA and grain disabled by default; vignette retained | F-11.3.4 | R-11.3.4 |
| US-11.3.5.1 | Effects artist (P-12) | I want screen-space distortion driven by scrolling normal maps for heat haze above fire, magical portals, and shockwave ripples, so that thermal and magical effects distort the view convincingly. | Visible screen-space distortion from normal maps; smooth continuous animation without tearing | F-11.3.5 | R-11.3.5 |
| US-11.3.5.2 | Engine tester (P-27) | I want to verify that mobile uses quarter-resolution distortion buffer, Switch and desktop use half-resolution, and the effect is disabled entirely on low-end mobile GPUs, so that distortion cost scales per platform. | Mobile: quarter-res; Switch/desktop: half-res; disabled on low-end mobile | F-11.3.5 | R-11.3.5 |

## Gameplay Overlays

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-11.3.6.1 | Player (P-23) | I want directional color flash and textured damage overlays (blood spatter, frost, cracked glass) triggered by gameplay events with configurable color, intensity, and decay, so that taking damage provides immediate visual feedback. | Directional flash with correct color, intensity, and decay; distinct response per damage type | F-11.3.6 | R-11.3.6 |
| US-11.3.6.2 | Effects artist (P-12) | I want artist-authored screen-space overlay textures with dissolve masks that composite additively with independent lifecycle timers, so that each damage type (fire, ice, poison) has a distinct screen effect. | Dissolve masks animate correctly; each damage type produces distinct overlay; independent lifecycle timers per overlay | F-11.3.6 | R-11.3.6 |
| US-11.3.6.3 | Engine tester (P-27) | I want to verify that mobile limits concurrent overlays to 2 and desktop to 4 with clamped total opacity, so that overlay overdraw is controlled per platform. | Mobile: 2 concurrent overlays; desktop: 4; composite opacity clamped | F-11.3.6 | R-11.3.6 |
