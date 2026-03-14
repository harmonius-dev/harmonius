# 5.1 — Audio Engine

## Sound Sources

### F-5.1.1 Sound Source Component

Attach point, line, and area sound emitters to entities via an ECS component, each carrying
gain, pitch, looping, and attenuation references. In an MMO world with hundreds of ambient
emitters (campfires, waterfalls, NPC chatter), a lightweight component is essential to keep
per-entity overhead minimal while feeding the spatial audio and mixer systems.

- **Requirements:** R-5.1.1
- **Dependencies:** None
- **Platform notes:** None

### F-5.1.2 Listener Component

Designate one or more listener entities that define the player's auditory perspective,
including position, orientation, and velocity for Doppler calculations. Multiple listeners
support split-screen and spectator modes; the default case assigns a single listener to the
active camera.

- **Requirements:** R-5.1.2
- **Dependencies:** F-5.1.1
- **Platform notes:** None

## Mixer Bus Hierarchy

### F-5.1.3 Hierarchical Mixer Bus Graph

Provide a directed acyclic graph of mixer buses (master, music, SFX, ambient, voice, UI)
where each bus carries gain, mute, solo, and a chain of insert effects. Child buses inherit
parent gain for global volume control, and buses can be added or rewired at runtime to
support dynamic mix states such as underwater or pause-menu ducking.

- **Requirements:** R-5.1.3
- **Dependencies:** None
- **Platform notes:** None

## Voice Management

### F-5.1.4 Voice Management and Priority System

Manage a fixed pool of hardware and software voices with priority-based allocation,
virtualization, and stealing. Each sound source declares a priority class (critical,
high, medium, low) and an audibility score derived from distance and occlusion. When the
voice budget is exceeded, the lowest-scoring voices are virtualized (tracked but silent)
and restored seamlessly when headroom returns. MMO raids with dozens of spell effects,
ambient loops, and voice chat demand strict budgeting.

- **Requirements:** R-5.1.4
- **Dependencies:** F-5.1.1, F-5.1.3
- **Platform notes:** None

## Playback

### F-5.1.5 Streaming Playback

Stream long-duration audio assets (music, ambience, dialogue) from disk in small
ring-buffer chunks using platform-native async I/O, avoiding the need to decode and hold
entire files in memory. Prefetch hinting allows the engine to begin streaming before
playback is triggered, eliminating audible startup latency for cinematic cues and zone
transitions.

- **Requirements:** R-5.1.5
- **Dependencies:** F-5.1.1
- **Platform notes:** Uses I/O completion ports on Windows, Grand Central Dispatch
  async I/O on macOS, and io_uring on Linux.

### F-5.1.6 Sample-Accurate Scheduling

Schedule sound start, stop, and parameter changes with sample-accurate timing relative to
the audio clock, enabling tight synchronization between layered loops, musical cues, and
gameplay events. Commands are queued on the game thread and executed on the audio thread at
the precise sample offset within the next buffer.

- **Requirements:** R-5.1.6
- **Dependencies:** F-5.1.1, F-5.1.3
- **Platform notes:** None

## Formats and Codecs

### F-5.1.7 Audio Format and Codec Support

Decode PCM (WAV), Vorbis, Opus, and FLAC at load or stream time, with a codec registry
that is extensible via plugins. Opus serves as the primary codec for voice chat and
bandwidth-constrained streaming; Vorbis covers legacy assets; FLAC provides lossless
reference audio. Format metadata (sample rate, channel count, loop points) is extracted
during asset import and cached in the asset registry.

- **Requirements:** R-5.1.7
- **Dependencies:** F-5.1.5
- **Platform notes:** Platform-supplied hardware decoders (e.g., Apple Audio Toolbox
  for AAC on macOS/iOS) may be used opportunistically but are never required.
