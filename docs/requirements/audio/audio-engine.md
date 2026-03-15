# R-5.1 — Audio Engine Requirements

## Sound Sources

### R-5.1.1 Sound Source Component

The engine **SHALL** attach point, line, and area sound emitters to entities via an ECS
component carrying gain, pitch, looping flag, and attenuation curve reference, with
per-entity overhead not exceeding 128 bytes.

- **Derived from:** [F-5.1.1](../../features/audio/audio-engine.md)
- **Rationale:** Lightweight ECS components are essential for worlds with hundreds of
  concurrent ambient emitters without degrading iteration throughput.
- **Verification:** Unit test: create 1,000 entities with sound source components of each
  emitter type (point, line, area). Assert all fields are readable, per-entity storage does
  not exceed 128 bytes, and iteration throughput exceeds 10 million entities/second.

### R-5.1.2 Listener Component

The engine **SHALL** designate one or more listener entities that define auditory perspective
via position, orientation, and velocity, and **SHALL** default to the active camera entity
when no explicit listener is assigned.

- **Derived from:** [F-5.1.2](../../features/audio/audio-engine.md)
- **Rationale:** Multiple listeners enable split-screen and spectator modes; a sensible
  default prevents silent audio when no listener is configured.
- **Verification:** Unit test: register two listener entities, verify both receive
  independent spatialized output. Remove all listeners and verify the active camera entity
  is automatically used. Assert listener velocity is used for Doppler calculations.

## Mixer Bus Hierarchy

### R-5.1.3 Hierarchical Mixer Bus Graph

The engine **SHALL** provide a DAG of mixer buses (master, music, SFX, ambient, voice, UI)
where each bus carries gain, mute, solo, and an ordered chain of insert effects, with child
buses inheriting parent gain and buses addable or rewirable at runtime.

- **Derived from:** [F-5.1.3](../../features/audio/audio-engine.md)
- **Rationale:** A hierarchical bus graph enables global volume control, per-category mixing,
  and dynamic mix states such as underwater ducking or pause-menu attenuation.
- **Verification:** Integration test: construct a 3-level bus hierarchy, set master gain to
  0.5, verify all descendant bus outputs are halved. Add a bus at runtime, route a source
  through it, and verify output. Mute a mid-level bus and verify all children are silent.

## Voice Management

### R-5.1.4 Voice Management and Priority System

The engine **SHALL** manage a fixed pool of voices with priority-based allocation using four
priority classes (critical, high, medium, low), virtualize the lowest-audibility voices when
the pool is exhausted, and restore virtualized voices within one audio buffer period when
headroom returns.

- **Derived from:** [F-5.1.4](../../features/audio/audio-engine.md)
- **Rationale:** Strict voice budgeting prevents audio overload during high-density scenarios
  such as raids with dozens of simultaneous spell effects and ambient loops.
- **Verification:** Unit test: allocate voices to fill the pool, trigger an additional
  critical-priority voice, and verify the lowest-audibility voice is virtualized. Free a
  voice and verify the virtualized voice restores playback within one buffer callback. Assert
  virtualized voices retain position and playback offset.

## Playback

### R-5.1.5 Streaming Playback

The engine **SHALL** stream long-duration audio assets from disk in ring-buffer chunks using
platform-native async I/O (IOCP on Windows, GCD on macOS, io_uring on Linux), and **SHALL**
support prefetch hinting that begins streaming before playback is triggered.

- **Derived from:** [F-5.1.5](../../features/audio/audio-engine.md)
- **Rationale:** Streaming avoids loading entire music and dialogue files into memory, and
  prefetching eliminates audible startup latency for cinematic cues and zone transitions.
- **Verification:** Integration test: stream a 5-minute audio file and verify peak memory
  consumption stays below 256 KiB per stream. Issue a prefetch hint 500 ms before playback
  and measure startup latency is under 10 ms. Verify each platform uses its native async
  I/O API (not stdlib file I/O).

### R-5.1.6 Sample-Accurate Scheduling

The engine **SHALL** schedule sound start, stop, and parameter changes with sample-accurate
timing relative to the audio clock, executing queued commands at the precise sample offset
within the next audio buffer.

- **Derived from:** [F-5.1.6](../../features/audio/audio-engine.md)
- **Rationale:** Sample-accurate timing is required for tight synchronization between layered
  loops, musical cues, and gameplay events.
