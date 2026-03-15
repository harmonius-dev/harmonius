# R-5.2 — Spatial Audio Requirements

## 3D Positioning

### R-5.2.1 3D Sound Positioning and Doppler

The engine **SHALL** position sound sources in world space and resolve per-listener panning,
distance gain, and Doppler pitch shift every audio frame, interpolating source and listener
transforms between game ticks to produce artifact-free motion.

- **Derived from:** [F-5.2.1](../../features/audio/spatial-audio.md)
- **Rationale:** Smooth interpolation prevents audible discontinuities when game-thread
  updates arrive at a lower rate than the audio buffer callback.
- **Verification:** Unit test: move a source at constant velocity past a stationary listener.
  Record the output and verify Doppler pitch shift matches the expected frequency ratio
  (v +/- vs) / (v +/- vl) within 1% tolerance. Verify no audible clicks or discontinuities
  when game tick rate is 30 Hz and audio buffer rate is 100 Hz.

### R-5.2.2 Distance Attenuation Curves

The engine **SHALL** apply configurable distance attenuation models (inverse, inverse-squared,
linear, logarithmic, and custom user-defined curves) to each sound source, with per-source
minimum and maximum distance parameters controlling rolloff range.

- **Derived from:** [F-5.2.2](../../features/audio/spatial-audio.md)
- **Rationale:** Tunable attenuation lets designers balance audibility of distant and nearby
  sources independently without global gain compression.
- **Verification:** Unit test: for each built-in attenuation model, place a source at 10
  distances between min and max range. Verify computed gain matches the model's mathematical
  formula within 0.1% tolerance. Define a custom curve via control points and verify gain
  interpolation matches expected values.

## Binaural and Ambisonics

### R-5.2.3 HRTF Binaural Rendering

The engine **SHALL** render spatialized audio through head-related transfer functions for
headphone output using swappable SOFA-format HRTF datasets, processing per-voice HRTF
filters within the voice budget via frequency-domain convolution.

- **Derived from:** [F-5.2.3](../../features/audio/spatial-audio.md)
- **Rationale:** HRTF binaural rendering provides accurate elevation and front-back cues
  that stereo panning alone cannot deliver, improving immersion on headphones.
- **Verification:** Integration test: render a source at 0, 90, 180, and 270 degrees
  azimuth and 0, 45, -45 degrees elevation. Verify left/right channel magnitude and phase
  differences match the loaded HRTF dataset within 1 dB and 0.1 ms tolerances. Swap the
  HRTF profile at runtime and verify the new profile takes effect within one buffer.

### R-5.2.4 Ambisonics Encoding and Decoding

The engine **SHALL** encode sound sources into first- or third-order Ambisonics for
intermediate mixing and decode to the listener's output format (stereo, 5.1, 7.1, binaural),
supporting rotation of the Ambisonics sound field to match listener orientation.

- **Derived from:** [F-5.2.4](../../features/audio/spatial-audio.md)
- **Rationale:** Ambisonics provides a speaker-layout-agnostic representation that simplifies
  source panning, reverb return mixing, and output to arbitrary channel configurations.
- **Verification:** Unit test: encode a mono source at a known azimuth/elevation into first-
  order Ambisonics (4 channels). Verify W, X, Y, Z coefficients match the spherical harmonic
  formula within 0.1% tolerance. Decode to stereo and 5.1, verify correct channel routing.
  Rotate the sound field 90 degrees and verify coefficients shift accordingly.

## Occlusion and Propagation

### R-5.2.5 Occlusion and Obstruction Filtering

The engine **SHALL** attenuate and low-pass filter sounds whose direct path to the listener
is blocked by geometry, using ray-casts against the shared BVH spatial index with
material-dependent transmission loss coefficients.

- **Derived from:** [F-5.2.5](../../features/audio/spatial-audio.md)
- **Rationale:** Occlusion filtering is essential for spatial believability so that sounds
  behind walls are muffled while sounds through doors remain partially audible.
- **Verification:** Integration test: place a source behind a wall with known material
  (e.g., wood, transmission loss = 12 dB). Verify output is attenuated by 12 dB +/- 1 dB
  and low-pass filtered. Replace the wall material with stone (transmission loss = 30 dB)
  and verify the increased attenuation. Verify ray-casts use the shared BVH, not a separate
  spatial index.

### R-5.2.6 Sound Propagation via Portals and Rays

The engine **SHALL** model indirect sound paths through portals and reflective surfaces using
a hybrid ray-and-portal graph, with the propagation solver running asynchronously and feeding
delay, gain, and filter parameters into per-voice diffraction and reflection taps.

- **Derived from:** [F-5.2.6](../../features/audio/spatial-audio.md)
- **Rationale:** Portal-based propagation enables realistic sound behavior in indoor
  environments where players hear combat echoing through corridors and around corners.
- **Verification:** Integration test: construct a two-room environment connected by a
  doorway portal. Place a source in room A and a listener in room B. Verify the indirect
  path through the portal produces a delayed, attenuated, and filtered signal compared to
  direct line-of-sight. Close the portal and verify the indirect path is removed. Verify
  the propagation solver does not block the audio thread.

## Reverb and Reflections

### R-5.2.7 Reverb Zones and Early Reflections

The engine **SHALL** define axis-aligned or convex reverb volumes that apply late
reverberation and early reflection patterns, blending smoothly when the listener crosses
zone boundaries and combining nested zones via priority ordering.

- **Derived from:** [F-5.2.7](../../features/audio/spatial-audio.md)
- **Rationale:** Reverb zones reinforce spatial cues in enclosed environments and prevent
  jarring acoustic changes when moving between spaces.
- **Verification:** Integration test: define two adjacent reverb zones with different decay
  times (1.0 s and 3.0 s). Move the listener across the boundary and verify the reverb
  tail crossfades smoothly over the transition region (no audible pop or discontinuity).
  Nest a small zone inside a larger one with higher priority and verify the inner zone's
  reverb overrides the outer zone's settings.

---

## Non-Functional Requirements

### R-5.2.NF1 Spatialization CPU Budget

The engine **SHALL** complete per-voice spatialization (panning, distance attenuation, Doppler,
HRTF convolution) within 2 microseconds per voice on the audio thread, enabling 256 voices to
be spatialized within the 0.5 ms audio thread budget (R-5.1.NF1).

- **Derived from:** [R-5.1.NF1](audio-engine.md), [F-5.2.1](../../features/audio/spatial-audio.md)
- **Rationale:** Spatialization runs per-voice every audio buffer. Its per-voice cost directly
  limits the achievable voice count within the thread time budget.
- **Verification:** Benchmark: spatialize 256 voices with HRTF enabled and measure per-voice
  CPU time. Assert p99 per-voice cost is below 2 microseconds on minimum-spec hardware.

### R-5.2.NF2 Propagation Solver Latency

The engine **SHALL** complete a full propagation update (portal graph traversal and reflection
ray tracing) within 4 ms on a worker thread, running asynchronously at no more than 10 Hz
without blocking the audio thread.

- **Derived from:** [F-5.2.6](../../features/audio/spatial-audio.md),
  [R-X.2.1](../cross-cutting.md) (Thread Ownership)
- **Rationale:** The propagation solver is the most expensive spatial audio operation. Running
  it asynchronously at a reduced rate prevents it from consuming the audio thread budget while
  still updating frequently enough to track source and listener movement.
- **Verification:** Benchmark: run the propagation solver on a scene with 20 portals, 50
  reflective surfaces, and 64 active sources. Assert p99 update time is below 4 ms and the
  audio thread is never blocked by the solver.
