# R-5.1 -- Audio Engine Requirements

## Sound Sources

1. **R-5.1.1** — The engine **SHALL** provide an ECS component for sound emission with point, line,
   and area emitter variants, each storing gain (linear 0.0-1.0), pitch multiplier, looping flag,
   and an attenuation curve reference. Per-entity storage **SHALL NOT** exceed 128 bytes.
   - **Rationale:** A lightweight ECS component enables hundreds of simultaneous ambient emitters
     without per-entity overhead degrading frame time.
   - **Verification:** Unit test: create 1,000 entities with sound source components of each emitter
     type and assert per-entity storage does not exceed 128 bytes. Integration test: verify audio
     output matches configured gain, pitch, and looping settings.

2. **R-5.1.1a** — The engine **SHALL** support point, line, and area emitter types per sound source
   component, where each type defines a distinct spatial emission pattern. The emitter type
   **SHALL** be selectable at authoring time and changeable at runtime without despawning the
   entity.
   - **Rationale:** Different world objects require different emission shapes: torches from a point,
     rivers from a line, and open fields from an area.
   - **Verification:** Unit test: set emitter type to point, line, and area on the same entity and
     verify each produces the expected spatial distribution. Verify runtime type change causes no
     audio discontinuity.

## Listener

3. **R-5.1.2** — The engine **SHALL** provide an ECS component that designates entities as audio
   listeners with position, orientation, and velocity. The engine **SHALL** support multiple
   simultaneous listeners for split-screen. When no explicit listener exists, the engine **SHALL**
   fall back to the active camera entity.
   - **Rationale:** Spatialization requires a reference perspective; split-screen requires
     independent perspectives; a camera fallback prevents silent audio when no listener is
     configured.
   - **Verification:** Unit test: remove all listeners and verify the active camera entity is used.
     Integration test: assign separate listeners for two split-screen players and verify each hears
     audio from their own perspective.

## Mixer Bus Hierarchy

4. **R-5.1.3** — The engine **SHALL** provide a directed acyclic graph of mixer buses with gain,
   mute, and solo controls per bus. Child buses **SHALL** inherit parent gain. Each bus **SHALL**
   support an ordered chain of insert effects using enum dispatch (no trait objects). Buses
   **SHALL** be addable and rewirable at runtime without audio discontinuity.
   - **Rationale:** Hierarchical mixing enables independent volume control for music, SFX, ambient,
     voice, and UI. Runtime rewiring supports dynamic mix states such as underwater ducking.
   - **Verification:** Unit test: set master gain to 0.5 and verify all descendant bus outputs are
     halved. Integration test: rewire a bus at runtime and verify no audible click or gap.

5. **R-5.1.3a** — The engine **SHALL** dispatch insert effects on mixer buses via enum-based static
   dispatch, not dynamic trait objects. Custom DSP nodes **SHALL** be registered via a node registry
   with stateless process callbacks.
   - **Rationale:** Enum dispatch avoids vtable indirection on the real-time audio thread, enabling
     the compiler to inline effect processing.
   - **Verification:** Inspect generated assembly to confirm no vtable lookups in the mixer bus
     processing path. Benchmark: compare enum dispatch vs trait object dispatch for a 4-insert
     chain.

## Voice Management

6. **R-5.1.4** — The engine **SHALL** manage a fixed pool of voices with priority-based allocation.
   Each sound source **SHALL** declare a priority class (critical, high, medium, low). When the pool
   is exhausted, the engine **SHALL** virtualize the lowest-audibility voice and restore it
   seamlessly within one buffer callback when headroom returns.
   - **Rationale:** Hardware voice budgets are finite; priority stealing ensures critical gameplay
     sounds always play even in chaotic scenes.
   - **Verification:** Stress test: fill the voice pool, trigger a critical-priority voice, and
     assert the lowest-audibility voice is virtualized. Free a voice and verify restoration within
     one buffer callback.

7. **R-5.1.4a** — The engine **SHALL** support configurable voice pool sizes per platform tier:
   16-32 on mobile, 32-64 on Switch, 128-256 on desktop. Pool size **SHALL** be configurable at
   engine initialization.
   - **Rationale:** Mobile and handheld platforms have fewer hardware mixing resources; per-tier
     sizing prevents exceeding platform audio budgets.
   - **Verification:** Unit test: initialize voice pools at each tier's minimum and maximum sizes
     and verify the pool accepts exactly the configured number of voices.

