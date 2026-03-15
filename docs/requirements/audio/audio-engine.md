# R-5.1 — Audio Engine User Stories

## F-5.1.1 Sound Source Component

## US-5.1.1.1 Attach Sound Sources to Entities

**As an** audio designer (P-14), **I want to** add a sound source component to any entity with
gain, pitch, looping, and attenuation settings, **so that** sounds emit from objects in the
world.

## US-5.1.1.2 Configure Emitter Type

**As an** audio designer (P-14), **I want to** choose between point, line, and area emitter
types per source, **so that** the spatial emission pattern matches the object's shape.

## US-5.1.1.3 Set Up Sound Sources in Editor

**As a** designer (P-5), **I want to** attach and configure sound source components on entities
in the visual editor, **so that** audio is authored without code.

## US-5.1.1.4 Verify Sound Source Component Overhead

**As an** engine tester (P-27), **I want to** create 1000 entities with sound sources of each
type and assert per-entity storage does not exceed 128 bytes, **so that** component overhead
stays lightweight.

## US-5.1.1.5 Implement Sound Source ECS Component

**As an** engine developer (P-26), **I want to** implement the sound source component with
point, line, and area variants storing gain, pitch, looping, and attenuation reference,
**so that** audio emitters are standard ECS entities.

## US-5.1.1.6 Hear Ambient Sounds from World Objects

**As a** player (P-23), **I want** campfires, waterfalls, and machinery to emit sounds from
their locations, **so that** the world sounds alive.

---

## F-5.1.2 Listener Component

## US-5.1.2.1 Designate Listener Entities

**As an** audio designer (P-14), **I want to** designate one or more listener entities that
define the player's auditory perspective, **so that** spatialization is relative to the correct
viewpoint.

## US-5.1.2.2 Set Up Split-Screen Listeners

**As a** designer (P-5), **I want to** assign separate listener entities for each split-screen
player, **so that** each player hears audio from their own perspective.

## US-5.1.2.3 Verify Default Camera Fallback

**As an** engine tester (P-27), **I want to** remove all explicit listeners and verify the
active camera entity is used automatically, **so that** audio never goes silent.

## US-5.1.2.4 Verify Listener Velocity for Doppler

**As an** engine tester (P-27), **I want to** verify listener velocity is used for Doppler
calculations, **so that** moving listeners hear correct pitch shifts.

## US-5.1.2.5 Implement Listener Component

**As an** engine developer (P-26), **I want to** implement the listener component with
position, orientation, and velocity, defaulting to the active camera when no explicit
listener exists, **so that** audio has a reference perspective.

---

## F-5.1.3 Hierarchical Mixer Bus Graph

## US-5.1.3.1 Configure Mixer Bus Hierarchy

**As an** audio designer (P-14), **I want to** create a hierarchy of mixer buses (master,
music, SFX, ambient, voice, UI) with gain, mute, and solo controls, **so that** categories
mix independently under global volume.

## US-5.1.3.2 Add Insert Effects to Buses

**As an** audio designer (P-14), **I want to** add ordered chains of insert effects to any
bus, **so that** DSP processing is per-category.

## US-5.1.3.3 Rewire Buses at Runtime

**As an** audio designer (P-14), **I want to** add and rewire buses at runtime for dynamic
mix states (underwater, pause menu), **so that** the mix adapts to gameplay context.

## US-5.1.3.4 Configure Bus Hierarchy in Editor

**As a** designer (P-5), **I want to** build and configure the mixer bus hierarchy in the
visual editor, **so that** the audio mix is authored without code.

## US-5.1.3.5 Verify Parent Gain Inheritance

**As an** engine tester (P-27), **I want to** set master gain to 0.5 and verify all descendant
bus outputs are halved, **so that** gain inheritance is correct.

## US-5.1.3.6 Verify Bus Muting

**As an** engine tester (P-27), **I want to** mute a mid-level bus and verify all children are
silent, **so that** muting propagates correctly.

## US-5.1.3.7 Implement Mixer Bus DAG

**As an** engine developer (P-26), **I want to** implement a DAG of mixer buses with gain,
mute, solo, insert chains, and runtime rewiring, **so that** hierarchical mixing is available.

