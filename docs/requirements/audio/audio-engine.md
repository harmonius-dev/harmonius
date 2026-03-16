# R-5.1 — Audio Engine Requirements

## Sound Sources

### R-5.1.1 Sound Source Component

The engine **SHALL** provide an ECS component for sound
emission with point, line, and area emitter variants,
each storing gain (linear 0.0-1.0), pitch multiplier,
looping flag, and an attenuation curve reference. Per-
entity storage for the component **SHALL NOT** exceed
128 bytes.

- **Derived from:**
  [F-5.1.1](../../features/audio/audio-engine.md)
- **Rationale:** A lightweight ECS component enables
  hundreds of simultaneous ambient emitters (campfires,
  waterfalls, machinery) without per-entity overhead
  degrading frame time.
- **Verification:** Unit test: create 1,000 entities
  with sound source components of each emitter type and
  assert per-entity storage does not exceed 128 bytes.
  Integration test: attach a sound source to an entity
  and verify audio output matches configured gain, pitch,
  and looping settings.

### R-5.1.1a Sound Source Emitter Type Selection

The engine **SHALL** support point, line, and area
emitter types per sound source component, where each
type defines a distinct spatial emission pattern. The
emitter type **SHALL** be selectable at authoring time
and changeable at runtime without despawning the entity.

- **Derived from:**
  [F-5.1.1](../../features/audio/audio-engine.md)
- **Rationale:** Different world objects require
  different emission shapes: torches emit from a point,
  rivers from a line, and open fields from an area.
- **Verification:** Unit test: set emitter type to
  point, line, and area on the same entity and verify
  each produces the expected spatial distribution.
  Verify runtime type change does not cause audio
  discontinuity.

## Listener

### R-5.1.2 Listener Component

The engine **SHALL** provide an ECS component that
designates entities as audio listeners with position,
orientation, and velocity. The engine **SHALL** support
multiple simultaneous listeners for split-screen. When
no explicit listener exists, the engine **SHALL** fall
back to the active camera entity.

- **Derived from:**
  [F-5.1.2](../../features/audio/audio-engine.md)
- **Rationale:** Spatialization requires a reference
  perspective; split-screen requires independent
  perspectives per player; a camera fallback prevents
  silent audio when no listener is configured.
- **Verification:** Unit test: remove all explicit
  listeners and verify the active camera entity is used.
  Integration test: assign separate listeners for two
  split-screen players and verify each hears audio
  from their own perspective. Verify listener velocity
  produces correct Doppler pitch shifts.

## Mixer Bus Hierarchy

### R-5.1.3 Hierarchical Mixer Bus Graph

The engine **SHALL** provide a directed acyclic graph of
mixer buses with gain, mute, and solo controls per bus.
Child buses **SHALL** inherit parent gain. Each bus
**SHALL** support an ordered chain of insert effects
using enum dispatch (no trait objects). Buses **SHALL**
be addable and rewirable at runtime without audio
discontinuity.

- **Derived from:**
  [F-5.1.3](../../features/audio/audio-engine.md)
- **Rationale:** Hierarchical mixing enables independent
  volume control for music, SFX, ambient, voice, and UI
  categories. Runtime rewiring supports dynamic mix
  states such as underwater ducking or pause menus.
- **Verification:** Unit test: set master gain to 0.5
  and verify all descendant bus outputs are halved. Mute
  a mid-level bus and verify all children are silent.
  Integration test: rewire a bus at runtime and verify no
  audible click or gap during the transition.

### R-5.1.3a Mixer Bus Insert Chain Dispatch

The engine **SHALL** dispatch insert effects on mixer
buses via enum-based static dispatch, not dynamic trait
objects. Each built-in effect type (filter, EQ, reverb,
compressor, limiter, delay, chorus, flanger, pitch
shifter) **SHALL** be a variant of a single effect enum.
Custom DSP nodes **SHALL** be registered via a node
registry with stateless process callbacks.

- **Derived from:**
  [F-5.1.3](../../features/audio/audio-engine.md),
  [F-5.3.8](../../features/audio/dsp-and-effects.md)
- **Rationale:** Enum dispatch avoids vtable indirection
  on the real-time audio thread, enabling the compiler
  to inline effect processing and eliminate branch
  misprediction overhead in tight DSP loops.