## Streaming Playback

8. **R-5.1.5** — The engine **SHALL** stream long-duration audio from disk in ring-buffer chunks
   using Tokio async I/O. The engine **SHALL NOT** use C++ stdlib file I/O. Peak memory per stream
   **SHALL NOT** exceed 256 KiB.
   - **Rationale:** Holding entire files in memory is infeasible for large asset libraries. Tokio
     async I/O avoids blocking the audio thread.
   - **Verification:** Integration test: stream a 5-minute file and assert peak memory stays below
     256 KiB per stream. Verify Tokio async I/O usage via instrumentation.

9. **R-5.1.5a** — The engine **SHALL** support prefetch hints that begin streaming audio before
   playback is triggered. Prefetch at least 500 ms before playback **SHALL** reduce startup latency
   to under 10 ms.
   - **Rationale:** Cinematic cues and zone transitions require instant audio start; prefetching
     eliminates audible delays.
   - **Verification:** Integration test: issue a prefetch hint 500 ms before playback and assert
     startup latency is under 10 ms.

## Sample-Accurate Scheduling

10. **R-5.1.6** — The engine **SHALL** schedule sound start, stop, and parameter changes at precise
    sample offsets within the next audio buffer. Two sounds at the same offset **SHALL** produce
    phase-aligned output with zero-sample deviation. The command queue **SHALL** be lock-free (SPSC
    ring buffer) between game and audio threads.
    - **Rationale:** Layered loops and musical cues require sub-sample synchronization. Lock-free
      communication prevents priority inversion on the real-time audio thread.
    - **Verification:** Unit test: schedule two sounds at the same sample offset and verify
      phase-aligned output within +/- 0 samples. Measure jitter over 1,000 commands.

## Codecs

11. **R-5.1.7** — The engine **SHALL** decode PCM (WAV), Vorbis, Opus, and FLAC formats. Format
    metadata (sample rate, channel count, loop points) **SHALL** be extracted during import and
    cached. The codec registry **SHALL** support runtime plugin registration without engine
    recompilation.
    - **Rationale:** Different asset types benefit from different codecs. Plugin extensibility
      supports proprietary formats.
    - **Verification:** Integration test: load and play one asset in each format and verify decoded
      output matches a reference waveform. Verify metadata extraction returns correct values for
      each format.

## Non-Functional Requirements

12. **R-5.1.NF1** — The engine **SHALL** complete all audio processing (mixing, DSP, spatialization)
    within 0.5 ms per buffer callback at 48 kHz / 512-sample buffers, consuming no more than one CPU
    core at steady state.
    - **Rationale:** Audio callbacks must complete before the hardware output buffer drains or
      audible glitches occur.
    - **Verification:** Benchmark: worst-case mix (maximum voice count, full DSP chain,
      spatialization on all voices) over 10,000 buffers. Assert p99 latency below 0.5 ms.

13. **R-5.1.NF2** — The engine **SHALL** support at least 256 simultaneous voices (128 real + 128
    virtualized) without exceeding the audio thread budget.
    - **Rationale:** Large-scale scenarios generate hundreds of concurrent sound sources.
    - **Verification:** Stress test: allocate 256 voices with spatialization and 2-insert DSP
      chains. Assert no audio underruns over 60 seconds.

14. **R-5.1.NF3** — The engine **SHALL** limit total audio system resident memory to 64 MiB on the
    minimum platform tier, excluding streaming ring buffers.
    - **Rationale:** Audio must coexist with rendering, physics, and animation within constrained
      system RAM.
    - **Verification:** Benchmark: load the maximum sound bank and measure peak resident audio
      memory. Assert it does not exceed 64 MiB.

15. **R-5.1.NF4** — The engine **SHALL** deliver audio output with end-to-end latency not exceeding
    20 ms at 48 kHz.
    - **Rationale:** Latency above 20 ms causes perceptible desynchronization between visuals and
      sound.
    - **Verification:** Integration test: trigger a sound event and measure time until first
      non-zero sample. Assert below 20 ms.
