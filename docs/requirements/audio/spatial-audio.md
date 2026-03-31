# R-5.2 -- Spatial Audio Requirements

## 3D Positioning

1. **R-5.2.1** — The engine **SHALL** position sound sources in world space and resolve per-listener
   panning, distance gain, and Doppler pitch shift every audio frame. The engine **SHALL**
   interpolate source and listener transforms between game ticks to produce smooth, artifact-free
   motion.
   - **Rationale:** Players localize events by sound direction; interpolation eliminates clicks when
     the game tick rate is lower than the audio buffer rate.
   - **Verification:** Integration test: move a source at constant velocity past a listener and
     verify Doppler pitch matches the expected ratio within 1%. Verify no clicks at 30 Hz game tick
     rate with 100 Hz audio buffer rate.

2. **R-5.2.2** — The engine **SHALL** support inverse, inverse-squared, linear, logarithmic, and
   custom user-defined attenuation curves per source. Each source **SHALL** have configurable
   minimum and maximum distance parameters.
   - **Rationale:** Different sounds require different rolloff; per-source min/max distance lets
     designers balance distant weaponry against nearby footsteps.
   - **Verification:** Unit test: place a source at 10 distances between min and max for each model
     and verify gain matches the formula within 0.1%.

## Binaural and Ambisonics

3. **R-5.2.3** — The engine **SHALL** render spatialized audio through HRTF using SOFA-format
   datasets. Profiles **SHALL** be swappable at runtime within one audio buffer. Per-voice HRTF
   filtering **SHALL** use frequency-domain convolution.
   - **Rationale:** HRTF provides accurate elevation and front-back cues on headphones. Runtime
     profile swapping accommodates different head sizes.
   - **Verification:** Integration test: render a source at 0, 90, 180, and 270 degrees azimuth and
     verify channel differences match the dataset within 1 dB and 0.1 ms.

4. **R-5.2.4** — The engine **SHALL** encode sources into first- or third-order Ambisonics and
   decode to stereo, 5.1, 7.1, and binaural. Sound field rotation **SHALL** produce correct
   coefficient shifts for listener head tracking.
   - **Rationale:** Ambisonics provides a speaker-agnostic representation that simplifies rotation,
     reverb return mixing, and smooth panning.
   - **Verification:** Unit test: encode a mono source at a known azimuth into first-order
     Ambisonics and verify W, X, Y, Z coefficients match spherical harmonics within 0.1%. Rotate 90
     degrees and verify shift.

5. **R-5.2.4a** — The engine **SHALL** support first-order Ambisonics on mobile and up to
   third-order on desktop. Order **SHALL** be configurable per platform tier at engine
   initialization.
   - **Rationale:** Higher orders require more channels and CPU; mobile cannot afford third-order
     overhead.
   - **Verification:** Unit test: initialize first-order on mobile and verify 4 channels.
     Third-order on desktop and verify 16 channels.

## Occlusion and Propagation

6. **R-5.2.5** — The engine **SHALL** attenuate and low-pass filter sounds whose direct path is
   blocked by geometry. Occlusion **SHALL** ray-cast against the shared BVH spatial index (not a
   separate structure). Per-material transmission loss coefficients **SHALL** be supported.
   - **Rationale:** Sound occlusion matching visual geometry reinforces spatial immersion. The
     shared BVH avoids a duplicate spatial index.
   - **Verification:** Integration test: place a source behind a wood wall (12 dB loss) and verify
     attenuation of 12 dB +/- 1 dB with low-pass filtering. Verify shared BVH usage via
     instrumentation.

7. **R-5.2.5a** — The engine **SHALL** support configurable occlusion ray count per voice per tier:
   1 on mobile, 2 on Switch, 4 on desktop. Mobile **SHALL** use binary occlusion. Desktop **SHALL**
   use multi-ray material-weighted transmission.
   - **Rationale:** Multi-ray occlusion is expensive; per-tier scaling prevents exceeding the audio
     CPU budget on constrained platforms.
   - **Verification:** Unit test: verify configured ray count per tier. Benchmark: measure per-voice
     occlusion cost at each tier and verify it stays within the spatialization budget.

8. **R-5.2.6** — The engine **SHALL** model indirect sound paths through portals and reflective
   surfaces using a hybrid ray-and-portal graph. The solver **SHALL** run asynchronously on a worker
   thread without blocking the audio thread. The solver **SHALL** feed delay, gain, and filter
   parameters into per-voice taps.
   - **Rationale:** Indirect paths enable realistic acoustics in indoor environments where sound
     echoes through corridors.
   - **Verification:** Integration test: place a source in room A and listener in room B via a
     portal; verify delayed and attenuated sound. Close the portal and verify path removal. Verify
     async execution.

9. **R-5.2.6a** — The engine **SHALL** support per-tier propagation complexity: portal-only on
   mobile, 1-bounce on Switch, full multi-bounce on desktop. Solver update rate **SHALL** be
   configurable: every 4-8 frames on mobile, every 1-2 frames on desktop.
   - **Rationale:** Full multi-bounce is too expensive for mobile; per-tier complexity prevents
     exceeding budget.
   - **Verification:** Unit test: verify propagation mode matches tier. Benchmark: 20 portals, 50
     surfaces, 64 sources at each tier and verify p99 update time within budget.

## Reverb

10. **R-5.2.7** — The engine **SHALL** support axis-aligned or convex reverb volumes with
    configurable decay, diffusion, and early reflection patterns. Nested zones **SHALL** combine via
    priority ordering. The engine **SHALL** blend smoothly at zone boundaries with no audible pop or
    discontinuity.
    - **Rationale:** Different spaces require distinct acoustics; smooth blending prevents jarring
      transitions.
    - **Verification:** Integration test: move the listener between zones with decay 1.0 s and 3.0 s
      and verify smooth crossfade. Nest zones and verify priority override.

11. **R-5.2.7a** — The engine **SHALL** support per-tier active reverb zone counts: 1-2 on mobile,
    3-4 on Switch, 6+ on desktop. Early reflections **SHALL** be disabled on mobile.
    - **Rationale:** Multiple concurrent reverb zones consume significant CPU; per-tier caps prevent
      exceeding budget.
    - **Verification:** Unit test: verify maximum active zone count matches tier. Verify early
      reflections are disabled on mobile configuration.

## Non-Functional Requirements

12. **R-5.2.NF1** — The engine **SHALL** complete per-voice spatialization (panning, distance
    attenuation, Doppler, HRTF) within 2 microseconds per voice, enabling 256 voices within the 0.5
    ms audio thread budget.
    - **Rationale:** Spatialization runs per-voice every buffer. Per-voice cost directly limits the
      achievable voice count.
    - **Verification:** Benchmark: spatialize 256 voices with HRTF and measure per-voice cost.
      Assert p99 below 2 microseconds.

13. **R-5.2.NF2** — The engine **SHALL** complete a full propagation update within 4 ms on a worker
    thread, running asynchronously at no more than 10 Hz without blocking the audio thread.
    - **Rationale:** The propagation solver is the most expensive spatial audio operation; async
      execution prevents it from consuming the audio thread budget.
    - **Verification:** Benchmark: 20 portals, 50 surfaces, 64 sources. Assert p99 update time below
      4 ms.
