# 5.2 — Spatial Audio

## 3D Positioning

### F-5.2.1 3D Sound Positioning and Doppler

Position sound sources in world space and resolve per-listener panning, distance gain, and Doppler
pitch shift every audio frame. The system interpolates source and listener transforms between game
ticks to produce smooth, artifact-free motion even when game-thread updates arrive at a lower rate
than the audio buffer callback.

- **Requirements:** R-5.2.1
- **Dependencies:** F-5.1.1, F-5.1.2
- **Platform notes:** 3D positioning runs on all platforms. Per-voice Doppler calculation is
  lightweight. Active spatialized voice count governed by F-5.1.4 tier limits.

### F-5.2.2 Distance Attenuation Curves

Apply configurable distance attenuation models (inverse, inverse-squared, linear, logarithmic, and
custom user-defined curves) to each sound source. Per-source minimum and maximum distance parameters
control the rolloff range. In MMO open worlds, tunable attenuation lets designers balance audibility
of distant siege weaponry against nearby footsteps without global gain compression.

- **Requirements:** R-5.2.2
- **Dependencies:** F-5.2.1
- **Platform notes:** Attenuation curves are per-source lookups, lightweight on all platforms. No
  platform-specific scaling required.

## Binaural and Ambisonics

### F-5.2.3 HRTF Binaural Rendering

Render spatialized audio through head-related transfer functions for headphone output, providing
accurate elevation and front-back cues. HRTF datasets are loaded as swappable profiles (SOFA format)
to accommodate different head sizes and personalization. A fast frequency-domain convolution path
handles the per-voice HRTF filter within the voice budget.

- **Requirements:** R-5.2.3
- **Dependencies:** F-5.2.1
- **Platform notes:** On platforms with native spatial audio APIs (Windows Spatial Audio, Apple
  Spatial Audio), the engine can delegate HRTF processing to the OS renderer.

### F-5.2.4 Ambisonics Encoding and Decoding

Encode sound sources and ambience beds into first- or third-order Ambisonics for intermediate
mixing, then decode to the listener's output format (stereo, 5.1, 7.1, binaural). Ambisonics
provides a speaker-layout-agnostic representation that simplifies rotation, reverb return mixing,
and smooth source panning across arbitrary channel configurations.

- **Requirements:** R-5.2.4
- **Dependencies:** F-5.2.1, F-5.1.3
- **Platform notes:** Ambisonics order scales per tier: mobile first-order only, desktop up to
  third-order. Higher orders require more channels and CPU for rotation.

## Occlusion and Propagation

### F-5.2.5 Occlusion and Obstruction Filtering

Attenuate and low-pass filter sounds whose direct path to the listener is blocked by geometry
(occlusion) or partially blocked (obstruction). Occlusion queries ray-cast against the shared BVH
spatial index used by physics, rendering, and AI, with material-dependent transmission loss
coefficients so that a wooden door and a stone wall produce different muffling characteristics.

- **Requirements:** R-5.2.5
- **Dependencies:** F-5.2.1, F-1.9.4 (Unified Spatial Query API), F-1.9.9 (AI Perception
  Integration)
- **Platform notes:** Occlusion ray count per voice scales per tier: mobile 1 ray, Switch 2, desktop
  4. Mobile uses simplified binary occlusion (occluded/not). Desktop uses multi-ray
  material-weighted transmission.

### F-5.2.6 Sound Propagation via Portals and Rays

Model indirect sound paths through portals (doorways, windows, tunnels) and reflective surfaces
using a hybrid ray-and-portal graph. The propagation solver runs asynchronously at a reduced rate
and feeds delay, gain, and filter parameters into per-voice diffraction and reflection taps. This
enables players in an MMO dungeon to hear combat echoing through corridors and around corners.

- **Requirements:** R-5.2.6
- **Dependencies:** F-5.2.5
- **Platform notes:** Propagation path complexity scales per tier: mobile uses portal- only (no ray
  reflections), Switch adds 1-bounce reflections, desktop supports full multi-bounce ray + portal
  propagation. Solver update rate: mobile every 4-8 frames, desktop every 1-2 frames.

## Reverb and Reflections

### F-5.2.7 Reverb Zones and Early Reflections

Define axis-aligned or convex reverb volumes that apply late reverberation and early reflection
patterns to sounds playing within or propagating through them. Zones blend smoothly when the
listener crosses boundaries, and nested zones combine via priority ordering. Early reflections are
derived from room geometry or authored reflection patterns to reinforce spatial cues in enclosed
environments such as taverns, caves, and throne rooms.

- **Requirements:** R-5.2.7
- **Dependencies:** F-5.2.1, F-5.3.2
- **Platform notes:** Active reverb zone count scales per tier: mobile 1-2 concurrent, Switch 3-4,
  desktop 6+. Early reflections disabled on mobile; uses algorithmic reverb only (see F-5.3.3).
