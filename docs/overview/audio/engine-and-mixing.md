# Engine and Mixing

Audio source playback, mixing, and output routing to devices.

## What it covers

- Audio source creation: loading from files or procedural generation.
- Playback control: play, pause, stop, looping, seek, pitch, volume.
- Real-time mixing: combining multiple sources into output buses.
- Bus routing: organizing sources into hierarchies (master, effects, dialogue, music).
- Volume envelopes: ADSR-style attack, decay, sustain, release.
- Parametric equalization and filtering per source or bus.
- Compression and limiting to control dynamic range.
- Multichannel output: mono, stereo, 5.1, 7.1, object-based formats.
- Sample-accurate scheduling for synchronized playback.
- Latency optimization for interactive responses.

## Concepts

### Audio Sources and Playback

Audio sources load from encoded files or generate procedurally. Playback is sample-accurate: each
source specifies a start time, and the engine renders audio at that sample clock. Pitch shift scales
playback speed; pitch shifting up to 2× plays faster, down to 0.5× plays slower. Volume applies per
source and per bus, enabling ducking (reducing music when dialogue plays).

### Mixing and Bus Hierarchies

Sources route to effects buses, which route to submix buses, which route to the master output. Each
bus has gain, EQ, and effects. Dialogue sources route to a dialogue bus with compression; game
effects route to a game bus with filtering. Music routes to a music bus. The master bus applies
final compression and limiting. This architecture enables volume ducking: the dialogue bus reduces
music bus volume when dialogue is active.

### Dynamics Control and EQ

Compression reduces the volume of loud peaks, bringing them closer to soft sections (dynamic range
compression). Limiting is a hard ceiling: audio above the threshold gets crushed. Parametric EQ
adjusts specific frequency ranges: boost bass (100 Hz), cut harsh mids (2 kHz), enhance presence
(5 kHz). These per-bus, enabling material-agnostic mixing of any audio type.

### Output and Multichannel

The engine outputs to stereo (left/right) by default. Surround formats (5.1, 7.1) route sources to
speaker arrays. Object-based audio spatializes sources as points in 3D space, with the renderer
placing them in the speaker array; height channels place sounds above/below the listener. Latency
is minimized by rendering audio ahead of playback, buffering to account for platform-level buffering.

## How it fits

- See [spatial-audio.md](./spatial-audio.md) for 3D positional playback.
- See [effects-and-dsp.md](./effects-and-dsp.md) for reverb, delay, and convolution effects.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for obstruction
  and occlusion.
