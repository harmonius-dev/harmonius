# R-5.1 -- Audio Engine Requirements

## Sound Sources

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.1 | The engine **SHALL** provide an ECS component for sound emission with point, line, and area emitter variants, each storing gain (linear 0.0-1.0), pitch multiplier, looping flag, and an attenuation curve reference. Per-entity storage for the component **SHALL NOT** exceed 128 bytes. | [F-5.1.1](../../features/audio/audio-engine.md) | A lightweight ECS component enables hundreds of simultaneous ambient emitters (campfires, waterfalls, machinery) without per-entity overhead degrading frame time. | Unit test: create 1,000 entities with sound source components of each emitter type and assert per-entity storage does not exceed 128 bytes. Integration test: attach a sound source to an entity and verify audio output matches configured gain, pitch, and looping settings. |
| R-5.1.1a | The engine **SHALL** support point, line, and area emitter types per sound source component, where each type defines a distinct spatial emission pattern. The emitter type **SHALL** be selectable at authoring time and changeable at runtime without despawning the entity. | [F-5.1.1](../../features/audio/audio-engine.md) | Different world objects require different emission shapes: torches emit from a point, rivers from a line, and open fields from an area. | Unit test: set emitter type to point, line, and area on the same entity and verify each produces the expected spatial distribution. Verify runtime type change does not cause audio discontinuity. |

## Listener

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.2 | The engine **SHALL** provide an ECS component that designates entities as audio listeners with position, orientation, and velocity. The engine **SHALL** support multiple simultaneous listeners for split-screen. When no explicit listener exists, the engine **SHALL** fall back to the active camera entity. | [F-5.1.2](../../features/audio/audio-engine.md) | Spatialization requires a reference perspective; split-screen requires independent perspectives per player; a camera fallback prevents silent audio when no listener is configured. | Unit test: remove all explicit listeners and verify the active camera entity is used. Integration test: assign separate listeners for two split-screen players and verify each hears audio from their own perspective. Verify listener velocity produces correct Doppler pitch shifts. |

## Mixer Bus Hierarchy

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.3 | The engine **SHALL** provide a directed acyclic graph of mixer buses with gain, mute, and solo controls per bus. Child buses **SHALL** inherit parent gain. Each bus **SHALL** support an ordered chain of insert effects using enum dispatch (no trait objects). Buses **SHALL** be addable and rewirable at runtime without audio discontinuity. | [F-5.1.3](../../features/audio/audio-engine.md) | Hierarchical mixing enables independent volume control for music, SFX, ambient, voice, and UI categories. Runtime rewiring supports dynamic mix states such as underwater ducking or pause menus. | Unit test: set master gain to 0.5 and verify all descendant bus outputs are halved. Mute a mid-level bus and verify all children are silent. Integration test: rewire a bus at runtime and verify no audible click or gap during the transition. |
| R-5.1.3a | The engine **SHALL** dispatch insert effects on mixer buses via enum-based static dispatch, not dynamic trait objects. Each built-in effect type (filter, EQ, reverb, compressor, limiter, delay, chorus, flanger, pitch shifter) **SHALL** be a variant of a single effect enum. Custom DSP nodes **SHALL** be registered via a node registry with stateless process callbacks. | [F-5.1.3](../../features/audio/audio-engine.md), [F-5.3.8](../../features/audio/dsp-and-effects.md) | Enum dispatch avoids vtable indirection on the real-time audio thread, enabling the compiler to inline effect processing and eliminate branch misprediction overhead in tight DSP loops. | Inspect generated assembly to confirm no vtable lookups in the mixer bus processing path. Benchmark: compare enum dispatch vs trait object dispatch for a 4-insert chain and verify enum dispatch is faster. |

## Voice Management

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.4 | The engine **SHALL** manage a fixed pool of voices with priority-based allocation. Each sound source **SHALL** declare a priority class (critical, high, medium, low). When the voice pool is exhausted, the engine **SHALL** virtualize the lowest-audibility voice (tracked but silent) and restore it seamlessly within one buffer callback when headroom returns. | [F-5.1.4](../../features/audio/audio-engine.md) | Hardware voice budgets are finite; priority stealing ensures critical gameplay sounds (alerts, dialogue) always play even in chaotic scenes with hundreds of concurrent emitters. | Stress test: fill the voice pool, trigger a critical-priority voice, and assert the lowest-audibility voice is virtualized. Free a voice and verify the virtualized voice restores playback within one buffer callback with correct position and offset. |
| R-5.1.4a | The engine **SHALL** support configurable voice pool sizes per platform tier: 16-32 voices on mobile, 32-64 on Switch, and 128-256 on desktop. The voice pool size **SHALL** be configurable at engine initialization. | [F-5.1.4](../../features/audio/audio-engine.md) | Mobile and handheld platforms have fewer hardware mixing resources; per-tier sizing prevents exceeding platform audio budgets. | Unit test: initialize voice pools at each tier's minimum and maximum sizes and verify the pool accepts exactly the configured number of voices. |