- **Verification:** Inspect generated assembly to confirm
  no vtable lookups in the mixer bus processing path.
  Benchmark: compare enum dispatch vs trait object
  dispatch for a 4-insert chain and verify enum dispatch
  is faster.

## Voice Management

### R-5.1.4 Voice Management and Priority System

The engine **SHALL** manage a fixed pool of voices with
priority-based allocation. Each sound source **SHALL**
declare a priority class (critical, high, medium, low).
When the voice pool is exhausted, the engine **SHALL**
virtualize the lowest-audibility voice (tracked but
silent) and restore it seamlessly within one buffer
callback when headroom returns.

- **Derived from:**
  [F-5.1.4](../../features/audio/audio-engine.md)
- **Rationale:** Hardware voice budgets are finite;
  priority stealing ensures critical gameplay sounds
  (alerts, dialogue) always play even in chaotic scenes
  with hundreds of concurrent emitters.
- **Verification:** Stress test: fill the voice pool,
  trigger a critical-priority voice, and assert the
  lowest-audibility voice is virtualized. Free a voice
  and verify the virtualized voice restores playback
  within one buffer callback with correct position and
  offset.

### R-5.1.4a Voice Pool Size Per Platform

The engine **SHALL** support configurable voice pool
sizes per platform tier: 16-32 voices on mobile, 32-64
on Switch, and 128-256 on desktop. The voice pool size
**SHALL** be configurable at engine initialization.

- **Derived from:**
  [F-5.1.4](../../features/audio/audio-engine.md)
- **Rationale:** Mobile and handheld platforms have fewer
  hardware mixing resources; per-tier sizing prevents
  exceeding platform audio budgets.
- **Verification:** Unit test: initialize voice pools at
  each tier's minimum and maximum sizes and verify the
  pool accepts exactly the configured number of voices.

## Streaming Playback

### R-5.1.5 Streaming Playback via Platform-Native I/O

The engine **SHALL** stream long-duration audio assets
from disk in ring-buffer chunks using platform-native
async I/O (IOCP on Windows, GCD Dispatch IO on macOS,
io_uring on Linux). The engine **SHALL NOT** use C++
stdlib file I/O for audio streaming. Peak memory per
stream **SHALL NOT** exceed 256 KiB.

- **Derived from:**
  [F-5.1.5](../../features/audio/audio-engine.md)
- **Rationale:** Holding entire music or dialogue files
  in memory is infeasible for large asset libraries.
  Platform-native async I/O avoids blocking the audio
  thread and maximizes throughput.
- **Verification:** Integration test: stream a 5-minute
  audio file and assert peak memory stays below 256 KiB
  per stream. Verify streaming uses the correct
  platform-native API via platform instrumentation.

### R-5.1.5a Prefetch Hint Support

The engine **SHALL** support prefetch hints that begin
streaming audio data before playback is triggered.
Issuing a prefetch hint at least 500 ms before playback
**SHALL** reduce startup latency to under 10 ms.

- **Derived from:**
  [F-5.1.5](../../features/audio/audio-engine.md)
- **Rationale:** Cinematic cues and zone transitions
  require instant audio start; prefetching eliminates
  audible startup delays.
- **Verification:** Integration test: issue a prefetch
  hint 500 ms before playback and assert startup latency
  is under 10 ms. Verify playback without prefetch has
  measurably higher startup latency.

## Sample-Accurate Scheduling

### R-5.1.6 Sample-Accurate Command Queue

The engine **SHALL** schedule sound start, stop, and
parameter changes at precise sample offsets within the
next audio buffer. Two sounds scheduled at the same
sample offset **SHALL** produce phase-aligned output
with zero-sample deviation. The command queue **SHALL**
be lock-free (SPSC ring buffer) between game thread
and audio thread.

- **Derived from:**
  [F-5.1.6](../../features/audio/audio-engine.md)
- **Rationale:** Layered loops and musical cues require
  sub-sample synchronization. Lock-free communication
  prevents priority inversion on the real-time audio
  thread.
- **Verification:** Unit test: schedule two sounds at
  the same sample offset and verify their output buffers
  are phase-aligned within +/- 0 samples. Measure
  scheduling jitter over 1,000 commands and verify
  zero-sample deviation.

## Codecs

### R-5.1.7 Audio Format and Codec Support

