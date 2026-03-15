# R-5.2 — Spatial Audio User Stories

## F-5.2.1 3D Sound Positioning and Doppler

## US-5.2.1.1 Position Sounds in World Space

**As an** audio designer (P-14), **I want** sound sources to be positioned in world space with
per-listener panning, distance gain, and Doppler pitch shift every audio frame, **so that**
sounds come from the correct direction and distance.

## US-5.2.1.2 Configure Doppler Effect Strength

**As an** audio designer (P-14), **I want to** adjust the Doppler effect strength per source,
**so that** fast-moving sounds have appropriate pitch shifts.

## US-5.2.1.3 Set Up 3D Sounds in Editor

**As a** designer (P-5), **I want to** place 3D sound sources on entities in the editor and
preview their spatial behavior, **so that** audio positioning is authored visually.

## US-5.2.1.4 Verify Doppler Pitch Accuracy

**As an** engine tester (P-27), **I want to** move a source at constant velocity past a
listener and verify Doppler pitch shift matches the expected ratio within 1%, **so that**
Doppler calculation is accurate.

## US-5.2.1.5 Verify Interpolation Smoothness

**As an** engine tester (P-27), **I want to** verify no audible clicks or discontinuities when
the game tick rate is 30 Hz and audio buffer rate is 100 Hz, **so that** interpolation is
artifact-free.

## US-5.2.1.6 Implement 3D Positioning with Transform Interpolation

**As an** engine developer (P-26), **I want to** implement per-voice 3D positioning that
interpolates source and listener transforms between game ticks, **so that** motion is smooth
at audio buffer rates.

## US-5.2.1.7 Hear Sounds Coming from Correct Directions

**As a** player (P-23), **I want** sounds to come from the direction of their source in the
world, **so that** I can localize events by sound.

---

## F-5.2.2 Distance Attenuation Curves

## US-5.2.2.1 Select Attenuation Model

**As an** audio designer (P-14), **I want to** choose between inverse, inverse-squared, linear,
logarithmic, and custom attenuation curves per source, **so that** rolloff behavior matches
the sound's design intent.

## US-5.2.2.2 Configure Min/Max Distance

**As an** audio designer (P-14), **I want to** set per-source minimum and maximum distance
parameters, **so that** rolloff range is controlled independently per sound.

## US-5.2.2.3 Define Custom Attenuation Curves

**As an** audio designer (P-14), **I want to** define custom attenuation curves via control
points, **so that** non-standard rolloff behaviors are supported.

## US-5.2.2.4 Configure Attenuation in Editor

**As a** designer (P-5), **I want to** set attenuation model and distances on sound sources in
the editor, **so that** distance falloff is tuned visually.

## US-5.2.2.5 Verify Attenuation Curve Accuracy

**As an** engine tester (P-27), **I want to** place a source at 10 distances between min and
max and verify gain matches the model formula within 0.1%, **so that** each attenuation model
is mathematically correct.

## US-5.2.2.6 Experience Distant Sounds Fading Naturally

**As a** player (P-23), **I want** distant sounds to fade naturally with distance, **so that**
the soundscape has realistic depth.

---

## F-5.2.3 HRTF Binaural Rendering

## US-5.2.3.1 Configure HRTF Profiles

**As an** audio designer (P-14), **I want to** load and swap HRTF datasets (SOFA format) for
binaural rendering, **so that** headphone spatialization matches different head sizes.

## US-5.2.3.2 Enable Binaural Rendering for Headphones

**As a** designer (P-5), **I want to** enable HRTF binaural rendering for headphone output in
the project audio settings, **so that** spatial audio quality is maximized on headphones.

## US-5.2.3.3 Verify HRTF Accuracy

**As an** engine tester (P-27), **I want to** render a source at 0, 90, 180, and 270 degrees
azimuth and verify channel differences match the HRTF dataset within 1 dB and 0.1 ms,
**so that** HRTF rendering accuracy is confirmed.

## US-5.2.3.4 Verify Runtime Profile Swap

**As an** engine tester (P-27), **I want to** swap the HRTF profile at runtime and verify the
new profile takes effect within one buffer, **so that** hot-swapping works correctly.