## Streaming Playback

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.5 | The engine **SHALL** stream long-duration audio assets from disk in ring-buffer chunks using platform-native async I/O (IOCP on Windows, GCD Dispatch IO on macOS, io_uring on Linux). The engine **SHALL NOT** use C++ stdlib file I/O for audio streaming. Peak memory per stream **SHALL NOT** exceed 256 KiB. | [F-5.1.5](../../features/audio/audio-engine.md) | Holding entire music or dialogue files in memory is infeasible for large asset libraries. Platform-native async I/O avoids blocking the audio thread and maximizes throughput. | Integration test: stream a 5-minute audio file and assert peak memory stays below 256 KiB per stream. Verify streaming uses the correct platform-native API via platform instrumentation. |
| R-5.1.5a | The engine **SHALL** support prefetch hints that begin streaming audio data before playback is triggered. Issuing a prefetch hint at least 500 ms before playback **SHALL** reduce startup latency to under 10 ms. | [F-5.1.5](../../features/audio/audio-engine.md) | Cinematic cues and zone transitions require instant audio start; prefetching eliminates audible startup delays. | Integration test: issue a prefetch hint 500 ms before playback and assert startup latency is under 10 ms. Verify playback without prefetch has measurably higher startup latency. |

## Sample-Accurate Scheduling

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.6 | The engine **SHALL** schedule sound start, stop, and parameter changes at precise sample offsets within the next audio buffer. Two sounds scheduled at the same sample offset **SHALL** produce phase-aligned output with zero-sample deviation. The command queue **SHALL** be lock-free (SPSC ring buffer) between game thread and audio thread. | [F-5.1.6](../../features/audio/audio-engine.md) | Layered loops and musical cues require sub-sample synchronization. Lock-free communication prevents priority inversion on the real-time audio thread. | Unit test: schedule two sounds at the same sample offset and verify their output buffers are phase-aligned within +/- 0 samples. Measure scheduling jitter over 1,000 commands and verify zero-sample deviation. |

## Codecs

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.7 | The engine **SHALL** decode PCM (WAV), Vorbis, Opus, and FLAC formats. Format metadata (sample rate, channel count, loop points) **SHALL** be extracted during asset import and cached. The codec registry **SHALL** support runtime registration of custom codec plugins without engine recompilation. | [F-5.1.7](../../features/audio/audio-engine.md) | Different asset types benefit from different codecs: Opus for voice chat, Vorbis for legacy assets, FLAC for lossless reference audio. Plugin extensibility supports proprietary formats. | Integration test: load and play one asset in each format and verify decoded output matches a reference waveform. Verify metadata extraction returns correct sample rate, channel count, and loop points for each format. |

## Non-Functional Requirements

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-5.1.NF1 | The engine **SHALL** complete all audio processing (mixing, DSP, spatialization) within 0.5 ms of wall-clock time per audio buffer callback at 48 kHz / 512-sample buffers, consuming no more than one CPU core's capacity at steady state. | Cross-cutting frame time budget | Audio callbacks must complete before the hardware output buffer drains or audible glitches (underruns) occur. | Benchmark: run a worst-case mix (maximum voice count, full DSP chain, spatialization on all voices) and measure wall-clock time per callback over 10,000 buffers. Assert p99 latency is below 0.5 ms on the minimum-spec hardware tier. |
| R-5.1.NF2 | The engine **SHALL** support at least 256 simultaneous voices (128 real + 128 virtualized) without exceeding the audio thread budget. | F-5.1.4 | Large-scale scenarios generate hundreds of concurrent sound sources. The voice pool must handle typical peak load. | Stress test: allocate 256 voices with full spatialization and a 2-insert DSP chain each. Assert no audio underruns over 60 seconds. |
| R-5.1.NF3 | The engine **SHALL** limit total audio system resident memory to 64 MiB on the minimum platform tier, excluding streaming ring buffers. | Cross-cutting system RAM budget | Audio must coexist with rendering, physics, and animation within constrained system RAM. | Benchmark: load the maximum sound bank configuration and measure peak resident audio memory. Assert it does not exceed 64 MiB. |
| R-5.1.NF4 | The engine **SHALL** deliver audio output with end-to-end latency not exceeding 20 ms at 48 kHz sample rate. | F-5.1.6 | Latency above 20 ms causes perceptible desynchronization between visual effects and sound. | Integration test: trigger a sound event and measure time until first non-zero sample in output. Assert latency is below 20 ms. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/audio/audio-engine.md](../../user-stories/audio/audio-engine.md).