The engine **SHALL** decode PCM (WAV), Vorbis, Opus, and
FLAC formats. Format metadata (sample rate, channel
count, loop points) **SHALL** be extracted during asset
import and cached. The codec registry **SHALL** support
runtime registration of custom codec plugins without
engine recompilation.

- **Derived from:**
  [F-5.1.7](../../features/audio/audio-engine.md)
- **Rationale:** Different asset types benefit from
  different codecs: Opus for voice chat, Vorbis for
  legacy assets, FLAC for lossless reference audio.
  Plugin extensibility supports proprietary formats.
- **Verification:** Integration test: load and play one
  asset in each format and verify decoded output matches
  a reference waveform. Verify metadata extraction
  returns correct sample rate, channel count, and loop
  points for each format.

---

## User Stories

## F-5.1.1 Sound Source Component

## US-5.1.1.1 Attach Sound Sources to Entities

**As an** audio designer (P-14), **I want to** add a sound source component to any entity with gain,
pitch, looping, and attenuation settings, **so that** sounds emit from objects in the world.

## US-5.1.1.2 Configure Emitter Type

**As an** audio designer (P-14), **I want to** choose between point, line, and area emitter types
per source, **so that** the spatial emission pattern matches the object's shape.

## US-5.1.1.3 Set Up Sound Sources in Editor

**As a** designer (P-5), **I want to** attach and configure sound source components on entities in
the visual editor, **so that** audio is authored without code.

## US-5.1.1.4 Verify Sound Source Component Overhead

**As an** engine tester (P-27), **I want to** create 1000 entities with sound sources of each type
and assert per-entity storage does not exceed 128 bytes, **so that** component overhead stays
lightweight.

## US-5.1.1.5 Implement Sound Source ECS Component

**As an** engine developer (P-26), **I want to** implement the sound source component with point,
line, and area variants storing gain, pitch, looping, and attenuation reference, **so that** audio
emitters are standard ECS entities.

## US-5.1.1.6 Hear Ambient Sounds from World Objects

**As a** player (P-23), **I want** campfires, waterfalls, and machinery to emit sounds from their
locations, **so that** the world sounds alive.

---

## F-5.1.2 Listener Component

## US-5.1.2.1 Designate Listener Entities

**As an** audio designer (P-14), **I want to** designate one or more listener entities that define
the player's auditory perspective, **so that** spatialization is relative to the correct viewpoint.

## US-5.1.2.2 Set Up Split-Screen Listeners

**As a** designer (P-5), **I want to** assign separate listener entities for each split-screen
player, **so that** each player hears audio from their own perspective.

## US-5.1.2.3 Verify Default Camera Fallback

**As an** engine tester (P-27), **I want to** remove all explicit listeners and verify the active
camera entity is used automatically, **so that** audio never goes silent.

## US-5.1.2.4 Verify Listener Velocity for Doppler

**As an** engine tester (P-27), **I want to** verify listener velocity is used for Doppler
calculations, **so that** moving listeners hear correct pitch shifts.

## US-5.1.2.5 Implement Listener Component

**As an** engine developer (P-26), **I want to** implement the listener component with position,
orientation, and velocity, defaulting to the active camera when no explicit listener exists, **so
that** audio has a reference perspective.

---

## F-5.1.3 Hierarchical Mixer Bus Graph

## US-5.1.3.1 Configure Mixer Bus Hierarchy

**As an** audio designer (P-14), **I want to** create a hierarchy of mixer buses (master, music,
SFX, ambient, voice, UI) with gain, mute, and solo controls, **so that** categories mix
independently under global volume.

## US-5.1.3.2 Add Insert Effects to Buses

**As an** audio designer (P-14), **I want to** add ordered chains of insert effects to any bus, **so
that** DSP processing is per-category.

## US-5.1.3.3 Rewire Buses at Runtime

**As an** audio designer (P-14), **I want to** add and rewire buses at runtime for dynamic mix
states (underwater, pause menu), **so that** the mix adapts to gameplay context.

## US-5.1.3.4 Configure Bus Hierarchy in Editor

**As a** designer (P-5), **I want to** build and configure the mixer bus hierarchy in the visual
editor, **so that** the audio mix is authored without code.

## US-5.1.3.5 Verify Parent Gain Inheritance

**As an** engine tester (P-27), **I want to** set master gain to 0.5 and verify all descendant bus
outputs are halved, **so that** gain inheritance is correct.

