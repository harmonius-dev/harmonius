# Spatial Audio

Positional 3D audio, HRTF rendering, and environmental acoustics.

## What it covers

- Panning: stereo and surround spatial positioning.
- Head-related transfer functions (HRTF): directional audio cues for 3D perception.
- Distance attenuation: volume decreasing with distance.
- Doppler effect: pitch shift for moving sources.
- Obstruction and occlusion: high-frequency filtering when sound passes through geometry.
- Direct and reflected sound paths modeling room impulse responses.
- Reverberation and echo in indoor spaces.
- Listener orientation: head tracking when available.
- Amortized spatial queries to reduce per-frame cost.
- Binaural rendering for headphone playback.

## Concepts

### 3D Sound Positioning

Spatial audio positions sources in 3D space relative to the listener. Panning maps source angle to
stereo or surround speaker levels. HRTF (Head-Related Transfer Function) applies frequency-dependent
filtering to create directional cues: sounds come from specific directions based on spectral
coloration. Mono sources feed HRTF; stereo sources route directly to output channels.

### Distance and Doppler

Volume decreases with distance following a distance curve: linear, logarithmic, or inverse-square.
The Doppler effect shifts pitch when sources move toward or away from the listener. Approaching
sirens sound higher; receding ambulances drop in pitch. Doppler shift scales with source velocity
relative to listener velocity.

### Environmental Acoustics

Obstruction and occlusion simulate sound traveling through geometry. Direct sound reaches the
listener unobstructed; reflected sound bounces off walls. Occluding geometry (wall between source
and listener) attenuates high frequencies while passing lows, creating the sensation of sound
through a barrier. Reverberation models room reflections via convolving the dry signal with a room
impulse response.

### Spatial Queries and Amortization

The audio engine raycasts from listener to source position, detecting obstruction. These queries
are expensive; the engine amortizes them across frames, updating a subset of sources per frame. A
priority queue keeps high-priority sources (nearby, important) updated frequently; distant ambient
sources update infrequently.

### Listener Orientation and Binaural Rendering

Listener position and orientation (forward, up vectors) define the audio coordinate frame. Sounds
ahead sound like they're in front; sounds to the left pan left. HRTF works best with full 3D
coordinates. Headphone binaural rendering applies cross-feed (minimal bleed between channels) for
natural imaging when listening on headphones.

## How it fits

- See [engine-and-mixing.md](./engine-and-mixing.md) for source playback and volume control.
- See [effects-and-dsp.md](./effects-and-dsp.md) for reverberation and echo effects.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for geometry
  queries and obstruction detection.