## US-5.2.3.5 Implement HRTF Convolution Pipeline

**As an** engine developer (P-26), **I want to** implement per-voice HRTF filtering using
frequency-domain convolution within the voice budget, **so that** binaural rendering is
efficient.

## US-5.2.3.6 Hear Precise Spatial Positioning on Headphones

**As a** player (P-23), **I want** sounds on headphones to have clear elevation and front-back
positioning, **so that** I can pinpoint sound sources in 3D space.

---

## F-5.2.4 Ambisonics Encoding and Decoding

## US-5.2.4.1 Configure Ambisonics Order

**As an** audio designer (P-14), **I want to** select first-order or third-order Ambisonics
for intermediate mixing, **so that** spatial resolution matches the platform budget.

## US-5.2.4.2 Configure Output Format Decoding

**As an** audio designer (P-14), **I want** Ambisonics to decode to the listener's output
format (stereo, 5.1, 7.1, binaural), **so that** any speaker layout is supported.

## US-5.2.4.3 Verify Ambisonics Encoding Accuracy

**As an** engine tester (P-27), **I want to** encode a mono source at a known azimuth into
first-order Ambisonics and verify W, X, Y, Z coefficients match spherical harmonics within
0.1%, **so that** encoding precision is confirmed.

## US-5.2.4.4 Verify Sound Field Rotation

**As an** engine tester (P-27), **I want to** rotate the Ambisonics field 90 degrees and
verify coefficients shift accordingly, **so that** rotation is correct.

## US-5.2.4.5 Implement Ambisonics Encode/Decode Pipeline

**As an** engine developer (P-26), **I want to** implement Ambisonics encoding, rotation, and
decoding to stereo, 5.1, 7.1, and binaural, **so that** speaker-agnostic mixing is available.

## US-5.2.4.6 Hear Correct Spatial Audio on Any Speaker Setup

**As a** player (P-23), **I want** spatial audio to work correctly whether I use stereo, 5.1,
or 7.1 speakers, **so that** my speaker setup is fully supported.

---

## F-5.2.5 Occlusion and Obstruction Filtering

## US-5.2.5.1 Configure Material Transmission Loss

**As an** audio designer (P-14), **I want to** set per-material transmission loss coefficients
(wood, stone, glass), **so that** different barriers produce different muffling.

## US-5.2.5.2 Set Up Occlusion Materials in Editor

**As a** designer (P-5), **I want to** assign audio occlusion materials to walls and barriers
in the editor, **so that** sound blocking matches the visual material.

## US-5.2.5.3 Verify Occlusion Attenuation Accuracy

**As an** engine tester (P-27), **I want to** place a source behind a wood wall (12 dB loss)
and verify output is attenuated by 12 dB +/- 1 dB with low-pass filtering, **so that**
material-dependent occlusion is accurate.

## US-5.2.5.4 Verify Shared BVH Usage

**As an** engine tester (P-27), **I want to** verify occlusion ray-casts use the shared BVH
spatial index (not a separate structure), **so that** spatial queries are unified.

## US-5.2.5.5 Implement Occlusion Filtering System

**As an** engine developer (P-26), **I want to** implement occlusion and obstruction filtering
that ray-casts against the shared BVH with material-dependent transmission loss, **so that**
sounds behind geometry are muffled.

## US-5.2.5.6 Hear Muffled Sounds Behind Walls

**As a** player (P-23), **I want** sounds behind walls to be muffled, with different materials
producing different levels of blocking, **so that** the soundscape matches the visual
environment.

---

## F-5.2.6 Sound Propagation via Portals and Rays

## US-5.2.6.1 Set Up Portal-Based Sound Propagation

**As an** audio designer (P-14), **I want to** define portals (doorways, windows, tunnels)
in the audio propagation graph, **so that** sound travels through openings in geometry.

## US-5.2.6.2 Configure Propagation Complexity Per Platform

**As a** designer (P-5), **I want to** set propagation path complexity per platform (portal-
only mobile, multi-bounce desktop), **so that** propagation cost matches platform budget.