## US-5.1.3.6 Verify Bus Muting

**As an** engine tester (P-27), **I want to** mute a mid-level bus and verify all children are
silent, **so that** muting propagates correctly.

## US-5.1.3.7 Implement Mixer Bus DAG

**As an** engine developer (P-26), **I want to** implement a DAG of mixer buses with gain, mute,
solo, insert chains, and runtime rewiring, **so that** hierarchical mixing is available.

## US-5.1.3.8 Adjust Volume Categories

**As a** player (P-23), **I want to** adjust music, SFX, and voice volumes independently, **so
that** the audio mix matches my preferences.

---

## F-5.1.4 Voice Management and Priority System

## US-5.1.4.1 Configure Voice Pool Size

**As an** audio designer (P-14), **I want to** set the voice pool size per platform (16-32 mobile,
128-256 desktop), **so that** hardware voice budget is respected.

## US-5.1.4.2 Assign Priority Classes to Sound Sources

**As an** audio designer (P-14), **I want to** assign priority classes (critical, high, medium, low)
to sound sources, **so that** important sounds are never stolen.

## US-5.1.4.3 Verify Voice Virtualization

**As an** engine tester (P-27), **I want to** fill the voice pool, trigger a critical-priority
voice, and assert the lowest-audibility voice is virtualized, **so that** priority stealing works
correctly.

## US-5.1.4.4 Verify Voice Restoration

**As an** engine tester (P-27), **I want to** free a voice and verify a virtualized voice restores
playback within one buffer callback, retaining position and offset, **so that** restoration is
seamless.

## US-5.1.4.5 Implement Voice Priority System

**As an** engine developer (P-26), **I want to** implement the voice pool with priority-based
allocation, audibility scoring, virtualization, and seamless restoration, **so that** voice
management stays within hardware limits.

## US-5.1.4.6 Hear Important Sounds in Busy Scenes

**As a** player (P-23), **I want** critical gameplay sounds (alerts, dialogue) to always play even
in chaotic scenes with many sounds, **so that** I never miss important audio cues.

---

## F-5.1.5 Streaming Playback

## US-5.1.5.1 Set Up Audio Streaming

**As an** audio designer (P-14), **I want** long-duration assets (music, ambience, dialogue) to
stream from disk in small chunks, **so that** entire files are not loaded into memory.

## US-5.1.5.2 Configure Prefetch Hints

**As an** audio designer (P-14), **I want to** issue prefetch hints before playback, **so that**
streaming begins before the sound is needed, eliminating startup latency.

## US-5.1.5.3 Verify Platform-Native Async I/O

**As an** engine tester (P-27), **I want to** verify streaming uses IOCP on Windows, GCD on macOS,
and io_uring on Linux (not stdlib file I/O), **so that** platform-native async I/O compliance is
confirmed.

## US-5.1.5.4 Verify Streaming Memory Cap

**As an** engine tester (P-27), **I want to** stream a 5-minute audio file and assert peak memory
stays below 256 KiB per stream, **so that** streaming memory is bounded.

## US-5.1.5.5 Verify Prefetch Eliminates Latency

**As an** engine tester (P-27), **I want to** issue a prefetch hint 500ms before playback and assert
startup latency is under 10ms, **so that** prefetching is effective.

## US-5.1.5.6 Implement Streaming with Platform-Native I/O

**As an** engine developer (P-26), **I want to** implement ring-buffer audio streaming using
platform-native async I/O with prefetch support, **so that** long audio files stream efficiently.

## US-5.1.5.7 Hear Music Without Loading Pauses

**As a** player (P-23), **I want** music and ambient tracks to start playing without audible loading
pauses, **so that** audio transitions feel seamless.

---

## F-5.1.6 Sample-Accurate Scheduling

## US-5.1.6.1 Schedule Sounds at Precise Sample Offsets

**As an** audio designer (P-14), **I want to** schedule sound start, stop, and parameter changes at
precise sample offsets, **so that** layered loops and musical cues align perfectly.

## US-5.1.6.2 Verify Sample-Accurate Alignment

**As an** engine tester (P-27), **I want to** schedule two sounds at the same sample offset and
verify their output buffers are phase-aligned within +/- 0 samples, **so that** sample accuracy is
confirmed.