- **Verification:** Unit test: schedule two sounds to start at the same sample offset and
  verify their output buffers are phase-aligned within +/- 0 samples. Schedule a stop at a
  specific sample and verify silence begins at exactly that sample. Measure scheduling
  jitter over 1,000 commands and verify zero-sample deviation.

## Formats and Codecs

### R-5.1.7 Audio Format and Codec Support

The engine **SHALL** decode PCM (WAV), Vorbis, Opus, and FLAC formats at load or stream
time, and **SHALL** provide an extensible codec registry that accepts plugin-registered
decoders at runtime.

- **Derived from:** [F-5.1.7](../../features/audio/audio-engine.md)
- **Rationale:** Multiple codecs cover different use cases (Opus for voice, Vorbis for legacy,
  FLAC for lossless), and extensibility supports proprietary or future formats.
- **Verification:** Integration test: load and play one asset in each supported format (WAV,
  Vorbis, Opus, FLAC) and verify correct decoded output via waveform comparison against a
  reference. Register a custom codec plugin at runtime and verify it decodes a test file.
  Verify format metadata (sample rate, channel count, loop points) is extracted correctly.

---

## Non-Functional Requirements

### R-5.1.NF1 Audio Thread Budget

The engine **SHALL** complete all audio processing (mixing, DSP, spatialization) within 0.5 ms
of wall-clock time per audio buffer callback at 48 kHz / 512-sample buffers, consuming no more
than one CPU core's capacity at steady state.

- **Derived from:** [R-X.1.1](../cross-cutting.md) (Frame Time Budget — audio 0.5 ms),
  [R-X.2.1](../cross-cutting.md) (Thread Ownership — audio thread)
- **Rationale:** Audio callbacks must complete before the hardware output buffer drains or
  audible glitches (underruns) occur. The 0.5 ms budget aligns with the cross-cutting frame
  time allocation.
- **Verification:** Benchmark: run a worst-case mix (maximum voice count, full DSP chain,
  spatialization on all voices) and measure wall-clock time per callback over 10,000 buffers.
  Assert p99 latency is below 0.5 ms on the minimum-spec hardware tier.

### R-5.1.NF2 Maximum Voice Count

The engine **SHALL** support at least 256 simultaneous voices (128 real + 128 virtualized)
without exceeding the audio thread budget (R-5.1.NF1).

- **Derived from:** [F-5.1.4](../../features/audio/audio-engine.md),
  [R-X.1.1](../cross-cutting.md) (Frame Time Budget)
- **Rationale:** Large-scale scenarios (raids, city hubs) generate hundreds of concurrent sound
  sources. The voice pool must be large enough to avoid audible voice stealing under typical
  peak load.
- **Verification:** Stress test: allocate 256 voices with full spatialization and a 2-insert
  DSP chain each. Assert no audio underruns over 60 seconds and thread time stays within
  budget.

### R-5.1.NF3 Audio Memory Budget

The engine **SHALL** limit total audio system resident memory (decode buffers, mixer state,
DSP scratch, voice pool) to 64 MiB on the minimum platform tier, excluding streaming ring
buffers which are governed by per-stream caps (R-5.1.5).

- **Derived from:** [R-X.1.3](../cross-cutting.md) (System RAM Budget)
- **Rationale:** Audio must coexist with rendering, physics, and animation within constrained
  system RAM. A hard cap prevents unbounded growth from loaded sound banks.
- **Verification:** Benchmark: load the maximum sound bank configuration and measure peak
  resident audio memory via the profiling API. Assert it does not exceed 64 MiB excluding
  stream ring buffers.

### R-5.1.NF4 Mixer Output Latency

The engine **SHALL** deliver audio output with end-to-end latency (game event to speaker
output) not exceeding 20 ms at 48 kHz sample rate with the default buffer configuration.

- **Derived from:** [F-5.1.6](../../features/audio/audio-engine.md),
  [R-X.1.1](../cross-cutting.md) (Frame Time Budget)
- **Rationale:** Latency above 20 ms causes perceptible desynchronization between visual
  effects and their corresponding sound effects, breaking immersion.
- **Verification:** Integration test: trigger a sound event and measure the time until the
  first non-zero sample appears in the audio output callback. Assert latency is below 20 ms
  across all supported platforms.