## US-5.1.3.8 Adjust Volume Categories

**As a** player (P-23), **I want to** adjust music, SFX, and voice volumes independently,
**so that** the audio mix matches my preferences.

---

## F-5.1.4 Voice Management and Priority System

## US-5.1.4.1 Configure Voice Pool Size

**As an** audio designer (P-14), **I want to** set the voice pool size per platform (16-32
mobile, 128-256 desktop), **so that** hardware voice budget is respected.

## US-5.1.4.2 Assign Priority Classes to Sound Sources

**As an** audio designer (P-14), **I want to** assign priority classes (critical, high, medium,
low) to sound sources, **so that** important sounds are never stolen.

## US-5.1.4.3 Verify Voice Virtualization

**As an** engine tester (P-27), **I want to** fill the voice pool, trigger a critical-priority
voice, and assert the lowest-audibility voice is virtualized, **so that** priority stealing
works correctly.

## US-5.1.4.4 Verify Voice Restoration

**As an** engine tester (P-27), **I want to** free a voice and verify a virtualized voice
restores playback within one buffer callback, retaining position and offset, **so that**
restoration is seamless.

## US-5.1.4.5 Implement Voice Priority System

**As an** engine developer (P-26), **I want to** implement the voice pool with priority-based
allocation, audibility scoring, virtualization, and seamless restoration, **so that** voice
management stays within hardware limits.

## US-5.1.4.6 Hear Important Sounds in Busy Scenes

**As a** player (P-23), **I want** critical gameplay sounds (alerts, dialogue) to always play
even in chaotic scenes with many sounds, **so that** I never miss important audio cues.

---

## F-5.1.5 Streaming Playback

## US-5.1.5.1 Set Up Audio Streaming

**As an** audio designer (P-14), **I want** long-duration assets (music, ambience, dialogue)
to stream from disk in small chunks, **so that** entire files are not loaded into memory.

## US-5.1.5.2 Configure Prefetch Hints

**As an** audio designer (P-14), **I want to** issue prefetch hints before playback, **so
that** streaming begins before the sound is needed, eliminating startup latency.

## US-5.1.5.3 Verify Platform-Native Async I/O

**As an** engine tester (P-27), **I want to** verify streaming uses IOCP on Windows, GCD on
macOS, and io_uring on Linux (not stdlib file I/O), **so that** platform-native async I/O
compliance is confirmed.

## US-5.1.5.4 Verify Streaming Memory Cap

**As an** engine tester (P-27), **I want to** stream a 5-minute audio file and assert peak
memory stays below 256 KiB per stream, **so that** streaming memory is bounded.

## US-5.1.5.5 Verify Prefetch Eliminates Latency

**As an** engine tester (P-27), **I want to** issue a prefetch hint 500ms before playback and
assert startup latency is under 10ms, **so that** prefetching is effective.

## US-5.1.5.6 Implement Streaming with Platform-Native I/O

**As an** engine developer (P-26), **I want to** implement ring-buffer audio streaming using
platform-native async I/O with prefetch support, **so that** long audio files stream
efficiently.

## US-5.1.5.7 Hear Music Without Loading Pauses

**As a** player (P-23), **I want** music and ambient tracks to start playing without audible
loading pauses, **so that** audio transitions feel seamless.

---

## F-5.1.6 Sample-Accurate Scheduling

## US-5.1.6.1 Schedule Sounds at Precise Sample Offsets

**As an** audio designer (P-14), **I want to** schedule sound start, stop, and parameter
changes at precise sample offsets, **so that** layered loops and musical cues align perfectly.

## US-5.1.6.2 Verify Sample-Accurate Alignment

**As an** engine tester (P-27), **I want to** schedule two sounds at the same sample offset
and verify their output buffers are phase-aligned within +/- 0 samples, **so that** sample
accuracy is confirmed.

## US-5.1.6.3 Verify Zero Scheduling Jitter

**As an** engine tester (P-27), **I want to** measure scheduling jitter over 1000 commands and
verify zero-sample deviation, **so that** timing precision is absolute.