## US-5.1.6.3 Verify Zero Scheduling Jitter

**As an** engine tester (P-27), **I want to** measure scheduling jitter over 1000 commands and
verify zero-sample deviation, **so that** timing precision is absolute.

## US-5.1.6.4 Implement Sample-Accurate Command Queue

**As an** engine developer (P-26), **I want to** implement a command queue where game-thread
commands execute at precise sample offsets within the next audio buffer, **so that** timing is
sample-accurate.

## US-5.1.6.5 Hear Perfectly Synchronized Layered Sounds

**As a** player (P-23), **I want** layered sound effects and music to play in perfect sync, **so
that** audio cues align with gameplay events.

---

## F-5.1.7 Audio Format and Codec Support

## US-5.1.7.1 Support Multiple Audio Formats

**As an** audio designer (P-14), **I want to** use PCM, Vorbis, Opus, and FLAC formats for different
asset types, **so that** the right codec is used for each purpose.

## US-5.1.7.2 Register Custom Codec Plugins

**As an** audio designer (P-14), **I want to** register custom codec plugins at runtime, **so that**
proprietary or future formats are supported without engine changes.

## US-5.1.7.3 Set Up Audio Format Preferences in Editor

**As a** designer (P-5), **I want to** configure format preferences per asset type in the editor,
**so that** codec selection is project-wide.

## US-5.1.7.4 Verify All Format Decode Correctness

**As an** engine tester (P-27), **I want to** load and play one asset in each format and verify
decoded output matches a reference waveform, **so that** all codecs decode correctly.

## US-5.1.7.5 Verify Metadata Extraction

**As an** engine tester (P-27), **I want to** verify format metadata (sample rate, channel count,
loop points) is extracted correctly during import, **so that** asset metadata is reliable.

## US-5.1.7.6 Implement Extensible Codec Registry

**As an** engine developer (P-26), **I want to** implement the codec registry supporting PCM,
Vorbis, Opus, and FLAC with plugin extensibility, **so that** audio decoding is unified.

## US-5.1.7.7 Hear Audio Without Format Worries

**As a** player (P-23), **I want** all audio to play correctly regardless of the underlying format,
**so that** sound quality is consistent.

---

## Non-Functional Requirements

### R-5.1.NF1 Audio Thread Budget

The engine **SHALL** complete all audio processing (mixing, DSP, spatialization) within 0.5 ms of
wall-clock time per audio buffer callback at 48 kHz / 512-sample buffers, consuming no more than one
CPU core's capacity at steady state.

- **Derived from:** Cross-cutting frame time budget
- **Rationale:** Audio callbacks must complete before the hardware output buffer drains or audible
  glitches (underruns) occur.
- **Verification:** Benchmark: run a worst-case mix (maximum voice count, full DSP chain,
  spatialization on all voices) and measure wall-clock time per callback over 10,000 buffers. Assert
  p99 latency is below 0.5 ms on the minimum-spec hardware tier.

### R-5.1.NF2 Maximum Voice Count

The engine **SHALL** support at least 256 simultaneous voices (128 real + 128 virtualized) without
exceeding the audio thread budget.

- **Derived from:** F-5.1.4
- **Rationale:** Large-scale scenarios generate hundreds of concurrent sound sources. The voice pool
  must handle typical peak load.
- **Verification:** Stress test: allocate 256 voices with full spatialization and a 2-insert DSP
  chain each. Assert no audio underruns over 60 seconds.

### R-5.1.NF3 Audio Memory Budget

The engine **SHALL** limit total audio system resident memory to 64 MiB on the minimum platform
tier, excluding streaming ring buffers.

- **Derived from:** Cross-cutting system RAM budget
- **Rationale:** Audio must coexist with rendering, physics, and animation within constrained system
  RAM.
- **Verification:** Benchmark: load the maximum sound bank configuration and measure peak resident
  audio memory. Assert it does not exceed 64 MiB.

### R-5.1.NF4 Mixer Output Latency

The engine **SHALL** deliver audio output with end-to-end latency not exceeding 20 ms at 48 kHz
sample rate.

- **Derived from:** F-5.1.6
- **Rationale:** Latency above 20 ms causes perceptible desynchronization between visual effects and
  sound.
- **Verification:** Integration test: trigger a sound event and measure time until first non-zero
  sample in output. Assert latency is below 20 ms.
