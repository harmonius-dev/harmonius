# 5.2 — Spatial Audio

## 3D Positioning

| ID      | Feature                          |
|---------|----------------------------------|
| F-5.2.1 | 3D Sound Positioning and Doppler |
| F-5.2.2 | Distance Attenuation Curves      |

1. **F-5.2.1** — Position sound sources in world space and resolve per-listener panning, distance
   gain, and Doppler pitch shift every audio frame. The system interpolates source and listener
   transforms between game ticks to produce smooth, artifact-free motion even when game-thread
   updates arrive at a lower rate than the audio buffer callback.
   - **Deps:** F-5.1.1, F-5.1.2
   - **Platform:** 3D positioning runs on all platforms. Per-voice Doppler calculation is
     lightweight. Active spatialized voice count governed by F-5.1.4 tier limits.
2. **F-5.2.2** — Apply configurable distance attenuation models (inverse, inverse-squared, linear,
   logarithmic, and custom user-defined curves) to each sound source. Per-source minimum and maximum
   distance parameters control the rolloff range. Tunable attenuation lets designers balance
   audibility of distant siege weaponry against nearby footsteps.
   - **Deps:** F-5.2.1
   - **Platform:** Attenuation curves are per-source lookups, lightweight on all platforms. No
     platform-specific scaling required.

## Binaural and Ambisonics

| ID      | Feature                          |
|---------|----------------------------------|
| F-5.2.3 | HRTF Binaural Rendering          |
| F-5.2.4 | Ambisonics Encoding and Decoding |

1. **F-5.2.3** — Render spatialized audio through head-related transfer functions for headphone
   output, providing accurate elevation and front-back cues. HRTF datasets are loaded as swappable
   profiles (SOFA format) to accommodate different head sizes and personalization. A fast
   frequency-domain convolution path handles the per-voice HRTF filter within the voice budget.
   - **Deps:** F-5.2.1
   - **Platform:** On platforms with native spatial audio APIs (Windows Spatial Audio, Apple Spatial
     Audio), the engine can delegate HRTF processing to the OS renderer.
2. **F-5.2.4** — Encode sound sources and ambience beds into first- or third-order Ambisonics for
   intermediate mixing, then decode to the listener's output format (stereo, 5.1, 7.1, binaural).
   Ambisonics provides a speaker-layout-agnostic representation that simplifies rotation, reverb
   return mixing, and smooth source panning across arbitrary channel configurations.
   - **Deps:** F-5.2.1, F-5.1.3
   - **Platform:** Ambisonics order scales per tier: mobile first-order only, desktop up to
     third-order. Higher orders require more channels and CPU for rotation.

## Occlusion and Propagation

| ID      | Feature                                |
|---------|----------------------------------------|
| F-5.2.5 | Occlusion and Obstruction Filtering    |
| F-5.2.6 | Sound Propagation via Portals and Rays |

1. **F-5.2.5** — Attenuate and low-pass filter sounds whose direct path to the listener is blocked
   by geometry (occlusion) or partially blocked (obstruction). Occlusion queries ray-cast against
   the shared BVH spatial index used by physics, rendering, and AI, with material-dependent
   transmission loss coefficients.
   - **Deps:** F-5.2.1, F-1.9.4 (Unified Spatial Query API), F-1.9.9 (AI Perception Integration)
   - **Platform:** Occlusion ray count per voice scales per tier: mobile 1 ray, Switch 2, desktop 4.
     Mobile uses simplified binary occlusion. Desktop uses multi-ray material-weighted transmission.
2. **F-5.2.6** — Model indirect sound paths through portals (doorways, windows, tunnels) and
   reflective surfaces using a hybrid ray-and-portal graph. The propagation solver runs
   asynchronously at a reduced rate and feeds delay, gain, and filter parameters into per-voice
   diffraction and reflection taps. Enables players to hear combat echoing through corridors and
   around corners.
   - **Deps:** F-5.2.5
   - **Platform:** Propagation path complexity scales per tier: mobile uses portal-only (no ray
     reflections), Switch adds 1-bounce reflections, desktop supports full multi-bounce ray + portal
     propagation. Solver update rate: mobile every 4-8 frames, desktop every 1-2 frames.

## Reverb and Reflections

| ID      | Feature                            |
|---------|------------------------------------|
| F-5.2.7 | Reverb Zones and Early Reflections |

1. **F-5.2.7** — Define axis-aligned or convex reverb volumes that apply late reverberation and
   early reflection patterns to sounds playing within or propagating through them. Zones blend
   smoothly when the listener crosses boundaries, and nested zones combine via priority ordering.
   Early reflections are derived from room geometry or authored reflection patterns.
   - **Deps:** F-5.2.1, F-5.3.2
   - **Platform:** Active reverb zone count scales per tier: mobile 1-2 concurrent, Switch 3-4,
     desktop 6+. Early reflections disabled on mobile; uses algorithmic reverb only (see F-5.3.3).

## Acoustic Materials and Parallel Propagation

| ID       | Feature                              |
|----------|--------------------------------------|
| F-5.2.8  | Acoustic Material Properties         |
| F-5.2.9  | Parallel Sound Propagation           |
| F-5.2.10 | 2D Sound Propagation                 |

1. **F-5.2.8** — Extend physics materials with per-surface acoustic properties: per-frequency-band
   absorption coefficients, transmission loss for occlusion, and surface scattering. The propagation
   solver (F-5.2.6) consumes these values when ray-casting through geometry, producing emergent
   acoustics — stone caves reverberate, carpeted rooms sound muffled, glass lets high frequencies
   through while blocking lows — with no manual reverb zone placement.
   - **Deps:** F-5.2.5, F-5.2.6, F-4.2.1 (Physics Materials)
   - **Platform:** Mobile uses 3-band absorption; desktop uses 8-band to match the DSP filter
     resolution. Material data is asset-authored and consumes negligible CPU at runtime.
2. **F-5.2.9** — Evaluate sound propagation in parallel across worker threads via the job system. A
   `par_for_each` over active sound sources filters by `Changed<Transform>` and amortizes ray
   tracing across frames (1/N sources per frame) so propagation scales with listener-visible source
   count rather than world population. Change detection prevents recomputation for static sources.
   - **Deps:** F-5.2.6, F-14.3.3 (Job System), F-1.1.x (Change Detection)
   - **Platform:** Mobile amortizes across 8 frames; desktop across 2 frames. Per-tier worker thread
     count bounds parallelism.
3. **F-5.2.10** — Trace sound propagation in 2D for top-down, side-view, and 2.5D games where no
   vertical structure exists. The solver collapses geometry to a 2D representation and casts rays
   within the XY plane, producing occlusion and portal propagation without paying 3D tracing cost.
   Enables stealth games with room-based audio and 2D platformers with environmental echo.
   - **Deps:** F-5.2.6
   - **Platform:** 2D propagation is cheaper than 3D on every tier — mobile supports 2D propagation
     at full update rate where 3D would be disabled.