## US-5.1.6.4 Implement Sample-Accurate Command Queue

**As an** engine developer (P-26), **I want to** implement a command queue where game-thread
commands execute at precise sample offsets within the next audio buffer, **so that** timing is
sample-accurate.

## US-5.1.6.5 Hear Perfectly Synchronized Layered Sounds

**As a** player (P-23), **I want** layered sound effects and music to play in perfect sync,
**so that** audio cues align with gameplay events.

---

## F-5.1.7 Audio Format and Codec Support

## US-5.1.7.1 Support Multiple Audio Formats

**As an** audio designer (P-14), **I want to** use PCM, Vorbis, Opus, and FLAC formats for
different asset types, **so that** the right codec is used for each purpose.

## US-5.1.7.2 Register Custom Codec Plugins

**As an** audio designer (P-14), **I want to** register custom codec plugins at runtime,
**so that** proprietary or future formats are supported without engine changes.

## US-5.1.7.3 Set Up Audio Format Preferences in Editor

**As a** designer (P-5), **I want to** configure format preferences per asset type in the
editor, **so that** codec selection is project-wide.

## US-5.1.7.4 Verify All Format Decode Correctness

**As an** engine tester (P-27), **I want to** load and play one asset in each format and verify
decoded output matches a reference waveform, **so that** all codecs decode correctly.

## US-5.1.7.5 Verify Metadata Extraction

**As an** engine tester (P-27), **I want to** verify format metadata (sample rate, channel
count, loop points) is extracted correctly during import, **so that** asset metadata is
reliable.

## US-5.1.7.6 Implement Extensible Codec Registry

**As an** engine developer (P-26), **I want to** implement the codec registry supporting PCM,
Vorbis, Opus, and FLAC with plugin extensibility, **so that** audio decoding is unified.

## US-5.1.7.7 Hear Audio Without Format Worries

**As a** player (P-23), **I want** all audio to play correctly regardless of the underlying
format, **so that** sound quality is consistent.

---

## Non-Functional Requirements

### R-5.1.NF1 Audio Thread Budget

The engine **SHALL** complete all audio processing (mixing, DSP, spatialization) within 0.5 ms
of wall-clock time per audio buffer callback at 48 kHz / 512-sample buffers, consuming no more
than one CPU core's capacity at steady state.

- **Derived from:** Cross-cutting frame time budget
- **Rationale:** Audio callbacks must complete before the hardware output buffer drains or
  audible glitches (underruns) occur.
- **Verification:** Benchmark: run a worst-case mix (maximum voice count, full DSP chain,
  spatialization on all voices) and measure wall-clock time per callback over 10,000 buffers.
  Assert p99 latency is below 0.5 ms on the minimum-spec hardware tier.

### R-5.1.NF2 Maximum Voice Count

The engine **SHALL** support at least 256 simultaneous voices (128 real + 128 virtualized)
without exceeding the audio thread budget.

- **Derived from:** F-5.1.4
- **Rationale:** Large-scale scenarios generate hundreds of concurrent sound sources. The voice
  pool must handle typical peak load.
- **Verification:** Stress test: allocate 256 voices with full spatialization and a 2-insert
  DSP chain each. Assert no audio underruns over 60 seconds.

### R-5.1.NF3 Audio Memory Budget

The engine **SHALL** limit total audio system resident memory to 64 MiB on the minimum platform
tier, excluding streaming ring buffers.

- **Derived from:** Cross-cutting system RAM budget
- **Rationale:** Audio must coexist with rendering, physics, and animation within constrained
  system RAM.
- **Verification:** Benchmark: load the maximum sound bank configuration and measure peak
  resident audio memory. Assert it does not exceed 64 MiB.

### R-5.1.NF4 Mixer Output Latency

The engine **SHALL** deliver audio output with end-to-end latency not exceeding 20 ms at
48 kHz sample rate.

- **Derived from:** F-5.1.6
- **Rationale:** Latency above 20 ms causes perceptible desynchronization between visual
  effects and sound.
- **Verification:** Integration test: trigger a sound event and measure time until first
  non-zero sample in output. Assert latency is below 20 ms.