## US-5.2.6.3 Verify Portal Propagation

**As an** engine tester (P-27), **I want to** place a source in room A and a listener in room
B connected by a portal, verify the indirect path produces delayed and attenuated sound, then
close the portal and verify the path is removed, **so that** portal propagation is correct.

## US-5.2.6.4 Verify Async Solver Does Not Block Audio Thread

**As an** engine tester (P-27), **I want to** verify the propagation solver runs asynchronously
without blocking the audio thread, **so that** propagation does not cause audio underruns.

## US-5.2.6.5 Benchmark Propagation Solver

**As an** engine tester (P-27), **I want to** benchmark the solver with 20 portals, 50
reflective surfaces, and 64 sources and assert p99 update time is below 4ms, **so that**
propagation cost meets the budget.

## US-5.2.6.6 Implement Hybrid Ray-Portal Propagation Solver

**As an** engine developer (P-26), **I want to** implement the asynchronous propagation solver
using a hybrid ray-and-portal graph that feeds delay, gain, and filter parameters into per-
voice taps, **so that** indirect sound paths are modeled.

## US-5.2.6.7 Hear Combat Echoing Through Corridors

**As a** player (P-23), **I want** sounds to echo through corridors and doorways, **so that**
indoor environments have realistic acoustic behavior.

---

## F-5.2.7 Reverb Zones and Early Reflections

## US-5.2.7.1 Define Reverb Volumes

**As an** audio designer (P-14), **I want to** define axis-aligned or convex reverb volumes
with decay time, diffusion, and early reflection patterns, **so that** different spaces have
distinct acoustic character.

## US-5.2.7.2 Configure Zone Blending and Priority

**As an** audio designer (P-14), **I want** nested zones to combine via priority ordering with
smooth blending at boundaries, **so that** transitions between spaces are seamless.

## US-5.2.7.3 Place Reverb Zones in Levels

**As a** designer (P-5), **I want to** place and configure reverb zones in the level editor,
**so that** room acoustics are part of level design.

## US-5.2.7.4 Verify Smooth Zone Transition

**As an** engine tester (P-27), **I want to** move the listener between two reverb zones with
decay times 1.0s and 3.0s and verify smooth crossfade with no pop or discontinuity, **so
that** zone transitions are seamless.

## US-5.2.7.5 Verify Nested Zone Priority

**As an** engine tester (P-27), **I want to** nest a small zone inside a larger one with
higher priority and verify the inner zone overrides, **so that** priority ordering works.

## US-5.2.7.6 Implement Reverb Zone System

**As an** engine developer (P-26), **I want to** implement reverb volumes with smooth
boundary blending, priority ordering, and early reflection patterns, **so that** spatial
reverb is available.

## US-5.2.7.7 Hear Different Acoustics in Caves vs Open Fields

**As a** player (P-23), **I want** caves to echo and open fields to sound dry, **so that**
the acoustic environment matches the visual space.

---

## Non-Functional Requirements

### R-5.2.NF1 Spatialization CPU Budget

The engine **SHALL** complete per-voice spatialization (panning, distance attenuation, Doppler,
HRTF convolution) within 2 microseconds per voice on the audio thread, enabling 256 voices to
be spatialized within the 0.5 ms audio thread budget.

- **Derived from:** R-5.1.NF1, F-5.2.1
- **Rationale:** Spatialization runs per-voice every audio buffer. Its per-voice cost directly
  limits the achievable voice count.
- **Verification:** Benchmark: spatialize 256 voices with HRTF enabled and measure per-voice
  CPU time. Assert p99 per-voice cost is below 2 microseconds.

### R-5.2.NF2 Propagation Solver Latency

The engine **SHALL** complete a full propagation update within 4 ms on a worker thread, running
asynchronously at no more than 10 Hz without blocking the audio thread.

- **Derived from:** F-5.2.6
- **Rationale:** The propagation solver is the most expensive spatial audio operation. Running
  it asynchronously prevents it from consuming the audio thread budget.
- **Verification:** Benchmark: run the solver on a scene with 20 portals, 50 surfaces, and
  64 sources. Assert p99 update time is below 4 ms.
